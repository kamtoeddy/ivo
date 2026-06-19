#![allow(type_alias_bounds)]

use futures::future::{BoxFuture, FutureExt};

use std::future::Future;

use crate::{
    erase_value, parse_or_panic,
    types::{DeleteHandler, ErasedValue, FailureHandler, IvoFieldValue, SuccessHandler},
    IvoErrorTool, IvoSchemaStruct, SharedData, SharedIvoContext, SharedIvoMiniContext,
    SharedRwCtxOptions, ValidatorResponse,
};

pub type TimestampResolver<T: IvoFieldValue> = Box<dyn Fn() -> T + Send + Sync + 'static>;

pub trait IntoDeleteHandler<O: IvoSchemaStruct, CtxOptions> {
    fn into_handler(self) -> DeleteHandler<O, CtxOptions>;
}

impl<F, Fut, Data, CtxOptions> IntoDeleteHandler<Data, CtxOptions> for F
where
    Data: IvoSchemaStruct,
    F: Fn(SharedData<Data>, SharedData<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    fn into_handler(self) -> DeleteHandler<Data, CtxOptions> {
        Box::new(move |data, o| Box::pin(self(data, o)))
    }
}

pub trait IntoFailureHandler<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> {
    fn into_handler(self) -> FailureHandler<I, O, CtxOptions>;
}

impl<F, Fut, I, O, CtxOptions> IntoFailureHandler<I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    F: Fn(SharedIvoContext<I, O>, SharedData<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    fn into_handler(self) -> FailureHandler<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub trait IntoSuccessHandler<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> {
    fn into_handler(self) -> SuccessHandler<I, O, CtxOptions>;
}

impl<F, Fut, I, O, CtxOptions> IntoSuccessHandler<I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    F: Fn(SharedIvoContext<I, O>, SharedData<CtxOptions>) -> Fut + Send + Sync + 'static,
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
    CtxOptions,
    ErrorTool: IvoErrorTool,
>
{
    fn into_uniform(self) -> UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>;
}

impl<F, Fut, T, I, O, CtxOptions, ErrorTool> IntoFieldValidator<T, I, O, CtxOptions, ErrorTool>
    for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    ErrorTool: IvoErrorTool,
    T: IvoFieldValue,
    F: Fn(T, SharedIvoContext<I, O>, SharedRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ValidatorResponse<T, ErrorTool::FieldMetadata>> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata> {
        Box::new(move |v, ctx, o| {
            Box::pin(self(parse_or_panic::<T>(&v, None), ctx, o).map(|r| r.map(|v| erase_value(v))))
        })
    }
}

pub trait IntoVirtualSanitizer<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> {
    fn into_uniform(self) -> UniformVirtualSanitizer<I, O, CtxOptions>;
}

impl<F, Fut, T, I, O, CtxOptions> IntoVirtualSanitizer<T, I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    T: IvoFieldValue,
    F: Fn(T, SharedIvoContext<I, O>, SharedRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformVirtualSanitizer<I, O, CtxOptions> {
        Box::new(move |v, ctx, o| {
            Box::pin(self(parse_or_panic(&v, None), ctx, o).map(|v| erase_value(v)))
        })
    }
}

pub trait IntoRequiredErrorResolver<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> {
    fn into_resolver(self) -> RequiredResolver<I, O, CtxOptions>;
}

impl<F, Fut, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions>
    IntoRequiredErrorResolver<I, O, CtxOptions> for F
where
    F: Fn(SharedIvoContext<I, O>, SharedRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = String> + Send + 'static,
{
    fn into_resolver(self) -> RequiredResolver<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o).map(|e| (true, e))))
    }
}

pub trait IntoRequiredResolver<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> {
    fn into_resolver(self) -> RequiredResolver<I, O, CtxOptions>;
}

impl<F, Fut, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions>
    IntoRequiredResolver<I, O, CtxOptions> for F
