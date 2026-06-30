mod internal;

use futures::future::{join_all, BoxFuture};
use futures::FutureExt;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::future::ready;
use std::sync::Arc;

use crate::model::internal::{FieldInfo, FieldInfoCollection};
use crate::schema::{
    fields::{
        base::{FieldType, InternalFieldConfig},
        types::{ComputableRequiredError, IsFieldProvisionEnabled, ValueResolverWithMiniContext},
        TimestampConfig,
    },
    Schema,
};
use crate::types::internal::{
    types::erase_value, DefaultErrorTool, FieldError, IvoErrorTool, IvoPartialStructMethods,
    IvoStruct, RwLock, UpdateError,
};
use crate::types::InternalIvoContext;

use crate::schema::options::types::{OnSuccessConfig, PostValidationConfig};

use crate::{IvoContext, IvoCtxOptions, IvoRwCtxOptions};

type AsyncHandlerTrigger<'a> = Box<dyn Fn() -> BoxFuture<'a, ()> + Send + Sync + 'a>;

impl<
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions: Sync + Send,
        Timestamp: Clone + Debug + Send + Sync + 'static,
        ErrorTool: IvoErrorTool,
    > Schema<I, O, CtxOptions, Timestamp, ErrorTool>
{
    pub fn model(&self) -> Model<'_, I, O, CtxOptions, Timestamp, ErrorTool> {
        Model { schema: self }
    }
}

pub struct Model<
    'schema,
    I: IvoStruct,
    O: IvoStruct = I,
    CtxOptions: Sync + Send = HashMap<String, ()>,
    Timestamp: Clone + Debug + Send + Sync + 'static = (),
    ErrorTool: IvoErrorTool = DefaultErrorTool,
> {
    schema: &'schema Schema<I, O, CtxOptions, Timestamp, ErrorTool>,
}

