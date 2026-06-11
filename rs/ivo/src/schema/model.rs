use crate::internal::InternalIvoContextMethods;
use crate::schema::core::Schema;
use crate::schema::error::{DefaultErrorTool, FieldError, IvoErrorTool, UpdateError};
use crate::schema::fields::base::{FieldType, InternalFieldConfig};
use crate::schema::fields::types::{
    ComputableRequired, ComputableRequiredError, ComputableWithMiniContext,
};
use crate::schema::internal::SchemaInternals;
use crate::utils::erased_value::ErasedValue;
use crate::{
    IvoContext, SharedCtxOptions, SharedIvoContext, SharedIvoMiniContext, SharedRwCtxOptions,
};

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

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

        // 1.) resolve constants & defaults
        let default_values = self
            .resolve_constants_and_defaults(mini_ctx, Arc::clone(&shared_rw_options))
            .await;

        let default_values =
            O::Partial::ivo_internal_from_optional_erased_map(PartialMapOfErasedValues {
                inner: default_values,
            });

        // println!("default values: {default_values:?}");

        let ctx = Arc::new(IvoContext::<I, O>::new_create_ctx(
            input.clone(),
            input.clone(),
            default_values,
        ));

        let erased_input_values = input.ivo_internal_to_optional_erased_map();

        // 2.) evaluate missing required fields
        let fields_provided = erased_input_values
            .inner
            .keys()
            .map(|f| f.to_owned())
            .collect::<Vec<String>>();

        let error_tool = self
            .evaluate_missing_required_fields(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if error_tool.is_loaded() {
            return Err((
                error_tool.payload(),
                self.prepare_failure_handlers(fields_provided, ctx, Arc::new(options)),
            ));
        }

        // Run validators for props in context
        let validation = self
            .run_validators(
                &fields_provided,
                &erased_input_values,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if validation.is_err() {
            return Err((
                validation.err().unwrap(),
                self.prepare_failure_handlers(fields_provided, ctx, Arc::new(options)),
            ));
        }

        let validated = validation.ok().unwrap();

        let ctx = Arc::new(IvoContext::<I, O>::new_create_ctx(
            validated.0,
            ctx.input_values(),
            validated.1,
        ));

        // self.add_timestamps(&mut context);

        return Ok((
            O::ivo_internal_dangerously_get_values_from_partial(ctx.values()),
            self.prepare_failure_handlers(fields_provided, ctx, Arc::new(options)),
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
        let ctx = Arc::new(IvoContext::<I, O>::new_update_ctx(
            O::Partial::default(),
            updates.clone(),
            updates.clone(),
            data.clone(),
            data.clone(),
        ));

        let erased_input_values = updates.ivo_internal_to_optional_erased_map();

        let fields_provided = erased_input_values
            .inner
            .keys()
            .map(|f| f.to_owned())
            .collect::<Vec<String>>();

        // if the updates provided are all none, the nothing to update
        if fields_provided.is_empty() {
            return Err((
                UpdateError::NothingToUpdate,
                self.prepare_failure_handlers(fields_provided, ctx, Arc::new(options)),
            ));
        }

        // let updatables = data.ivo_internal_get_erased_updates_from_erased_values(&updatables);

        let shared_rw_options = Arc::new(RwLock::new(options.clone()));

        // 2.) evaluate missing required fields
        let error_tool = self
            .evaluate_missing_required_fields(
                &fields_provided,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if error_tool.is_loaded() {
            return Err((
                UpdateError::ValidationError(error_tool.payload()),
                self.prepare_failure_handlers(fields_provided, ctx, Arc::new(options)),
            ));
        }

        // Run validators for props in context
        let validation = self
            .run_validators(
                &fields_provided,
                &erased_input_values,
                Arc::clone(&ctx),
                Arc::clone(&shared_rw_options),
            )
            .await;

        if validation.is_err() {
            return Err((
                UpdateError::ValidationError(validation.err().unwrap()),
                self.prepare_failure_handlers(fields_provided, ctx, Arc::new(options)),
            ));
        }

        let (validated_inputs, validated_outputs) = validation.ok().unwrap();

        let ctx = Arc::new(IvoContext::<I, O>::new_update_ctx(
            validated_outputs.clone(),
            validated_inputs,
            ctx.input_values(),
            data.clone(),
            data.clone(),
        ));

        let (updated_values, has_updated_fields) =
            data.ivo_internal_get_updates_from_partial(&validated_outputs);

        if !has_updated_fields {
            return Err((
                UpdateError::NothingToUpdate,
                self.prepare_failure_handlers(fields_provided, ctx, Arc::new(options)),
            ));
        }

        // assign timestamps if configured

        // return updated values
        Ok((
            updated_values,
            self.prepare_failure_handlers(fields_provided, ctx, Arc::new(options)),
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

    async fn run_validators(
        &self,
        fields: &Vec<String>,
        erased_input_values: &PartialMapOfErasedValues,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> Result<(I::Partial, O::Partial), ErrorTool::ErrorPayload> {
        let mut validators = vec![];
        let schema_output_fields = O::ivo_internal_field_names();
        let schema_input_fields = I::ivo_internal_field_names();

        for field_name in fields {
            if let Some(InternalFieldConfig {
                depends_on,
                validator,
                ..
            }) = self.schema.get_field_config(field_name)
            {
                if let Some(validator) = validator {
                    validators.push((
                        field_name,
                        validator,
                        schema_output_fields.contains(field_name),
                        schema_input_fields.contains(field_name),
                    ));

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
                                validators.push((field_name, validator, false, true));

                                continue;
                            }
                            _ => {}
                        }
                    }

                    continue;
                }
            }
        }

        if validators.is_empty() {
            return Ok(self.parse_ctx_values(ctx, HashMap::new(), HashMap::new()));
        }

        let tasks =
            validators
                .into_iter()
                .map(async |(field_name, validator, is_output, is_input)| {
                    (
                        field_name,
                        validator(
                            erased_input_values.inner.get(field_name).cloned().unwrap(),
                            Arc::clone(&ctx),
                            Arc::clone(&options),
                        )
                        .await,
                        is_output,
                        is_input,
                    )
                });

        let mut error_tool = ErrorTool::new();
        let mut validated_outputs = HashMap::new();
        let mut validated_inputs = HashMap::new();

        for (field_name, result, is_output, is_input) in join_all(tasks).await {
            match result {
                Err((reason, metadata)) => {
                    error_tool.add(field_name, FieldError { reason, metadata });
                }
                Ok(value) => {
                    if is_output {
                        validated_outputs.insert(field_name.to_owned(), value.clone());
                    }

                    if is_input {
                        validated_inputs.insert(field_name.to_owned(), value);
                    }
                }
            }
        }

        if error_tool.is_loaded() {
            return Err(error_tool.payload());
        }

        Ok(self.parse_ctx_values(ctx, validated_inputs, validated_outputs))
    }

    fn parse_ctx_values(
        &self,
        ctx: SharedIvoContext<I, O>,
        validated_inputs: HashMap<String, ErasedValue>,
        validated_outputs: HashMap<String, ErasedValue>,
    ) -> (I::Partial, O::Partial) {
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
        )
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
        fields_provided: &Vec<String>,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> ErrorTool {
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
            return error_tool;
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

        error_tool
    }

    fn prepare_failure_handlers(
        &self,
        fields_provided: Vec<String>,
        ctx: SharedIvoContext<I, O>,
        options: SharedCtxOptions<CtxOptions>,
    ) -> AsyncHandlerTrigger<'schema> {
        let mut handlers = vec![];

        for (field_name, config) in self.schema.get_field_configs() {
            if !fields_provided.contains(field_name) {
                continue;
            }

            if let Some(h_vec) = &config.on_failure_fns {
                handlers.extend(h_vec)
            }
        }

        Box::new(move || {
            let tasks = handlers
                .iter()
                .map(|h| h(Arc::clone(&ctx), Arc::clone(&options)))
                .collect::<Vec<_>>();

            Box::pin(async { for _ in join_all(tasks).await {} })
        })
    }

    fn _add_timestamps(&self, _data: &mut O::Partial) {
        // if self.schema.timestamp_tool.with_timestamps() {
        //     let now = chrono::Utc::now().to_rfc3339();

        //     let keys = &self.schema.timestamp_tool.get_keys();

        //     if let Some(created_at_key) = keys.created_at.clone() {
        //         context.insert(created_at_key, ErasedValue::String(now.clone()));
        //     }

        //     if let Some(updated_at_key) = keys.updated_at.clone() {
        //         context.insert(updated_at_key, ErasedValue::String(now));
        //     }
        // }
    }

    /// Resolve defaults iteratively based on dependencies.
    /// It will repeatedly evaluate defaults whose dependencies are satisfied (present in `context`).
    /// If there are unresolved defaults at the end and the schema option `error_on_unresolved_defaults` is true,
    /// this function returns Err(SchemaError) containing the unresolved properties; otherwise returns Ok(()).
    /// Return whether a name is a defined property
    fn _is_prop(&self, prop: &str) -> bool {
        self.schema.props.contains(prop)
    }

    /// Return whether a name is a defined virtual
    fn _is_virtual(&self, prop: &str) -> bool {
        self.schema.virtuals.contains(prop)
    }

    /// Return whether a name is a defined constant
    fn _is_constant(&self, prop: &str) -> bool {
        self.schema.constants.contains(prop)
    }

    /// Resolve defaults iteratively based on dependencies.
    /// It will repeatedly evaluate defaults whose dependencies are satisfied (present in `context`).
    /// If unresolved defaults remain and schema option `error_on_unresolved_defaults` is true,
    /// returns Err(SchemaError) listing the unresolved props.
    pub fn resolve_defaults(&self, context: &mut HashMap<String, ErasedValue>) {
        let mut _pending: HashSet<String> = self
            .schema
            .get_field_configs()
            .iter()
            .filter_map(|(k, def)| {
                if def.default.is_some() {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .filter(|k| !context.contains_key(k))
            .collect();
    }

    /// Resolve constants iteratively; constants may depend on other values in context.
    /// If unresolved constants remain and schema option `error_on_unresolved_constants` is true,
    /// returns Err(SchemaError) listing unresolved constants; otherwise returns Ok(())
    pub fn resolve_constants(&self, context: &mut HashMap<String, ErasedValue>) {
        let mut _pending: HashSet<String> = self
            .schema
            .constants
            .iter()
            .filter(|k| !context.contains_key(*k))
            .cloned()
            .collect();

        // todo!()
    }
}
