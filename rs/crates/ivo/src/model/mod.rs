mod internal;

use futures::future::{join_all, BoxFuture};
use futures::FutureExt;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::future::ready;
use std::sync::Arc;

use crate::model::internal::{FieldInfo, FieldInfoCollection};
use crate::schema::error_tool::{DefaultErrorTool, FieldError, IvoErrorTool, UpdateError};
use crate::schema::fields::base::{FieldType, InternalFieldConfig};
use crate::schema::fields::types::{
    ComputableRequiredError, IsFieldProvisionEnabled, ValueResolverWithMiniContext,
};
use crate::schema::fields::TimestampConfig;
use crate::schema::Schema;

use crate::schema::options::types::{OnSuccessConfig, PostValidationConfig};

use crate::{
    IvoContext, SharedCtxOptions, SharedIvoContext, SharedIvoMiniContext, SharedRwCtxOptions,
};

use crate::types::{erase_value, IvoSchemaStruct, IvoStructPartialMethods, RwLock};

type AsyncHandlerTrigger<'a> = Box<dyn Fn() -> BoxFuture<'a, ()> + Send + Sync + 'a>;

impl<
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Sync + Send,
        Timestamp: Clone + Debug + Send + Sync + 'static,
        ErrorTool: IvoErrorTool,
    > Schema<I, O, CtxOptions, Timestamp, ErrorTool>
{
    pub fn get_model(&self) -> Model<'_, I, O, CtxOptions, Timestamp, ErrorTool> {
        Model { schema: self }
    }
}

pub struct Model<
    'schema,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct = I,
    CtxOptions: Sync + Send = HashMap<String, ()>,
    Timestamp: Clone + Debug + Send + Sync + 'static = (),
    ErrorTool: IvoErrorTool = DefaultErrorTool,
> {
    schema: &'schema Schema<I, O, CtxOptions, Timestamp, ErrorTool>,
}

