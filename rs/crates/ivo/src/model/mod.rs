mod internal;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::future::ready;
use std::sync::Arc;

use crate::model::internal::{FieldInfo, FieldInfoCollection};
use crate::schema::error_tool::{DefaultErrorTool, FieldError, IvoErrorTool, UpdateError};
use crate::schema::fields::base::{FieldType, InternalFieldConfig};
use crate::schema::fields::types::{ComputableRequiredError, ComputableWithMiniContext};
use crate::schema::fields::TimestampConfig;
use crate::schema::Schema;

use crate::schema::options::types::{OnSuccessConfig, PostValidationConfig};

use crate::{
    erase_value, IvoContext, SharedCtxOptions, SharedIvoContext, SharedIvoMiniContext,
    SharedRwCtxOptions,
};

use futures::future::{join_all, BoxFuture};
use futures::FutureExt;

use crate::types::{
    ErasedValue, IvoSchemaStruct, IvoStructPartialFromToErasedMap, IvoStructPartialMethods,
    Partial, PartialMapOfErasedValues, RwLock,
};

type AsyncHandlerTrigger<'a> = Box<dyn Fn() -> BoxFuture<'a, ()> + Send + Sync + 'a>;

impl<
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone + Sync + Send,
        ErrorTool: IvoErrorTool,
        Timestamp: Clone + Debug + Send + Sync + 'static,
    > Schema<I, O, CtxOptions, ErrorTool, Timestamp>
{
    pub fn get_model(&self) -> Model<'_, I, O, CtxOptions, ErrorTool, Timestamp> {
        Model { schema: self }
    }
}

pub struct Model<
    'schema,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct = I,
    CtxOptions: Clone + Sync + Send = HashMap<String, ()>,
    ErrorTool: IvoErrorTool = DefaultErrorTool,
    Timestamp: Clone + Debug + Send + Sync + 'static = (),
> {
    schema: &'schema Schema<I, O, CtxOptions, ErrorTool, Timestamp>,
}

