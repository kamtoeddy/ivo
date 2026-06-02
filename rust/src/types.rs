use std::collections::HashMap;

use futures::future::BoxFuture;
use serde_json::Value;

use crate::traits::IvoSchemaStruct;

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

pub type UniformValidator<I, O, CtxOptions> = Box<
    dyn Fn(Value, IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<Value> + Send + Sync + 'static,
>;

pub type UniformAsyncValidator<I, O, CtxOptions> = Box<
    dyn Fn(Value, IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ValidatorResponse<Value>>
        + Send
        + Sync
        + 'static,
>;

pub type UniformReValidator<I, O, CtxOptions> = Box<
    dyn Fn(Value, IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<Value> + Send + Sync + 'static,
>;

pub type UniformAsyncReValidator<I, O, CtxOptions> = Box<
    dyn Fn(Value, IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ValidatorResponse<Value>>
        + Send
        + Sync
        + 'static,
>;

pub type UniformVirtualSanitiser<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, Value> + Send + Sync + 'static>;

pub type UniformEnumErrorResolver =
    Box<dyn Fn((Value, Vec<Value>)) -> String + Send + Sync + 'static>;

pub type UniformResolverWithMutSummary<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> Value + Send + Sync + 'static>;

pub type UniformAsyncResolverWithMutSummary<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, Value> + Send + Sync + 'static>;

pub type UniformResolverWithMiniSummary<CtxOptions> =
    Box<dyn Fn(IvoMiniSummary<CtxOptions>) -> Value + Send + Sync + 'static>;

pub type UniformAsyncResolverWithMiniSummary<CtxOptions> =
    Box<dyn Fn(IvoMiniSummary<CtxOptions>) -> BoxFuture<'static, Value> + Send + Sync + 'static>;

pub enum ComputableEnumeratedError {
    Static(String),
    Func(UniformEnumErrorResolver),
}

pub enum ComputableWithMiniSummary<T, CtxOptions: Clone> {
    Static(T),
    AsyncFunc(UniformAsyncResolverWithMiniSummary<CtxOptions>),
    SyncFunc(UniformResolverWithMiniSummary<CtxOptions>),
}

pub enum ComputableInit<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    False,
    Func(ResolverWithMutSummaryFn<bool, I, O, CtxOptions>),
}

pub enum ComputableRequired<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    Static(True),
    Func(RequiredResolverFn<I, O, CtxOptions>),
}

pub type Context = HashMap<String, Value>;

#[derive(Clone)]
pub struct IvoMiniSummary<CtxOptions: Clone> {
    pub context: Context,
    pub options: CtxOptions,
}

impl<CtxOptions: Clone> IvoMiniSummary<CtxOptions> {
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

#[derive(Clone)]
// pub struct IvoSummary<CtxOptions: Clone> {
pub struct IvoSummary<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    // _input: PhantomData<I>,
    pub changes: Option<HashMap<String, Value>>,
    pub context: Context,
    pub input: I::Partial,
    pub input_values: HashMap<String, Value>,
    pub is_update: bool,
    pub previous_values: Option<O>,
    pub values: O,
    pub options: CtxOptions,
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> IvoSummary<I, O, CtxOptions> {
    pub fn new(
        changes: Option<HashMap<String, Value>>,
        context: Context,
        input: I::Partial,
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

    // pub fn is_update(&self) -> bool {
    //     self.is_update
    // }

    pub fn update_options(&mut self) {
        todo!()
    }
}

pub enum FieldValidator<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    Async(UniformAsyncValidator<I, O, CtxOptions>),
    Sync(UniformValidator<I, O, CtxOptions>),
}

pub enum FieldReValidator<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    Async(UniformAsyncReValidator<I, O, CtxOptions>),
    Sync(UniformReValidator<I, O, CtxOptions>),
}

pub type RequiredResolverFn<I, O, CtxOptions> = Box<
    dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, (bool, String)>
        + Send
        + Sync
        + 'static,
>;

pub enum ResolverWithMutSummary<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    Async(AsyncResolverWithMutSummaryFn<T, I, O, CtxOptions>),
    Sync(ResolverWithMutSummaryFn<T, I, O, CtxOptions>),
}

pub type AsyncResolverWithMutSummaryFn<T, I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, T> + Send + Sync + 'static>;

pub type ResolverWithMutSummaryFn<T, I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> T + Send + Sync + 'static>;

pub type BooleanResolverWithMutSummary<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static>;

pub type VirtualSanitiser<T, I, O, CtxOptions> = AsyncResolverWithMutSummaryFn<T, I, O, CtxOptions>;

pub type DeleteHandler<O, CtxOptions> =
    Box<dyn Fn(&O, &CtxOptions) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type FailureHandler<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type SuccessHandler<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type ValidatorResponse<T> = Result<T, (String, Option<Value>)>;

pub type ValidatorFn<T> = Box<dyn Fn(Value) -> ValidatorResponse<T> + Send + Sync + 'static>;
