use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IvoError {
    NothingToUpdate,
    ValidationError(IvoValidationError),
}

pub type IvoValidationError = HashMap<String, Vec<FieldError>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldError {
    pub reason: String,
    pub metadata: Option<serde_json::Value>,
}

impl IvoError {
    pub fn validation_error(field: &str, reason: &str) -> Self {
        let mut errors = HashMap::new();

        errors.insert(
            field.to_string(),
            vec![FieldError {
                reason: reason.to_string(),
                metadata: None,
            }],
        );

        IvoError::ValidationError(errors)
    }
}