impl<
        'schema,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Sync + Send,
        Timestamp: Clone + Debug + Send + Sync + 'static,
        ErrorTool: IvoErrorTool,
    > Model<'schema, I, O, CtxOptions, Timestamp, ErrorTool>
{
    pub async fn create(
        &self,
        input: &I::Partial,
        options: CtxOptions,
    ) -> Result<
        (O, AsyncHandlerTrigger<'schema>),
        (ErrorTool::ErrorPayload, AsyncHandlerTrigger<'schema>),
    > {
        let shared_rw_options = Arc::new(RwLock::new(options));

        // 1) Resolve constants & defaults
        let default_values = self
            .resolve_constants_and_defaults(Arc::new(input.clone()), Arc::clone(&shared_rw_options))
            .await;

        let mut ctx = Arc::new(IvoContext::<I, O>::new_create_ctx(
            input.clone(),
            input.clone(),
            default_values,
        ));

        let (input, fields_provided) = self
            .filter_input_fields_allowed(
                None,
                input,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        Arc::make_mut(&mut ctx).set_input(input);

        let r = self
            .evaluate_missing_required_fields(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if r.is_err() {
            return Err((
                r.err().unwrap(),
                self.prepare_failure_handlers(
                    fields_provided.fields,
                    ctx,
                    Arc::new(unwrap_async_lock(shared_rw_options)),
                ),
            ));
        }

        // 3) Run validators
        let r = self
            .validate(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if r.is_err() {
            return Err((
                r.err().unwrap(),
                self.prepare_failure_handlers(
                    fields_provided.fields,
                    ctx,
                    Arc::new(unwrap_async_lock(shared_rw_options)),
                ),
            ));
        }

        let (validated_inputs, validated_outputs, should_update_ctx) = r.ok().unwrap();

        if should_update_ctx {
            Arc::make_mut(&mut ctx)
                .set_input(validated_inputs)
                .set_changes(validated_outputs);
        }

        // 4) Run re_validators
        let r = self
            .re_validate(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if r.is_err() {
            return Err((
                r.err().unwrap(),
                self.prepare_failure_handlers(
                    fields_provided.fields,
                    ctx,
                    Arc::new(unwrap_async_lock(shared_rw_options)),
                ),
            ));
        }

        let (validated_inputs, validated_outputs, should_update_ctx) = r.ok().unwrap();

        if should_update_ctx {
            Arc::make_mut(&mut ctx)
                .set_input(validated_inputs)
                .set_changes(validated_outputs);
        }

        // 5) Run post-validators
        let r = self
            .post_validate(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if r.is_err() {
            return Err((
                r.err().unwrap(),
                self.prepare_failure_handlers(
                    fields_provided.fields,
                    ctx,
                    Arc::new(unwrap_async_lock(shared_rw_options)),
                ),
            ));
        }

        let (validated_inputs, validated_outputs, should_update_ctx) = r.ok().unwrap();

        if should_update_ctx {
            Arc::make_mut(&mut ctx)
                .set_input(validated_inputs)
                .set_changes(validated_outputs);
        }

        // 6) Sanitize virtuals
        let (validated_inputs, should_update_ctx) = self
            .sanitize_virtuals(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if should_update_ctx {
            Arc::make_mut(&mut ctx).set_input(validated_inputs);
        }

        // 7) Resolve values of dependent fields
        let (mut validated_outputs, mut dependent_fields_resolved) = self
            .resolve_dependent_values(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if !dependent_fields_resolved.is_empty() {
            Arc::make_mut(&mut ctx).set_changes(validated_outputs);
        }

        while !dependent_fields_resolved.is_empty() {
            let col = FieldInfoCollection::from_fields(
                &self.schema,
                dependent_fields_resolved,
                &fields_provided.schema_input_fields,
                &fields_provided.schema_output_fields,
            );

            (validated_outputs, dependent_fields_resolved) = self
                .resolve_dependent_values(&col, Arc::clone(&ctx), Arc::clone(&shared_rw_options))
                .await;

            if !dependent_fields_resolved.is_empty() {
                Arc::make_mut(&mut ctx).set_changes(validated_outputs);
            }
        }

        // 8) Generate and set timestamps
        let (values, should_update_ctx) = self.attach_timestamps(ctx.values(), false);

        if should_update_ctx {
            Arc::make_mut(&mut ctx).set_changes(values.clone());
        }

        return Ok((
            O::ivo_internal_dangerously_get_values_from_partial(values),
            self.prepare_success_handlers(
                fields_provided,
                ctx,
                Arc::new(unwrap_async_lock(shared_rw_options)),
            ),
        ));
    }

    pub async fn update(
        &self,
        data: &O,
        updates: &I::Partial,
        options: CtxOptions,
    ) -> Result<
        (O::Partial, AsyncHandlerTrigger<'schema>),
        (UpdateError<ErrorTool>, AsyncHandlerTrigger<'schema>),
    > {
        let old_partial_values: O::Partial = data.clone().into();

        let mut ctx = Arc::new(IvoContext::<I, O>::new_update_ctx(
            O::Partial::default(),
            updates.clone(),
            updates.clone(),
            data.clone(),
            data.clone(),
        ));

        let shared_rw_options = Arc::new(RwLock::new(options));

        let (input, fields_provided) = self
            .filter_input_fields_allowed(
                Some(&old_partial_values),
                updates,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        Arc::make_mut(&mut ctx).set_input(input);

        // if the updates provided are all none, the nothing to update
        if fields_provided.fields.is_empty() {
            return Err((
                UpdateError::NothingToUpdate,
                self.prepare_failure_handlers(
                    vec![],
                    ctx,
                    Arc::new(unwrap_async_lock(shared_rw_options)),
                ),
            ));
        }

        // 1) Evaluate missing required fields
        let r = self
            .evaluate_missing_required_fields(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if r.is_err() {
            return Err((
                UpdateError::ValidationError(r.err().unwrap()),
                self.prepare_failure_handlers(
                    fields_provided.fields,
                    ctx,
                    Arc::new(unwrap_async_lock(shared_rw_options)),
                ),
            ));
        }

        // 2) Run validators
        let r = self
            .validate(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if r.is_err() {
            return Err((
                UpdateError::ValidationError(r.err().unwrap()),
                self.prepare_failure_handlers(
                    fields_provided.fields,
                    ctx,
                    Arc::new(unwrap_async_lock(shared_rw_options)),
                ),
            ));
        }

        let (validated_inputs, validated_outputs, should_update_ctx) = r.ok().unwrap();

        if should_update_ctx {
            Arc::make_mut(&mut ctx)
                .set_input(validated_inputs)
                .set_changes(validated_outputs.clone())
                .set_full_values(data.ivo_internal_clone_with(validated_outputs));
        }

        // 3) Run re_validators
        let r = self
            .re_validate(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if r.is_err() {
            return Err((
                UpdateError::ValidationError(r.err().unwrap()),
                self.prepare_failure_handlers(
                    fields_provided.fields,
                    ctx,
                    Arc::new(unwrap_async_lock(shared_rw_options)),
                ),
            ));
        }

        let (validated_inputs, validated_outputs, should_update_ctx) = r.ok().unwrap();

        if should_update_ctx {
            Arc::make_mut(&mut ctx)
                .set_input(validated_inputs)
                .set_changes(validated_outputs.clone())
                .set_full_values(data.ivo_internal_clone_with(validated_outputs));
        }

        // 4) Run post-validators
        let r = self
            .post_validate(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if r.is_err() {
            return Err((
                UpdateError::ValidationError(r.err().unwrap()),
                self.prepare_failure_handlers(
                    fields_provided.fields,
                    ctx,
                    Arc::new(unwrap_async_lock(shared_rw_options)),
                ),
            ));
        }

        let (validated_inputs, validated_outputs, should_update_ctx) = r.ok().unwrap();

        if should_update_ctx {
            Arc::make_mut(&mut ctx)
                .set_input(validated_inputs)
                .set_changes(validated_outputs.clone())
                .set_full_values(data.ivo_internal_clone_with(validated_outputs));
        }

        // 5) Sanitize virtuals
        let (validated_inputs, should_update_ctx) = self
            .sanitize_virtuals(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if should_update_ctx {
            Arc::make_mut(&mut ctx).set_input(validated_inputs);
        }

        let erased_updates = ctx.values();

        let fields_updated_vec = fields_provided
            .fields
            .iter()
            .filter_map(|f| {
                if f.is_input && !f.is_output {
                    return Some(f.clone());
                }

                if !old_partial_values.ivo_internal_is_value_equal(
                    &f.name,
                    &erased_updates.ivo_internal_get_erased_value(&f.name),
                ) {
                    return Some(f.clone());
                }

                None
            })
            .collect();

        let fields_updated = FieldInfoCollection::from_fields(
            &self.schema,
            fields_updated_vec,
            &fields_provided.schema_input_fields,
            &fields_provided.schema_output_fields,
        );

        // 6) Resolve values of dependent fields
        let (mut validated_outputs, mut dependent_fields_resolved) = self
            .resolve_dependent_values(
                &fields_updated,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if !dependent_fields_resolved.is_empty() {
            Arc::make_mut(&mut ctx)
                .set_changes(validated_outputs.clone())
                .set_full_values(data.ivo_internal_clone_with(validated_outputs));
        }

        while !dependent_fields_resolved.is_empty() {
            let col = FieldInfoCollection::from_fields(
                &self.schema,
                dependent_fields_resolved,
                &fields_provided.schema_input_fields,
                &fields_provided.schema_output_fields,
            );

            (validated_outputs, dependent_fields_resolved) = self
                .resolve_dependent_values(&col, Arc::clone(&ctx), Arc::clone(&shared_rw_options))
                .await;

            if !dependent_fields_resolved.is_empty() {
                Arc::make_mut(&mut ctx)
                    .set_changes(validated_outputs.clone())
                    .set_full_values(data.ivo_internal_clone_with(validated_outputs));
            }
        }

        let (updated_values, has_updated_fields) =
            data.ivo_internal_get_updates_from_partial(&ctx.changes());

        if !has_updated_fields {
            return Err((
                UpdateError::NothingToUpdate,
                self.prepare_failure_handlers(
                    fields_provided.fields,
                    ctx,
                    Arc::new(unwrap_async_lock(shared_rw_options)),
                ),
            ));
        }

        // 7) Generate and set timestamps
        let (updated_values, should_update_ctx) = self.attach_timestamps(updated_values, true);

        if should_update_ctx {
            Arc::make_mut(&mut ctx)
                .set_changes(updated_values.clone())
                .set_full_values(data.ivo_internal_clone_with(updated_values.clone()));
        }

        Ok((
            updated_values,
            self.prepare_success_handlers(
                fields_updated,
                ctx,
                Arc::new(unwrap_async_lock(shared_rw_options)),
            ),
        ))
    }

    pub async fn delete(&self, data: O, options: CtxOptions) {
        let data = Arc::new(data);
        let options = Arc::new(options);
        let mut handlers = vec![];

        for (_, config) in self.schema.field_configs.iter() {
            if let Some(h_vec) = &config.on_delete_fns {
                handlers.extend(h_vec);

                continue;
            }
        }

        if let Some(h_vec) = &self.schema.options.on_delete_fns {
            handlers.extend(h_vec);
        }

        if !handlers.is_empty() {
            let tasks = handlers
                .iter()
                .map(|h| h(Arc::clone(&data), Arc::clone(&options)));

            for _ in join_all(tasks).await {}
        }
    }

    async fn validate<'a>(
        &self,
        fields_provided: &'a FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> Result<(I::Partial, O::Partial, bool), ErrorTool::ErrorPayload> {
        let mut validators = Vec::with_capacity(fields_provided.fields.len());

        for field_info in fields_provided.fields.iter() {
            if let Some(InternalFieldConfig {
                validator: Some(validator),
                ..
            }) = self.schema.field_configs.get(&field_info.config_name)
            {
                validators.push((field_info, validator));
            }
        }

        if validators.is_empty() {
            return Ok((ctx.input(), ctx.values(), false));
        }

        let mut validated_inputs = ctx.input();

        let tasks = validators.into_iter().map(async |(f, validator)| {
            (
                f,
                validator(
                    validated_inputs.ivo_internal_get_erased_value(&f.name),
                    Arc::clone(&ctx),
                    Arc::clone(&options),
                )
                .await,
            )
        });

        let mut error_tool = ErrorTool::new();
        let mut validated_outputs = if ctx.is_update() {
            ctx.changes()
        } else {
            ctx.values()
        };
        let mut has_updates = false;

        for (field_info, result) in join_all(tasks).await {
            let field_name = field_info.name.clone();

            match result {
                Err((reason, metadata)) => {
                    error_tool.add(field_name.as_str(), FieldError { reason, metadata });
                }
                Ok(value) => {
                    if field_info.is_input {
                        validated_inputs.ivo_internal_set(&field_name, &value);
                        has_updates = true;
                    }

                    if field_info.is_output {
                        validated_outputs.ivo_internal_set(&field_name, &value);
                        has_updates = true;
                    }
                }
            }
        }

        if error_tool.has_errors() {
            return Err(error_tool.payload());
        }

        Ok((validated_inputs, validated_outputs, has_updates))
    }

    async fn re_validate<'a>(
        &self,
        fields_provided: &'a FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> Result<(I::Partial, O::Partial, bool), ErrorTool::ErrorPayload> {
        let mut re_validators = Vec::with_capacity(fields_provided.fields.len());

        for field_info in fields_provided.fields.iter() {
            if let Some(InternalFieldConfig {
                re_validator: Some(re_validator),
                ..
            }) = self.schema.field_configs.get(&field_info.config_name)
            {
                re_validators.push((field_info, re_validator));
            }
        }

        if re_validators.is_empty() {
            return Ok((ctx.input(), ctx.values(), false));
        }

        let mut validated_inputs = ctx.input();

        let tasks = re_validators.into_iter().map(async |(f, validator)| {
            (
                f,
                validator(
                    validated_inputs.ivo_internal_get_erased_value(&f.name),
                    Arc::clone(&ctx),
                    Arc::clone(&options),
                )
                .await,
            )
        });

        let mut error_tool = ErrorTool::new();
        let mut validated_outputs = if ctx.is_update() {
            ctx.changes()
        } else {
            ctx.values()
        };
        let mut has_updates = false;

        for (field_info, result) in join_all(tasks).await {
            let field_name = field_info.name.clone();

            match result {
                Err((reason, metadata)) => {
                    error_tool.add(field_name.as_str(), FieldError { reason, metadata });
                }
                Ok(value) => {
                    if field_info.is_input {
                        validated_inputs.ivo_internal_set(&field_name, &value);
                        has_updates = true;
                    }

                    if field_info.is_output {
                        validated_outputs.ivo_internal_set(&field_name, &value);
                        has_updates = true;
                    }
                }
            }
        }

        if error_tool.has_errors() {
            return Err(error_tool.payload());
        }

        Ok((validated_inputs, validated_outputs, has_updates))
    }

    async fn post_validate<'a>(
        &self,
        fields_provided: &'a FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> Result<(I::Partial, O::Partial, bool), ErrorTool::ErrorPayload> {
        let mut pre_validators = vec![];
        let mut post_validators = vec![];

        if let Some(configs) = &self.schema.options.post_validate {
            for PostValidationConfig {
                fields,
                pre_validator,
                validators,
            } in configs
            {
                if fields
                    .iter()
                    .any(|f| fields_provided.contains(&f.to_string()))
                {
                    if pre_validator.is_some() {
                        pre_validators.push((fields, pre_validator.as_ref().unwrap()));
                    }

                    for validator in validators {
                        post_validators.push((fields, validator));
                    }

                    continue;
                }
            }
        }

        if post_validators.is_empty() {
            return Ok((ctx.input(), ctx.values(), false));
        }

        let is_update = ctx.is_update();
        let mut ctx = ctx.clone();
        let mut error_tool = ErrorTool::new();
        let mut validated_inputs = ctx.input();
        let mut validated_outputs = if is_update {
            ctx.changes()
        } else {
            ctx.values()
        };
        let mut has_updates = false;

        if !pre_validators.is_empty() {
            let tasks = pre_validators.into_iter().map(|(fields, validator)| {
                validator(Arc::clone(&ctx), Arc::clone(&options)).map(move |r| (fields, r))
            });

            for (fields, pre_validation) in join_all(tasks).await {
                if pre_validation.is_err() {
                    for (field_name, (reason, metadata)) in pre_validation.err().unwrap() {
                        let field_name = field_name.as_str();

                        if fields.contains(&field_name) {
                            error_tool.add(field_name, FieldError { reason, metadata });
                        }
                    }

                    continue;
                }

                for (field_name, value) in pre_validation.ok().unwrap().data {
                    if let Some(field_info) = fields_provided.get(&field_name) {
                        if field_info.is_input {
                            validated_inputs.ivo_internal_set(&field_info.name, &value);
                            has_updates = true;
                        }

                        if field_info.is_output {
                            validated_outputs.ivo_internal_set(&field_info.name, &value);
                            has_updates = true;
                        }
                    }
                }
            }
        }

        if error_tool.has_errors() {
            return Err(error_tool.payload());
        }

        // update the ctx if the pre validator returned any values
        if has_updates {
            Arc::make_mut(&mut ctx)
                .set_input(validated_inputs)
                .set_changes(validated_outputs.clone());

            if let Some(values) = ctx.full_values() {
                Arc::make_mut(&mut ctx)
                    .set_full_values(values.ivo_internal_clone_with(validated_outputs));
            }
        }

        let tasks = post_validators.into_iter().map(|(fields, validator)| {
            validator(Arc::clone(&ctx), Arc::clone(&options)).map(move |r| (fields, r))
        });

        let mut validated_inputs = ctx.input();
        let mut validated_outputs = if is_update {
            ctx.changes()
        } else {
            ctx.values()
        };
        let mut has_updates = has_updates;

        for (fields, validation) in join_all(tasks).await {
            if validation.is_err() {
                for (field_name, (reason, metadata)) in validation.err().unwrap() {
                    let field_name = field_name.as_str();

                    if fields.contains(&field_name) {
                        error_tool.add(field_name, FieldError { reason, metadata });
                    }
                }

                continue;
            }

            for (field_name, value) in validation.ok().unwrap().data {
                if let Some(field_info) = fields_provided.get(&field_name) {
                    if field_info.is_input {
                        validated_inputs.ivo_internal_set(&field_info.name, &value);
                        has_updates = true;
                    }

                    if field_info.is_output {
                        validated_outputs.ivo_internal_set(&field_info.name, &value);
                        has_updates = true;
                    }
                }
            }
        }

        if error_tool.has_errors() {
            return Err(error_tool.payload());
        }

        Ok((validated_inputs, validated_outputs, has_updates))
    }

    async fn sanitize_virtuals<'a>(
        &self,
        fields_provided: &'a FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> (I::Partial, bool) {
        let mut sanitizers = Vec::with_capacity(fields_provided.fields.len());

        for field_info in fields_provided.fields.iter() {
            if let Some(InternalFieldConfig {
                field_type: FieldType::Virtual,
                sanitizer: Some(sanitizer),
                ..
            }) = self.schema.field_configs.get(&field_info.config_name)
            {
                sanitizers.push((field_info, sanitizer));
            }
        }

        if sanitizers.is_empty() {
            return (ctx.input(), false);
        }

        let input_values = ctx.input();

        let tasks = sanitizers.into_iter().map(async |(field_info, sanitizer)| {
            (
                field_info,
                sanitizer(
                    input_values.ivo_internal_get_erased_value(&field_info.name),
                    Arc::clone(&ctx),
                    Arc::clone(&options),
                )
                .await,
            )
        });

        let mut validated_inputs = ctx.input();
        let mut has_updates = false;

        for (f, value) in join_all(tasks).await {
            validated_inputs.ivo_internal_set(&f.name, &value);
            has_updates = true;
        }

        (validated_inputs, has_updates)
    }

    async fn resolve_dependent_values<'a>(
        &self,
        fields_changed: &'a FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> (O::Partial, Vec<FieldInfo>) {
        let mut resolvers = vec![];
        let previous_values: Option<O::Partial> = ctx.previous_values().map(|v| v.into());
        let is_update = previous_values.as_ref().is_some();
        let previous_values = previous_values.unwrap_or_default();

        for (field_name, config) in self.schema.field_configs.iter() {
            match config {
                InternalFieldConfig {
                    field_type: FieldType::Dependent,
                    default: Some(ValueResolverWithMiniContext::Static(default_value)),
                    should_update: Some(IsFieldProvisionEnabled::Readonly),
                    ..
                } => {
                    // readonly means: don't update if value has changed
                    // i.e: prev_value != default_value
                    if is_update
                        && !previous_values.ivo_internal_is_value_equal(&field_name, default_value)
                    {
                        continue;
                    }
                }
                InternalFieldConfig {
                    field_type: FieldType::Dependent,
                    depends_on,
                    resolver,
                    ..
                } => {
                    if depends_on
                        .as_ref()
                        .unwrap()
                        .iter()
                        .any(|parent| fields_changed.contains(&parent.to_string()))
                    {
                        resolvers.push((field_name, resolver.as_ref().unwrap()));
                    }
                }
                _ => {}
            }
        }

        if resolvers.is_empty() {
            return (ctx.values(), vec![]);
        }

        let tasks = resolvers.into_iter().map(async |(field_info, resolver)| {
            (
                field_info,
                resolver(Arc::clone(&ctx), Arc::clone(&options)).await,
            )
        });

        let values = ctx.values();
        let mut updated_values = values.clone();
        let mut fields_updated = vec![];

        for (field_name, value) in join_all(tasks).await {
            // only keep fields that have been updated
            if !values.ivo_internal_is_value_equal(&field_name, &value) {
                updated_values.ivo_internal_set(field_name, &value);

                fields_updated.push(FieldInfo {
                    config_name: field_name.clone(),
                    is_input: false,
                    is_output: true,
                    name: field_name.clone(),
                });
            }
        }

        (updated_values, fields_updated)
    }

    async fn resolve_constants_and_defaults(
        &self,
        mini_ctx: SharedIvoMiniContext<I>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> O::Partial {
        let mut default_values = O::Partial::default();
        let mut resolvers = vec![];

        for (field_name, config) in self.schema.field_configs.iter() {
            // constants
            match &config.value {
                Some(ValueResolverWithMiniContext::Static(value)) => {
                    default_values.ivo_internal_set(field_name, value);
                    continue;
                }
                Some(ValueResolverWithMiniContext::Func(resolver)) => {
                    resolvers.push((field_name.to_string(), resolver));
                    continue;
                }
                _ => {}
            }

            // other fields with default values/resolvers
            match &config.default {
                Some(ValueResolverWithMiniContext::Static(value)) => {
                    default_values.ivo_internal_set(field_name, value);
                }
                Some(ValueResolverWithMiniContext::Func(resolver)) => {
                    resolvers.push((field_name.to_string(), resolver));
                }
                _ => {}
            }
        }

        if resolvers.is_empty() {
            return default_values;
        }

        let tasks = resolvers.into_iter().map(async |(field_name, resolver)| {
            (
                field_name.clone(),
                resolver(Arc::clone(&mini_ctx), Arc::clone(&options)).await,
            )
        });

        for (field_name, value) in join_all(tasks).await {
            default_values.ivo_internal_set(&field_name, &value);
        }

        default_values
    }

    async fn filter_input_fields_allowed<'a>(
        &'a self,
        previous_values: Option<&O::Partial>,
        input_values: &I::Partial,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> (
        I::Partial,
        FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>,
    ) {
        let is_update = previous_values.is_some();
        let previous_values = previous_values.cloned().unwrap_or_default();

        let mut resolvers = vec![];
        let mut input = input_values.clone();
        let mut fields_provided = FieldInfoCollection::new(&self.schema);
        let mut field_info_vec = vec![];

        if is_update {
            for (field_name, value) in input_values.ivo_internal_to_erased_tuples() {
                let field_info = fields_provided.get(&field_name).unwrap();

                if (field_info.is_input && !field_info.is_output)
                    || !previous_values.ivo_internal_is_value_equal(&field_name, &value)
                {
                    field_info_vec.push(field_info);
                }
            }
        } else {
            for field_name in input_values.ivo_internal_fields_provided() {
                field_info_vec.push(fields_provided.get(&field_name).unwrap());
            }
        }

        let mut final_field_info_vec = vec![];

        for field_info in field_info_vec.iter() {
            match self
                .schema
                .field_configs
                .get(&field_info.config_name)
                .as_ref()
                .unwrap()
            {
                InternalFieldConfig {
                    field_type: FieldType::Lax | FieldType::Required | FieldType::Virtual,
                    default,
                    should_ignore,
                    should_init,
                    should_update,
                    ..
                } => {
                    if let Some(resolver) = should_ignore {
                        resolvers.push((field_info, resolver, true));

                        continue;
                    }

                    let source = if is_update {
                        should_update
                    } else {
                        should_init
                    };

                    match source {
                        Some(IsFieldProvisionEnabled::False) => {
                            input.ivo_internal_remove_value(&field_info.name);
                        }
                        Some(IsFieldProvisionEnabled::Func(resolver)) => {
                            resolvers.push((field_info, resolver, false));
                        }
                        Some(IsFieldProvisionEnabled::Readonly) if is_update => {
                            if let Some(ValueResolverWithMiniContext::Static(value)) = default {
                                // readonly means: don't update if value has changed
                                // i.e: prev_value != default_value
                                if !previous_values
                                    .ivo_internal_is_value_equal(&field_info.name, value)
                                {
                                    input.ivo_internal_remove_value(&field_info.name);
                                }
                            }
                        }
                        _ => final_field_info_vec.push(field_info.to_owned()),
                    };
                }
                _ => final_field_info_vec.push(field_info.to_owned()),
            }
        }

        if resolvers.is_empty() {
            fields_provided.set_fields(final_field_info_vec);

            return (input, fields_provided);
        }

        let tasks = resolvers
            .into_iter()
            .map(async |(field_info, resolver, negate)| {
                (
                    field_info,
                    resolver(Arc::clone(&ctx), Arc::clone(&options))
                        .map(|r| if negate { !r } else { r })
                        .await,
                )
            });

        for (field_info, should_init) in join_all(tasks).await {
            if should_init {
                final_field_info_vec.push(field_info.to_owned());

                continue;
            }

            input.ivo_internal_remove_value(&field_info.name);
        }

        fields_provided.set_fields(final_field_info_vec);

        return (input, fields_provided);
    }

    async fn evaluate_missing_required_fields<'a>(
        &self,
        fields_provided: &'a FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> Result<(), ErrorTool::ErrorPayload> {
        let mut error_tool = ErrorTool::new();
        let mut resolvers = vec![];
        let is_update = ctx.is_update();

        for (field_name, config) in self.schema.field_configs.iter() {
            if fields_provided.contains(field_name) {
                continue;
            }

            match config {
                InternalFieldConfig {
                    field_type: FieldType::Required,
                    required_error,
                    ..
                } => {
                    if is_update {
                        continue;
                    }

                    match required_error {
                        Some(ComputableRequiredError::Static(msg)) => {
                            error_tool.add(
                                field_name,
                                FieldError {
                                    reason: msg.to_string(),
                                    metadata: None,
                                },
                            );
                        }
                        Some(ComputableRequiredError::Func(resolver)) => {
                            resolvers.push((field_name, resolver));
                        }
                        _ => {
                            error_tool.add(
                                field_name,
                                FieldError {
                                    reason: format!("\"{field_name}\" is required!"),
                                    metadata: None,
                                },
                            );
                        }
                    }

                    continue;
                }
                InternalFieldConfig {
                    field_type: FieldType::Lax | FieldType::Virtual,
                    required_fn: Some(resolver),
                    ..
                } => resolvers.push((field_name, resolver)),
                _ => (),
            }
        }

        if resolvers.is_empty() {
            if error_tool.has_errors() {
                return Err(error_tool.payload());
            }

            return Ok(());
        }

        let tasks = resolvers.into_iter().map(async |(field_name, resolver)| {
            (
                field_name,
                resolver(Arc::clone(&ctx), Arc::clone(&options)).await,
            )
        });

        for (field_name, (is_required, reason)) in join_all(tasks).await {
            if is_required {
                error_tool.add(
                    field_name,
                    FieldError {
                        reason,
                        metadata: None,
                    },
                );
            }
        }

        if error_tool.has_errors() {
            return Err(error_tool.payload());
        }

        return Ok(());
    }

    fn prepare_failure_handlers(
        &self,
        fields_provided: Vec<FieldInfo>,
        ctx: SharedIvoContext<I, O>,
        options: SharedCtxOptions<CtxOptions>,
    ) -> AsyncHandlerTrigger<'schema> {
        if fields_provided.is_empty() {
            return Box::new(|| Box::pin(ready(())));
        }

        let mut handlers = Vec::with_capacity(fields_provided.len());

        for field_info in fields_provided.iter() {
            if let Some(InternalFieldConfig {
                on_failure_fns: Some(h_vec),
                ..
            }) = self.schema.field_configs.get(&field_info.config_name)
            {
                handlers.extend(h_vec)
            }
        }

        if handlers.is_empty() {
            return Box::new(|| Box::pin(ready(())));
        }

        Box::new(move || {
            let tasks = handlers
                .iter()
                .map(|h| h(Arc::clone(&ctx), Arc::clone(&options)))
                .collect::<Vec<_>>();

            Box::pin(async { for _ in join_all(tasks).await {} })
        })
    }

    fn prepare_success_handlers<'a>(
        &self,
        fields_updated: FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>,
        ctx: SharedIvoContext<I, O>,
        options: SharedCtxOptions<CtxOptions>,
    ) -> AsyncHandlerTrigger<'schema> {
        let mut field_names = HashSet::new();

        for field_info in fields_updated.fields.iter() {
            field_names.insert(field_info.config_name.clone());
        }

        if ctx.is_update() {
            for field_name in ctx.changes().ivo_internal_fields_provided() {
                field_names.insert(field_name);
            }
        } else {
            for field_name in ctx.values().ivo_internal_fields_provided() {
                field_names.insert(field_name);
            }
        }

        let mut handlers = vec![];

        for field_name in field_names {
            if let Some(InternalFieldConfig {
                on_success_fns: Some(h_vec),
                ..
            }) = self.schema.field_configs.get(&field_name)
            {
                handlers.extend(h_vec)
            }
        }

        if let Some(configs) = &self.schema.options.on_success_fns {
            for OnSuccessConfig {
                fields,
                handlers: h_vec,
            } in configs
            {
                if fields
                    .iter()
                    .any(|f| fields_updated.contains(&f.to_string()))
                {
                    handlers.extend(h_vec);
                }
            }
        }

        if handlers.is_empty() {
            return Box::new(|| Box::pin(ready(())));
        }

        Box::new(move || {
            let tasks = handlers
                .iter()
                .map(|h| h(Arc::clone(&ctx), Arc::clone(&options)))
                .collect::<Vec<_>>();

            Box::pin(async { for _ in join_all(tasks).await {} })
        })
    }

    fn attach_timestamps(&self, mut data: O::Partial, is_update: bool) -> (O::Partial, bool) {
        let mut was_updated = false;

        if let Some(TimestampConfig {
            created_at,
            resolver,
            updated_at,
            with_optional_updated_at,
        }) = self.schema.timestamp_configs.as_ref()
        {
            let now = resolver();

            if !is_update {
                if let Some(created_at) = created_at {
                    data.ivo_internal_set(&created_at.to_string(), &erase_value(now.clone()));
                    was_updated = true;
                }
            }

            if let Some(updated_at) = updated_at {
                if *with_optional_updated_at {
                    if is_update {
                        data.ivo_internal_set(&updated_at.to_string(), &erase_value(Some(now)));
                    } else {
                        data.ivo_internal_set(
                            &updated_at.to_string(),
                            &erase_value::<Option<Timestamp>>(None),
                        );
                    }
                } else {
                    data.ivo_internal_set(&updated_at.to_string(), &erase_value(now));
                }

                was_updated = true;
            }
        }

        (data, was_updated)
    }
}

/// this is a sync alternative to: shared_rw_options.read().await.clone()
fn unwrap_async_lock<T>(lock: Arc<RwLock<T>>) -> T {
    match Arc::into_inner(lock).unwrap().try_unwrap() {
        Ok(raw_data) => raw_data,
        _ => panic!("error unwrapping shared RwLock"),
    }
}