where
    F: Fn(SharedIvoContext<I, O>, SharedRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = RequiredError> + Send + 'static,
{
    fn into_resolver(self) -> RequiredResolver<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub trait IntoResolver<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> {
    fn into_resolver(self) -> Resolver<T, I, O, CtxOptions>;
}

impl<F, Fut, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions>
    IntoResolver<T, I, O, CtxOptions> for F
where
    T: 'static,
    F: Fn(SharedIvoContext<I, O>, SharedRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_resolver(self) -> Resolver<T, I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub trait IntoUniformResolver<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> {
    fn into_uniform(self) -> UniformResolver<I, O, CtxOptions>;
}

impl<F, Fut, T, I, O, CtxOptions> IntoUniformResolver<T, I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    T: IvoFieldValue,
    F: Fn(SharedIvoContext<I, O>, SharedRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformResolver<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o).map(|v| erase_value(v))))
    }
}

pub trait IntoResolverWithMiniContext<T, I: IvoSchemaStruct, CtxOptions> {
    fn into_uniform(self) -> UniformResolverWithMiniContext<I, CtxOptions>;
}

impl<F, Fut, T, I: IvoSchemaStruct, CtxOptions> IntoResolverWithMiniContext<T, I, CtxOptions> for F
where
    T: IvoFieldValue,
    F: Fn(SharedIvoMiniContext<I>, SharedRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformResolverWithMiniContext<I, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o).map(|v| erase_value(v))))
    }
}

pub type UniformValidator<I, O, CtxOptions, FieldMetadata> = Box<
    dyn Fn(
            ErasedValue,
            SharedIvoContext<I, O>,
            SharedRwCtxOptions<CtxOptions>,
        ) -> BoxFuture<'static, ValidatorResponse<ErasedValue, FieldMetadata>>
        + Send
        + Sync
        + 'static,
>;

pub type UniformVirtualSanitizer<I, O, CtxOptions> = Box<
    dyn Fn(
            ErasedValue,
            SharedIvoContext<I, O>,
            SharedRwCtxOptions<CtxOptions>,
        ) -> BoxFuture<'static, ErasedValue>
        + Send
        + Sync
        + 'static,
>;

pub type UniformResolver<I, O, CtxOptions> = Box<
    dyn Fn(
            SharedIvoContext<I, O>,
            SharedRwCtxOptions<CtxOptions>,
        ) -> BoxFuture<'static, ErasedValue>
        + Send
        + Sync
        + 'static,
>;

pub type UniformResolverWithMiniContext<I, CtxOptions> = Box<
    dyn Fn(
            SharedIvoMiniContext<I>,
            SharedRwCtxOptions<CtxOptions>,
        ) -> BoxFuture<'static, ErasedValue>
        + Send
        + Sync
        + 'static,
>;

pub enum ComputableWithMiniContext<T, I: IvoSchemaStruct, CtxOptions> {
    Static(T),
    Func(UniformResolverWithMiniContext<I, CtxOptions>),
}

pub enum ComputableInit<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> {
    False,
    Func(BooleanResolver<I, O, CtxOptions>),
}

pub enum ComputableRequiredError<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> {
    Static(&'static str),
    Func(RequiredResolver<I, O, CtxOptions>),
}

pub type RequiredError = (bool, String);

pub type RequiredResolver<I, O, CtxOptions> = Box<
    dyn Fn(
            SharedIvoContext<I, O>,
            SharedRwCtxOptions<CtxOptions>,
        ) -> BoxFuture<'static, RequiredError>
        + Send
        + Sync
        + 'static,
>;

pub type Resolver<T, I, O, CtxOptions> = Box<
    dyn Fn(SharedIvoContext<I, O>, SharedRwCtxOptions<CtxOptions>) -> BoxFuture<'static, T>
        + Send
        + Sync
        + 'static,
>;

pub type BooleanResolver<I, O, CtxOptions> = Resolver<bool, I, O, CtxOptions>;

pub type VirtualSanitizer<T, I, O, CtxOptions> = Box<
    dyn Fn(T, SharedIvoContext<I, O>, SharedRwCtxOptions<CtxOptions>) -> BoxFuture<'static, T>
        + Send
        + Sync
        + 'static,
>;
