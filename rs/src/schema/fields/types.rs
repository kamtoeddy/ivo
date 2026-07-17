#![expect(type_alias_bounds)]

use futures::future::{BoxFuture, FutureExt};

use std::future::{ready, Future};

use crate::{
    __private_types::{types::BooleanResolver, ValidatorResponse},
    schema::types::{DeleteHandler, FailureHandler, FieldValue, SuccessHandler},
    types::internal::types::{erase_value, parse_or_panic, ErasedValue},
    IvoContext, IvoErrorSanitizer, IvoRwCtxOptions, IvoShared, IvoSharedInput, IvoStruct,
};

pub type TimestampResolver<T: FieldValue> = Box<dyn Fn() -> T + Send + Sync + 'static>;

pub trait IntoDeleteHandler<O: IvoStruct, CtxOptions> {
    fn into_handler(self) -> DeleteHandler<O, CtxOptions>;
}

impl<F, Fut, Data, CtxOptions> IntoDeleteHandler<Data, CtxOptions> for F
where
    Data: IvoStruct,
    F: Fn(IvoShared<Data>, IvoShared<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    fn into_handler(self) -> DeleteHandler<Data, CtxOptions> {
        Box::new(move |data, o| Box::pin(self(data, o)))
    }
}

pub trait IntoFailureHandler<I: IvoStruct, O: IvoStruct, CtxOptions> {
    fn into_handler(self) -> FailureHandler<I, O, CtxOptions>;
}

impl<F, Fut, I, O, CtxOptions> IntoFailureHandler<I, O, CtxOptions> for F
where
    I: IvoStruct,
    O: IvoStruct,
    F: Fn(IvoContext<I, O>, IvoShared<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    fn into_handler(self) -> FailureHandler<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub trait IntoSuccessHandler<I: IvoStruct, O: IvoStruct, CtxOptions> {
    fn into_handler(self) -> SuccessHandler<I, O, CtxOptions>;
}

impl<F, Fut, I, O, CtxOptions> IntoSuccessHandler<I, O, CtxOptions> for F
where
    I: IvoStruct,
    O: IvoStruct,
    F: Fn(IvoContext<I, O>, IvoShared<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    fn into_handler(self) -> SuccessHandler<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub trait IntoFieldValidator<
    T,
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer,
>
{
    fn into_uniform(self) -> UniformValidator<I, O, CtxOptions, ErrorSanitizer::Metadata>;
}

impl<F, Fut, T, I, O, CtxOptions, ErrorSanitizer>
    IntoFieldValidator<T, I, O, CtxOptions, ErrorSanitizer> for F
where
    I: IvoStruct,
    O: IvoStruct,
    ErrorSanitizer: IvoErrorSanitizer,
    T: FieldValue,
    F: Fn(T, IvoContext<I, O>, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ValidatorResponse<T, ErrorSanitizer::Metadata>> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformValidator<I, O, CtxOptions, ErrorSanitizer::Metadata> {
        Box::new(move |v, ctx, o| {
            Box::pin(
                self(parse_or_panic::<T>(&v, None), ctx, o)
                    .map(|result| result.map(|option| option.map(|value| erase_value(value)))),
            )
        })
    }
}

pub trait IntoVirtualSanitizer<T, I: IvoStruct, O: IvoStruct, CtxOptions> {
    fn into_uniform(self) -> UniformVirtualSanitizer<I, O, CtxOptions>;
}

impl<F, Fut, T, I, O, CtxOptions> IntoVirtualSanitizer<T, I, O, CtxOptions> for F
where
    I: IvoStruct,
    O: IvoStruct,
    T: FieldValue,
    F: Fn(T, IvoContext<I, O>, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformVirtualSanitizer<I, O, CtxOptions> {
        Box::new(move |v, ctx, o| {
            Box::pin(self(parse_or_panic(&v, None), ctx, o).map(|v| erase_value(v)))
        })
    }
}

pub trait IntoRequiredErrorResolver<I: IvoStruct, O: IvoStruct, CtxOptions> {
    fn into_resolver(self) -> RequiredResolver<I, O, CtxOptions>;
}

impl<F, I: IvoStruct, O: IvoStruct, CtxOptions> IntoRequiredErrorResolver<I, O, CtxOptions> for F
where
    F: Fn(IvoContext<I, O>, IvoRwCtxOptions<CtxOptions>) -> String + Send + Sync + 'static,
{
    fn into_resolver(self) -> RequiredResolver<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(ready(Some(self(ctx, o)))))
    }
}

