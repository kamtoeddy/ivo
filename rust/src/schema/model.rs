use crate::schema::core::SchemaCore;
use crate::schema::error::{DefaultErrorTool, FieldError, IvoErrorTool, UpdateError};
use crate::types::Context;
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

use crate::traits::{HasPartial, Partial};

pub struct Model<
    Input: Serialize + HasPartial,
    Output: DeserializeOwned + HasPartial,
    CtxOptions = HashMap<String, Value>,
    ErrorTool: IvoErrorTool = DefaultErrorTool,
> {
    schema: SchemaCore<Input, Output, CtxOptions>,
    _error_tool: PhantomData<ErrorTool>,
}

impl<
        Input: Serialize + HasPartial,
        Output: DeserializeOwned + HasPartial,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > Model<Input, Output, CtxOptions, ErrorTool>
{
    pub fn new(schema: SchemaCore<Input, Output, CtxOptions>) -> Self {
        Self {
            schema,
            _error_tool: PhantomData,
        }
    }

    pub fn create(&self, input: &Input) -> Result<(Output, fn()), (ErrorTool::ErrorPayload, fn())> {
        let value = json!(input);

        match value {
            Value::Object(input_kv) => {
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

                error_tool.add(
                    "lol".into(),
                    FieldError {
                        reason: "()".into(),
                        metadata: None,
                    },
                );

                if error_tool.is_loaded() {
                    return Err((error_tool.payload(), || {}));
                }

                self.add_timestamps(&mut context);

                Ok((
                    serde_json::from_value(json!(context)).expect("json parse error"),
                    || {},
                ))
            }
            _ => unreachable!(),
        }
    }

    pub fn update(
        &self,
        _data: &Output,
        updates: &Partial<Input>,
    ) -> Result<(Partial<Output>, fn()), (UpdateError<ErrorTool>, fn())> {
        let value = json!(updates);

        match value {
            Value::Object(_) => Ok((
                serde_json::from_value(value).expect("json parse error"),
                || {},
            )),
            _ => Err((UpdateError::NothingToUpdate, || {})),
        }
    }

    pub fn delete(&self, _data: &Output) {
        todo!()
    }

    fn add_timestamps(&self, context: &mut Context) {
        if self.schema.timestamp_tool.with_timestamps() {
            let now = chrono::Utc::now().to_rfc3339();

            let keys = &self.schema.timestamp_tool.get_keys();

            if let Some(created_at_key) = keys.created_at.clone() {
                context.insert(created_at_key, Value::String(now.clone()));
            }

            if let Some(updated_at_key) = keys.updated_at.clone() {
                context.insert(updated_at_key, Value::String(now));
            }
        }
    }

    /// Resolve defaults iteratively based on dependencies.
    /// It will repeatedly evaluate defaults whose dependencies are satisfied (present in `context`).
    /// If unresolved defaults remain and schema option `error_on_unresolved_defaults` is true,
    /// returns Err(SchemaError) listing the unresolved props.
    pub fn resolve_defaults(&self, context: &mut HashMap<String, Value>) {
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

        todo!()
    }

    /// Resolve constants iteratively; constants may depend on other values in context.
    /// If unresolved constants remain and schema option `error_on_unresolved_constants` is true,
    /// returns Err(SchemaError) listing unresolved constants; otherwise returns Ok(())
    pub fn resolve_constants(&self, context: &mut HashMap<String, Value>) {
        let mut _pending: HashSet<String> = self
            .schema
            .constants
            .iter()
            .filter(|k| !context.contains_key(*k))
            .cloned()
            .collect();

        todo!()
    }
}
