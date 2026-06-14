use std::collections::HashMap;
use std::future::ready;
use std::sync::Arc;

use crate::internal::InternalIvoContextMethods;
use crate::schema::core::Schema;
use crate::schema::error::{DefaultErrorTool, FieldError, IvoErrorTool, UpdateError};
use crate::schema::fields::base::{FieldType, InternalFieldConfig};
use crate::schema::fields::types::{
    ComputableRequired, ComputableRequiredError, ComputableWithMiniContext,
};

use crate::schema::internal::{InputFieldCollection, InputFieldInfo, SchemaInternals};
use crate::schema::options::types::{OnSuccessConfig, PostValidationConfig};
use crate::utils::erased_value::ErasedValue;
use crate::{
    IvoContext, SharedCtxOptions, SharedIvoContext, SharedIvoMiniContext, SharedRwCtxOptions,
};

use futures::future::{join_all, BoxFuture};
use futures::FutureExt;

use crate::types::{
    IvoSchemaStruct, Partial, PartialFromToMap, PartialMapOfErasedValues, RwLock,
    WithUpdateDetailsForPartials,
};

type AsyncHandlerTrigger<'a> = Box<dyn Fn() -> BoxFuture<'a, ()> + Send + Sync + 'a>;

impl<
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone + Sync + Send,
        ErrorTool: IvoErrorTool,
    > Schema<I, O, CtxOptions, ErrorTool>
{
    pub fn get_model(&self) -> Model<'_, I, O, CtxOptions, ErrorTool> {
        Model { schema: self }
    }
}

pub struct Model<
    'schema,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct = I,
    CtxOptions: Clone + Sync + Send = HashMap<String, ()>,
    ErrorTool: IvoErrorTool = DefaultErrorTool,
> {
    schema: &'schema Schema<I, O, CtxOptions, ErrorTool>,
}

