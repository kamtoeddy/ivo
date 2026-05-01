use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde_json::Value;

#[derive(Debug)]
pub struct True;

// Optional: implement Deref to make it behave like bool
impl std::ops::Deref for True {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &true
    }
}

pub type CtxOptions = HashMap<String, Value>;

pub enum ComputedValue<I, O> {
    Static(Value),
    Func(Box<dyn Fn(&MutableSummary<I, O>) -> Value + Send + Sync>),
}

pub enum RequiredField<I, O> {
    True,
    BoolFunc(Box<dyn Fn(&MutableSummary<I, O>) -> bool + Send + Sync>),
    BoolErrorFunc(Box<dyn Fn(&MutableSummary<I, O>) -> Value + Send + Sync>),
}

pub enum ComputableValueWithContext<I, O, T = Value> {
    Static(T),
    Func(Box<dyn Fn(&Context<I, O>) -> T + Send + Sync>),
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

pub type FieldValidatorFn<I, O, T = Value> =
    Box<dyn Fn(&Value, &Context<I, O>) -> ValidatorResponse<T> + Send + Sync>;
pub type ResolverFn<I, O, T = Value> = Box<dyn Fn(&MutableSummary<I, O>) -> T + Send + Sync>;

// #[async_trait]
pub trait Validator<I, O>: Send + Sync {
    // async fn validate(
    //     &self,
    //     value: &serde_json::Value,
    //     summary: &MutableSummary<I, O>,
    // ) -> Result<serde_json::Value, String>;
}

// #[async_trait]
pub trait PostValidator<I, O>: Send + Sync {
    // Returns a map of field names to updated values on success,
    // or a map of field names to error messages on failure.
    // async fn validate(
    //     &self,
    //     summary: &MutableSummary<I, O>,
    // ) -> Result<HashMap<String, Value>, HashMap<String, String>>;
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
