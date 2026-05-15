use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UpdateError<E: IvoErrorTool> {
    NothingToUpdate,
    ValidationError(E::ErrorPayload),
}

pub type DefaultErrorPayload = HashMap<String, Vec<FieldError>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldError {
    pub reason: String,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidationErrorMessage {
    NothingToUpdate,
    ValidationError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IValidationError {
    pub message: ValidationErrorMessage,
    pub payload: HashMap<String, FieldError>,
}

// ErrorTool trait
pub trait IvoErrorTool {
    type ErrorPayload: Serialize + DeserializeOwned;

    fn new() -> Self;
    fn add(&mut self, field: String, error: FieldError) -> &mut Self;
    fn is_loaded(&self) -> bool;
    fn payload(&self) -> Self::ErrorPayload;
}

// DefaultErrorTool implementation
pub struct DefaultErrorTool {
    payload: DefaultErrorPayload,
}

impl DefaultErrorTool {
    pub fn new() -> Self {
        Self {
            payload: HashMap::new(),
        }
    }
}

impl IvoErrorTool for DefaultErrorTool {
    type ErrorPayload = DefaultErrorPayload;

    fn new() -> Self {
        Self {
            payload: HashMap::new(),
        }
    }

    fn add(&mut self, field: String, value: FieldError) -> &mut Self {
        if !self.payload.contains_key(&field) {
            self.payload.entry(field).or_default().push(value);
        }

        self
    }

    fn is_loaded(&self) -> bool {
        !self.payload.is_empty()
    }

    fn payload(&self) -> DefaultErrorPayload {
        self.payload.clone()
    }
}

pub struct SchemaError {
    payload: HashMap<String, Vec<String>>,
}

impl SchemaError {
    pub fn new() -> Self {
        Self {
            payload: HashMap::new(),
        }
    }

    pub fn is_payload_loaded(&self) -> bool {
        !self.payload.is_empty()
    }

    pub fn add(&mut self, field: &str, value: String) -> &mut Self {
        let entry = self
            .payload
            .entry(field.to_string())
            .or_insert_with(Vec::new);

        if !entry.contains(&value) {
            entry.push(value);
        }

        self
    }

    pub fn throw(&self) {
        println!("\nSchema errors:");

        for (prop, errors) in &self.payload {
            println!();

            if errors.len() == 1 {
                println!("  [{prop}]: {}", errors[0]);

                continue;
            }

            println!("  [{prop}]:");

            for (i, m) in errors.iter().enumerate() {
                println!("    { }) {m}", i + 1);
            }
        }

        println!("\nYour schema has some errors");
    }
}
