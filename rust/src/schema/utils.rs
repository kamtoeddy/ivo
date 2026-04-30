use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

// Types

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidationErrorMessage {
    NothingToUpdate,
    ValidationError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldError {
    pub reason: String,
    pub metadata: Option<Value>,
}

pub type ErrorPayload = HashMap<String, Vec<String>>; // for SchemaErrorTool usage

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IValidationError {
    pub message: ValidationErrorMessage,
    pub payload: HashMap<String, FieldError>,
}

// ErrorTool trait
pub trait ErrorTool {
    fn data(&self) -> IValidationError;
    fn fields(&self) -> Vec<String>;
    fn is_loaded(&self) -> bool;
    fn set(&mut self, field: String, error: FieldError, _value: Option<Value>) -> &mut Self;
    fn set_message(&mut self, message: ValidationErrorMessage) -> &mut Self;
}

// DefaultErrorTool implementation
pub struct DefaultErrorTool {
    message: ValidationErrorMessage,
    payload: HashMap<String, FieldError>,
}

impl DefaultErrorTool {
    pub fn new(message: ValidationErrorMessage) -> Self {
        Self {
            message,
            payload: HashMap::new(),
        }
    }
}

impl ErrorTool for DefaultErrorTool {
    fn data(&self) -> IValidationError {
        IValidationError {
            message: self.message.clone(),
            payload: self.payload.clone(),
        }
    }

    fn fields(&self) -> Vec<String> {
        self.payload.keys().cloned().collect()
    }

    fn is_loaded(&self) -> bool {
        !self.payload.is_empty()
    }

    fn set(&mut self, field: String, value: FieldError, _orig: Option<Value>) -> &mut Self {
        if !self.payload.contains_key(&field) {
            self.payload.insert(field, value);
            return self;
        }

        let current = self.payload.get_mut(&field).unwrap();

        if let (Some(new_meta), Some(cur_meta)) = (&value.metadata, &mut current.metadata) {
            // merge objects
            if new_meta.is_object() && cur_meta.is_object() {
                if let (Some(new_map), Some(cur_map)) =
                    (new_meta.as_object(), cur_meta.as_object_mut())
                {
                    for (k, v) in new_map.iter() {
                        cur_map.insert(k.clone(), v.clone());
                    }
                }
            } else {
                // replace
                current.metadata = Some(new_meta.clone());
            }
        } else if value.metadata.is_some() {
            current.metadata = value.metadata.clone();
        }

        self
    }

    fn set_message(&mut self, message: ValidationErrorMessage) -> &mut Self {
        self.message = message;
        self
    }
}

// SchemaErrorTool - used for schema validation during construction
#[derive(Debug)]
pub struct SchemaError {
    pub payload: HashMap<String, Vec<String>>,
}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid schema: {:?}", self.payload)
    }
}

impl std::error::Error for SchemaError {}

pub struct SchemaErrorTool {
    payload: HashMap<String, Vec<String>>,
}

impl SchemaErrorTool {
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

    pub fn throw(&self) -> Result<(), SchemaError> {
        if self.is_payload_loaded() {
            Err(SchemaError {
                payload: self.payload.clone(),
            })
        } else {
            Ok(())
        }
    }
}

// TimeStampTool
#[derive(Debug, Clone)]
pub struct TimeStampKeys {
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

pub struct TimeStampTool {
    keys: TimeStampKeys,
    nullable: bool,
}

impl TimeStampTool {
    const IS_UPDATED_AT_NULLABLE_DEFAULT: bool = true;

    pub fn new(timestamps: Option<&Value>) -> Self {
        // timestamps: Option<Boolean | Object>
        if timestamps.is_none() {
            return Self {
                keys: TimeStampKeys {
                    created_at: None,
                    updated_at: None,
                },
                nullable: false,
            };
        }

        match timestamps.unwrap() {
            Value::Bool(b) => {
                if *b {
                    return Self {
                        keys: TimeStampKeys {
                            created_at: Some("created_at".into()),
                            updated_at: Some("updated_at".into()),
                        },
                        nullable: Self::IS_UPDATED_AT_NULLABLE_DEFAULT,
                    };
                } else {
                    return Self {
                        keys: TimeStampKeys {
                            created_at: None,
                            updated_at: None,
                        },
                        nullable: false,
                    };
                }
            }
            Value::Object(map) => {
                let created_at = map
                    .get("created_at")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                let updated_at = match map.get("updated_at") {
                    Some(Value::Object(o)) => {
                        o.get("key").and_then(|v| v.as_str()).map(|s| s.to_string())
                    }
                    Some(Value::String(s)) => Some(s.clone()),
                    _ => None,
                };

                let nullable = match map.get("updated_at") {
                    Some(Value::Object(o)) => o
                        .get("nullable")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(Self::IS_UPDATED_AT_NULLABLE_DEFAULT),
                    _ => Self::IS_UPDATED_AT_NULLABLE_DEFAULT,
                };

                return Self {
                    keys: TimeStampKeys {
                        created_at,
                        updated_at,
                    },
                    nullable,
                };
            }
            _ => {
                return Self {
                    keys: TimeStampKeys {
                        created_at: None,
                        updated_at: None,
                    },
                    nullable: false,
                }
            }
        }
    }

    pub fn get_keys(&self) -> &TimeStampKeys {
        &self.keys
    }

    pub fn is_timestamp_key(&self, key: &str) -> bool {
        if let Some(ref k) = self.keys.created_at {
            if k == key {
                return true;
            }
        }
        if let Some(ref k) = self.keys.updated_at {
            if k == key {
                return true;
            }
        }
        false
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable
    }

    pub fn with_timestamps(&self) -> bool {
        self.keys.created_at.is_some() || self.keys.updated_at.is_some()
    }
}
