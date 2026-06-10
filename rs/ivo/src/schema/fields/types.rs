use futures::future::{BoxFuture, FutureExt};
use std::{fmt::Debug, future::Future};

use crate::{
    erase_value, parse_or_panic,
    types::{DeleteHandler, FailureHandler, SuccessHandler, True},
    ErasedValue, IvoContext, IvoErrorTool, IvoMiniContext, IvoSchemaStruct, ValidatorResponse,
};

pub trait IntoDeleteHandler<O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_handler(self) -> DeleteHandler<O, CtxOptions>;
}

impl<F, Fut, Data, CtxOptions: Clone> IntoDeleteHandler<Data, CtxOptions> for F
where
    Data: IvoSchemaStruct,
    F: Fn(Data, CtxOptions) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    fn into_handler(self) -> DeleteHandler<Data, CtxOptions> {
        Box::new(move |data, o| Box::pin(self(data, o)))
    }
}

pub trait IntoFailureHandler<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_handler(self) -> FailureHandler<I, O, CtxOptions>;
}

impl<F, Fut, I, O, CtxOptions: Clone> IntoFailureHandler<I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    F: Fn(IvoContext<I, O>, CtxOptions) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    fn into_handler(self) -> FailureHandler<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub trait IntoSuccessHandler<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_handler(self) -> SuccessHandler<I, O, CtxOptions>;
}

impl<F, Fut, I, O, CtxOptions: Clone> IntoSuccessHandler<I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    F: Fn(IvoContext<I, O>, CtxOptions) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    fn into_handler(self) -> SuccessHandler<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub trait IntoFieldValidator<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrorTool: IvoErrorTool,
>
{
    fn into_uniform(self) -> UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>;
}

impl<F, Fut, T, I, O, CtxOptions: Clone, ErrorTool>
    IntoFieldValidator<T, I, O, CtxOptions, ErrorTool> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    ErrorTool: IvoErrorTool,
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(T, IvoContext<I, O>, CtxOptions) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ValidatorResponse<T, ErrorTool::FieldMetadata>> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata> {
        Box::new(move |v, ctx, o| {
            Box::pin(self(parse_or_panic::<T>(&v), ctx, o).map(|r| r.map(|v| erase_value(v))))
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
    F: Fn(IvoContext<I, O>, CtxOptions) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformVirtualSanitiser<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o).map(|v| erase_value(v))))
    }
}

pub trait IntoRequiredErrorResolver<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_resolver(self) -> RequiredResolver<I, O, CtxOptions>;
}

impl<F, Fut, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone + Send + 'static>
    IntoRequiredErrorResolver<I, O, CtxOptions> for F
where
    F: Fn(IvoContext<I, O>, CtxOptions) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = String> + Send + 'static,
{
    fn into_resolver(self) -> RequiredResolver<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o).map(|e| (true, e))))
    }
}

pub trait IntoRequiredResolver<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_resolver(self) -> RequiredResolver<I, O, CtxOptions>;
}

impl<F, Fut, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone + Send + 'static>
    IntoRequiredResolver<I, O, CtxOptions> for F
where
    F: Fn(IvoContext<I, O>, CtxOptions) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = RequiredError> + Send + 'static,
{
    fn into_resolver(self) -> RequiredResolver<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub trait IntoResolverWithMutSummary<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_resolver(self) -> ResolverWithMutSummary<T, I, O, CtxOptions>;
}

impl<F, Fut, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    IntoResolverWithMutSummary<T, I, O, CtxOptions> for F
where
    T: 'static,
    F: Fn(IvoContext<I, O>, CtxOptions) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_resolver(self) -> ResolverWithMutSummary<T, I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
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
    F: Fn(IvoContext<I, O>, CtxOptions) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformResolverWithMutSummary<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o).map(|v| erase_value(v))))
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
    F: Fn(IvoMiniContext<I, O>, CtxOptions) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformResolverWithMiniSummary<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o).map(|v| erase_value(v))))
    }
}

pub type UniformValidator<I, O, CtxOptions, FieldMetadata> = Box<
    dyn Fn(
            &ErasedValue,
            IvoContext<I, O>,
            CtxOptions,
        ) -> BoxFuture<'static, ValidatorResponse<ErasedValue, FieldMetadata>>
        + Send
        + Sync
        + 'static,
>;

pub type UniformVirtualSanitiser<I, O, CtxOptions> = Box<
    dyn Fn(IvoContext<I, O>, CtxOptions) -> BoxFuture<'static, ErasedValue> + Send + Sync + 'static,
>;

pub type UniformResolverWithMutSummary<I, O, CtxOptions> = Box<
    dyn Fn(IvoContext<I, O>, CtxOptions) -> BoxFuture<'static, ErasedValue> + Send + Sync + 'static,
>;

pub type UniformResolverWithMiniSummary<I, O, CtxOptions> = Box<
    dyn Fn(IvoMiniContext<I, O>, CtxOptions) -> BoxFuture<'static, ErasedValue>
        + Send
        + Sync
        + 'static,
>;

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
    dyn Fn(IvoContext<I, O>, CtxOptions) -> BoxFuture<'static, RequiredError>
        + Send
        + Sync
        + 'static,
>;

pub type ResolverWithMutSummary<T, I, O, CtxOptions> =
    Box<dyn Fn(IvoContext<I, O>, CtxOptions) -> BoxFuture<'static, T> + Send + Sync + 'static>;

pub type BooleanResolverWithMutSummary<I, O, CtxOptions> =
    ResolverWithMutSummary<bool, I, O, CtxOptions>;

pub type VirtualSanitiser<T, I, O, CtxOptions> = ResolverWithMutSummary<T, I, O, CtxOptions>;