impl<
        'schema,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone + Sync + Send,
        ErrorTool: IvoErrorTool,
    > Model<'schema, I, O, CtxOptions, ErrorTool>
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
        let fields_provided = self.make_input_fields_collection(&erased_input_values);

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
                    fields_provided,
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
                    fields_provided,
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
                    fields_provided,
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
                    fields_provided,
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

        let erased_input_values = updates.ivo_internal_to_optional_erased_map();
        let fields_provided = self.make_input_fields_collection(&erased_input_values);

        // if the updates provided are all none, the nothing to update
        if fields_provided.fields.is_empty() {
            return Err((
                UpdateError::NothingToUpdate,
                self.prepare_failure_handlers(fields_provided, ctx, Arc::new(options)),
            ));
        }

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
                    fields_provided,
                    ctx,
                    Arc::new((shared_rw_options).read().await.clone()),
                ),
            ));
        }

        // 2) Run validators
        let r = self
            .validate(
                &fields_provided,
                &erased_input_values,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        drop(erased_input_values);

        if r.is_err() {
            return Err((
                UpdateError::ValidationError(r.err().unwrap()),
                self.prepare_failure_handlers(
                    fields_provided,
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
                    fields_provided,
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
                    fields_provided,
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
            ctx = Arc::new(IvoContext::<I, O>::new_create_ctx(
                validated_inputs,
                ctx.input_values(),
                validated_outputs,
            ));
        }
        // 6) Resolve values of dependent fields

        // println!("\n----------------------------------------------------------------------------------------------------------------------------------");

        // println!(
        //     "initial data: {:?} \n\ninput_values: {:?} \n\nchanges: {:?} \n\nvalidated inputs: {:?} \n\nvalidated outputs: {:?} \n\ninputs: {:?} \n\nvalues: {:?} \n\n should_gen_new_ctx: {}",
        //     data,
        //     ctx.input_values(),
        //     ctx.changes(),
        //     validated_inputs,
        //     validated_outputs,
        //     ctx.input(),
        //     ctx.values(),
        //     should_gen_new_ctx
        // );

        // println!("----------------------------------------------------------------------------------------------------------------------------------\n");

        let (updated_values, has_updated_fields) =
            data.ivo_internal_get_updates_from_partial(&ctx.changes());

        if !has_updated_fields {
            return Err((
                UpdateError::NothingToUpdate,
                self.prepare_failure_handlers(
                    fields_provided,
                    ctx,
                    Arc::new(unwrap_async_lock(shared_rw_options)),
                ),
            ));
        }

        // 7) Generate and set timestamps

        Ok((
            updated_values,
            self.prepare_success_handlers(
                fields_provided,
                ctx,
                Arc::new(unwrap_async_lock(shared_rw_options)),
            ),
        ))
    }

    pub async fn delete(&self, data: O, options: CtxOptions) {
        let data = Arc::new(data);
        let options = Arc::new(options);
        let mut handlers = vec![];

        for (_, config) in self.schema.get_field_configs() {
            if let Some(h_vec) = &config.on_delete_fns {
                handlers.extend(h_vec);

                continue;
            }
        }

        if let Some(h_vec) = &self.schema.options().on_delete_fns {
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
        fields_provided: &'a InputFieldCollection<'a, I, O, CtxOptions, ErrorTool>,
        erased_input_values: &PartialMapOfErasedValues,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> Result<(I::Partial, O::Partial, bool), ErrorTool::ErrorPayload> {
        let mut validators = vec![];

        for field_info in fields_provided.fields.iter() {
            if let Some(InternalFieldConfig {
                validator: Some(validator),
                ..
            }) = self.schema.get_field_config(&field_info.config_name)
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

        Ok(self.merge_ctx_values(ctx, validated_inputs, validated_outputs))
    }

    async fn re_validate<'a>(
        &self,
        fields_provided: &'a InputFieldCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> Result<(I::Partial, O::Partial, bool), ErrorTool::ErrorPayload> {
        let mut re_validators = vec![];

        for field_info in fields_provided.fields.iter() {
            if let Some(InternalFieldConfig {
                re_validator: Some(re_validator),
                ..
            }) = self.schema.get_field_config(&field_info.config_name)
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

        Ok(self.merge_ctx_values(ctx, validated_inputs, validated_outputs))
    }

    async fn post_validate<'a>(
        &self,
        fields_provided: &'a InputFieldCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> Result<(I::Partial, O::Partial, bool), ErrorTool::ErrorPayload> {
        let mut pre_validators = vec![];
        let mut post_validators = vec![];

        if let Some(configs) = &self.schema.options().post_validate {
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
                self.merge_ctx_values(ctx.clone(), validated_inputs, validated_outputs);

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

        Ok(self.merge_ctx_values(ctx, validated_inputs, validated_outputs))
    }

    async fn sanitize_virtuals<'a>(
        &self,
        fields_provided: &'a InputFieldCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> (I::Partial, O::Partial, bool) {
        let mut sanitizers = vec![];

        for field_info in fields_provided.fields.iter() {
            if let Some(InternalFieldConfig {
                field_type: FieldType::Virtual,
                sanitizer: Some(sanitizer),
                ..
            }) = self.schema.get_field_config(&field_info.config_name)
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

        self.merge_ctx_values(ctx, validated_inputs, HashMap::new())
    }

    async fn resolve_dependent_values<'a>(
        &self,
        fields_provided: &'a InputFieldCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> (I::Partial, O::Partial, bool) {
        let mut resolvers = vec![];

        for (field_name, config) in self.schema.get_field_configs() {
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
                    .any(|parent| fields_provided.contains(&parent.to_string()))
                {
                    resolvers.push((field_name, resolver.as_ref().unwrap()));
                }
            }
        }

        if resolvers.is_empty() {
            return (ctx.input(), ctx.values(), false);
        }

        let tasks = resolvers.into_iter().map(async |(field_info, resolver)| {
            (
                field_info,
                resolver(Arc::clone(&ctx), Arc::clone(&options)).await,
            )
        });

        let values = ctx.values();
        let mut validated_outputs = HashMap::new();
        let mut resolved = vec![];

        for (field_name, value) in join_all(tasks).await {
            // only keep fields that have been updated
            if !values.ivo_internal_is_value_equal(field_name, &value) {
                validated_outputs.insert(field_name.clone(), value.clone());

                resolved.push(InputFieldInfo {
                    config_name: field_name.clone(),
                    is_input: false,
                    is_output: true,
                    name: field_name.clone(),
                });
            }
        }

        // resolve recurssively here

        self.merge_ctx_values(ctx, HashMap::new(), validated_outputs)
    }

    async fn resolve_constants_and_defaults(
        &self,
        mini_ctx: SharedIvoMiniContext<I>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> HashMap<String, ErasedValue> {
        let mut default_values = HashMap::new();
        let mut resolvers = vec![];

        for (field_name, config) in self.schema.get_field_configs() {
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
        fields_provided: &'a InputFieldCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> Result<(), ErrorTool::ErrorPayload> {
        let mut error_tool = ErrorTool::new();
        let mut resolvers = vec![];
        let is_update = ctx.is_update();

        for (field_name, config) in self.schema.get_field_configs() {
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

                    match &required_error {
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
                // conditionally required configs
                InternalFieldConfig {
                    field_type: FieldType::Lax | FieldType::Virtual,
                    required,
                    ..
                } => {
                    match &required {
                        Some(ComputableRequired::Func(resolver)) => {
                            resolvers.push((field_name, resolver));
                        }
                        _ => {}
                    }

                    continue;
                }
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

    fn prepare_failure_handlers<'a>(
        &self,
        fields_provided: InputFieldCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: SharedIvoContext<I, O>,
        options: SharedCtxOptions<CtxOptions>,
    ) -> AsyncHandlerTrigger<'schema> {
        let mut handlers = vec![];

        for field_info in fields_provided.fields.iter() {
            if let Some(InternalFieldConfig {
                on_failure_fns: Some(h_vec),
                ..
            }) = self.schema.get_field_config(&field_info.config_name)
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
        fields_provided: InputFieldCollection<'a, I, O, CtxOptions, ErrorTool>,
        ctx: SharedIvoContext<I, O>,
        options: SharedCtxOptions<CtxOptions>,
    ) -> AsyncHandlerTrigger<'schema> {
        let mut handlers = vec![];

        for field_info in fields_provided.fields.iter() {
            if let Some(InternalFieldConfig {
                on_success_fns: Some(h_vec),
                ..
            }) = self.schema.get_field_config(&field_info.config_name)
            {
                handlers.extend(h_vec)
            }
        }

        if let Some(configs) = &self.schema.options().on_success_fns {
            for OnSuccessConfig {
                fields,
                handlers: h_vec,
            } in configs
            {
                if fields
                    .iter()
                    .any(|f| fields_provided.contains(&f.to_string()))
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

    fn merge_ctx_values(
        &self,
        ctx: SharedIvoContext<I, O>,
        validated_inputs: HashMap<String, ErasedValue>,
        validated_outputs: HashMap<String, ErasedValue>,
    ) -> (I::Partial, O::Partial, bool) {
        // let (updated_inputs, do_inputs_have_updates) = ctx
        //     .input()
        //     .ivo_internal_clone_with_erased_updates(&validated_inputs);

        // let (updated_outputs, do_outputs_have_updates) = ctx
        //     .values()
        //     .ivo_internal_clone_with_erased_updates(&validated_outputs);

        // (
        //     updated_inputs,
        //     updated_outputs,
        //     do_inputs_have_updates || do_outputs_have_updates,
        // )
        let mut old_inputs = ctx.input().ivo_internal_to_optional_erased_map();

        for (field, value) in validated_inputs {
            old_inputs
                .inner
                .entry(field)
                .and_modify(|e| *e = value.clone())
                .or_insert(value);
        }

        let mut old_outputs = ctx.values().ivo_internal_to_optional_erased_map();

        for (field, value) in validated_outputs {
            old_outputs
                .inner
                .entry(field)
                .and_modify(|e| *e = value.clone())
                .or_insert(value);
        }

        (
            I::Partial::ivo_internal_from_optional_erased_map(old_inputs),
            O::Partial::ivo_internal_from_optional_erased_map(old_outputs),
            true,
        )
    }

    fn make_input_fields_collection(
        &self,
        erased_input_values: &PartialMapOfErasedValues,
    ) -> InputFieldCollection<'_, I, O, CtxOptions, ErrorTool> {
        InputFieldCollection::new(self.schema, erased_input_values)
    }
}

/// this is a sync alternative to: shared_rw_options.read().await.clone()
fn unwrap_async_lock<T>(lock: Arc<RwLock<T>>) -> T {
    match Arc::into_inner(lock).unwrap().try_unwrap() {
        Ok(raw_data) => raw_data,
        _ => panic!("error unwrapping shared RwLock"),
    }
}
