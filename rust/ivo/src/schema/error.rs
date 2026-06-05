use std::collections::HashMap;

use crate::utils::styled_text::Stylable;

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
    fn add(&mut self, field: &str, error: FieldError<Self::FieldMetadata>) -> &mut Self;
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
    type FieldMetadata = DefaultFieldErrorMetadata;
    type ErrorPayload = DefaultErrorPayload;

    fn new() -> Self {
        Self {
            payload: HashMap::new(),
        }
    }

    fn add(&mut self, field: &str, value: FieldError) -> &mut Self {
        if !self.payload.contains_key(field) {
            self.payload
                .entry(field.to_string())
                .or_default()
                .push(value);
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

// const CLI_COLOR_RED: &'static str = "\x1b[31m";
// const CLI_COLOR_RESET: &'static str = "\x1b[0m";

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
        let entry = self.payload.entry(field.to_string()).or_default();

        if !entry.contains(&value) {
            entry.push(value);
        }

        self
    }

    pub fn throw(self) {
        // println!("\n{}", "Schema errors:".colored_red());
        let mut err = format!("\n{}", "Schema errors:".font_bold());

        let mut pv: Vec<_> = self.payload.into_iter().collect();
        pv.sort_by(|a, b| a.0.cmp(&b.0));

        for (prop, errors) in pv {
            err += "\n";
            // println!();

            if errors.len() == 1 {
                err += format!("  {}", "[".colored_red()).as_str();
                err += format!("{}", prop.font_bold().colored_red()).as_str();
                err += format!("{}", "]: ".colored_red()).as_str();
                err += format!("{}\n", errors[0].colored_red()).as_str();

                continue;
            }

            err += format!("  {}", "[".colored_red()).as_str();
            err += format!("{}", prop.font_bold()).as_str();
            err += format!("{}\n", "]".colored_red()).as_str();

            for (i, m) in errors.iter().enumerate() {
                let idx = i + 1;
                err += format!("    {}) ", idx.colored_red()).as_str();
                err += format!("{}\n", m.colored_red()).as_str();
                // println!("    { }) {m}", i + 1);
            }
        }

        print!("{}", err.colored_red());

        println!("\n{}", "Invalid schema detected".colored_red(),);
        // panic!("\nInvalid schema detected");
    }
}
