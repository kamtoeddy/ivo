use std::fmt::Debug;

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

pub struct IvoMiniSummary<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    input: I::Partial,
    input_values: I::Partial,
    values: O::Partial,
    options: CtxOptions,
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> IvoMiniSummary<I, O, CtxOptions> {
    pub fn new(
        input: I::Partial,
        input_values: I::Partial,
        values: O::Partial,
        options: CtxOptions,
    ) -> Self {
        Self {
            input,
            input_values,
            values,
            options,
        }
    }

    /// contains validated and up to date version of input_values
    pub fn input(&self) -> I::Partial {
        self.input.clone()
    }

    /// contains values provided at the start of the process
    pub fn input_values(&self) -> I::Partial {
        self.input_values.clone()
    }

    /// subset of output values related to current process
    pub fn values(&self) -> O::Partial {
        self.values.clone()
    }

    pub fn options(&self) -> &CtxOptions {
        &self.options
    }

    pub fn update_options(&mut self) {
        todo!()
    }
}

pub enum IvoSummary<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    Create {
        input: I::Partial,
        input_values: I::Partial,
        values: O::Partial,
        options: CtxOptions,
    },
    Update {
        changes: O::Partial,
        input: I::Partial,
        input_values: I::Partial,
        previous_values: O,
        values: O,
        options: CtxOptions,
    },
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> IvoSummary<I, O, CtxOptions> {
    pub fn for_new(
        input: I::Partial,
        input_values: I::Partial,
        values: O::Partial,
        options: CtxOptions,
    ) -> Self {
        Self::Create {
            input,
            input_values,
            values,
            options,
        }
    }

    pub fn for_update(
        changes: O::Partial,
        input: I::Partial,
        input_values: I::Partial,
        previous_values: O,
        values: O,
        options: CtxOptions,
    ) -> Self {
        Self::Update {
            changes,
            input,
            input_values,
            previous_values,
            values,
            options,
        }
    }

    /// subset of output values related to which will be
    /// part of the final output of the current process
    pub fn changes(&self) -> O::Partial {
        match &self {
            IvoSummary::Create { values, .. } => values,
            IvoSummary::Update { changes, .. } => changes,
        }
        .clone()
    }

    pub fn is_update(&self) -> bool {
        matches!(self, IvoSummary::Update { .. })
    }

    /// contains validated and up to date version of input_values
    pub fn input(&self) -> I::Partial {
        match &self {
            IvoSummary::Create { input, .. } => input,
            IvoSummary::Update { input, .. } => input,
        }
        .clone()
    }

    /// contains values provided at the start of the current process
    pub fn input_values(&self) -> I::Partial {
        match &self {
            IvoSummary::Create { input_values, .. } => input_values,
            IvoSummary::Update { input_values, .. } => input_values,
        }
        .clone()
    }

    /// subset of output values related to current process
    pub fn values(&self) -> O::Partial {
        match &self {
            IvoSummary::Create { values, .. } => values.clone(),
            IvoSummary::Update { values, .. } => values.clone().into(),
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

pub type UniformValidator<I, O, CtxOptions, FieldMetadata> = Box<
    dyn Fn(
            ErasedValue,
            IvoSummary<I, O, CtxOptions>,
        ) -> BoxFuture<'static, ValidatorResponse<ErasedValue, FieldMetadata>>
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

pub type UniformResolverWithMutSummary<I, O, CtxOptions> = Box<
    dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ErasedValue> + Send + Sync + 'static,
>;

pub type UniformResolverWithMiniSummary<I, O, CtxOptions> = Box<
    dyn Fn(IvoMiniSummary<I, O, CtxOptions>) -> BoxFuture<'static, ErasedValue>
        + Send
        + Sync
        + 'static,
>;

pub enum ComputableEnumeratedError<ErrT: IvoErrorTool> {
    Static(String),
    Func(UniformEnumErrorResolver<ErrT::FieldMetadata>),
}

pub enum ComputableWithMiniSummary<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    Static(T),
    Func(UniformResolverWithMiniSummary<I, O, CtxOptions>),
}

pub enum ComputableInit<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    False,
    Func(BooleanResolverWithMutSummary<I, O, CtxOptions>),
}

pub enum ComputableRequired<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    Static(True),
    Func(RequiredResolver<I, O, CtxOptions>),
}

pub type RequiredResolver<I, O, CtxOptions> = Box<
    dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, (bool, String)>
        + Send
        + Sync
        + 'static,
>;

pub type ResolverWithMutSummary<T, I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, T> + Send + Sync + 'static>;

pub type BooleanResolverWithMutSummary<I, O, CtxOptions> =
    ResolverWithMutSummary<bool, I, O, CtxOptions>;

pub type VirtualSanitiser<T, I, O, CtxOptions> = ResolverWithMutSummary<T, I, O, CtxOptions>;

pub type DeleteHandler<O, CtxOptions> =
    Box<dyn Fn(O, CtxOptions) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type FailureHandler<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type SuccessHandler<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type ValidatorResponse<T, ErrorMetadata = DefaultFieldErrorMetadata> =
    Result<T, ValidatorError<ErrorMetadata>>;

pub type ValidatorError<FieldErrorMetadata> = (String, Option<FieldErrorMetadata>);
