use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde_json::Value;

pub type CtxOptions = HashMap<String, Value>;

pub enum ComputedValue<I, O> {
    Static(Value),
    Func(Box<dyn Fn(&MutableSummary<I, O>) -> Value + Send + Sync>),
}

pub struct Context<I, O> {
    pub input: Arc<I>,
    pub values: Arc<O>,
    pub options: Arc<Mutex<CtxOptions>>,
}

pub struct ImmutableSummary<I, O> {
    pub changes: Option<HashMap<String, Value>>,
    pub context: Context<I, O>,
    pub input_values: HashMap<String, Value>,
    pub is_update: bool,
    pub previous_values: Option<Arc<O>>,
    pub values: Arc<O>,
}

pub struct MutableSummary<I, O> {
    pub summary: ImmutableSummary<I, O>,
}

impl<I, O> Context<I, O> {
    pub fn get_options(&self) -> CtxOptions {
        self.options.lock().unwrap().clone()
    }

    pub fn update_options(&self, updates: CtxOptions) {
        let mut opts = self.options.lock().unwrap();
        for (k, v) in updates {
            opts.insert(k, v);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValidatorError {
    pub reason: String,
    pub metadata: Option<Value>,
}

pub type ValidatorResponse<T> = Result<T, ValidatorError>;

pub type ValidatorFn<T> = Box<dyn Fn(&Value) -> ValidatorResponse<T> + Send + Sync>;