pub trait IntoRequiredResolver<I: IvoStruct, O: IvoStruct, CtxOptions> {
    fn into_resolver(self) -> RequiredResolver<I, O, CtxOptions>;
}

impl<F, Fut, I: IvoStruct, O: IvoStruct, CtxOptions> IntoRequiredResolver<I, O, CtxOptions> for F
where
    F: Fn(IvoContext<I, O>, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = RequiredError> + Send + 'static,
{
    fn into_resolver(self) -> RequiredResolver<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub trait IntoUniformResolver<T, I: IvoStruct, O: IvoStruct, CtxOptions> {
    fn into_uniform(self) -> UniformResolver<I, O, CtxOptions>;
}

impl<F, Fut, T, I, O, CtxOptions> IntoUniformResolver<T, I, O, CtxOptions> for F
where
    I: IvoStruct,
    O: IvoStruct,
    T: FieldValue,
    F: Fn(IvoContext<I, O>, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformResolver<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o).map(|v| erase_value(v))))
    }
}

pub trait IntoValueResolverWithSharedInput<T, I: IvoStruct, CtxOptions> {
    fn into_uniform(self) -> UniformValueResolverWithSharedInput<I, CtxOptions>;
}

impl<F, Fut, T, I: IvoStruct, CtxOptions> IntoValueResolverWithSharedInput<T, I, CtxOptions> for F
where
    T: FieldValue,
    F: Fn(IvoSharedInput<I>, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformValueResolverWithSharedInput<I, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o).map(|v| erase_value(v))))
    }
}

pub trait IntoBooleanResolver<I: IvoStruct, O: IvoStruct, CtxOptions> {
    fn into_resolver(self) -> BooleanResolver<I, O, CtxOptions>;
}

impl<F, Fut, I: IvoStruct, O: IvoStruct, CtxOptions> IntoBooleanResolver<I, O, CtxOptions> for F
where
    F: Fn(IvoContext<I, O>, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = bool> + Send + 'static,
{
    fn into_resolver(self) -> BooleanResolver<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub type UniformValidator<I, O, CtxOptions, Metadata> = Box<
    dyn Fn(
            ErasedValue,
            IvoContext<I, O>,
            IvoRwCtxOptions<CtxOptions>,
        ) -> BoxFuture<'static, ValidatorResponse<ErasedValue, Metadata>>
        + Send
        + Sync
        + 'static,
>;

pub type UniformVirtualSanitizer<I, O, CtxOptions> = Box<
    dyn Fn(
            ErasedValue,
            IvoContext<I, O>,
            IvoRwCtxOptions<CtxOptions>,
        ) -> BoxFuture<'static, ErasedValue>
        + Send
        + Sync
        + 'static,
>;

pub type UniformResolver<I, O, CtxOptions> = Box<
    dyn Fn(IvoContext<I, O>, IvoRwCtxOptions<CtxOptions>) -> BoxFuture<'static, ErasedValue>
        + Send
        + Sync
        + 'static,
>;

pub type UniformValueResolverWithSharedInput<I, CtxOptions> = Box<
    dyn Fn(IvoSharedInput<I>, IvoRwCtxOptions<CtxOptions>) -> BoxFuture<'static, ErasedValue>
        + Send
        + Sync
        + 'static,
>;

pub enum ValueResolverWithSharedInput<T, I: IvoStruct, CtxOptions> {
    Static(T),
    Func(UniformValueResolverWithSharedInput<I, CtxOptions>),
}

pub enum IsFieldProvisionEnabled<I: IvoStruct, O: IvoStruct, CtxOptions> {
    False,
    Readonly,
    Func(BooleanResolver<I, O, CtxOptions>),
}

pub enum ComputableRequiredError<I: IvoStruct, O: IvoStruct, CtxOptions> {
    Static(&'static str),
    Func(RequiredResolver<I, O, CtxOptions>),
}

pub type RequiredError = Option<String>;

pub type RequiredResolver<I: IvoStruct, O: IvoStruct, CtxOptions> = Box<
    dyn Fn(IvoContext<I, O>, IvoRwCtxOptions<CtxOptions>) -> BoxFuture<'static, RequiredError>
        + Send
        + Sync
        + 'static,
>;

pub type VirtualSanitizer<T, I, O, CtxOptions> = Box<
    dyn Fn(T, IvoContext<I, O>, IvoRwCtxOptions<CtxOptions>) -> BoxFuture<'static, T>
        + Send
        + Sync
        + 'static,
>;
