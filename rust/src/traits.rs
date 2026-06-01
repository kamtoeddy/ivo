use std::future::Future;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

use crate::{
    types::{
        IvoMiniSummary, IvoSummary, UniformAsyncResolverWithMiniSummary,
        UniformAsyncResolverWithMutSummary, UniformAsyncValidator, UniformEnumErrorResolver,
        UniformResolverWithMiniSummary, UniformResolverWithMutSummary, UniformValidator,
    },
    ValidatorResponse,
};

pub trait IvoSchemaStruct: DeserializeOwned + Serialize + HasPartial {}

pub trait HasPartial {
    type Partial: Clone + Serialize + DeserializeOwned;
}

pub type Partial<T> = <T as HasPartial>::Partial;

pub trait IntoFieldValidator<CtxOptions: Clone> {
    fn into_uniform(self) -> UniformValidator<CtxOptions>;
}

impl<F, T, CtxOptions: Clone> IntoFieldValidator<CtxOptions> for F
where
    T: Serialize + Send + 'static,
    F: Fn(Value, &IvoSummary<CtxOptions>) -> ValidatorResponse<T> + Clone + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformValidator<CtxOptions> {
        Box::new(move |value, summary| self(value, summary).map(|v| json!(v)))
    }
}

pub trait IntoAsyncFieldValidator<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn into_uniform(self) -> UniformAsyncValidator<CtxOptions>;
}

impl<
        F,
        Fut,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone + Send + Sync + 'static,
    > IntoAsyncFieldValidator<I, O, CtxOptions> for F
where
    T: Serialize + Send + 'static,
    F: Fn(Value, &IvoSummary<CtxOptions>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = ValidatorResponse<T>> + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformAsyncValidator<CtxOptions> {
        Box::new(move |value, summary| {
            let validator = self.clone();
            let value = value.clone();
            let summary = summary.clone();

            Box::pin(async move { validator(value, &summary).await.map(|v| json!(v)) })
        })
    }
}

pub trait IntoEnumErrorResolver<T> {
    fn into_uniform(self) -> UniformEnumErrorResolver;
}

impl<F, T> IntoEnumErrorResolver<T> for F
where
    T: DeserializeOwned + Send + 'static,
    F: Fn((Value, &Vec<T>)) -> &'static str + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformEnumErrorResolver {
        Box::new(move |d: (Value, &Vec<Value>)| {
            self((
                d.0,
                &d.1.iter()
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
    fn into_uniform(self) -> UniformResolverWithMutSummary<CtxOptions>;
}

impl<F, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    IntoResolverWithMutSummary<T, I, O, CtxOptions> for F
where
    T: Serialize + 'static,
    F: Fn(&IvoSummary<CtxOptions>) -> T + Send + Sync + 'static,
{
    fn into_uniform(self) -> UniformResolverWithMutSummary<CtxOptions> {
        Box::new(move |summary| json!(self(summary)))
    }
}

pub trait IntoAsyncResolverWithMutSummary<T, CtxOptions: Clone> {
    fn into_uniform(self) -> UniformAsyncResolverWithMutSummary<CtxOptions>;
}

impl<F, Fut, T, CtxOptions: Clone + Send + Sync + 'static>
    IntoAsyncResolverWithMutSummary<T, CtxOptions> for F
where
    T: Serialize + 'static,
    F: Fn(&IvoSummary<CtxOptions>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformAsyncResolverWithMutSummary<CtxOptions> {
        Box::new(move |summary| {
            let resolver = self.clone();
            let summary = summary.clone();

            Box::pin(async move { json!(resolver(&summary).await) })
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
    F: Fn(&IvoMiniSummary<CtxOptions>) -> T + Send + Sync + 'static,
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
    F: Fn(&IvoMiniSummary<CtxOptions>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    fn into_uniform(self) -> UniformAsyncResolverWithMiniSummary<CtxOptions> {
        Box::new(move |summary| {
            let resolver = self.clone();
            let summary = summary.clone();

            Box::pin(async move { json!(resolver(&summary).await) })
        })
    }
}
