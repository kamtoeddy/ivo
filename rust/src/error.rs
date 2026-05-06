use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UpdateError {
    NothingToUpdate,
    ValidationError(ErrorPayload),
}

pub type ErrorPayload = HashMap<String, Vec<FieldError>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldError {
    pub reason: String,
    pub metadata: Option<serde_json::Value>,
}
