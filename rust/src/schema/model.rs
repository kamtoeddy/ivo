use crate::schema::core::SchemaCore;
use crate::schema::error::{DefaultErrorTool, FieldError, IvoErrorTool, UpdateError};
use crate::types::{Context, ErasedStuff};

use std::collections::{HashMap, HashSet};

use futures::future::{join_all, BoxFuture};
use futures::stream::{FuturesUnordered, StreamExt};
use serde_json::json;

use crate::traits::{IvoSchemaStruct, Partial};

pub type AsyncTriggerFn = Box<dyn Fn() -> BoxFuture<'static, ()> + Send + Sync>;

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrorTool: IvoErrorTool>
    SchemaCore<I, O, CtxOptions, ErrorTool>
{
    pub fn get_model(&self) -> Model<'_, I, O, CtxOptions, ErrorTool> {
        Model { schema: self }
    }
}

pub struct Model<
    'schema,
    Input: IvoSchemaStruct,
    Output: IvoSchemaStruct,
    CtxOptions: Clone = HashMap<String, ErasedStuff>,
    ErrorTool: IvoErrorTool = DefaultErrorTool,
> {
    schema: &'schema SchemaCore<Input, Output, CtxOptions, ErrorTool>,
}

impl<
        'schema,
        Input: IvoSchemaStruct,
        Output: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrorTool: IvoErrorTool,
    > Model<'schema, Input, Output, CtxOptions, ErrorTool>
{
    pub async fn create(
        &self,
        input: &Partial<Input>,
        options: CtxOptions,
    ) -> Result<(Output, AsyncTriggerFn), (ErrorTool::ErrorPayload, AsyncTriggerFn)> {
        let value = json!(input);

        match value {
            ErasedStuff::Object(input_kv) => {
                let mut error_tool = ErrorTool::new();

                // Build initial context from input (filter to schema props)
                let mut context: Context = HashMap::new();

                for (k, v) in input_kv.into_iter() {
                    if self.schema.is_prop(&k)
                        || self.schema.is_virtual(&k)
                        || self.schema.is_constant(&k)
                    {
                        context.insert(k, v);
                        continue;
                    }

                    if let Some(virtual_prop) = self.schema.alias_to_virtual_map.get(&k) {
                        context.insert(virtual_prop.clone(), v);
                    }
                }

                // Resolve defaults iteratively (handles dependencies)
                self.resolve_defaults(&mut context);

                // Resolve constants iteratively (may depend on defaults)
                self.resolve_constants(&mut context);

                // Run validators for props in context
                self.run_async_validator(input, options).await;

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
                        Box::new(move || Box::pin(async move {})),
                    ));
                }

                self.add_timestamps(&mut context);

                Ok((
                    serde_json::from_value(json!(context)).expect("json parse error"),
                    Box::new(move || Box::pin(async move {})),
                ))
            }
            _ => unreachable!(),
        }
    }

    pub async fn update(
        &self,
        _data: &Output,
        updates: &Partial<Input>,
        options: CtxOptions,
    ) -> Result<(Partial<Output>, AsyncTriggerFn), (UpdateError<ErrorTool>, AsyncTriggerFn)> {
        let value = json!(updates);

        match value {
            ErasedStuff::Object(_) => {
                // Run validators for props in context
                self.run_async_validator(updates, options).await;

                Ok((
                    serde_json::from_value(value).expect("json parse error"),
                    Box::new(move || Box::pin(async move {})),
                ))
            }
            _ => Err((
                UpdateError::NothingToUpdate,
                Box::new(move || Box::pin(async move {})),
            )),
        }
    }

    pub async fn delete(&self, data: &Output) {
        let handle_delete_async = async |_data: &Output| {};

        let mut tasks = FuturesUnordered::new();

        for _ in 11..=21 {
            tasks.push(handle_delete_async(data));
        }

        while tasks.next().await.is_some() {}
    }

    async fn run_async_validator(&self, _input: &Partial<Input>, _options: CtxOptions) {
        println!("running validations\n");

        // if let Some(def) = self.schema.get_definition("username") {
        //     if let Some(FieldReValidator::Async(validator)) = &def.re_validator {
        //         // let validator = validators();
        //         let r = validator(
        //             json!("IVO test ErasedStuff"),
        //             IvoSummary::for_new(
        //                 HashMap::new(),
        //                 input.clone(),
        //                 HashMap::new(),
        //                 // Default::default(),
        //                 options,
        //             ),
        //         )
        //         .await;

        //         println!("async results for validations of username {:?}", r)
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

    fn add_timestamps(&self, _context: &mut Context) {
        // if self.schema.timestamp_tool.with_timestamps() {
        //     let now = chrono::Utc::now().to_rfc3339();

        //     let keys = &self.schema.timestamp_tool.get_keys();

        //     if let Some(created_at_key) = keys.created_at.clone() {
        //         context.insert(created_at_key, ErasedStuff::String(now.clone()));
        //     }

        //     if let Some(updated_at_key) = keys.updated_at.clone() {
        //         context.insert(updated_at_key, ErasedStuff::String(now));
        //     }
        // }
    }

    /// Resolve defaults iteratively based on dependencies.
    /// It will repeatedly evaluate defaults whose dependencies are satisfied (present in `context`).
    /// If unresolved defaults remain and schema option `error_on_unresolved_defaults` is true,
    /// returns Err(SchemaError) listing the unresolved props.
    pub fn resolve_defaults(&self, context: &mut HashMap<String, ErasedStuff>) {
        let mut _pending: HashSet<String> = self
            .schema
            .get_definitions()
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
    pub fn resolve_constants(&self, context: &mut HashMap<String, ErasedStuff>) {
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
