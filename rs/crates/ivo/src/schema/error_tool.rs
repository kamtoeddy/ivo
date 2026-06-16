use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum UpdateError<E: IvoErrorTool> {
    NothingToUpdate,
    ValidationError(E::ErrorPayload),
}

pub type DefaultFieldErrorMetadata = ();
pub type DefaultErrorPayload = HashMap<String, Vec<FieldError>>;

#[derive(Debug, Clone)]
pub struct FieldError<FieldMetadata = DefaultFieldErrorMetadata> {
    pub reason: String,
    pub metadata: Option<FieldMetadata>,
}

// ErrorTool trait
pub trait IvoErrorTool {
    type FieldMetadata;
    type ErrorPayload;

    fn new() -> Self;

    fn add(&mut self, field_name: &str, error: FieldError<Self::FieldMetadata>) -> &mut Self;

    fn has_errors(&self) -> bool;

    fn payload(self) -> Self::ErrorPayload;
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
    type FieldMetadata = DefaultFieldErrorMetadata;
    type ErrorPayload = DefaultErrorPayload;

    fn new() -> Self {
        Self {
            payload: HashMap::new(),
        }
    }

    fn add(&mut self, field_name: &str, value: FieldError) -> &mut Self {
        self.payload
            .entry(field_name.to_string())
            .and_modify(|e| e.push(value.clone()))
            .or_insert_with(|| vec![value]);

        self
    }

    fn has_errors(&self) -> bool {
        !self.payload.is_empty()
    }

    fn payload(self) -> DefaultErrorPayload {
        self.payload
    }
}
