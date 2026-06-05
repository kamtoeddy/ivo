use std::{collections::HashMap, fmt::Debug, future::Future};

use futures::FutureExt;

use crate::{
    schema::error::IvoErrorTool,
    types::{
        DeleteHandler, FailureHandler, IvoMiniSummary, IvoSummary, RequiredResolverFn,
        ResolverWithMutSummaryFn, SuccessHandler, UniformAsyncResolverWithMiniSummary,
        UniformAsyncResolverWithMutSummary, UniformAsyncValidator, UniformEnumErrorResolver,
        UniformResolverWithMiniSummary, UniformResolverWithMutSummary, UniformValidator,
        UniformVirtualSanitiser, ValidatorError,
    },
    utils::erased_value::{erase_value, parse_or_panic, ErasedValue},
    ValidatorResponse,
};

pub trait IvoSchemaStruct:
    Debug + Eq + Send + Sync + 'static + HasFields + HasPartial + FromMap + ToMap
{
}

pub trait FromMap: Sized {
    fn from_ivo_internal_map(map: &HashMap<String, ErasedValue>) -> Result<Self, String>;
}

pub trait PartialFromMap {
    fn from_ivo_internal_map(map: &HashMap<String, ErasedValue>) -> Self;
}

pub trait ToMap {
    fn to_ivo_internal_map(&self) -> HashMap<String, ErasedValue>;
}

pub trait HasFields {
    fn ivo_internal_fields() -> Vec<String>;
}

pub trait HasPartial {
    type Partial: Debug + Send + Sync + Clone + PartialFromMap + ToMap;
}

pub type Partial<T> = <T as HasPartial>::Partial;

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

impl<F, T, I, O, CtxOptions: Clone, ErrT> IntoFieldValidator<T, I, O, CtxOptions, ErrT> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    ErrT: IvoErrorTool,
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(T, IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<T, ErrT::FieldMetadata>
        + Clone
        + Send
        + Sync
        + 'static,
{
    fn into_uniform(self) -> UniformValidator<I, O, CtxOptions, ErrT::FieldMetadata> {
        Box::new(move |v, s| self(parse_or_panic::<T>(&v), s).map(|v| erase_value(v)))
    }
}

pub trait IntoAsyncFieldValidator<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
>
{
    fn into_uniform(self) -> UniformAsyncValidator<I, O, CtxOptions, ErrT::FieldMetadata>;
}

impl<F, Fut, T, I, O, CtxOptions: Clone, ErrT> IntoAsyncFieldValidator<T, I, O, CtxOptions, ErrT>
    for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    ErrT: IvoErrorTool,
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(T, IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ValidatorResponse<T, ErrT::FieldMetadata>> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformAsyncValidator<I, O, CtxOptions, ErrT::FieldMetadata> {
        Box::new(move |v: ErasedValue, s: IvoSummary<I, O, CtxOptions>| {
            let sv = erase_value(String::from("sv lol"));
            parse_or_panic::<T>(&sv);
            parse_or_panic::<T>(&v); // this fails

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
    fn into_resolver(self) -> RequiredResolverFn<I, O, CtxOptions>;
}

impl<F, Fut, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone + Send + 'static>
    IntoRequiredResolverFn<I, O, CtxOptions> for F
where
    F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = (bool, String)> + Send + 'static,
{
    fn into_resolver(self) -> RequiredResolverFn<I, O, CtxOptions> {
        Box::new(move |s| Box::pin(self(s)))
    }
}

pub trait IntoResolverWithMutSummaryFn<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
{
    fn into_resolver(self) -> ResolverWithMutSummaryFn<T, I, O, CtxOptions>;
}

impl<F, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    IntoResolverWithMutSummaryFn<T, I, O, CtxOptions> for F
where
    T: 'static,
    F: Fn(IvoSummary<I, O, CtxOptions>) -> T + Send + Sync + 'static,
{
    fn into_resolver(self) -> ResolverWithMutSummaryFn<T, I, O, CtxOptions> {
        Box::new(move |s| self(s))
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

impl<F, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    IntoUniformResolverWithMutSummary<T, I, O, CtxOptions> for F
where
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(IvoSummary<I, O, CtxOptions>) -> T + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformResolverWithMutSummary<I, O, CtxOptions> {
        Box::new(move |s| erase_value(self(s)))
    }
}

pub trait IntoAsyncResolverWithMutSummary<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
>
{
    fn into_uniform(self) -> UniformAsyncResolverWithMutSummary<I, O, CtxOptions>;
}

impl<F, Fut, T, I, O, CtxOptions: Clone> IntoAsyncResolverWithMutSummary<T, I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformAsyncResolverWithMutSummary<I, O, CtxOptions> {
        Box::new(move |s| Box::pin(self(s).map(|v| erase_value(v))))
    }
}

pub trait IntoResolverWithMiniSummary<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
{
    fn into_uniform(self) -> UniformResolverWithMiniSummary<CtxOptions>;
}

impl<F, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    IntoResolverWithMiniSummary<T, I, O, CtxOptions> for F
where
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(IvoMiniSummary<CtxOptions>) -> T + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformResolverWithMiniSummary<CtxOptions> {
        Box::new(move |s| erase_value(self(s)))
    }
}

pub trait IntoAsyncResolverWithMiniSummary<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
>
{
    fn into_uniform(self) -> UniformAsyncResolverWithMiniSummary<CtxOptions>;
}

impl<F, Fut, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    IntoAsyncResolverWithMiniSummary<T, I, O, CtxOptions> for F
where
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(IvoMiniSummary<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformAsyncResolverWithMiniSummary<CtxOptions> {
        Box::new(move |s| Box::pin(self(s).map(|v| erase_value(v))))
    }
}
