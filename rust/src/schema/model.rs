use crate::schema::core::SchemaCore;
use crate::schema::utils::{
    DefaultErrorTool, ErrorTool, FieldError, IValidationError, ValidationErrorMessage,
};
use serde_json::Value;
use std::collections::HashMap;

pub struct ModelTool {
    schema: SchemaCore,
}

impl ModelTool {
    pub fn new(schema: SchemaCore) -> Self {
        Self { schema }
    }

    pub fn create(
        &self,
        input: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, IValidationError> {
        let mut error_tool = DefaultErrorTool::new(ValidationErrorMessage::ValidationError);

        // Build initial context from input (filter to schema props)
        let mut context: HashMap<String, Value> = HashMap::new();

        for (k, v) in input.into_iter() {
            if self.schema.is_prop(&k) || self.schema.is_virtual(&k) || self.schema.is_constant(&k)
            {
                context.insert(k, v);
            }
        }

        // Resolve defaults iteratively (handles dependencies)
        self.schema.resolve_defaults(&mut context);

        // Resolve constants iteratively (may depend on defaults)
        self.schema.resolve_constants(&mut context);

        // Run validators for props in context
        for prop in self.schema.props.iter() {
            if let Some(value) = context.get(prop) {
                if let Some(res) = self.schema.run_validators(prop, value) {
                    match res {
                        Ok(v) => {
                            context.insert(prop.clone(), v);
                        }
                        Err((err, metadata)) => {
                            error_tool.set(
                                prop.clone(),
                                FieldError {
                                    reason: err.to_string(),
                                    metadata,
                                    value: Some(value.clone()),
                                },
                            );
                        }
                    }
                }
            } else if self
                .schema
                .get_definition(prop)
                .map_or(false, |d| d.required == Some(true))
            {
                error_tool.set(
                    prop.clone(),
                    FieldError {
                        reason: "required".into(),
                        metadata: None,
                        value: None,
                    },
                );
            }
        }

        // timestamps
        if self.schema.timestamp_tool.with_timestamps() {
            let now = chrono::Utc::now().to_rfc3339();

            if let Some(created_at_key) = &self.schema.timestamp_tool.get_keys().created_at {
                context.insert(created_at_key.clone(), Value::String(now.clone()));
            }

            if let Some(updated_at_key) = &self.schema.timestamp_tool.get_keys().updated_at {
                context.insert(updated_at_key.clone(), Value::String(now));
            }
        }

        if error_tool.is_loaded() {
            return Err(error_tool.data());
        }

        Ok(context)
    }
}
