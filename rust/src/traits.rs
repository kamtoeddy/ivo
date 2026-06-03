use std::future::Future;

use futures::FutureExt;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

use crate::{
    types::{
        DeleteHandler, FailureHandler, IvoMiniSummary, IvoSummary, RequiredResolverFn,
        ResolverWithMutSummaryFn, SuccessHandler, UniformAsyncReValidator,
        UniformAsyncResolverWithMiniSummary, UniformAsyncResolverWithMutSummary,
        UniformAsyncValidator, UniformEnumErrorResolver, UniformReValidator,
        UniformResolverWithMiniSummary, UniformResolverWithMutSummary, UniformValidator,
        UniformVirtualSanitiser,
    },
    ValidatorResponse,
};

pub trait IvoSchemaStruct:
    Send + Sync + 'static + DeserializeOwned + Serialize + HasFields + HasPartial
{
}

pub trait HasFields {
    fn fields() -> Vec<String>;
}

pub trait HasPartial {
    type Partial: Send + Sync + Clone + Serialize + DeserializeOwned;
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

pub trait IntoFieldValidator<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_uniform(self) -> UniformValidator<I, O, CtxOptions>;
}

impl<F, T, I, O, CtxOptions: Clone> IntoFieldValidator<T, I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    T: DeserializeOwned + Serialize + Send + 'static,
    F: Fn(T, IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<T> + Clone + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformValidator<I, O, CtxOptions> {
        Box::new(move |value, summary| {
            self(
                serde_json::from_value(value).expect("Failed to parse value"),
                summary,
            )
            .map(|v| json!(v))
        })
    }
}

pub trait IntoAsyncFieldValidator<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_uniform(self) -> UniformAsyncValidator<I, O, CtxOptions>;
}

impl<F, Fut, T, I, O, CtxOptions: Clone> IntoAsyncFieldValidator<T, I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    T: DeserializeOwned + Serialize + Send + 'static,
    F: Fn(T, IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ValidatorResponse<T>> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformAsyncValidator<I, O, CtxOptions> {
        Box::new(move |v, s| {
            Box::pin(
                self(serde_json::from_value(v).expect("Failed to parse value"), s).map(
                    |v| match v {
                        Ok(d) => Ok(json!(d)),
                        Err(e) => Err(e),
                    },
                ),
            )
        })
    }
}

pub trait IntoFieldReValidator<
    T: DeserializeOwned + Serialize,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
>
{
    fn into_uniform(self) -> UniformReValidator<I, O, CtxOptions>;
}

impl<F, T, I, O, CtxOptions: Clone> IntoFieldReValidator<T, I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    T: DeserializeOwned + Serialize + Send + 'static,
    F: Fn(T, IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<T> + Clone + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformReValidator<I, O, CtxOptions> {
        Box::new(move |value, summary| {
            self(
                serde_json::from_value(value).expect("Failed to parse value"),
                summary,
            )
            .map(|v| json!(v))
        })
    }
}

pub trait IntoAsyncFieldReValidator<
    T: DeserializeOwned + Serialize,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
>
{
    fn into_uniform(self) -> UniformAsyncReValidator<I, O, CtxOptions>;
}

impl<F, Fut, T, I, O, CtxOptions: Clone> IntoAsyncFieldReValidator<T, I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    T: DeserializeOwned + Serialize + Send + 'static,
    F: Fn(T, IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ValidatorResponse<T>> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformAsyncReValidator<I, O, CtxOptions> {
        Box::new(move |v, s| {
            Box::pin(
                self(serde_json::from_value(v).expect("Failed to parse value"), s).map(
                    |v| match v {
                        Ok(d) => Ok(json!(d)),
                        Err(e) => Err(e),
                    },
                ),
            )
        })
    }
}

pub trait IntoVirtualSanitizer<
    T: Serialize,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
>
{
    fn into_uniform(self) -> UniformVirtualSanitiser<I, O, CtxOptions>;
}

impl<F, Fut, T, I, O, CtxOptions: Clone> IntoVirtualSanitizer<T, I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    T: DeserializeOwned + Serialize + Send + 'static,
    F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformVirtualSanitiser<I, O, CtxOptions> {
        Box::new(move |s| Box::pin(self(s).map(|v| json!(v))))
    }
}

pub trait IntoEnumErrorResolver<T> {
    fn into_uniform(self) -> UniformEnumErrorResolver;
}

impl<F, T> IntoEnumErrorResolver<T> for F
where
    T: DeserializeOwned + Send + 'static,
    F: Fn((T, Vec<T>)) -> String + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformEnumErrorResolver {
        Box::new(move |(v, list)| {
            self((
                serde_json::from_value(v).expect("Failed to deserialize some value"),
                list.iter()
                    .map(|v| {
                        serde_json::from_value(v.clone())
                            .expect("Failed to deserialize some values")
                    })
                    .collect(),
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
    T: Serialize + 'static,
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
    T: Serialize + 'static,
    F: Fn(IvoSummary<I, O, CtxOptions>) -> T + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformResolverWithMutSummary<I, O, CtxOptions> {
        Box::new(move |s| json!(self(s)))
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
    T: Serialize + 'static,
    F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformAsyncResolverWithMutSummary<I, O, CtxOptions> {
        Box::new(move |s| Box::pin(self(s).map(|v| json!(v))))
    }
}

pub trait IntoResolverWithMiniSummary<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
{
    fn into_uniform(self) -> UniformResolverWithMiniSummary<CtxOptions>;
}

impl<F, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    IntoResolverWithMiniSummary<T, I, O, CtxOptions> for F
where
    T: Serialize + 'static,
    F: Fn(IvoMiniSummary<CtxOptions>) -> T + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformResolverWithMiniSummary<CtxOptions> {
        Box::new(move |s| json!(self(s)))
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
    T: Serialize + 'static,
    F: Fn(IvoMiniSummary<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformAsyncResolverWithMiniSummary<CtxOptions> {
        Box::new(move |s| Box::pin(self(s).map(|v| json!(v))))
    }
}
