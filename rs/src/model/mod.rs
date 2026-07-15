mod internal;

use futures::future::{join_all, BoxFuture};
use futures::FutureExt;
use std::collections::HashSet;
use std::fmt::Debug;
use std::future::ready;
use std::sync::Arc;

use crate::__private_types::types::{BooleanResolver, IgnoreUpdateOptionResolver};
use crate::__private_types::{types::PartialErrorsMethods, IvoInputStruct};
use crate::model::internal::FieldInfoCollection;
use crate::schema::options::types::{
    IgnoreOptionConfig, IgnoreUpdateOptionConfig, UniformIgnoreResolver,
};
use crate::schema::{
    fields::{
        base::{FieldType, InternalFieldConfig},
        types::{
            ComputableRequiredError, IsFieldProvisionEnabled, RequiredResolver,
            ValueResolverWithSharedInput,
        },
        TimestampConfig,
    },
    options::types::{
        OnSuccessConfig, PostValidationConfig, RequiredOptionConfig, UniformRequiredResolver,
    },
};
use crate::types::{
    internal::{
        types::erase_value, FieldError, IvoErrorTool, IvoRwLock, IvoStruct, PartialStructMethods,
    },
    InternalIvoContext,
};
use crate::{IvoContext, IvoCtxOptions, IvoRwCtxOptions, Model};

type AsyncHandlerTrigger<'a> = Box<dyn FnOnce() -> BoxFuture<'a, ()> + Send + Sync + 'a>;

