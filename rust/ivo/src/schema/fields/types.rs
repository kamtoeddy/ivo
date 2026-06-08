use futures::future::{BoxFuture, FutureExt};
use std::{fmt::Debug, future::Future};

use crate::{
    erase_value, parse_or_panic,
    types::{DeleteHandler, FailureHandler, SuccessHandler, True},
    ErasedValue, IvoErrorTool, IvoMiniSummary, IvoSchemaStruct, IvoSummary, ValidatorError,
    ValidatorResponse,
};

pub trait IntoDeleteHandler<O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_handler(self) -> DeleteHandler<O, CtxOptions>;
}

impl<F, Fut, O, CtxOptions: Clone> IntoDeleteHandler<O, CtxOptions> for F
where
    O: IvoSchemaStruct,
    F: Fn(O, CtxOptions) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    fn into_handler(self) -> DeleteHandler<O, CtxOptions> {
        Box::new(move |o, s| Box::pin(self(o, s)))
    }
}

pub trait IntoFailureHandler<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_handler(self) -> FailureHandler<I, O, CtxOptions>;
}

impl<F, Fut, I, O, CtxOptions: Clone> IntoFailureHandler<I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    fn into_handler(self) -> FailureHandler<I, O, CtxOptions> {
        Box::new(move |s| Box::pin(self(s)))
    }
}

pub trait IntoSuccessHandler<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_handler(self) -> SuccessHandler<I, O, CtxOptions>;
}

impl<F, Fut, I, O, CtxOptions: Clone> IntoSuccessHandler<I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    fn into_handler(self) -> SuccessHandler<I, O, CtxOptions> {
        Box::new(move |s| Box::pin(self(s)))
    }
}

pub trait IntoFieldValidator<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
>
{
    fn into_uniform(self) -> UniformValidator<I, O, CtxOptions, ErrT::FieldMetadata>;
}

impl<F, Fut, T, I, O, CtxOptions: Clone, ErrT> IntoFieldValidator<T, I, O, CtxOptions, ErrT> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    ErrT: IvoErrorTool,
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(T, IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ValidatorResponse<T, ErrT::FieldMetadata>> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformValidator<I, O, CtxOptions, ErrT::FieldMetadata> {
        Box::new(move |v, s| {
            Box::pin(self(parse_or_panic::<T>(&v), s).map(|r| r.map(|v| erase_value(v))))
        })
    }
}

pub trait IntoVirtualSanitizer<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_uniform(self) -> UniformVirtualSanitiser<I, O, CtxOptions>;
}

impl<F, Fut, T, I, O, CtxOptions: Clone> IntoVirtualSanitizer<T, I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformVirtualSanitiser<I, O, CtxOptions> {
        Box::new(move |s| Box::pin(self(s).map(|v| erase_value(v))))
    }
}

pub trait IntoEnumErrorResolver<T, ErrT: IvoErrorTool> {
    fn into_uniform(self) -> UniformEnumErrorResolver<ErrT::FieldMetadata>;
}

impl<F, T, ErrT> IntoEnumErrorResolver<T, ErrT> for F
where
    ErrT: IvoErrorTool,
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn((T, Vec<T>)) -> ValidatorError<ErrT::FieldMetadata> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformEnumErrorResolver<ErrT::FieldMetadata> {
        Box::new(move |(v, list)| {
            self((
                parse_or_panic::<T>(&v),
                list.into_iter().map(|v| parse_or_panic::<T>(&v)).collect(),
            ))
        })
    }
}

pub trait IntoRequiredResolverFn<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_resolver(self) -> RequiredResolver<I, O, CtxOptions>;
}

impl<F, Fut, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone + Send + 'static>
    IntoRequiredResolverFn<I, O, CtxOptions> for F
where
    F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = RequiredError> + Send + 'static,
{
    fn into_resolver(self) -> RequiredResolver<I, O, CtxOptions> {
        Box::new(move |s| Box::pin(self(s)))
    }
}

pub trait IntoResolverWithMutSummaryFn<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
{
    fn into_resolver(self) -> ResolverWithMutSummary<T, I, O, CtxOptions>;
}

impl<F, Fut, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    IntoResolverWithMutSummaryFn<T, I, O, CtxOptions> for F
where
    T: 'static,
    F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_resolver(self) -> ResolverWithMutSummary<T, I, O, CtxOptions> {
        Box::new(move |s| Box::pin(self(s)))
    }
}

pub trait IntoUniformResolverWithMutSummary<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
>
{
    fn into_uniform(self) -> UniformResolverWithMutSummary<I, O, CtxOptions>;
}

impl<F, Fut, T, I, O, CtxOptions: Clone> IntoUniformResolverWithMutSummary<T, I, O, CtxOptions>
    for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformResolverWithMutSummary<I, O, CtxOptions> {
        Box::new(move |s| Box::pin(self(s).map(|v| erase_value(v))))
    }
}

pub trait IntoResolverWithMiniSummary<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
{
    fn into_uniform(self) -> UniformResolverWithMiniSummary<I, O, CtxOptions>;
}

impl<F, Fut, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    IntoResolverWithMiniSummary<T, I, O, CtxOptions> for F
where
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(IvoMiniSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformResolverWithMiniSummary<I, O, CtxOptions> {
        Box::new(move |s| Box::pin(self(s).map(|v| erase_value(v))))
    }
}

pub trait IntoBooleanResolverWithMutSummary<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
>
{
    fn into_resolver(self) -> BooleanResolverWithMutSummary<I, O, CtxOptions>;
}

impl<F, Fut, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    IntoBooleanResolverWithMutSummary<I, O, CtxOptions> for F
where
    F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = bool> + Send + 'static,
{
    fn into_resolver(self) -> BooleanResolverWithMutSummary<I, O, CtxOptions> {
        Box::new(move |s| Box::pin(self(s)))
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

pub enum ComputableRequiredError<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    Static(&'static str),
    Func(RequiredResolver<I, O, CtxOptions>),
}

pub type RequiredError = (bool, String);

pub type RequiredResolver<I, O, CtxOptions> = Box<
    dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, RequiredError>
        + Send
        + Sync
        + 'static,
>;

pub type ResolverWithMutSummary<T, I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, T> + Send + Sync + 'static>;

pub type BooleanResolverWithMutSummary<I, O, CtxOptions> =
    ResolverWithMutSummary<bool, I, O, CtxOptions>;

pub type VirtualSanitiser<T, I, O, CtxOptions> = ResolverWithMutSummary<T, I, O, CtxOptions>;