impl<
        'schema,
        I: IvoStruct,
        O: IvoStruct,
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
        let mut ctx = Arc::new(InternalIvoContext::<I, O>::new_create_ctx(
            input.clone(),
            input.clone(),
            O::Partial::default(),
        ));

        // filter out ignored fields
        let (input, output, relevant_fields_provided, fields_provided) = self
            .filter_input_fields_allowed(
                None,
                input,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        // Resolve constants & defaults
        let output = self
            .attach_constants_and_defaults(output, &input, Arc::clone(&shared_rw_options))
            .await;

        Arc::make_mut(&mut ctx)
            .set_input(input.clone())
            .set_changes(output);

        if let Err(payload) = self
            .evaluate_missing_required_fields(
                &relevant_fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            return Err((
                payload,
                self.prepare_failure_handlers(
                    fields_provided.fields,
                    ctx,
                    Arc::new(unwrap_async_lock(shared_rw_options)),
                ),
            ));
        }

        // Run validators
        match self
            .validate(
                &relevant_fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            Ok(Some((validated_inputs, validated_outputs))) => {
                Arc::make_mut(&mut ctx)
                    .set_input(validated_inputs)
                    .set_changes(validated_outputs);
            }
            Err(payload) => {
                return Err((
                    payload,
                    self.prepare_failure_handlers(
                        fields_provided.fields,
                        ctx,
                        Arc::new(unwrap_async_lock(shared_rw_options)),
                    ),
                ));
            }
            _ => (),
        };

        // Run re_validators
        match self
            .re_validate(
                &relevant_fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            Ok(Some((validated_inputs, validated_outputs))) => {
                Arc::make_mut(&mut ctx)
                    .set_input(validated_inputs)
                    .set_changes(validated_outputs);
            }
            Err(payload) => {
                return Err((
                    payload,
                    self.prepare_failure_handlers(
                        fields_provided.fields,
                        ctx,
                        Arc::new(unwrap_async_lock(shared_rw_options)),
                    ),
                ));
            }
            _ => (),
        };

        // Run post-validators
        match self
            .post_validate(
                &relevant_fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            Ok(Some((validated_inputs, validated_outputs))) => {
                Arc::make_mut(&mut ctx)
                    .set_input(validated_inputs)
                    .set_changes(validated_outputs);
            }
            Err(payload) => {
                return Err((
                    payload,
                    self.prepare_failure_handlers(
                        fields_provided.fields,
                        ctx,
                        Arc::new(unwrap_async_lock(shared_rw_options)),
                    ),
                ));
            }
            _ => (),
        };

        // Sanitize virtuals
        if let Some(sanitized_inputs) = self
            .sanitize_virtuals(
                &relevant_fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            Arc::make_mut(&mut ctx).set_input(sanitized_inputs);
        }

        // Resolve values of dependent fields
        let mut dependent_fields_col = FieldInfoCollection::from_fields(
            self.schema,
            relevant_fields_provided.fields.clone(),
            &relevant_fields_provided.schema_input_fields,
            &relevant_fields_provided.schema_output_fields,
        );

        while let Some((validated_outputs, dependent_fields_resolved)) = self
            .resolve_dependent_values(
                &dependent_fields_col,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            Arc::make_mut(&mut ctx).set_changes(validated_outputs);
            dependent_fields_col.set_fields(dependent_fields_resolved);
        }

        // Generate and set timestamps
        let (values, should_update_ctx) = self.attach_timestamps(ctx.values(), false);

        if should_update_ctx {
            Arc::make_mut(&mut ctx).set_changes(values.clone());
        }

        Ok((
            O::ivo_internal_dangerously_get_values_from_partial(values),
            self.prepare_success_handlers(
                relevant_fields_provided,
                ctx,
                Arc::new(unwrap_async_lock(shared_rw_options)),
            ),
        ))
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

        let mut ctx = Arc::new(InternalIvoContext::<I, O>::new_update_ctx(
            O::Partial::default(),
            updates.clone(),
            updates.clone(),
            data.clone(),
            data.clone(),
        ));

        let shared_rw_options = Arc::new(RwLock::new(options));

        let (input, output, relevant_fields_provided, fields_provided) = self
            .filter_input_fields_allowed(
                Some(&old_partial_values),
                updates,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        Arc::make_mut(&mut ctx).set_input(input).set_changes(output);

        // if the updates provided are all none, the nothing to update
        if relevant_fields_provided.fields.is_empty() {
            return Err((
                UpdateError::NothingToUpdate,
                self.prepare_failure_handlers(
                    fields_provided.fields,
                    ctx,
                    Arc::new(unwrap_async_lock(shared_rw_options)),
                ),
            ));
        }

        // Evaluate missing required fields
        if let Err(payload) = self
            .evaluate_missing_required_fields(
                &relevant_fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            return Err((
                UpdateError::ValidationError(payload),
                self.prepare_failure_handlers(
                    fields_provided.fields,
                    ctx,
                    Arc::new(unwrap_async_lock(shared_rw_options)),
                ),
            ));
        }

        // Run validators
        match self
            .validate(
                &relevant_fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            Ok(Some((validated_inputs, validated_outputs))) => {
                Arc::make_mut(&mut ctx)
                    .set_input(validated_inputs)
                    .set_changes(validated_outputs.clone())
                    .set_full_values(data.ivo_internal_clone_with(validated_outputs));
            }
            Err(payload) => {
                return Err((
                    UpdateError::ValidationError(payload),
                    self.prepare_failure_handlers(
                        fields_provided.fields,
                        ctx,
                        Arc::new(unwrap_async_lock(shared_rw_options)),
                    ),
                ))
            }
            _ => (),
        };

        // Run re_validators
        match self
            .re_validate(
                &relevant_fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            Ok(Some((validated_inputs, validated_outputs))) => {
                Arc::make_mut(&mut ctx)
                    .set_input(validated_inputs)
                    .set_changes(validated_outputs.clone())
                    .set_full_values(data.ivo_internal_clone_with(validated_outputs));
            }
            Err(payload) => {
                return Err((
                    UpdateError::ValidationError(payload),
                    self.prepare_failure_handlers(
                        fields_provided.fields,
                        ctx,
                        Arc::new(unwrap_async_lock(shared_rw_options)),
                    ),
                ))
            }
            _ => (),
        };

        // Run post-validators
        match self
            .post_validate(
                &relevant_fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            Ok(Some((validated_inputs, validated_outputs))) => {
                Arc::make_mut(&mut ctx)
                    .set_input(validated_inputs)
                    .set_changes(validated_outputs.clone())
                    .set_full_values(data.ivo_internal_clone_with(validated_outputs));
            }
            Err(payload) => {
                return Err((
                    UpdateError::ValidationError(payload),
                    self.prepare_failure_handlers(
                        fields_provided.fields,
                        ctx,
                        Arc::new(unwrap_async_lock(shared_rw_options)),
                    ),
                ))
            }
            _ => (),
        };

        // Sanitize virtuals
        if let Some(sanitized_inputs) = self
            .sanitize_virtuals(
                &relevant_fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            Arc::make_mut(&mut ctx).set_input(sanitized_inputs);
        }

        // Resolve values of dependent fields
        let updated_values = ctx.values();

        let fields_updated_vec = relevant_fields_provided
            .fields
            .iter()
            .filter_map(|f| {
                if f.is_input && !f.is_output {
                    return Some(f.clone());
                }

                if !old_partial_values.ivo_internal_is_value_equal(
                    &f.name,
                    &updated_values.ivo_internal_get_erased_value(&f.name),
                ) {
                    return Some(f.clone());
                }

                None
            })
            .collect();

        let mut fields_updated = FieldInfoCollection::from_fields(
            self.schema,
            fields_updated_vec,
            &relevant_fields_provided.schema_input_fields,
            &relevant_fields_provided.schema_output_fields,
        );

        let mut dependent_fields_col = fields_updated.clone();

        while let Some((validated_outputs, dependent_fields_resolved)) = self
            .resolve_dependent_values(
                &dependent_fields_col,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            for field_info in dependent_fields_resolved.iter() {
                fields_updated.add(field_info.clone());
            }

            dependent_fields_col.set_fields(dependent_fields_resolved);
            Arc::make_mut(&mut ctx)
                .set_changes(validated_outputs.clone())
                .set_full_values(data.ivo_internal_clone_with(validated_outputs));
        }

        let Some(updated_values) = data.ivo_internal_get_updates_from_partial(&ctx.changes())
        else {
            return Err((
                UpdateError::NothingToUpdate,
                self.prepare_failure_handlers(
                    fields_provided.fields,
                    ctx,
                    Arc::new(unwrap_async_lock(shared_rw_options)),
                ),
            ));
        };

        // Generate and set timestamps
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
        ctx: IvoContext<I, O>,
        options: IvoRwCtxOptions<CtxOptions>,
    ) -> Result<Option<(I::Partial, O::Partial)>, ErrorTool::ErrorPayload> {
        let raw_inputs = ctx.raw_input();
        let mut validators = Vec::with_capacity(fields_provided.fields.len());
        let mut validated_inputs = ctx.input();
        let mut validated_outputs = if ctx.is_update() {
            ctx.changes()
        } else {
            ctx.values()
        };
        let mut has_updates = false;

        for field_info in fields_provided.fields.iter() {
            if let Some(InternalFieldConfig {
                field_type,
                validator,
                ..
            }) = self.schema.field_configs.get(&field_info.config_name)
            {
                if let Some(validator) = validator {
                    validators.push((field_info, validator));
                } else if matches!(field_type, FieldType::Lax) {
                    let field_name = &field_info.name;
                    let value = raw_inputs.ivo_internal_get_erased_value(field_name);

                    validated_inputs.ivo_internal_set(field_name, &value);
                    validated_outputs.ivo_internal_set(field_name, &value);
                    has_updates = true
                }
            }
        }

        if validators.is_empty() {
            if has_updates {
                return Ok(Some((validated_inputs, validated_outputs)));
            }

            return Ok(None);
        }

        let tasks = validators.into_iter().map(async |(f, validator)| {
            (
                f,
                validator(
                    raw_inputs.ivo_internal_get_erased_value(&f.name),
                    Arc::clone(&ctx),
                    Arc::clone(&options),
                )
                .await,
            )
        });

        let mut error_tool = ErrorTool::new();

        for (field_info, result) in join_all(tasks).await {
            let field_name = field_info.name.clone();

            match result {
                Err((reason, metadata)) => {
                    error_tool.add(field_name.as_str(), FieldError { reason, metadata });
                }
                Ok(Some(value)) => {
                    has_updates = true;

                    if field_info.is_input {
                        validated_inputs.ivo_internal_set(&field_name, &value);
                    }

                    if field_info.is_output {
                        validated_outputs.ivo_internal_set(&field_name, &value);
                    }
                }
                Ok(None) => {
                    if field_info.is_output {
                        has_updates = true;

                        validated_outputs.ivo_internal_set(
                            &field_name,
                            &validated_outputs.ivo_internal_get_erased_value(&field_name),
                        );
                    }
                }
            }
        }

        if error_tool.has_errors() {
            return Err(error_tool.payload());
        }

        if has_updates {
            return Ok(Some((validated_inputs, validated_outputs)));
        }

        Ok(None)
    }

    async fn re_validate<'a>(
        &self,
        fields_provided: &'a FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>,
        ctx: IvoContext<I, O>,
        options: IvoRwCtxOptions<CtxOptions>,
    ) -> Result<Option<(I::Partial, O::Partial)>, ErrorTool::ErrorPayload> {
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
            return Ok(None);
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
                Ok(Some(value)) => {
                    has_updates = true;

                    if field_info.is_input {
                        validated_inputs.ivo_internal_set(&field_name, &value);
                    }

                    if field_info.is_output {
                        validated_outputs.ivo_internal_set(&field_name, &value);
                    }
                }
                _ => {}
            }
        }

        if error_tool.has_errors() {
            return Err(error_tool.payload());
        }

        if has_updates {
            return Ok(Some((validated_inputs, validated_outputs)));
        }

        Ok(None)
    }

    async fn post_validate<'a>(
        &self,
        fields_provided: &'a FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>,
        ctx: IvoContext<I, O>,
        options: IvoRwCtxOptions<CtxOptions>,
    ) -> Result<Option<(I::Partial, O::Partial)>, ErrorTool::ErrorPayload> {
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
                    if let Some(ref pre_validator) = pre_validator {
                        pre_validators.push((fields, pre_validator));
                    }

                    for validator in validators {
                        post_validators.push((fields, validator));
                    }

                    continue;
                }
            }
        }

        if post_validators.is_empty() {
            return Ok(None);
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
                match pre_validation {
                    Err(payload) => {
                        for (field_name, (reason, metadata)) in payload {
                            let field_name = field_name.as_str();

                            if fields.contains(&field_name) {
                                error_tool.add(field_name, FieldError { reason, metadata });
                            }
                        }
                    }
                    Ok(Some(updates)) => {
                        for (field_name, value) in updates.ivo_internal_enumerate() {
                            if let Some(field_info) = fields_provided.get(&field_name) {
                                if !fields.contains(&field_info.config_name.as_str()) {
                                    continue;
                                }

                                has_updates = true;

                                if field_info.is_input {
                                    validated_inputs.ivo_internal_set(&field_info.name, &value);
                                }

                                if field_info.is_output {
                                    validated_outputs.ivo_internal_set(&field_info.name, &value);
                                }
                            }
                        }
                    }
                    _ => (),
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

        for (fields, validation) in join_all(tasks).await {
            match validation {
                Err(payload) => {
                    for (field_name, (reason, metadata)) in payload {
                        let field_name = field_name.as_str();

                        if fields.contains(&field_name) {
                            error_tool.add(field_name, FieldError { reason, metadata });
                        }
                    }
                }
                Ok(Some(updates)) => {
                    for (field_name, value) in updates.ivo_internal_enumerate() {
                        if let Some(field_info) = fields_provided.get(&field_name) {
                            if !fields.contains(&field_info.config_name.as_str()) {
                                continue;
                            }

                            has_updates = true;

                            if field_info.is_input {
                                validated_inputs.ivo_internal_set(&field_info.name, &value);
                            }

                            if field_info.is_output {
                                validated_outputs.ivo_internal_set(&field_info.name, &value);
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        if error_tool.has_errors() {
            return Err(error_tool.payload());
        }

        if has_updates {
            return Ok(Some((validated_inputs, validated_outputs)));
        }

        Ok(None)
    }

    async fn sanitize_virtuals<'a>(
        &self,
        fields_provided: &'a FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>,
        ctx: IvoContext<I, O>,
        options: IvoRwCtxOptions<CtxOptions>,
    ) -> Option<I::Partial> {
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
            return None;
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

        let mut input_values = ctx.input();

        for (f, value) in join_all(tasks).await {
            input_values.ivo_internal_set(&f.name, &value);
        }

        Some(input_values)
    }

    async fn resolve_dependent_values<'a>(
        &self,
        fields_changed: &'a FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>,
        ctx: IvoContext<I, O>,
        options: IvoRwCtxOptions<CtxOptions>,
    ) -> Option<(O::Partial, Vec<FieldInfo>)> {
        let mut resolvers = vec![];
        let previous_values: Option<O::Partial> = ctx.previous_values().map(|v| v.into());
        let is_update = previous_values.as_ref().is_some();
        let previous_values = previous_values.unwrap_or_default();

        for (field_name, config) in self.schema.field_configs.iter() {
            match config {
                InternalFieldConfig {
                    field_type: FieldType::Dependent,
                    depends_on: Some(ref depends_on),
                    resolver: Some(ref resolver),
                    ..
                } if depends_on
                    .iter()
                    .any(|parent| fields_changed.contains(&parent.to_string())) =>
                {
                    if !is_update {
                        resolvers.push((field_name, resolver));

                        continue;
                    }

                    // handle readonly during updates
                    if let InternalFieldConfig {
                        field_type: FieldType::Dependent,
                        default: Some(ValueResolverWithMiniContext::Static(default_value)),
                        ignore_update: Some(IsFieldProvisionEnabled::Readonly),
                        ..
                    } = config
                    {
                        // readonly means: don't update if value has changed
                        // i.e: only update if prev_value == default_value
                        if previous_values.ivo_internal_is_value_equal(field_name, default_value) {
                            resolvers.push((field_name, resolver));
                        }

                        continue;
                    }

                    resolvers.push((field_name, resolver));
                }
                _ => {}
            }
        }

        if resolvers.is_empty() {
            return None;
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
            if !values.ivo_internal_is_value_equal(field_name, &value) {
                updated_values.ivo_internal_set(field_name, &value);

                fields_updated.push(FieldInfo {
                    config_name: field_name.clone(),
                    is_input: false,
                    is_output: true,
                    name: field_name.clone(),
                });
            }
        }

        if fields_updated.is_empty() {
            return None;
        }

        Some((updated_values, fields_updated))
    }

    async fn attach_constants_and_defaults(
        &self,
        mut output: O::Partial,
        input: &I::Partial,
        options: IvoRwCtxOptions<CtxOptions>,
    ) -> O::Partial {
        let mut resolvers = vec![];
        let fields_provided = input.ivo_internal_fields_provided();

        for (field_name, config) in self.schema.field_configs.iter() {
            if matches!(config.field_type, FieldType::Lax) && fields_provided.contains(field_name) {
                continue;
            }

            match config {
                InternalFieldConfig {
                    field_type: FieldType::Constant,
                    value,
                    ..
                } => match value {
                    Some(ValueResolverWithMiniContext::Static(value)) => {
                        output.ivo_internal_set(field_name, value);

                        continue;
                    }
                    Some(ValueResolverWithMiniContext::Func(resolver)) => {
                        resolvers.push((field_name.to_string(), resolver));
                        continue;
                    }
                    _ => {}
                },
                InternalFieldConfig {
                    field_type: FieldType::Dependent | FieldType::Lax,
                    default: Some(default),
                    ..
                } => match default {
                    ValueResolverWithMiniContext::Static(value) => {
                        output.ivo_internal_set(field_name, value);
                    }
                    ValueResolverWithMiniContext::Func(resolver) => {
                        resolvers.push((field_name.to_string(), resolver));
                    }
                },
                _ => {}
            }
        }

        if resolvers.is_empty() {
            return output;
        }

        let shared_input = Arc::new(input.clone());

        let tasks = resolvers.into_iter().map(async |(field_name, resolver)| {
            (
                field_name.clone(),
                resolver(Arc::clone(&shared_input), Arc::clone(&options)).await,
            )
        });

        for (field_name, value) in join_all(tasks).await {
            output.ivo_internal_set(&field_name, &value);
        }

        output
    }

    async fn filter_input_fields_allowed<'a>(
        &'a self,
        previous_values: Option<&O::Partial>,
        input_values: &I::Partial,
        ctx: IvoContext<I, O>,
        options: IvoRwCtxOptions<CtxOptions>,
    ) -> (
        I::Partial,
        O::Partial,
        FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>,
        FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>,
    ) {
        let is_update = previous_values.is_some();
        let previous_values = previous_values.cloned().unwrap_or_default();

        let mut resolvers = vec![];
        let mut input = input_values.clone();
        let mut output = O::Partial::default();
        let mut fields_provided = FieldInfoCollection::new(self.schema);
        let mut field_info_vec = vec![];

        if is_update {
            if let Some(ref resolver) = self.schema.options.ignore_update {
                if resolver(
                    (ctx.input(), ctx.full_values().unwrap()),
                    Arc::clone(&options),
                )
                .await
                {
                    return (input, output, fields_provided.clone(), fields_provided);
                }
            }

            for (field_name, value) in input_values.ivo_internal_enumerate() {
                let field_info = fields_provided.get(&field_name).unwrap();
                fields_provided.add(field_info.clone());

                if (field_info.is_input && !field_info.is_output)
                    || !previous_values.ivo_internal_is_value_equal(&field_name, &value)
                {
                    field_info_vec.push(field_info);
                }
            }
        } else {
            for field_name in input_values.ivo_internal_fields_provided() {
                let field_info = fields_provided.get(&field_name).unwrap();
                fields_provided.add(field_info.clone());

                field_info_vec.push(field_info);
            }
        }

        let mut final_field_info_vec = vec![];

        for field_info in field_info_vec.iter() {
            if let Some(InternalFieldConfig {
                field_type: FieldType::Lax | FieldType::Required | FieldType::Virtual,
                default,
                ignore,
                ignore_init,
                ignore_update,
                ..
            }) = self.schema.field_configs.get(&field_info.config_name)
            {
                if let Some(resolver) = ignore {
                    resolvers.push((field_info, resolver));

                    continue;
                }

                let source = if is_update {
                    ignore_update
                } else {
                    ignore_init
                };

                match source {
                    Some(IsFieldProvisionEnabled::False) => {
                        input.ivo_internal_unset(&field_info.name);
                    }
                    Some(IsFieldProvisionEnabled::Func(resolver)) => {
                        resolvers.push((field_info, resolver));
                    }
                    Some(IsFieldProvisionEnabled::Readonly) if is_update => {
                        if let Some(ValueResolverWithMiniContext::Static(value)) = default {
                            // readonly means: only allow update if value prev_value == default_value

                            if previous_values.ivo_internal_is_value_equal(&field_info.name, value)
                            {
                                final_field_info_vec.push(field_info.to_owned());

                                if field_info.is_output {
                                    output.ivo_internal_set(
                                        &field_info.name,
                                        &input.ivo_internal_get_erased_value(&field_info.name),
                                    );
                                }

                                continue;
                            }
                        }
                    }
                    _ => {
                        final_field_info_vec.push(field_info.to_owned());

                        if field_info.is_output {
                            output.ivo_internal_set(
                                &field_info.name,
                                &input.ivo_internal_get_erased_value(&field_info.name),
                            );
                        }
                    }
                };
            }
        }

        let mut relevant_fields_provided = FieldInfoCollection::new(self.schema);

        if resolvers.is_empty() {
            relevant_fields_provided.set_fields(final_field_info_vec);

            return (input, output, relevant_fields_provided, fields_provided);
        }

        let tasks = resolvers.into_iter().map(async |(field_info, resolver)| {
            (
                field_info,
                resolver(Arc::clone(&ctx), Arc::clone(&options)).await,
            )
        });

        for (field_info, ignore) in join_all(tasks).await {
            if ignore {
                input.ivo_internal_unset(&field_info.name);

                continue;
            }

            final_field_info_vec.push(field_info.to_owned());

            if field_info.is_output {
                output.ivo_internal_set(
                    &field_info.name,
                    &input.ivo_internal_get_erased_value(&field_info.name),
                );
            }

            continue;
        }

        relevant_fields_provided.set_fields(final_field_info_vec);

        (input, output, relevant_fields_provided, fields_provided)
    }

    async fn evaluate_missing_required_fields<'a>(
        &self,
        fields_provided: &'a FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>,
        ctx: IvoContext<I, O>,
        options: IvoRwCtxOptions<CtxOptions>,
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
                } if !is_update => {
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
                    field_type: FieldType::Lax,
                    required_fn: Some(resolver),
                    ..
                } => resolvers.push((field_name, resolver)),
                InternalFieldConfig {
                    alias,
                    field_type: FieldType::Virtual,
                    required_fn: Some(resolver),
                    ..
                } => resolvers.push((alias.as_ref().unwrap_or(field_name), resolver)),
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

        for (field_name, required) in join_all(tasks).await {
            if let Some(reason) = required {
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

        Ok(())
    }

    fn prepare_failure_handlers(
        &self,
        fields_provided: Vec<FieldInfo>,
        ctx: IvoContext<I, O>,
        options: IvoCtxOptions<CtxOptions>,
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
        ctx: IvoContext<I, O>,
        options: IvoCtxOptions<CtxOptions>,
    ) -> AsyncHandlerTrigger<'schema> {
        let mut field_names = HashSet::new();

        for field_info in fields_updated.fields.iter() {
            field_names.insert(field_info.config_name.clone());
        }

        let candidate_field_names = if ctx.is_update() {
            ctx.changes().ivo_internal_fields_provided()
        } else {
            ctx.values().ivo_internal_fields_provided()
        };

        for field_name in candidate_field_names {
            field_names.insert(field_name);
        }

        let mut handlers = vec![];

        for field_name in field_names.iter() {
            if let Some(InternalFieldConfig {
                on_success_fns: Some(h_vec),
                ..
            }) = self.schema.field_configs.get(field_name)
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
                if fields.is_empty() || fields.iter().any(|f| field_names.contains(*f)) {
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
            let mut now = None;

            if !is_update {
                if let Some(created_at) = created_at {
                    let value = if let Some(value) = now.clone() {
                        value
                    } else {
                        let value = resolver();

                        now = Some(value.clone());

                        value
                    };

                    data.ivo_internal_set(created_at, &erase_value(value));
                    was_updated = true;
                }
            }

            if let Some(updated_at) = updated_at {
                let is_optional = *with_optional_updated_at;

                if is_optional && !is_update {
                    data.ivo_internal_set(updated_at, &erase_value::<Option<Timestamp>>(None));
                } else {
                    let value = now.unwrap_or_else(resolver);

                    if is_optional {
                        data.ivo_internal_set(updated_at, &erase_value(Some(value)));
                    } else {
                        data.ivo_internal_set(updated_at, &erase_value(value));
                    }
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
