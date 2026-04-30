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
        if let Err(schema_err) = self.schema.resolve_defaults(&mut context) {
            let mut payload = std::collections::HashMap::new();
            for (k, msgs) in schema_err.payload.into_iter() {
                let reason = msgs
                    .into_iter()
                    .next()
                    .unwrap_or_else(|| "Unresolved default".to_string());
                payload.insert(
                    k,
                    FieldError {
                        reason,
                        metadata: None,
                    },
                );
            }

            return Err(IValidationError {
                message: ValidationErrorMessage::ValidationError,
                payload,
            });
        }

        // Resolve constants iteratively (may depend on defaults)
        if let Err(schema_err) = self.schema.resolve_constants(&mut context) {
            let mut payload = std::collections::HashMap::new();
            for (k, msgs) in schema_err.payload.into_iter() {
                let reason = msgs
                    .into_iter()
                    .next()
                    .unwrap_or_else(|| "Unresolved constant".to_string());
                payload.insert(
                    k,
                    FieldError {
                        reason,
                        metadata: None,
                    },
                );
            }

            return Err(IValidationError {
                message: ValidationErrorMessage::ValidationError,
                payload,
            });
        }

        // Run validators for props in context
        for prop in self.schema.props.iter() {
            if let Some(value) = context.get(prop) {
                if let Some(res) = self.schema.run_validators(prop, value) {
                    match res {
                        crate::validators::ValidationResponse::Valid(v) => {
                            context.insert(prop.clone(), v);
                        }
                        crate::validators::ValidationResponse::Invalid(err) => {
                            let field_error = FieldError {
                                reason: err.reason,
                                metadata: err.metadata,
                            };
                            error_tool.set(prop.clone(), field_error, Some(value.clone()));
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
                    },
                    None,
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
