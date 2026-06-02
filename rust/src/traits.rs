use std::future::Future;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

use crate::{
    types::{
        IvoMiniSummary, IvoSummary, UniformAsyncReValidator, UniformAsyncResolverWithMiniSummary,
        UniformAsyncResolverWithMutSummary, UniformAsyncValidator, UniformEnumErrorResolver,
        UniformReValidator, UniformResolverWithMiniSummary, UniformResolverWithMutSummary,
        UniformValidator, UniformVirtualSanitiser,
    },
    ValidatorResponse,
};

pub trait IvoSchemaStruct:
    Send + Sync + 'static + DeserializeOwned + Serialize + HasPartial
{
}

pub trait HasPartial {
    type Partial: Send + Sync + Clone + Serialize + DeserializeOwned;
}

pub type Partial<T> = <T as HasPartial>::Partial;

pub trait IntoFieldValidator<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_uniform(self) -> UniformValidator<I, O, CtxOptions>;
}

impl<F, T, I, O, CtxOptions: Clone> IntoFieldValidator<T, I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    T: Serialize + Send + 'static,
    F: Fn(Value, IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<T>
        + Clone
        + Send
        + Sync
        + 'static,
{
    fn into_uniform(self) -> UniformValidator<I, O, CtxOptions> {
        Box::new(move |value, summary| self(value, summary).map(|v| json!(v)))
    }
}

pub trait IntoAsyncFieldValidator<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_uniform(self) -> UniformAsyncValidator<I, O, CtxOptions>;
}

impl<F, Fut, T, I, O, CtxOptions: Clone + Send + Sync + 'static>
    IntoAsyncFieldValidator<T, I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    T: Serialize + Send + 'static,
    F: Fn(Value, IvoSummary<I, O, CtxOptions>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = ValidatorResponse<T>> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformAsyncValidator<I, O, CtxOptions> {
        Box::new(move |value, summary| {
            let validator = self.clone();

            Box::pin(async move { validator(value, summary).await.map(|v| json!(v)) })
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

impl<F, Fut, T, I, O, CtxOptions: Clone + Send + Sync + 'static>
    IntoAsyncFieldReValidator<T, I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    T: DeserializeOwned + Serialize + Send + 'static,
    F: Fn(T, IvoSummary<I, O, CtxOptions>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = ValidatorResponse<T>> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformAsyncReValidator<I, O, CtxOptions> {
        Box::new(move |value, summary| {
            let validator = self.clone();

            Box::pin(async move {
                validator(
                    serde_json::from_value(value).expect("Failed to parse value"),
                    summary,
                )
                .await
                .map(|v| json!(v))
            })
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

impl<F, Fut, T, I, O, CtxOptions: Clone + Send + Sync + 'static>
    IntoVirtualSanitizer<T, I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    T: DeserializeOwned + Serialize + Send + 'static,
    F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformVirtualSanitiser<I, O, CtxOptions> {
        Box::new(move |summary| {
            let sanitizer = self.clone();

            Box::pin(async move { json!(sanitizer(summary).await) })
        })
    }
}

pub trait IntoEnumErrorResolver<T> {
    fn into_uniform(self) -> UniformEnumErrorResolver;
}

impl<F, T> IntoEnumErrorResolver<T> for F
where
    T: DeserializeOwned + Send + 'static,
    F: Fn((Value, Vec<T>)) -> &'static str + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformEnumErrorResolver {
        Box::new(move |d| {
            self((
                d.0,
                d.1.iter()
                    .map(|v| {
                        serde_json::from_value(v.clone())
                            .expect("Failed to deserialize some values")
                    })
                    .collect(),
            ))
        })
    }
}

pub trait IntoResolverWithMutSummary<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_uniform(self) -> UniformResolverWithMutSummary<I, O, CtxOptions>;
}

impl<F, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    IntoResolverWithMutSummary<T, I, O, CtxOptions> for F
where
    T: Serialize + 'static,
    F: Fn(IvoSummary<I, O, CtxOptions>) -> T + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformResolverWithMutSummary<I, O, CtxOptions> {
        Box::new(move |summary| json!(self(summary)))
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

impl<F, Fut, T, I, O, CtxOptions: Clone + Send + Sync + 'static>
    IntoAsyncResolverWithMutSummary<T, I, O, CtxOptions> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    T: Serialize + 'static,
    F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformAsyncResolverWithMutSummary<I, O, CtxOptions> {
        Box::new(move |summary| {
            let resolver = self.clone();

            Box::pin(async move { json!(resolver(summary).await) })
        })
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
        Box::new(move |summary| json!(self(summary)))
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

impl<
        F,
        Fut,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone + Send + Sync + 'static,
    > IntoAsyncResolverWithMiniSummary<T, I, O, CtxOptions> for F
where
    T: Serialize + 'static,
    F: Fn(IvoMiniSummary<CtxOptions>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformAsyncResolverWithMiniSummary<CtxOptions> {
        Box::new(move |summary| {
            let resolver = self.clone();

            Box::pin(async move { json!(resolver(summary).await) })
        })
    }
}
