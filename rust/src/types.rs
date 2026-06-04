use std::{any::Any, collections::HashMap};

use futures::future::BoxFuture;
use serde_json::Value;

use crate::traits::IvoSchemaStruct;

pub trait CloneableAny: Any + Send + Sync {
    fn clone_box(&self) -> Box<dyn CloneableAny>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// 2. Implement this trait for ANY type that already implements Clone + Any + Send + Sync
impl<T> CloneableAny for T
where
    T: Clone + Any + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableAny> {
        Box::new(self.clone()) // This triggers the concrete type's clone method!
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// 3. Implement standard Clone for our uniform Box type
impl Clone for ErasedStuff {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub type ErasedStuff = Box<dyn CloneableAny + Send + Sync>;

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
    dyn Fn(ErasedStuff, IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<ErasedStuff>
        + Send
        + Sync
        + 'static,
>;

pub type UniformAsyncValidator<I, O, CtxOptions> = Box<
    dyn Fn(
            ErasedStuff,
            IvoSummary<I, O, CtxOptions>,
        ) -> BoxFuture<'static, ValidatorResponse<ErasedStuff>>
        + Send
        + Sync
        + 'static,
>;

pub type UniformReValidator<I, O, CtxOptions> = Box<
    dyn Fn(ErasedStuff, IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<ErasedStuff>
        + Send
        + Sync
        + 'static,
>;

pub type UniformAsyncReValidator<I, O, CtxOptions> = Box<
    dyn Fn(
            ErasedStuff,
            IvoSummary<I, O, CtxOptions>,
        ) -> BoxFuture<'static, ValidatorResponse<ErasedStuff>>
        + Send
        + Sync
        + 'static,
>;

pub type UniformVirtualSanitiser<I, O, CtxOptions> = Box<
    dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ErasedStuff> + Send + Sync + 'static,
>;

pub type UniformEnumErrorResolver =
    Box<dyn Fn((ErasedStuff, Vec<ErasedStuff>)) -> String + Send + Sync + 'static>;

pub type UniformResolverWithMutSummary<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> ErasedStuff + Send + Sync + 'static>;

pub type UniformAsyncResolverWithMutSummary<I, O, CtxOptions> = Box<
    dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ErasedStuff> + Send + Sync + 'static,
>;

pub type UniformResolverWithMiniSummary<CtxOptions> =
    Box<dyn Fn(IvoMiniSummary<CtxOptions>) -> ErasedStuff + Send + Sync + 'static>;

pub type UniformAsyncResolverWithMiniSummary<CtxOptions> = Box<
    dyn Fn(IvoMiniSummary<CtxOptions>) -> BoxFuture<'static, ErasedStuff> + Send + Sync + 'static,
>;

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

pub type Context = HashMap<String, ErasedStuff>;

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

type InputValues = HashMap<String, ErasedStuff>;
type Changes = HashMap<String, ErasedStuff>;

#[derive(Clone)]
// pub struct IvoSummary<CtxOptions: Clone> {
pub enum IvoSummary<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    Create {
        context: Context,
        input: I::Partial,
        input_values: InputValues,
        // values: O,
        options: CtxOptions,
    },
    Update {
        changes: Changes,
        context: Context,
        input: I::Partial,
        input_values: InputValues,
        previous_values: O,
        values: O,
        options: CtxOptions,
    },
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> IvoSummary<I, O, CtxOptions> {
    pub fn for_new(
        context: Context,
        input: I::Partial,
        input_values: HashMap<String, ErasedStuff>,
        // values: O,
        options: CtxOptions,
    ) -> Self {
        Self::Create {
            context,
            input,
            input_values,
            // values,
            options,
        }
    }

    pub fn for_update(
        changes: HashMap<String, ErasedStuff>,
        context: Context,
        input: I::Partial,
        input_values: HashMap<String, ErasedStuff>,
        previous_values: O,
        values: O,
        options: CtxOptions,
    ) -> Self {
        Self::Update {
            changes,
            context,
            input,
            input_values,
            previous_values,
            values,
            options,
        }
    }

    pub fn changes(&self) -> Option<&Changes> {
        match &self {
            IvoSummary::Update { changes, .. } => Some(changes),
            _ => None,
        }
    }

    pub fn is_update(&self) -> bool {
        matches!(self, IvoSummary::Update { .. })
    }

    pub fn input(&self) -> &I::Partial {
        match &self {
            IvoSummary::Create { input, .. } => input,
            IvoSummary::Update { input, .. } => input,
        }
    }

    pub fn input_values(&self) -> &InputValues {
        match &self {
            IvoSummary::Create { input_values, .. } => input_values,
            IvoSummary::Update { input_values, .. } => input_values,
        }
    }

    pub fn values(&self) -> Option<&O> {
        match &self {
            // IvoSummary::Create { values, .. } => values,
            IvoSummary::Update { values, .. } => Some(values),
            _ => None,
        }
    }

    pub fn get_options(&self) -> &CtxOptions {
        match &self {
            IvoSummary::Create { options, .. } => options,
            IvoSummary::Update { options, .. } => options,
        }
    }

    pub fn get_options_mut(&self) -> CtxOptions {
        match &self {
            IvoSummary::Create { options, .. } => options.clone(),
            IvoSummary::Update { options, .. } => options.clone(),
        }
    }

    pub fn update_options(&mut self) {
        // todo!()
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
    Box<dyn Fn(O, CtxOptions) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type FailureHandler<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type SuccessHandler<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type ValidatorResponse<T> = Result<T, (String, Option<Value>)>;

pub type ValidatorFn<T> = Box<dyn Fn(T) -> ValidatorResponse<T> + Send + Sync + 'static>;
