use std::collections::HashMap;

use futures::future::BoxFuture;
use serde_json::Value;

use crate::traits::{HasPartial, Partial};

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

// pub type CtxOptions = HashMap<String, Value>;

pub type EnumeratedErrorResolver<T> = Box<dyn Fn((Value, &Vec<T>)) -> &str + Send + Sync>;

pub enum ComputableEnumeratedError<T> {
    Static(String),
    Func(EnumeratedErrorResolver<T>),
}

pub enum ComputableWithMiniSummary<T, CtxOptions> {
    Static(T),
    AsyncFunc(AsyncResolverWithMiniSummaryFn<T, CtxOptions>),
    SyncFunc(ResolverWithMiniSummaryFn<T, CtxOptions>),
}

pub enum ComputableInit<I: HasPartial, O: HasPartial, CtxOptions> {
    False,
    Func(ResolverWithMutSummaryFn<bool, I, O, CtxOptions>),
}

pub enum ComputableRequired<I: HasPartial, O: HasPartial, CtxOptions> {
    Static(True),
    Func(RequiredResolverFn<I, O, CtxOptions>),
}

pub type Context = HashMap<String, Value>;

pub struct IvoMiniSummary<CtxOptions> {
    context: Context,
    options: CtxOptions,
}

impl<CtxOptions> IvoMiniSummary<CtxOptions> {
    pub fn new(context: Context, options: CtxOptions) -> Self {
        Self { context, options }
    }

    pub fn ctx(&self) -> &Context {
        &self.context
    }

    pub fn options(&self) -> &CtxOptions {
        &self.options
    }

    pub fn update_options(&mut self) {
        todo!()
    }
}

pub struct IvoSummary<I: HasPartial, O: HasPartial, CtxOptions> {
    changes: Option<HashMap<String, Value>>,
    context: Context,
    input: Partial<I>,
    input_values: HashMap<String, Value>,
    is_update: bool,
    previous_values: Option<O>,
    values: O,
    options: CtxOptions,
}

impl<I: HasPartial, O: HasPartial, CtxOptions> IvoSummary<I, O, CtxOptions> {
    pub fn new(
        changes: Option<HashMap<String, Value>>,
        context: Context,
        input: Partial<I>,
        input_values: HashMap<String, Value>,
        is_update: bool,
        previous_values: Option<O>,
        values: O,
        options: CtxOptions,
    ) -> Self {
        Self {
            changes,
            context,
            input,
            input_values,
            is_update,
            previous_values,
            values,
            options,
        }
    }

    pub fn changes(&self) -> &Option<HashMap<String, Value>> {
        &self.changes
    }

    pub fn ctx(&self) -> &Context {
        &self.context
    }

    pub fn input(&self) -> &Partial<I> {
        &self.input
    }

    pub fn input_values(&self) -> &HashMap<String, Value> {
        &self.input_values
    }

    pub fn is_update(&self) -> bool {
        self.is_update
    }

    pub fn previous_values(&self) -> &Option<O> {
        &self.previous_values
    }

    pub fn values(&self) -> &O {
        &self.values
    }

    pub fn options(&self) -> &CtxOptions {
        &self.options
    }

    pub fn update_options(&mut self) {
        todo!()
    }
}

pub enum FieldValidator<T, I: HasPartial, O: HasPartial, CtxOptions> {
    Async(AsyncFieldValidatorFn<T, I, O, CtxOptions>),
    Sync(FieldValidatorFn<T, I, O, CtxOptions>),
}

pub type AsyncFieldValidatorFn<T, I, O, CtxOptions> = Box<
    dyn Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ValidatorResponse<T>>
        + Send
        + Sync
        + 'static,
>;

pub type FieldValidatorFn<T, I, O, CtxOptions> =
    Box<dyn Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<T> + Send + Sync>;

pub type RequiredResolverFn<I, O, CtxOptions> =
    Box<dyn Fn(&mut IvoSummary<I, O, CtxOptions>) -> (bool, &str) + Send + Sync>;

pub type AsyncResolverWithMiniSummaryFn<T, CtxOptions> =
    Box<dyn Fn(&mut IvoMiniSummary<CtxOptions>) -> BoxFuture<'static, T> + Send + Sync + 'static>;

pub type ResolverWithMiniSummaryFn<T, CtxOptions> =
    Box<dyn Fn(&mut IvoMiniSummary<CtxOptions>) -> T + Send + Sync>;

pub enum ResolverWithMutSummary<T, I: HasPartial, O: HasPartial, CtxOptions> {
    Async(AsyncResolverWithMutSummaryFn<T, I, O, CtxOptions>),
    Sync(ResolverWithMutSummaryFn<T, I, O, CtxOptions>),
}

pub type AsyncResolverWithMutSummaryFn<T, I, O, CtxOptions> =
    Box<dyn Fn(&mut IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, T> + Send + Sync + 'static>;

pub type ResolverWithMutSummaryFn<T, I, O, CtxOptions> =
    Box<dyn Fn(&mut IvoSummary<I, O, CtxOptions>) -> T + Send + Sync>;

pub type BooleanResolverWithMutSummary<I, O, CtxOptions> =
    Box<dyn Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync>;

pub type VirtualSanitiser<T, I, O, CtxOptions> = ResolverWithMutSummaryFn<T, I, O, CtxOptions>;

pub type DeleteHandler<O, CtxOptions> = Box<dyn Fn(&O, &CtxOptions)>;
pub type FailureHandler<I, O, CtxOptions> = Box<dyn Fn(&IvoSummary<I, O, CtxOptions>)>;
pub type SuccessHandler<I, O, CtxOptions> = Box<dyn Fn(&IvoSummary<I, O, CtxOptions>)>;

pub type ValidatorResponse<T> = Result<T, (&'static str, Option<Value>)>;

pub type ValidatorFn<T> = Box<dyn Fn(&Value) -> ValidatorResponse<T> + Send + Sync + 'static>;