impl<
        I: IvoInputStruct<ErrorTool>,
        O: IvoStruct,
        CtxOptions: Clone + Sync + Send,
        Timestamp: Clone + Debug + Send + Sync + 'static,
        ErrorTool: IvoErrorTool,
    > Model<I, O, CtxOptions, Timestamp, ErrorTool>
{
    pub async fn create(
        &self,
        input: &I::Partial,
        options: CtxOptions,
    ) -> Result<
        (O, AsyncHandlerTrigger<'_>, CtxOptions),
        (ErrorTool::ErrorPayload, AsyncHandlerTrigger<'_>, CtxOptions),
    > {
        let shared_rw_options = Arc::new(IvoRwLock::new(options));
        let mut ctx = Arc::new(InternalIvoContext::<I, O>::new_create_ctx(
            input.clone(),
            input.clone(),
            O::Partial::default(),
        ));

        // filter out ignored fields
        let (input, output, fields_collection) = self
            .filter_input_fields_allowed(
                None,
                input,
                FieldInfoCollection::new(&self.field_configs),
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

        match self
            .evaluate_missing_required_fields(
                &fields_collection,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            Ok(col) => col,
            Err(payload) => {
                let final_ctx_options = unwrap_async_lock(shared_rw_options);

                return Err((
                    payload,
                    self.prepare_failure_handlers(
                        fields_collection,
                        ctx,
                        Arc::new(final_ctx_options.clone()),
                    ),
                    final_ctx_options,
                ));
            }
        };

        // Run validators
        match self
            .validate(
                &fields_collection,
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
                let final_ctx_options = unwrap_async_lock(shared_rw_options);

                return Err((
                    payload,
                    self.prepare_failure_handlers(
                        fields_collection,
                        ctx,
                        Arc::new(final_ctx_options.clone()),
                    ),
                    final_ctx_options,
                ));
            }
            _ => (),
        };

        // Run re_validators
        match self
            .re_validate(
                &fields_collection,
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
                let final_ctx_options = unwrap_async_lock(shared_rw_options);

                return Err((
                    payload,
                    self.prepare_failure_handlers(
                        fields_collection,
                        ctx,
                        Arc::new(final_ctx_options.clone()),
                    ),
                    final_ctx_options,
                ));
            }
            _ => (),
        };

        // Run post-validators
        match self
            .post_validate(
                &fields_collection,
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
                let final_ctx_options = unwrap_async_lock(shared_rw_options);

                return Err((
                    payload,
                    self.prepare_failure_handlers(
                        fields_collection.clone(),
                        ctx,
                        Arc::new(final_ctx_options.clone()),
                    ),
                    final_ctx_options,
                ));
            }
            _ => (),
        };

        // Sanitize virtuals
        if let Some(sanitized_inputs) = self
            .sanitize_virtuals(
                &fields_collection,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            Arc::make_mut(&mut ctx).set_input(sanitized_inputs);
        }

        // Resolve values of dependent fields
        let mut dependent_fields_col = fields_collection.cloned_from_relevant_dependent_fields();

        while let Some((validated_outputs, fields_changed)) = self
            .resolve_dependent_values(
                &dependent_fields_col,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            dependent_fields_col =
                dependent_fields_col.new_with_dependent_fields_changed(fields_changed.clone());

            Arc::make_mut(&mut ctx).set_changes(validated_outputs);
        }

        // Generate and set timestamps
        let (values, should_update_ctx) = self.attach_timestamps(ctx.values(), false);

        if should_update_ctx {
            Arc::make_mut(&mut ctx).set_changes(values.clone());
        }

        let final_ctx_options = unwrap_async_lock(shared_rw_options);

        Ok((
            O::ivo_internal_dangerously_get_values_from_partial(values),
            self.prepare_success_handlers(
                fields_collection,
                ctx,
                Arc::new(final_ctx_options.clone()),
            ),
            final_ctx_options,
        ))
    }

    pub async fn update(
        &self,
        data: &O,
        updates: &I::Partial,
        options: CtxOptions,
    ) -> Result<
        (O::Partial, AsyncHandlerTrigger<'_>, CtxOptions),
        (
            Option<ErrorTool::ErrorPayload>,
            AsyncHandlerTrigger<'_>,
            CtxOptions,
        ),
    > {
        let old_partial_values: O::Partial = data.clone().into();

        let mut ctx = Arc::new(InternalIvoContext::<I, O>::new_update_ctx(
            O::Partial::default(),
            updates.clone(),
            updates.clone(),
            data.clone(),
            data.clone(),
        ));

        let shared_rw_options = Arc::new(IvoRwLock::new(options));

        let (input, output, fields_collection) = self
            .filter_input_fields_allowed(
                Some(&old_partial_values),
                updates,
                FieldInfoCollection::new(&self.field_configs),
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        Arc::make_mut(&mut ctx).set_input(input).set_changes(output);

        // if the updates provided are all none, the nothing to update
        if fields_collection.relevant_fields_provided().is_empty() {
            let final_ctx_options = unwrap_async_lock(shared_rw_options);

            return Err((
                None,
                self.prepare_failure_handlers(
                    fields_collection,
                    ctx,
                    Arc::new(final_ctx_options.clone()),
                ),
                final_ctx_options,
            ));
        }

        // Evaluate missing required fields
        match self
            .evaluate_missing_required_fields(
                &fields_collection,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            Ok(col) => col,
            Err(payload) => {
                let final_ctx_options = unwrap_async_lock(shared_rw_options);

                return Err((
                    Some(payload),
                    self.prepare_failure_handlers(
                        fields_collection,
                        ctx,
                        Arc::new(final_ctx_options.clone()),
                    ),
                    final_ctx_options,
                ));
            }
        };

        // Run validators
        match self
            .validate(
                &fields_collection,
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
                let final_ctx_options = unwrap_async_lock(shared_rw_options);

                return Err((
                    Some(payload),
                    self.prepare_failure_handlers(
                        fields_collection,
                        ctx,
                        Arc::new(final_ctx_options.clone()),
                    ),
                    final_ctx_options,
                ));
            }
            _ => (),
        };

        // Run re_validators
        match self
            .re_validate(
                &fields_collection,
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
                let final_ctx_options = unwrap_async_lock(shared_rw_options);

                return Err((
                    Some(payload),
                    self.prepare_failure_handlers(
                        fields_collection,
                        ctx,
                        Arc::new(final_ctx_options.clone()),
                    ),
                    final_ctx_options,
                ));
            }
            _ => (),
        };

        // Run post-validators
        match self
            .post_validate(
                &fields_collection,
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
                let final_ctx_options = unwrap_async_lock(shared_rw_options);

                return Err((
                    Some(payload),
                    self.prepare_failure_handlers(
                        fields_collection,
                        ctx,
                        Arc::new(final_ctx_options.clone()),
                    ),
                    final_ctx_options,
                ));
            }
            _ => (),
        };

        // Sanitize virtuals
        if let Some(sanitized_inputs) = self
            .sanitize_virtuals(
                &fields_collection,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            Arc::make_mut(&mut ctx).set_input(sanitized_inputs);
        }

        // Resolve values of dependent fields
        let mut input = ctx.input();
        let mut changes = ctx.changes();
        let updated_values = ctx.values();

        let mut relevant_fields_provided = HashSet::new();

        for field_name in fields_collection.relevant_fields_provided() {
            let field_info = fields_collection.get(field_name);

            if !field_info.is_output {
                relevant_fields_provided.insert(field_name.clone());

                continue;
            }

            if !old_partial_values.ivo_internal_is_value_equal(
                field_info.name,
                &updated_values.ivo_internal_get_erased_value(field_info.name),
            ) {
                relevant_fields_provided.insert(field_name.clone());

                continue;
            }

            input.ivo_internal_unset(field_name);
            changes.ivo_internal_unset(field_name);
        }

        Arc::make_mut(&mut ctx)
            .set_input(input)
            .set_changes(changes.clone())
            .set_full_values(data.ivo_internal_clone_with(changes));

        let fields_collection =
            fields_collection.new_with_relevant_fields_provided(relevant_fields_provided);

        let mut dependent_fields_col = fields_collection.cloned_from_relevant_dependent_fields();

        while let Some((validated_outputs, fields_changed)) = self
            .resolve_dependent_values(
                &dependent_fields_col,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
        {
            dependent_fields_col =
                dependent_fields_col.new_with_dependent_fields_changed(fields_changed.clone());

            Arc::make_mut(&mut ctx)
                .set_changes(validated_outputs.clone())
                .set_full_values(data.ivo_internal_clone_with(validated_outputs));
        }

        let Some(updated_values) = data.ivo_internal_get_updates_from_partial(&ctx.changes())
        else {
            let final_ctx_options = unwrap_async_lock(shared_rw_options);

            return Err((
                None,
                self.prepare_failure_handlers(
                    fields_collection,
                    ctx,
                    Arc::new(final_ctx_options.clone()),
                ),
                final_ctx_options,
            ));
        };

        // Generate and set timestamps
        let (updated_values, should_update_ctx) = self.attach_timestamps(updated_values, true);

        if should_update_ctx {
            Arc::make_mut(&mut ctx)
                .set_changes(updated_values.clone())
                .set_full_values(data.ivo_internal_clone_with(updated_values.clone()));
        }

        let final_ctx_options = unwrap_async_lock(shared_rw_options);

        Ok((
            updated_values,
            self.prepare_success_handlers(
                fields_collection,
                ctx,
                Arc::new(final_ctx_options.clone()),
            ),
            final_ctx_options,
        ))
    }

    pub async fn delete(&self, data: &O, options: CtxOptions) {
        let data = Arc::new(data.clone());
        let options = Arc::new(options);
        let mut handlers = vec![];

        for config in self.field_configs.values() {
            if let Some(h_vec) = &config.on_delete_fns {
                handlers.extend(h_vec);

                continue;
            }
        }

        if let Some(h_vec) = &self.options.on_delete_fns {
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
        fields_collection: &FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: IvoContext<I, O>,
        options: IvoRwCtxOptions<CtxOptions>,
    ) -> Result<Option<(I::Partial, O::Partial)>, ErrorTool::ErrorPayload> {
        let relevant_fields_provided = fields_collection.relevant_fields_provided();
        let raw_inputs = ctx.raw_input();
        let mut validators = Vec::with_capacity(relevant_fields_provided.len());
        let mut validated_inputs = ctx.input();
        let mut validated_outputs = if ctx.is_update() {
            ctx.changes()
        } else {
            ctx.values()
        };
        let mut has_updates = false;

        for field_name in relevant_fields_provided.iter() {
            let field_info = fields_collection.get(field_name);

            if let Some(InternalFieldConfig {
                field_type,
                validator,
                ..
            }) = self.field_configs.get(&field_info.config_name)
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
                    raw_inputs.ivo_internal_get_erased_value(f.name),
                    Arc::clone(&ctx),
                    Arc::clone(&options),
                )
                .await,
            )
        });

        let mut error_tool = ErrorTool::new();

        for (field_info, result) in join_all(tasks).await {
            let field_name = field_info.name;

            match result {
                Err((reason, metadata)) => {
                    error_tool.add(field_name, FieldError { reason, metadata });
                }
                Ok(Some(value)) => {
                    has_updates = true;

                    if field_info.is_input {
                        validated_inputs.ivo_internal_set(field_name, &value);
                    }

                    if field_info.is_output {
                        validated_outputs.ivo_internal_set(field_name, &value);
                    }
                }
                Ok(None) => {
                    if field_info.is_output {
                        has_updates = true;

                        validated_outputs.ivo_internal_set(
                            field_name,
                            &validated_outputs.ivo_internal_get_erased_value(field_name),
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
        fields_collection: &FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: IvoContext<I, O>,
        options: IvoRwCtxOptions<CtxOptions>,
    ) -> Result<Option<(I::Partial, O::Partial)>, ErrorTool::ErrorPayload> {
        let relevant_fields_provided = fields_collection.relevant_fields_provided();
        let mut re_validators = Vec::with_capacity(relevant_fields_provided.len());

        for field_name in relevant_fields_provided.iter() {
            let field_info = fields_collection.get(field_name);

            if let Some(InternalFieldConfig {
                re_validator: Some(re_validator),
                ..
            }) = self.field_configs.get(&field_info.config_name)
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
                    validated_inputs.ivo_internal_get_erased_value(f.name),
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
            let field_name = field_info.name;

            match result {
                Err((reason, metadata)) => {
                    error_tool.add(field_name, FieldError { reason, metadata });
                }
                Ok(Some(value)) => {
                    has_updates = true;

                    if field_info.is_input {
                        validated_inputs.ivo_internal_set(field_name, &value);
                    }

                    if field_info.is_output {
                        validated_outputs.ivo_internal_set(field_name, &value);
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
        fields_collection: &FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: IvoContext<I, O>,
        options: IvoRwCtxOptions<CtxOptions>,
    ) -> Result<Option<(I::Partial, O::Partial)>, ErrorTool::ErrorPayload> {
        let mut pre_validators = vec![];
        let mut post_validators = vec![];

        if let Some(configs) = &self.options.post_validate {
            for PostValidationConfig {
                fields,
                pre_validator,
                validators,
            } in configs
            {
                if fields
                    .iter()
                    .any(|f| fields_collection.is_relevant_config_name(f))
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
                    Err(errors) => {
                        for (field_name, (reason, metadata)) in errors.entries() {
                            let field_info = fields_collection.get(&field_name);

                            if fields.contains(&field_info.config_name) {
                                error_tool.add(&field_name, FieldError { reason, metadata });
                            }
                        }
                    }
                    Ok(Some(updates)) => {
                        for (field_name, value) in updates.ivo_internal_enumerate_fields_available()
                        {
                            let field_info = fields_collection.get(&field_name);

                            if !fields.contains(&field_info.config_name) {
                                continue;
                            }

                            has_updates = true;

                            if field_info.is_input {
                                validated_inputs.ivo_internal_set(field_info.name, &value);
                            }

                            if field_info.is_output {
                                validated_outputs.ivo_internal_set(field_info.name, &value);
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
                Err(errors) => {
                    for (field_name, (reason, metadata)) in errors.entries() {
                        let field_info = fields_collection.get(&field_name);

                        if fields.contains(&field_info.config_name) {
                            error_tool.add(&field_name, FieldError { reason, metadata });
                        }
                    }
                }
                Ok(Some(updates)) => {
                    for (field_name, value) in updates.ivo_internal_enumerate_fields_available() {
                        let field_info = fields_collection.get(&field_name);

                        if !fields.contains(&field_info.config_name) {
                            continue;
                        }

                        has_updates = true;

                        if field_info.is_input {
                            validated_inputs.ivo_internal_set(field_info.name, &value);
                        }

                        if field_info.is_output {
                            validated_outputs.ivo_internal_set(field_info.name, &value);
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
        fields_collection: &FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: IvoContext<I, O>,
        options: IvoRwCtxOptions<CtxOptions>,
    ) -> Option<I::Partial> {
        let relevant_fields_provided = fields_collection.relevant_fields_provided();
        let mut sanitizers = Vec::with_capacity(relevant_fields_provided.len());

        for field_name in relevant_fields_provided.iter() {
            let field_info = fields_collection.get(field_name);

            if let Some(InternalFieldConfig {
                field_type: FieldType::Virtual,
                sanitizer: Some(sanitizer),
                ..
            }) = self.field_configs.get(&field_info.config_name)
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
                    input_values.ivo_internal_get_erased_value(field_info.name),
                    Arc::clone(&ctx),
                    Arc::clone(&options),
                )
                .await,
            )
        });

        let mut input_values = ctx.input();

        for (f, value) in join_all(tasks).await {
            input_values.ivo_internal_set(f.name, &value);
        }

        Some(input_values)
    }

    async fn resolve_dependent_values<'a>(
        &self,
        fields_collection: &FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: IvoContext<I, O>,
        options: IvoRwCtxOptions<CtxOptions>,
    ) -> Option<(O::Partial, HashSet<String>)> {
        let mut resolvers = vec![];
        let previous_values: Option<O::Partial> = ctx.previous_values().map(|v| v.into());
        let is_update = previous_values.as_ref().is_some();
        let previous_values = previous_values.unwrap_or_default();

        for (field_name, config) in self.field_configs.iter() {
            match config {
                InternalFieldConfig {
                    field_type: FieldType::Dependent,
                    depends_on: Some(ref depends_on),
                    resolver: Some(ref resolver),
                    ..
                } if depends_on
                    .iter()
                    .any(|parent| fields_collection.is_relevant_dependent_config_name(parent)) =>
                {
                    if !is_update {
                        resolvers.push((field_name, resolver));

                        continue;
                    }

                    // handle readonly during updates
                    if let InternalFieldConfig {
                        field_type: FieldType::Dependent,
                        default: Some(ValueResolverWithSharedInput::Static(default_value)),
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

        let tasks = resolvers.into_iter().map(async |(field_name, resolver)| {
            (
                field_name,
                resolver(Arc::clone(&ctx), Arc::clone(&options)).await,
            )
        });

        let values = ctx.values();
        let mut updated_values = values.clone();
        let mut fields_changed = HashSet::new();

        for (field_name, value) in join_all(tasks).await {
            // only keep fields that have been updated
            if !values.ivo_internal_is_value_equal(field_name, &value) {
                updated_values.ivo_internal_set(field_name, &value);

                fields_changed.insert(field_name.to_string());
            }
        }

        if fields_changed.is_empty() {
            return None;
        }

        Some((updated_values, fields_changed))
    }

    async fn attach_constants_and_defaults(
        &self,
        mut output: O::Partial,
        input: &I::Partial,
        options: IvoRwCtxOptions<CtxOptions>,
    ) -> O::Partial {
        let mut resolvers = vec![];
        let fields_provided = input.ivo_internal_fields_available();

        for (field_name, config) in self.field_configs.iter() {
            if matches!(config.field_type, FieldType::Lax)
                && fields_provided.contains(&field_name.to_string())
            {
                continue;
            }

            match config {
                InternalFieldConfig {
                    field_type: FieldType::Constant,
                    value,
                    ..
                } => match value {
                    Some(ValueResolverWithSharedInput::Static(value)) => {
                        output.ivo_internal_set(field_name, value);

                        continue;
                    }
                    Some(ValueResolverWithSharedInput::Func(resolver)) => {
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
                    ValueResolverWithSharedInput::Static(value) => {
                        output.ivo_internal_set(field_name, value);
                    }
                    ValueResolverWithSharedInput::Func(resolver) => {
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
        mut fields_collection: FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: IvoContext<I, O>,
        options: IvoRwCtxOptions<CtxOptions>,
    ) -> (
        I::Partial,
        O::Partial,
        FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool>,
    ) {
        let is_update = previous_values.is_some();
        let previous_values = previous_values.cloned().unwrap_or_default();

        let mut tasks = vec![];
        let mut entity_resolvers = vec![];
        let mut input = input_values.clone();
        let mut output = O::Partial::default();
        let mut fields_provided = HashSet::new();
        let mut relevant_fields_provided = HashSet::new();

        if is_update {
            if let Some(ref configs) = self.options.ignore_update {
                for config in configs.iter() {
                    if config.fields.is_empty() {
                        entity_resolvers.push(config.resolver.as_ref());
                    }
                }
            }

            for (field_name, value) in input_values.ivo_internal_enumerate_fields_available() {
                let field_info = fields_collection.get(&field_name);

                if (field_info.is_input && !field_info.is_output)
                    || !previous_values.ivo_internal_is_value_equal(&field_name, &value)
                {
                    relevant_fields_provided.insert(field_name.clone());

                    if field_info.is_output {
                        output.ivo_internal_set(
                            &field_name,
                            &input.ivo_internal_get_erased_value(&field_name),
                        );
                    }
                } else {
                    input.ivo_internal_unset(&field_name);
                }

                fields_provided.insert(field_name);
            }
        } else {
            for field_name in input_values.ivo_internal_fields_available() {
                let field_info = fields_collection.get(&field_name);

                if field_info.is_output {
                    output.ivo_internal_set(
                        &field_name,
                        &input.ivo_internal_get_erased_value(&field_name),
                    );
                }

                fields_provided.insert(field_name.clone());
                relevant_fields_provided.insert(field_name);
            }
        }

        if !entity_resolvers.is_empty() {
            let tasks = entity_resolvers
                .iter()
                .map(|resolver| {
                    resolver(
                        ctx.input(),
                        ctx.full_values().unwrap(),
                        Arc::clone(&options),
                    )
                })
                .collect::<Vec<_>>();

            for ignore in join_all(tasks).await {
                if ignore {
                    return (input, output, fields_collection);
                }
            }
        }

        fields_collection = fields_collection
            .new_with_fields_provided(fields_provided)
            .new_with_relevant_fields_provided(relevant_fields_provided.clone());

        for field_name in fields_collection.relevant_fields_provided() {
            let field_info = fields_collection.get(field_name);

            if let Some(InternalFieldConfig {
                field_type: FieldType::Lax | FieldType::Required | FieldType::Virtual,
                default,
                ignore,
                ignore_init,
                ignore_update,
                ..
            }) = self.field_configs.get(&field_info.config_name)
            {
                if let Some(resolver) = ignore {
                    tasks.push((
                        vec![field_info.name],
                        <BooleanResolver<I, O, CtxOptions> as UniformIgnoreResolver<
                            I,
                            O,
                            CtxOptions,
                            ErrorTool,
                        >>::resolve(
                            resolver, Arc::clone(&ctx), Arc::clone(&options)
                        ),
                    ));

                    continue;
                }

                let source = if is_update {
                    ignore_update
                } else {
                    ignore_init
                };

                match source {
                    Some(IsFieldProvisionEnabled::False) => {
                        input.ivo_internal_unset(field_name);
                        relevant_fields_provided.remove(field_name);

                        if field_info.is_output {
                            output.ivo_internal_unset(field_name);
                        }
                    }
                    Some(IsFieldProvisionEnabled::Func(resolver)) => {
                        tasks.push((
                            vec![field_info.name],
                            <BooleanResolver<I, O, CtxOptions> as UniformIgnoreResolver<
                                I,
                                O,
                                CtxOptions,
                                ErrorTool,
                            >>::resolve(
                                resolver, Arc::clone(&ctx), Arc::clone(&options)
                            ),
                        ));
                    }
                    Some(IsFieldProvisionEnabled::Readonly) if is_update => {
                        if let Some(ValueResolverWithSharedInput::Static(default_value)) = default {
                            // readonly means: only allow update if value prev_value == default_value

                            if !previous_values
                                .ivo_internal_is_value_equal(field_name, default_value)
                            {
                                input.ivo_internal_unset(field_name);
                                relevant_fields_provided.remove(field_name);

                                if field_info.is_output {
                                    output.ivo_internal_unset(field_name);
                                }
                            }

                            continue;
                        }

                        // readonly for required fields
                        input.ivo_internal_unset(field_name);
                        relevant_fields_provided.remove(field_name);

                        if field_info.is_output {
                            output.ivo_internal_unset(field_name);
                        }
                    }
                    _ => {}
                };
            }
        }

        let relevant_config_names = relevant_fields_provided
            .iter()
            .map(|field_name| fields_collection.get(field_name).config_name)
            .collect::<HashSet<_>>();

        if let Some(ref configs) = self.options.ignore {
            for IgnoreOptionConfig { fields, resolver } in configs {
                if fields
                    .iter()
                    .any(|name| relevant_config_names.contains(name))
                {
                    tasks.push((
                        fields.clone(),
                        <BooleanResolver<I, O, CtxOptions> as UniformIgnoreResolver<
                            I,
                            O,
                            CtxOptions,
                            ErrorTool,
                        >>::resolve(
                            resolver, Arc::clone(&ctx), Arc::clone(&options)
                        ),
                    ));
                }
            }
        }

        if is_update {
            if let Some(ref configs) = self.options.ignore_update {
                for IgnoreUpdateOptionConfig { fields, resolver } in configs {
                    if fields
                        .iter()
                        .any(|name| relevant_config_names.contains(name))
                    {
                        tasks.push((
                        fields.clone(),
                        <IgnoreUpdateOptionResolver<I, O, CtxOptions> as UniformIgnoreResolver<
                            I,
                            O,
                            CtxOptions,
                            ErrorTool,
                        >>::resolve(
                            resolver, Arc::clone(&ctx), Arc::clone(&options)
                        ),
                    ));
                    }
                }
            }
        }

        if tasks.is_empty() {
            drop(tasks);

            return (
                input,
                output,
                fields_collection.new_with_relevant_fields_provided(relevant_fields_provided),
            );
        }

        let tasks = tasks
            .into_iter()
            .map(|(names, fut_ignore)| async { (names, fut_ignore.await) });

        for (config_names, ignore) in join_all(tasks).await {
            for config_name in config_names {
                let field_info = fields_collection.get(config_name);
                let field_name = field_info.name;

                if ignore {
                    input.ivo_internal_unset(field_name);
                    relevant_fields_provided.remove(field_name);

                    if field_info.is_output {
                        output.ivo_internal_unset(field_name);
                    }

                    continue;
                }

                relevant_fields_provided.insert(field_name.to_string());
            }
        }

        (
            input,
            output,
            fields_collection.new_with_relevant_fields_provided(relevant_fields_provided),
        )
    }

    async fn evaluate_missing_required_fields<'a>(
        &self,
        fields_collection: &FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: IvoContext<I, O>,
        options: IvoRwCtxOptions<CtxOptions>,
    ) -> Result<(), ErrorTool::ErrorPayload> {
        let mut error_tool = ErrorTool::new();
        let mut resolvers = vec![];
        let is_update = ctx.is_update();

        for (field_name, config) in self.field_configs.iter() {
            if fields_collection.is_relevant_config_name(field_name) {
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

        let mut tasks = resolvers
            .iter()
            .map(|(field_name, resolver)| {
                let field_info = fields_collection.get(field_name);

                let r = <RequiredResolver<I, O, CtxOptions> as UniformRequiredResolver<
                    I,
                    O,
                    CtxOptions,
                    ErrorTool,
                >>::resolve(
                    resolver,
                    HashSet::from([field_info.name]),
                    Arc::clone(&ctx),
                    Arc::clone(&options),
                );

                r
            })
            .collect::<Vec<_>>();

        if let Some(ref configs) = self.options.required {
            for config in configs {
                if config
                    .fields
                    .iter()
                    .any(|field_name| fields_collection.is_relevant_config_name(field_name))
                {
                    continue;
                }

                let field_names = config
                    .fields
                    .iter()
                    .map(|config_name| fields_collection.get(config_name).name)
                    .collect::<HashSet<_>>();

                let r = <RequiredOptionConfig<I, O, CtxOptions, ErrorTool> as UniformRequiredResolver<
                    I,
                    O,
                    CtxOptions,
                    ErrorTool,
                >>::resolve(
                    config,
                    field_names,
                    Arc::clone(&ctx),
                    Arc::clone(&options),
                );

                tasks.push(r);
            }
        }

        if tasks.is_empty() {
            if error_tool.has_errors() {
                return Err(error_tool.payload());
            }

            return Ok(());
        }

        for values in join_all(tasks).await {
            for (field_name, error) in values.unwrap_or_default() {
                error_tool.add(&field_name, error);
            }
        }

        if error_tool.has_errors() {
            return Err(error_tool.payload());
        }

        Ok(())
    }

    fn prepare_failure_handlers<'a>(
        &self,
        fields_collection: FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: IvoContext<I, O>,
        options: IvoCtxOptions<CtxOptions>,
    ) -> AsyncHandlerTrigger<'_> {
        let fields_provided = fields_collection.fields_provided();

        if fields_provided.is_empty() {
            return Box::new(|| Box::pin(ready(())));
        }

        let mut handlers = Vec::with_capacity(fields_provided.len());

        for field_name in fields_provided {
            let field_info = fields_collection.get(field_name);

            if let Some(InternalFieldConfig {
                on_failure_fns: Some(h_vec),
                ..
            }) = self.field_configs.get(field_info.config_name)
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
        fields_collection: FieldInfoCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: IvoContext<I, O>,
        options: IvoCtxOptions<CtxOptions>,
    ) -> AsyncHandlerTrigger<'_> {
        let mut relevant_success_fields = fields_collection
            .relevant_fields_provided()
            .iter()
            // some relevant fields may be virtual aliases
            .map(|name| fields_collection.get(name).config_name.to_string())
            .collect::<HashSet<_>>();

        let candidate_field_names = if ctx.is_update() {
            ctx.changes().ivo_internal_fields_available()
        } else {
            ctx.values().ivo_internal_fields_available()
        };

        for field_name in candidate_field_names {
            relevant_success_fields.insert(field_name);
        }

        let mut handlers = vec![];

        for field_name in relevant_success_fields.iter() {
            if let Some(InternalFieldConfig {
                on_success_fns: Some(h_vec),
                ..
            }) = self.field_configs.get(field_name.as_str())
            {
                handlers.extend(h_vec)
            }
        }

        if let Some(configs) = &self.options.on_success_fns {
            for OnSuccessConfig {
                fields,
                handlers: h_vec,
            } in configs
            {
                if fields.is_empty() || fields.iter().any(|f| relevant_success_fields.contains(*f))
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
        }) = self.timestamp_configs.as_ref()
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
fn unwrap_async_lock<T>(lock: Arc<IvoRwLock<T>>) -> T {
    match Arc::into_inner(lock).unwrap().try_unwrap() {
        Ok(raw_data) => raw_data,
        _ => panic!("error unwrapping shared IvoRwLock"),
    }
}
