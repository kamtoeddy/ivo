use crate::internal::InitializableIvoContext;
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

        println!("default values: {default_values:?}");

        let ctx = Arc::new(IvoContext::<I, O>::for_new(
            input.clone(),
            input.clone(),
            default_values,
        ));

        // 2.) evaluate missing required fields
        let fields_provided = input
            .ivo_internal_to_optional_erased_map()
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

        let mut error_tool = ErrorTool::new();
        // let input_values = input.ivo_internal_to_optional_erased_map();

        // println!();
        // for _ in input_values.inner {
        //     // println!("'{k}' was provided");
        // }
        // println!();

        // Build initial context from input (filter to schema props)

        // for (k, v) in input_kv.into_iter() {
        //     if self.schema.is_prop(&k)
        //         || self.schema.is_virtual(&k)
        //         || self.schema.is_constant(&k)
        //     {
        //         context.insert(k, v);
        //         continue;
        //     }

        //     if let Some(virtual_prop) = self.schema.alias_to_virtual_map.get(&k) {
        //         context.insert(virtual_prop.clone(), v);
        //     }
        // }

        // Resolve defaults iteratively (handles dependencies)
        // self.resolve_defaults(&mut context);

        // Resolve constants iteratively (may depend on defaults)
        // self.resolve_constants(&mut context);

        // Run validators for props in context
        self.run_validators(input, Arc::clone(&shared_rw_options))
            .await;

        error_tool.add(
            "lol",
            FieldError {
                reason: "()".into(),
                metadata: None,
            },
        );

        if error_tool.is_loaded() {
            return Err((
                error_tool.payload(),
                self.prepare_failure_handlers(fields_provided, ctx, Arc::new(options)),
            ));
        }

        // self.add_timestamps(&mut context);

        // let output = O::ivo_internal_from_erased_map(&context);

        // Ok((output, Box::new(move || Box::pin(async  {}))))

        return Err((
            error_tool.payload(),
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
        let ctx = Arc::new(IvoContext::<I, O>::for_update(
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

        let mut updatables = HashMap::new();

        for (k, v) in erased_input_values.inner.iter() {
            updatables.insert(k.clone(), v.clone());
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
        self.run_validators(updates, Arc::clone(&shared_rw_options))
            .await;

        //
        let (updated_values, has_updated_fields) =
            data.ivo_internal_get_updates_from_erased_values(&updatables);

        // let (updated_values, has_updated_fields) =
        //     data.ivo_internal_get_updates_from_partial(updates);

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

        if let Some(handlers) = &self.schema.options().on_delete_fns {
            let tasks = handlers
                .iter()
                .map(|h| h(Arc::clone(&data), Arc::clone(&options)));

            for _ in join_all(tasks).await {}
        }
    }

    async fn run_validators(&self, _input: &Partial<I>, _options: SharedRwCtxOptions<CtxOptions>) {
        // if let Some(def) = self.schema.get_definition("username") {
        //     if let Some(FieldReValidator::Async(validator)) = &def.re_validator {
        //         let r = validator(
        //             erase_value(String::from("IVO test ErasedValue")),
        //             IvoSummary::for_new(
        //                 HashMap::new(),
        //                 input.clone(),
        //                 HashMap::new(),
        //                 // Default::default(),
        //                 options,
        //             ),
        //         )
        //         .await
        //         .map(|v| parse_or_panic::<String>(&v));
        //     }
        // }

        let ids = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let tasks = ids.into_iter().map(validate_async);

        async fn validate_async(id: usize) -> (usize, usize) {
            // Simulate some async I/O work

            let v = (id, id * 2);

            // tokio::task::spawn_blocking(|| async {})
            //     .await
            //     .unwrap()
            //     .await;

            v
        }

        for _ in join_all(tasks).await {}
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
                    default_values.insert(field_name, value.clone());
                    continue;
                }
                Some(ComputableWithMiniContext::Func(resolver)) => {
                    resolvers.push((field_name, resolver));
                    continue;
                }
                _ => {}
            }

            // other fields with default values/resolvers
            match &config.default {
                Some(ComputableWithMiniContext::Static(value)) => {
                    default_values.insert(field_name, value.clone());
                }
                Some(ComputableWithMiniContext::Func(resolver)) => {
                    resolvers.push((field_name, resolver));
                }
                _ => {}
            }
        }

        let tasks = resolvers.into_iter().map(async |(field_name, resolver)| {
            (
                field_name,
                resolver(Arc::clone(&mini_ctx), Arc::clone(&options)).await,
            )
        });

        for (field_name, value) in join_all(tasks).await {
            default_values.insert(field_name, value);
        }

        default_values
            .into_iter()
            .map(|(f, v)| (f.to_owned(), v))
            .collect()
    }

    async fn evaluate_missing_required_fields(
        &self,
        fields_provided: &Vec<String>,
        ctx: SharedIvoContext<I, O>,
        options: SharedRwCtxOptions<CtxOptions>,
    ) -> ErrorTool {
        let mut error_tool = ErrorTool::new();
        let mut resolvers = vec![];

        for (field_name, config) in self.schema.get_field_configs() {
            if fields_provided.contains(field_name) {
                continue;
            }

            match config {
                InternalFieldConfig {
                    field_type: FieldType::Required,
                    required,
                    required_error,
                    ..
                } => {
                    match &required_error {
                        Some(ComputableRequiredError::Static(msg)) => {
                            error_tool.add(
                                field_name,
                                FieldError {
                                    reason: msg.to_string(),
                                    metadata: None,
                                },
                            );

                            continue;
                        }
                        Some(ComputableRequiredError::Func(resolver)) => {
                            resolvers.push((field_name, resolver));

                            continue;
                        }
                        _ => {}
                    }

                    match &required {
                        Some(ComputableRequired::Func(_)) => {
                            error_tool.add(
                                field_name,
                                FieldError {
                                    reason: format!("\"{field_name}\" is required!"),
                                    metadata: None,
                                },
                            );

                            continue;
                        }
                        _ => {}
                    }

                    error_tool.add(
                        field_name,
                        FieldError {
                            reason: format!("\"{field_name}\" is required!"),
                            metadata: None,
                        },
                    );

                    continue;
                }
                _ => {}
            }
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
