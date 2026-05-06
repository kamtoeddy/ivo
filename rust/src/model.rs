use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::error::{IvoError, IvoValidationError};

// 1. The Magic Trait
pub trait HasPartial {
    type Partial: Serialize + DeserializeOwned;
}

// 2. The TypeScript-style Utility Alias
pub type Partial<T> = <T as HasPartial>::Partial;

pub enum CreateOutcome<Output: DeserializeOwned> {
    Fail {
        error: IvoValidationError,
        handle_failure: fn(),
    },
    Success {
        data: Output,
        handle_success: fn(),
    },
}

pub enum UpdateOutcome<Output: DeserializeOwned + HasPartial> {
    Fail {
        error: IvoError,
        handle_failure: fn(),
    },
    Success {
        data: Partial<Output>,
        handle_success: fn(),
    },
}

pub struct Model<Input: Serialize, Output: DeserializeOwned> {
    input: HashMap<String, Input>,
    output: HashMap<String, Output>,
}

impl<Input: Serialize + HasPartial, Output: DeserializeOwned + HasPartial> Model<Input, Output> {
    pub fn new() -> Self {
        Self {
            input: HashMap::new(),
            output: HashMap::new(),
        }
    }

    pub fn create(&self, input: &Input) -> CreateOutcome<Output> {
        let value = json!(input);

        match value {
            Value::Object(_) => CreateOutcome::Success {
                data: serde_json::from_value(value).expect("json parse error"),
                handle_success: || {},
            },
            _ => CreateOutcome::Fail {
                error: HashMap::new(),
                handle_failure: || {},
            },
        }
    }

    pub fn update(&self, data: &Output, updates: &Partial<Input>) -> UpdateOutcome<Output> {
        let value = json!(updates);

        match value {
            Value::Object(v) => UpdateOutcome::Success {
                data: serde_json::from_value(json!(v)).expect("json parse error"),
                handle_success: || {},
            },
            _ => UpdateOutcome::Fail {
                error: IvoError::NothingToUpdate,
                handle_failure: || {},
            },
        }
    }

    pub fn delete(&self, data: Output) {
        todo!()
    }
}
