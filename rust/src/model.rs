use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
    error::{ErrorPayload, UpdateError},
    traits::{HasPartial, Partial},
};

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

    pub fn create(&self, input: &Input) -> Result<(Output, fn()), (ErrorPayload, fn())> {
        let value = json!(input);

        match value {
            Value::Object(_) => Ok((
                serde_json::from_value(value).expect("json parse error"),
                || {},
            )),
            _ => Err((HashMap::new(), || {})),
        }
    }

    pub fn update(
        &self,
        data: &Output,
        updates: &Partial<Input>,
    ) -> Result<(Partial<Output>, fn()), (UpdateError, fn())> {
        let value = json!(updates);

        match value {
            Value::Object(_) => Ok((
                serde_json::from_value(value).expect("json parse error"),
                || {},
            )),
            _ => Err((UpdateError::NothingToUpdate, || {})),
        }
    }

    pub fn delete(&self, data: Output) {
        todo!()
    }
}
