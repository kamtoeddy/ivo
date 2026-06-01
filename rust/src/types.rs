use std::collections::HashMap;

use futures::future::BoxFuture;
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

pub type UniformValidator<CtxOptions> =
    Box<dyn Fn(Value, IvoSummary<CtxOptions>) -> ValidatorResponse<Value> + Send + Sync + 'static>;

pub type UniformAsyncValidator<CtxOptions> = Box<
    dyn Fn(Value, IvoSummary<CtxOptions>) -> BoxFuture<'static, ValidatorResponse<Value>>
        + Send
        + Sync
        + 'static,
>;

pub type UniformReValidator<CtxOptions> =
    Box<dyn Fn(Value, IvoSummary<CtxOptions>) -> ValidatorResponse<Value> + Send + Sync + 'static>;

pub type UniformAsyncReValidator<CtxOptions> = Box<
    dyn Fn(Value, IvoSummary<CtxOptions>) -> BoxFuture<'static, ValidatorResponse<Value>>
        + Send
        + Sync
        + 'static,
>;

pub type UniformVirtualSanitiser<CtxOptions> =
    Box<dyn Fn(IvoSummary<CtxOptions>) -> BoxFuture<'static, Value> + Send + Sync + 'static>;

pub type UniformEnumErrorResolver =
    Box<dyn Fn((Value, &Vec<Value>)) -> &'static str + Send + Sync + 'static>;

pub type UniformResolverWithMutSummary<CtxOptions> =
    Box<dyn Fn(IvoSummary<CtxOptions>) -> Value + Send + Sync + 'static>;

pub type UniformAsyncResolverWithMutSummary<CtxOptions> =
    Box<dyn Fn(IvoSummary<CtxOptions>) -> BoxFuture<'static, Value> + Send + Sync + 'static>;

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

pub enum ComputableInit<CtxOptions: Clone> {
    False,
    Func(ResolverWithMutSummaryFn<bool, CtxOptions>),
}

pub enum ComputableRequired<CtxOptions: Clone> {
    Static(True),
    Func(RequiredResolverFn<CtxOptions>),
}

pub type Context = HashMap<String, Value>;

#[derive(Clone)]
pub struct IvoMiniSummary<CtxOptions: Clone> {
    context: Context,
    options: CtxOptions,
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
pub struct IvoSummary<CtxOptions: Clone> {
    // pub struct IvoSummary<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    // _input: PhantomData<I>,
    changes: Option<HashMap<String, Value>>,
    context: Context,
    // input: Partial<I>,
    input_values: HashMap<String, Value>,
    is_update: bool,
    // previous_values: Option<O>,
    // values: O,
    options: CtxOptions,
}

impl<CtxOptions: Clone> IvoSummary<CtxOptions> {
    pub fn new(
        changes: Option<HashMap<String, Value>>,
        context: Context,
        // input: Partial<I>,
        input_values: HashMap<String, Value>,
        is_update: bool,
        // previous_values: Option<O>,
        // values: O,
        options: CtxOptions,
    ) -> Self {
        Self {
            changes,
            context,
            // _input: PhantomData,
            input_values,
            is_update,
            // previous_values,
            // values,
            options,
        }
    }

    pub fn changes(&self) -> &Option<HashMap<String, Value>> {
        &self.changes
    }

    pub fn ctx(&self) -> &Context {
        &self.context
    }

    // pub fn input(&self) -> &Partial<I> {
    //     &self.input
    // }

    pub fn input_values(&self) -> &HashMap<String, Value> {
        &self.input_values
    }

    pub fn is_update(&self) -> bool {
        self.is_update
    }

    // pub fn previous_values(&self) -> &Option<O> {
    //     &self.previous_values
    // }

    // pub fn values(&self) -> &O {
    //     &self.values
    // }

    pub fn options(&self) -> &CtxOptions {
        &self.options
    }

    pub fn update_options(&mut self) {
        todo!()
    }
}

pub enum FieldValidator<CtxOptions: Clone> {
    Async(UniformAsyncValidator<CtxOptions>),
    Sync(UniformValidator<CtxOptions>),
}

pub enum FieldReValidator<CtxOptions: Clone> {
    Async(UniformAsyncReValidator<CtxOptions>),
    Sync(UniformReValidator<CtxOptions>),
}

pub type RequiredResolverFn<CtxOptions> = Box<
    dyn Fn(IvoSummary<CtxOptions>) -> BoxFuture<'static, (bool, &'static str)>
        + Send
        + Sync
        + 'static,
>;

pub enum ResolverWithMutSummary<T, CtxOptions: Clone> {
    Async(AsyncResolverWithMutSummaryFn<T, CtxOptions>),
    Sync(ResolverWithMutSummaryFn<T, CtxOptions>),
}

pub type AsyncResolverWithMutSummaryFn<T, CtxOptions> =
    Box<dyn Fn(IvoSummary<CtxOptions>) -> BoxFuture<'static, T> + Send + Sync + 'static>;

pub type ResolverWithMutSummaryFn<T, CtxOptions> =
    Box<dyn Fn(IvoSummary<CtxOptions>) -> T + Send + Sync + 'static>;

pub type BooleanResolverWithMutSummary<CtxOptions> =
    Box<dyn Fn(IvoSummary<CtxOptions>) -> bool + Send + Sync + 'static>;

pub type VirtualSanitiser<T, CtxOptions> = AsyncResolverWithMutSummaryFn<T, CtxOptions>;

pub type DeleteHandler<O, CtxOptions> =
    Box<dyn Fn(&O, &CtxOptions) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type FailureHandler<CtxOptions> =
    Box<dyn Fn(IvoSummary<CtxOptions>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type SuccessHandler<CtxOptions> =
    Box<dyn Fn(IvoSummary<CtxOptions>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type ValidatorResponse<T> = Result<T, (&'static str, Option<Value>)>;

pub type ValidatorFn<T> = Box<dyn Fn(Value) -> ValidatorResponse<T> + Send + Sync + 'static>;
