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
use crate::schema::options::types::OnSuccessConfig;
use crate::utils::erased_value::ErasedValue;
use crate::{
    IvoContext, SharedCtxOptions, SharedIvoContext, SharedIvoMiniContext, SharedRwCtxOptions,
};

use futures::future::{join_all, BoxFuture};

use crate::types::{IvoSchemaStruct, Partial, PartialFromToMap, PartialMapOfErasedValues, RwLock};

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
        let shared_rw_options = Arc::new(RwLock::new(options.clone()));
        let mini_ctx = Arc::new(input.clone());

        // 1) Resolve constants & defaults
        let default_values = self
            .resolve_constants_and_defaults(mini_ctx, Arc::clone(&shared_rw_options))
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

        let fields_provided = self.parse_fields_provided(&erased_input_values);

        self.evaluate_missing_required_fields(
            &fields_provided,
            Arc::clone(&ctx),
            Arc::clone(&shared_rw_options),
        )
        .await
        .map_err(|payload| {
            (
                payload,
                self.prepare_failure_handlers(
                    fields_provided.clone(),
                    ctx.clone(),
                    Arc::new(options.clone()),
                ),
            )
        })?;

        // 3) Run validators
        let (validated_inputs, validated_outputs, should_gen_new_ctx) = self
            .validate(
                &fields_provided,
                &erased_input_values,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
            .map_err(|payload| {
                (
                    payload,
                    self.prepare_failure_handlers(
                        fields_provided.clone(),
                        ctx.clone(),
                        Arc::new(options.clone()),
                    ),
                )
            })?;

        if should_gen_new_ctx {
            ctx = Arc::new(IvoContext::<I, O>::new_create_ctx(
                validated_inputs,
                ctx.input_values(),
                validated_outputs,
            ));
        }

        // 4) Run re_validators
        let (validated_inputs, validated_outputs, should_gen_new_ctx) = self
            .re_validate(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
            .map_err(|payload| {
                (
                    payload,
                    self.prepare_failure_handlers(
                        fields_provided.clone(),
                        ctx.clone(),
                        Arc::new(options.clone()),
                    ),
                )
            })?;

        if should_gen_new_ctx {
            ctx = Arc::new(IvoContext::<I, O>::new_create_ctx(
                validated_inputs,
                ctx.input_values(),
                validated_outputs,
            ));
        }

        // 5) Run post-validators

        // 6) Sanitize virtuals

        // 7) Resolve values of dependent fields

        // 8) Generate and set timestamps

        return Ok((
            O::ivo_internal_dangerously_get_values_from_partial(ctx.values()),
            self.prepare_success_handlers(fields_provided, ctx, Arc::new(options)),
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

        let fields_provided = self.parse_fields_provided(&erased_input_values);

        // if the updates provided are all none, the nothing to update
        if fields_provided.fields.is_empty() {
            return Err((
                UpdateError::NothingToUpdate,
                self.prepare_failure_handlers(fields_provided, ctx, Arc::new(options)),
            ));
        }

        // let updatables = data.ivo_internal_get_erased_updates_from_erased_values(&updatables);

        let shared_rw_options = Arc::new(RwLock::new(options.clone()));

        // 1) Evaluate missing required fields
        self.evaluate_missing_required_fields(
            &fields_provided,
            Arc::clone(&ctx),
            Arc::clone(&shared_rw_options),
        )
        .await
        .map_err(|p| {
            (
                UpdateError::ValidationError(p),
                self.prepare_failure_handlers(
                    fields_provided.clone(),
                    ctx.clone(),
                    Arc::new(options.clone()),
                ),
            )
        })?;

        // 2) Run validators
        let (validated_inputs, validated_outputs, should_gen_new_ctx) = self
            .validate(
                &fields_provided,
                &erased_input_values,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
            .map_err(|payload| {
                (
                    UpdateError::ValidationError(payload),
                    self.prepare_failure_handlers(
                        fields_provided.clone(),
                        ctx.clone(),
                        Arc::new(options.clone()),
                    ),
                )
            })?;

        drop(erased_input_values);

        if should_gen_new_ctx {
            ctx = Arc::new(IvoContext::<I, O>::new_update_ctx(
                validated_outputs.clone(),
                validated_inputs,
                ctx.input_values(),
                data.clone(),
                data.clone(),
            ));
        }

        // 3) Run re_validators
        let (validated_inputs, validated_outputs, should_gen_new_ctx) = self
            .re_validate(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await
            .map_err(|payload| {
                (
                    UpdateError::ValidationError(payload),
                    self.prepare_failure_handlers(
                        fields_provided.clone(),
                        ctx.clone(),
                        Arc::new(options.clone()),
                    ),
                )
            })?;

        if should_gen_new_ctx {
            ctx = Arc::new(IvoContext::<I, O>::new_update_ctx(
                validated_outputs.clone(),
                validated_inputs,
                ctx.input_values(),
                data.clone(),
                data.clone(),
            ));
        }

        // 4) Run post-validators

        // 5) Sanitize virtuals

        // 6) Resolve values of dependent fields

        let (updated_values, has_updated_fields) =
            data.ivo_internal_get_updates_from_partial(&validated_outputs);

        if !has_updated_fields {
            return Err((
                UpdateError::NothingToUpdate,
                self.prepare_failure_handlers(fields_provided, ctx, Arc::new(options)),
            ));
        }

        // 7) Generate and set timestamps

        Ok((
            updated_values,
            self.prepare_success_handlers(fields_provided, ctx, Arc::new(options)),
        ))
    }

    pub async fn delete(&self, data: O, options: CtxOptions) {
        let data = Arc::new(data);
        let options = Arc::new(options);
        let mut handlers = vec![];

        for (_, config) in self.schema.get_field_configs() {
            match &config.on_delete_fns {
                Some(h_vec) => {
                    handlers.extend(h_vec);
                    continue;
                }
                _ => {}
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

    async fn validate(
        &self,
        fields_provided: &InputFieldCollection,
        erased_input_values: &PartialMapOfErasedValues,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> Result<(I::Partial, O::Partial, bool), ErrorTool::ErrorPayload> {
        let mut validators = vec![];

        for field_info in fields_provided.fields.iter() {
            if let Some(InternalFieldConfig { validator, .. }) =
                self.schema.get_field_config(&field_info.config_name)
            {
                if let Some(validator) = validator {
                    validators.push((field_info, validator));

                    continue;
                }
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

        if error_tool.is_loaded() {
            return Err(error_tool.payload());
        }

        Ok(self.parse_ctx_values(ctx, validated_inputs, validated_outputs))
    }

    async fn re_validate(
        &self,
        fields_provided: &InputFieldCollection,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> Result<(I::Partial, O::Partial, bool), ErrorTool::ErrorPayload> {
        let mut re_validators = vec![];

        let erased_input_values: PartialMapOfErasedValues =
            ctx.input().ivo_internal_to_optional_erased_map();

        for field_info in fields_provided.fields.iter() {
            if let Some(InternalFieldConfig { re_validator, .. }) =
                self.schema.get_field_config(&field_info.config_name)
            {
                if let Some(re_validator) = re_validator {
                    re_validators.push((field_info, re_validator));

                    continue;
                }
            }
        }

        if re_validators.is_empty() {
            return Ok((ctx.input(), ctx.values(), false));
        }

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

        if error_tool.is_loaded() {
            return Err(error_tool.payload());
        }

        Ok(self.parse_ctx_values(ctx, validated_inputs, validated_outputs))
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

    async fn evaluate_missing_required_fields(
        &self,
        fields_provided: &InputFieldCollection,
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
            if error_tool.is_loaded() {
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

        if error_tool.is_loaded() {
            return Err(error_tool.payload());
        }

        return Ok(());
    }

    fn prepare_failure_handlers(
        &self,
        fields_provided: InputFieldCollection,
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

    fn prepare_success_handlers(
        &self,
        fields_provided: InputFieldCollection,
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

    fn parse_ctx_values(
        &self,
        ctx: SharedIvoContext<I, O>,
        validated_inputs: HashMap<String, ErasedValue>,
        validated_outputs: HashMap<String, ErasedValue>,
    ) -> (I::Partial, O::Partial, bool) {
        let mut old_outputs = ctx.values().ivo_internal_to_optional_erased_map();

        for (field, value) in validated_outputs {
            old_outputs
                .inner
                .entry(field)
                .and_modify(|e| *e = value.clone())
                .or_insert(value);
        }

        let mut old_inputs = ctx.input().ivo_internal_to_optional_erased_map();

        for (field, value) in validated_inputs {
            old_inputs
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

    fn parse_fields_provided(
        &self,
        erased_input_values: &PartialMapOfErasedValues,
    ) -> InputFieldCollection {
        let fields_names = erased_input_values
            .inner
            .keys()
            .map(|f| f.to_owned())
            .collect::<Vec<String>>();

        let schema_output_fields = O::ivo_internal_field_names();
        let schema_input_fields = I::ivo_internal_field_names();

        let mut fields = Vec::with_capacity(fields_names.len());

        for field_name in fields_names.iter() {
            if let Some(InternalFieldConfig { depends_on, .. }) =
                self.schema.get_field_config(field_name)
            {
                if depends_on.is_none() {
                    fields.push(InputFieldInfo {
                        config_name: field_name.clone(),
                        is_input: schema_input_fields.contains(field_name),
                        is_output: schema_output_fields.contains(field_name),
                        name: field_name.clone(),
                    });

                    continue;
                }

                // otherwise, field_name is an alias for a virtual field
                // the current config depends on
                if let Some(depends_on) = depends_on {
                    for parent_name in depends_on {
                        match self.schema.get_field_config(parent_name) {
                            Some(InternalFieldConfig {
                                alias: Some(alias),
                                field_type: FieldType::Virtual,
                                validator: Some(validator),
                                ..
                            }) if alias == field_name => {
                                fields.push(InputFieldInfo {
                                    config_name: parent_name.to_string(),
                                    is_input: true,
                                    is_output: false,
                                    name: field_name.clone(),
                                });
                                continue;
                            }
                            _ => {}
                        }
                    }

                    continue;
                }
            }
        }

        InputFieldCollection::new(fields)
    }
}
