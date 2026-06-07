use std::{collections::HashMap, fmt::Debug};

use futures::future::BoxFuture;

use crate::{
    schema::error::{DefaultFieldErrorMetadata, IvoErrorTool},
    traits::IvoSchemaStruct,
    utils::erased_value::ErasedValue,
};

#[derive(Debug)]
pub struct True;

// Marker Types
pub struct Yes;
pub struct No;
pub struct YesComputed;

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

pub type UniformAsyncValidator<I, O, CtxOptions, FieldErrorMetadata> = Box<
    dyn Fn(
            ErasedValue,
            IvoSummary<I, O, CtxOptions>,
        ) -> BoxFuture<'static, ValidatorResponse<ErasedValue, FieldErrorMetadata>>
        + Send
        + Sync
        + 'static,
>;

pub type UniformValidator<I, O, CtxOptions, FieldErrorMetadata> = Box<
    dyn Fn(
            ErasedValue,
            IvoSummary<I, O, CtxOptions>,
        ) -> ValidatorResponse<ErasedValue, FieldErrorMetadata>
        + Send
        + Sync
        + 'static,
>;

pub type UniformVirtualSanitiser<I, O, CtxOptions> = Box<
    dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ErasedValue> + Send + Sync + 'static,
>;

pub type UniformEnumErrorResolver<FieldErrorMetadata> = Box<
    dyn Fn((ErasedValue, Vec<ErasedValue>)) -> ValidatorError<FieldErrorMetadata>
        + Send
        + Sync
        + 'static,
>;

pub type UniformResolverWithMutSummary<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> ErasedValue + Send + Sync + 'static>;

pub type UniformAsyncResolverWithMutSummary<I, O, CtxOptions> = Box<
    dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ErasedValue> + Send + Sync + 'static,
>;

pub type UniformResolverWithMiniSummary<CtxOptions> =
    Box<dyn Fn(IvoMiniSummary<CtxOptions>) -> ErasedValue + Send + Sync + 'static>;

pub type UniformAsyncResolverWithMiniSummary<CtxOptions> = Box<
    dyn Fn(IvoMiniSummary<CtxOptions>) -> BoxFuture<'static, ErasedValue> + Send + Sync + 'static,
>;

pub enum ComputableEnumeratedError<ErrT: IvoErrorTool> {
    Static(String),
    Func(UniformEnumErrorResolver<ErrT::FieldMetadata>),
}

pub enum ComputableWithMiniSummary<T, CtxOptions: Clone> {
    Static(T),
    AsyncFunc(UniformAsyncResolverWithMiniSummary<CtxOptions>),
    SyncFunc(UniformResolverWithMiniSummary<CtxOptions>),
}

pub enum ComputableInit<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    False,
    AsyncFunc(AsyncResolverWithMutSummaryFn<bool, I, O, CtxOptions>),
    SyncFunc(ResolverWithMutSummaryFn<bool, I, O, CtxOptions>),
}

pub enum ComputableRequired<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    Static(True),
    Func(RequiredResolverFn<I, O, CtxOptions>),
}

pub type Context = HashMap<String, ErasedValue>;

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

type InputValues = HashMap<String, ErasedValue>;
type Changes = HashMap<String, ErasedValue>;

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
        input_values: HashMap<String, ErasedValue>,
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
        changes: HashMap<String, ErasedValue>,
        context: Context,
        input: I::Partial,
        input_values: HashMap<String, ErasedValue>,
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

pub enum FieldValidator<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
> {
    Async(UniformAsyncValidator<I, O, CtxOptions, ErrT::FieldMetadata>),
    Sync(UniformValidator<I, O, CtxOptions, ErrT::FieldMetadata>),
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

pub type PostValidatorValue = Vec<(&'static str, ErasedValue)>;
pub type PostValidatorError<FieldErrorMetadata> =
    Vec<(&'static str, ValidatorError<FieldErrorMetadata>)>;

pub type PostValidatorFn<I, O, CtxOptions, FieldErrorMetadata> = Box<
    dyn Fn(
            IvoSummary<I, O, CtxOptions>,
        )
            -> BoxFuture<'static, Result<PostValidatorValue, PostValidatorError<FieldErrorMetadata>>>
        + Send
        + Sync
        + 'static,
>;

pub type DeleteHandler<O, CtxOptions> =
    Box<dyn Fn(O, CtxOptions) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type FailureHandler<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type SuccessHandler<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type ValidatorResponse<T, ErrorMetadata = DefaultFieldErrorMetadata> =
    Result<T, ValidatorError<ErrorMetadata>>;

pub type ValidatorError<FieldErrorMetadata> = (String, Option<FieldErrorMetadata>);
