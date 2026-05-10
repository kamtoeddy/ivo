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

#[derive(Debug)]
pub struct False;

// Optional: implement Deref to make it behave like bool
impl std::ops::Deref for False {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &false
    }
}

pub type CtxOptions = HashMap<String, Value>;

pub type EnumeratedErrorResolver<T> = Box<dyn Fn((Value, &Vec<T>)) -> &str + Send + Sync>;

pub enum ComputableEnumeratedError<T> {
    Static(String),
    Func(EnumeratedErrorResolver<T>),
}

pub enum ComputableWithContext<I, O, T = Value> {
    Static(T),
    Func(ResolverWithContextFn<I, O, T>),
}

pub enum ComputableInit<I, O> {
    False,
    Func(ResolverWithMutSummaryFn<I, O, bool>),
}

pub enum ComputableRequired<I, O> {
    Static(True),
    Func(RequiredResolverFn<I, O>),
}

pub struct Context<I, O> {
    pub input: Arc<I>,
    pub values: Arc<O>,
    pub options: Arc<Mutex<CtxOptions>>,
}

pub struct DeletionContext<O> {
    pub values: Arc<O>,
    pub options: Arc<Mutex<CtxOptions>>,
}

pub enum IvoSummary<I, O> {
    Create {
        context: Context<I, O>,
        input: Arc<I>,
        values: Arc<O>,
    },
    Update {
        changes: HashMap<String, Value>,
        context: Context<I, O>,
        input: Arc<I>,
        previous_values: Arc<O>,
        values: Arc<O>,
    },
}

// impl<I, O> IvoSummary<I, O> {
//     pub fn get_options(&self) -> &mut CtxOptions;
// }

pub struct ImmutableSummary<I, O> {
    pub changes: Option<Context<I, O>>,
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

pub type RequiredResolverFn<I, O> = Box<dyn Fn(&Context<I, O>) -> (bool, &str) + Send + Sync>;

pub type ResolverWithContextFn<I, O, T = Value> = Box<dyn Fn(&Context<I, O>) -> T + Send + Sync>;

pub type ResolverWithMutSummaryFn<I, O, T = Value> =
    Box<dyn Fn(&MutableSummary<I, O>) -> T + Send + Sync>;

pub type BooleanResolverWithMutSummary<I, O> =
    Box<dyn Fn(&MutableSummary<I, O>) -> bool + Send + Sync>;

pub type VirtualSanitiser<I, O, T = Value> = ResolverWithMutSummaryFn<I, O, T>;

pub type DeleteHandler<O> = Box<dyn Fn(&DeletionContext<O>)>;
pub type FailureHandler<I, O> = Box<dyn Fn(&ImmutableSummary<I, O>)>;
pub type SuccessHandler<I, O> = Box<dyn Fn(&ImmutableSummary<I, O>)>;

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

pub type ValidatorResponse<T> = Result<T, (&'static str, Option<Value>)>;

pub type ValidatorFn<T> = Box<dyn Fn(&Value) -> ValidatorResponse<T> + Send + Sync>;
