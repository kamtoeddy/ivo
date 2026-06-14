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

    fn errors(&self) -> Vec<(String, Vec<FieldError<Self::FieldMetadata>>)>;

    fn has_errors(&self) -> bool;

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
    type FieldMetadata = DefaultFieldErrorMetadata;
    type ErrorPayload = DefaultErrorPayload;

    fn new() -> Self {
        Self {
            payload: HashMap::new(),
        }
    }

    fn add(&mut self, field_name: &str, value: FieldError) -> &mut Self {
        if !self.payload.contains_key(field_name) {
            self.payload
                .entry(field_name.to_string())
                .or_default()
                .push(value);
        }

        self
    }

    fn errors(&self) -> Vec<(String, Vec<FieldError<Self::FieldMetadata>>)> {
        self.payload
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect()
    }

    fn has_errors(&self) -> bool {
        !self.payload.is_empty()
    }

    fn payload(&self) -> DefaultErrorPayload {
        self.payload.clone()
    }
}

pub struct SchemaError {
    payload: HashMap<String, Vec<String>>,
}

#[allow(dead_code)]
impl SchemaError {
    pub fn new() -> Self {
        Self {
            payload: HashMap::new(),
        }
    }

    pub fn has_errors(&self) -> bool {
        !self.payload.is_empty()
    }

    pub fn add(&mut self, field: &str, value: String) -> &mut Self {
        let entry = self.payload.entry(field.to_string()).or_default();

        if !entry.contains(&value) {
            entry.push(value);
        }

        self
    }

    pub fn throw(self) {
        const CLI_STYLE_COLOR_RED: &str = "\x1b[31m";
        const CLI_STYLE_RESET: &str = "\x1b[0m";

        println!("\n{} Schema errors:", CLI_STYLE_COLOR_RED);

        let mut pv: Vec<_> = self.payload.into_iter().collect();
        pv.sort_by(|a, b| a.0.cmp(&b.0));

        for (prop, errors) in pv {
            println!();

            if errors.len() == 1 {
                println!(" [{prop}]: {}", errors[0]);

                continue;
            }

            println!(" [{prop}]:");

            for (i, m) in errors.iter().enumerate() {
                println!("    { }) {m}", i + 1);
            }
        }

        println!("\n{}Invalid schema detected", CLI_STYLE_RESET);
        // panic!("\nInvalid schema detected");
    }
}