impl<
        'schema,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone + Sync + Send,
        ErrorTool: IvoErrorTool,
        Timestamp: Clone + Debug + Send + Sync + 'static,
    > Model<'schema, I, O, CtxOptions, ErrorTool, Timestamp>
{
    pub async fn create(
        &self,
        input: &Partial<I>,
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

        let default_values =
            O::Partial::ivo_internal_from_optional_erased_map(PartialMapOfErasedValues {
                inner: default_values,
            });

        let mut ctx = Arc::new(IvoContext::<I, O>::new_create_ctx(
            input.clone(),
            input.clone(),
            default_values,
        ));

        let erased_input_values = input.ivo_internal_to_optional_erased_map();
        let fields_provided = FieldInfoCollection::new(&self.schema, &erased_input_values);

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
                &erased_input_values,
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

        let (validated_inputs, validated_outputs, should_gen_new_ctx) = r.ok().unwrap();

        if should_gen_new_ctx {
            ctx = Arc::new(IvoContext::<I, O>::new_create_ctx(
                validated_inputs,
                ctx.input_values(),
                validated_outputs,
            ));
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

        let (validated_inputs, validated_outputs, should_gen_new_ctx) = r.ok().unwrap();

        if should_gen_new_ctx {
            ctx = Arc::new(IvoContext::<I, O>::new_create_ctx(
                validated_inputs,
                ctx.input_values(),
                validated_outputs,
            ));
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

        let (validated_inputs, validated_outputs, should_gen_new_ctx) = r.ok().unwrap();

        if should_gen_new_ctx {
            ctx = Arc::new(IvoContext::<I, O>::new_create_ctx(
                validated_inputs,
                ctx.input_values(),
                validated_outputs,
            ));
        }

        // 6) Sanitize virtuals
        let (validated_inputs, validated_outputs, should_gen_new_ctx) = self
            .sanitize_virtuals(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if should_gen_new_ctx {
            ctx = Arc::new(IvoContext::<I, O>::new_create_ctx(
                validated_inputs,
                ctx.input_values(),
                validated_outputs,
            ));
        }

        // 7) Resolve values of dependent fields
        let (
            mut validated_inputs,
            mut validated_outputs,
            mut should_gen_new_ctx,
            mut dependent_fields_resolved,
        ) = self
            .resolve_dependent_values(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if should_gen_new_ctx {
            ctx = Arc::new(IvoContext::<I, O>::new_create_ctx(
                validated_inputs,
                ctx.input_values(),
                validated_outputs,
            ));
        }

        while !dependent_fields_resolved.is_empty() {
            let col = FieldInfoCollection::from_fields(
                &self.schema,
                dependent_fields_resolved,
                &fields_provided.schema_input_fields,
                &fields_provided.schema_output_fields,
            );

            (
                validated_inputs,
                validated_outputs,
                should_gen_new_ctx,
                dependent_fields_resolved,
            ) = self
                .resolve_dependent_values(&col, Arc::clone(&ctx), Arc::clone(&shared_rw_options))
                .await;

            if should_gen_new_ctx {
                ctx = Arc::new(IvoContext::<I, O>::new_create_ctx(
                    validated_inputs,
                    ctx.input_values(),
                    validated_outputs,
                ));
            }
        }

        // 8) Generate and set timestamps

        return Ok((
            O::ivo_internal_dangerously_get_values_from_partial(ctx.values()),
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
        updates: &Partial<I>,
        options: CtxOptions,
    ) -> Result<
        (Partial<O>, AsyncHandlerTrigger<'schema>),
        (UpdateError<ErrorTool>, AsyncHandlerTrigger<'schema>),
    > {
        let mut ctx = Arc::new(IvoContext::<I, O>::new_update_ctx(
            O::Partial::default(),
            updates.clone(),
            updates.clone(),
            data.clone(),
            data.clone(),
        ));

        let old_partial_values: O::Partial = data.clone().into();
        let erased_input_values = updates.ivo_internal_to_optional_erased_map();
        let mut fields_provided = FieldInfoCollection::new(&self.schema, &erased_input_values);
        let mut updated_fields_vec = Vec::with_capacity(fields_provided.fields.len());
        let mut erased_updates = PartialMapOfErasedValues::new();

        for (field_name, value) in erased_input_values.inner.iter() {
            let f = fields_provided.get(field_name).unwrap();

            if (f.is_input && !f.is_output)
                || !old_partial_values.ivo_internal_is_value_equal(field_name, value)
            {
                updated_fields_vec.push(f.clone());

                erased_updates
                    .inner
                    .insert(field_name.to_owned(), value.clone());
            }
        }

        drop(erased_input_values);

        // if the updates provided are all none, the nothing to update
        if updated_fields_vec.is_empty() {
            return Err((
                UpdateError::NothingToUpdate,
                self.prepare_failure_handlers(Vec::with_capacity(0), ctx, Arc::new(options)),
            ));
        }

        fields_provided.set_fields(updated_fields_vec);
        let shared_rw_options = Arc::new(RwLock::new(options));

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
                    Arc::new((shared_rw_options).read().await.clone()),
                ),
            ));
        }

        // 2) Run validators
        let r = self
            .validate(
                &fields_provided,
                &erased_updates,
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

        let (validated_inputs, validated_outputs, should_gen_new_ctx) = r.ok().unwrap();

        if should_gen_new_ctx {
            ctx = Arc::new(IvoContext::<I, O>::new_update_ctx(
                validated_outputs.clone(),
                validated_inputs,
                ctx.input_values(),
                data.clone(),
                data.ivo_internal_clone_with(validated_outputs),
            ));
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

        let (validated_inputs, validated_outputs, should_gen_new_ctx) = r.ok().unwrap();

        if should_gen_new_ctx {
            ctx = Arc::new(IvoContext::<I, O>::new_update_ctx(
                validated_outputs.clone(),
                validated_inputs,
                ctx.input_values(),
                data.clone(),
                data.ivo_internal_clone_with(validated_outputs),
            ));
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

        let (validated_inputs, validated_outputs, should_gen_new_ctx) = r.ok().unwrap();

        if should_gen_new_ctx {
            ctx = Arc::new(IvoContext::<I, O>::new_update_ctx(
                validated_outputs.clone(),
                validated_inputs,
                ctx.input_values(),
                data.clone(),
                data.ivo_internal_clone_with(validated_outputs),
            ));
        }

        // 5) Sanitize virtuals
        let (validated_inputs, validated_outputs, should_gen_new_ctx) = self
            .sanitize_virtuals(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if should_gen_new_ctx {
            ctx = Arc::new(IvoContext::<I, O>::new_update_ctx(
                validated_outputs.clone(),
                validated_inputs,
                ctx.input_values(),
                data.clone(),
                data.ivo_internal_clone_with(validated_outputs),
            ));
        }

        let erased_updates = ctx.values().ivo_internal_to_optional_erased_map();

        let fields_updated_vec = fields_provided
            .fields
            .iter()
            .filter_map(|f| {
                if f.is_input && !f.is_output {
                    return Some(f.clone());
                }

                if !old_partial_values.ivo_internal_is_value_equal(
                    &f.name,
                    &erased_updates.inner.get(&f.name).unwrap(),
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
        let (
            mut validated_inputs,
            mut validated_outputs,
            mut should_gen_new_ctx,
            mut dependent_fields_resolved,
        ) = self
            .resolve_dependent_values(
                &fields_updated,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if should_gen_new_ctx {
            ctx = Arc::new(IvoContext::<I, O>::new_update_ctx(
                validated_outputs.clone(),
                validated_inputs,
                ctx.input_values(),
                data.clone(),
                data.ivo_internal_clone_with(validated_outputs),
            ));
        }

        while !dependent_fields_resolved.is_empty() {
            let col = FieldInfoCollection::from_fields(
                &self.schema,
                dependent_fields_resolved,
                &fields_provided.schema_input_fields,
                &fields_provided.schema_output_fields,
            );

            (
                validated_inputs,
                validated_outputs,
                should_gen_new_ctx,
                dependent_fields_resolved,
            ) = self
                .resolve_dependent_values(&col, Arc::clone(&ctx), Arc::clone(&shared_rw_options))
                .await;

            if should_gen_new_ctx {
                ctx = Arc::new(IvoContext::<I, O>::new_update_ctx(
                    validated_outputs.clone(),
                    validated_inputs,
                    ctx.input_values(),
                    data.clone(),
                    data.ivo_internal_clone_with(validated_outputs),
                ));
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
        fields_provided: &'a FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool, Timestamp>,
        erased_input_values: &PartialMapOfErasedValues,
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

        let tasks = validators.into_iter().map(async |(f, validator)| {
            (
                f,
                validator(
                    erased_input_values.inner.get(&f.name).cloned().unwrap(),
                    Arc::clone(&ctx),
                    Arc::clone(&options),
                )
                .await,
            )
        });

        let mut error_tool = ErrorTool::new();
        let mut validated_outputs = HashMap::new();
        let mut validated_inputs = HashMap::new();

        for (f, result) in join_all(tasks).await {
            let field_name = f.name.clone();

            match result {
                Err((reason, metadata)) => {
                    error_tool.add(field_name.as_str(), FieldError { reason, metadata });
                }
                Ok(value) => {
                    if f.is_input {
                        validated_inputs.insert(field_name.clone(), value.clone());
                    }

                    if f.is_output {
                        validated_outputs.insert(field_name, value);
                    }
                }
            }
        }

        if error_tool.has_errors() {
            return Err(error_tool.payload());
        }

        Ok(self.extract_updated_ctx_values(ctx, validated_inputs, validated_outputs))
    }

    async fn re_validate<'a>(
        &self,
        fields_provided: &'a FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool, Timestamp>,
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

        let erased_input_values = ctx.input().ivo_internal_to_optional_erased_map();

        let tasks = re_validators
            .into_iter()
            .map(async |(field_info, validator)| {
                (
                    field_info,
                    validator(
                        erased_input_values
                            .inner
                            .get(&field_info.name)
                            .cloned()
                            .unwrap(),
                        Arc::clone(&ctx),
                        Arc::clone(&options),
                    )
                    .await,
                )
            });

        let mut error_tool = ErrorTool::new();
        let mut validated_outputs = HashMap::new();
        let mut validated_inputs = HashMap::new();

        for (field_info, result) in join_all(tasks).await {
            let field_name = field_info.name.clone();

            match result {
                Err((reason, metadata)) => {
                    error_tool.add(field_name.as_str(), FieldError { reason, metadata });
                }
                Ok(value) => {
                    if field_info.is_input {
                        validated_inputs.insert(field_name.clone(), value.clone());
                    }

                    if field_info.is_output {
                        validated_outputs.insert(field_name, value);
                    }
                }
            }
        }

        if error_tool.has_errors() {
            return Err(error_tool.payload());
        }

        Ok(self.extract_updated_ctx_values(ctx, validated_inputs, validated_outputs))
    }

    async fn post_validate<'a>(
        &self,
        fields_provided: &'a FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool, Timestamp>,
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

        let mut ctx = ctx.clone();
        let mut error_tool = ErrorTool::new();
        let mut validated_outputs = HashMap::new();
        let mut validated_inputs = HashMap::new();

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
                            validated_inputs.insert(field_info.name.clone(), value.clone());
                        }

                        if field_info.is_output {
                            validated_outputs.insert(field_info.name, value);
                        }
                    }
                }
            }
        }

        if error_tool.has_errors() {
            return Err(error_tool.payload());
        }

        // update the ctx if the pre validator returned any values
        if !validated_inputs.is_empty() || !validated_outputs.is_empty() {
            let (input, changes, _) =
                self.extract_updated_ctx_values(ctx.clone(), validated_inputs, validated_outputs);

            ctx = match &*ctx {
                IvoContext::Update {
                    input_values,
                    previous_values,
                    values,
                    ..
                } => Arc::new(IvoContext::new_update_ctx(
                    changes.clone(),
                    input,
                    input_values.clone(),
                    previous_values.clone(),
                    values.ivo_internal_clone_with(changes),
                )),
                _ => Arc::new(IvoContext::new_create_ctx(
                    input,
                    ctx.input_values(),
                    changes,
                )),
            }
        }

        let tasks = post_validators.into_iter().map(|(fields, validator)| {
            validator(Arc::clone(&ctx), Arc::clone(&options)).map(move |r| (fields, r))
        });

        let mut validated_outputs = HashMap::new();
        let mut validated_inputs = HashMap::new();

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
                        validated_inputs.insert(field_info.name.clone(), value.clone());
                    }

                    if field_info.is_output {
                        validated_outputs.insert(field_info.name, value);
                    }
                }
            }
        }

        if error_tool.has_errors() {
            return Err(error_tool.payload());
        }

        Ok(self.extract_updated_ctx_values(ctx, validated_inputs, validated_outputs))
    }

    async fn sanitize_virtuals<'a>(
        &self,
        fields_provided: &'a FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool, Timestamp>,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> (I::Partial, O::Partial, bool) {
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
            return (ctx.input(), ctx.values(), false);
        }

        let erased_input_values = ctx.input().ivo_internal_to_optional_erased_map();

        let tasks = sanitizers.into_iter().map(async |(field_info, sanitizer)| {
            (
                field_info,
                sanitizer(
                    erased_input_values
                        .inner
                        .get(&field_info.name)
                        .cloned()
                        .unwrap(),
                    Arc::clone(&ctx),
                    Arc::clone(&options),
                )
                .await,
            )
        });

        let mut validated_inputs = HashMap::new();

        for (f, value) in join_all(tasks).await {
            validated_inputs.insert(f.name.clone(), value.clone());
        }

        self.extract_updated_ctx_values(ctx, validated_inputs, HashMap::new())
    }

    async fn resolve_dependent_values<'a>(
        &self,
        fields_changed: &'a FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool, Timestamp>,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> (I::Partial, O::Partial, bool, Vec<FieldInfo>) {
        let mut resolvers = vec![];

        for (field_name, config) in self.schema.field_configs.iter() {
            if let InternalFieldConfig {
                field_type: FieldType::Dependent,
                depends_on,
                resolver,
                ..
            } = config
            {
                if depends_on
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|parent| fields_changed.contains(&parent.to_string()))
                {
                    resolvers.push((field_name, resolver.as_ref().unwrap()));
                }
            }
        }

        if resolvers.is_empty() {
            return (ctx.input(), ctx.values(), false, Vec::with_capacity(0));
        }

        let tasks = resolvers.into_iter().map(async |(field_info, resolver)| {
            (
                field_info,
                resolver(Arc::clone(&ctx), Arc::clone(&options)).await,
            )
        });

        let values = ctx.values();
        let mut validated_outputs = HashMap::new();
        let mut fields_updated = vec![];

        for (field_name, value) in join_all(tasks).await {
            // only keep fields that have been updated
            if !values.ivo_internal_is_value_equal(&field_name, &value) {
                validated_outputs.insert(field_name.clone(), value.clone());

                fields_updated.push(FieldInfo {
                    config_name: field_name.clone(),
                    is_input: false,
                    is_output: true,
                    name: field_name.clone(),
                });
            }
        }

        if fields_updated.is_empty() {
            return (ctx.input(), values, false, Vec::with_capacity(0));
        }

        let (i, o, should) =
            self.extract_updated_ctx_values(ctx, HashMap::new(), validated_outputs);

        (i, o, should, fields_updated)
    }

    async fn resolve_constants_and_defaults(
        &self,
        mini_ctx: SharedIvoMiniContext<I>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> HashMap<String, ErasedValue> {
        let mut default_values = HashMap::new();
        let mut resolvers = vec![];

        for (field_name, config) in self.schema.field_configs.iter() {
            // constants
            match &config.value {
                Some(ComputableWithMiniContext::Static(value)) => {
                    default_values.insert(field_name.to_string(), value.clone());
                    continue;
                }
                Some(ComputableWithMiniContext::Func(resolver)) => {
                    resolvers.push((field_name.to_string(), resolver));
                    continue;
                }
                _ => {}
            }

            // other fields with default values/resolvers
            match &config.default {
                Some(ComputableWithMiniContext::Static(value)) => {
                    default_values.insert(field_name.to_string(), value.clone());
                }
                Some(ComputableWithMiniContext::Func(resolver)) => {
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
            default_values.insert(field_name, value);
        }

        default_values
    }

    async fn evaluate_missing_required_fields<'a>(
        &self,
        fields_provided: &'a FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool, Timestamp>,
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
        fields_updated: FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool, Timestamp>,
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

    fn extract_updated_ctx_values(
        &self,
        ctx: SharedIvoContext<I, O>,
        validated_inputs: HashMap<String, ErasedValue>,
        validated_outputs: HashMap<String, ErasedValue>,
    ) -> (I::Partial, O::Partial, bool) {
        let (updated_inputs, do_inputs_have_updates) = ctx
            .input()
            .ivo_internal_clone_with_erased_updates(&validated_inputs);

        let output_values = if ctx.is_update() {
            ctx.changes()
        } else {
            ctx.values()
        };

        let (updated_outputs, do_outputs_have_updates) =
            output_values.ivo_internal_clone_with_erased_updates(&validated_outputs);

        (
            updated_inputs,
            updated_outputs,
            do_inputs_have_updates || do_outputs_have_updates,
        )
    }

    fn _attach_time_stamps(&self, data: O::Partial, is_update: bool) -> O::Partial {
        if let Some(TimestampConfig {
            created_at,
            resolver,
            updated_at,
            with_optional_updated_at,
        }) = self.schema._timestamp_configs.as_ref()
        {
            let now = resolver();

            if !is_update {
                if let Some(_created_at) = created_at {
                    // data.set(created_at, erase_value(now));
                }
            }

            if let Some(_updated_at) = updated_at {
                if *with_optional_updated_at {
                    if is_update {
                        // data.set(updated_at, erase_value(Some(now)));
                    } else {
                        // data.set(updated_at, erase_value::<Option<Timestamp>>(None));
                    }
                } else {
                    // data.set(updated_at, erase_value(now));
                }
            }

            erase_value(now);
        }

        data
        // updated_outputs
    }
}

/// this is a sync alternative to: shared_rw_options.read().await.clone()
fn unwrap_async_lock<T>(lock: Arc<RwLock<T>>) -> T {
    match Arc::into_inner(lock).unwrap().try_unwrap() {
        Ok(raw_data) => raw_data,
        _ => panic!("error unwrapping shared RwLock"),
    }
}
