use std::{future::Future, marker::PhantomData};

use serde::Serialize;
use serde_json::{json, Value};

use crate::{
    schema::properties::base::{InternalIvoProperty, IvoPropertyBuilder},
    traits::{IntoAsyncResolverWithMiniSummary, IntoResolverWithMiniSummary, IvoSchemaStruct},
    types::{ComputableWithMiniSummary, DeleteHandler, IvoSummary, SuccessHandler},
};

// Marker Types
pub struct Yes;
pub struct No;

pub struct SchemaBuilder<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    HasDefault,
    HasDelete,
    HasSuccess,
> {
    _d: PhantomData<T>,
    _i: PhantomData<I>,
    _default: PhantomData<HasDefault>,
    _del_handlers: PhantomData<HasDelete>,
    _success_handlers: PhantomData<HasSuccess>,
    // actual data...
    value: Option<ComputableWithMiniSummary<Value, CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<CtxOptions>>>,
}

impl<
        HasDefault,
        HasDelete,
        HasSuccess,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        T,
        CtxOptions: Clone,
    > Default for SchemaBuilder<T, I, O, CtxOptions, HasDefault, HasDelete, HasSuccess>
{
    fn default() -> Self {
        Self {
            value: None,
            on_delete_fns: None,
            on_success_fns: None,
            _d: PhantomData,
            _i: PhantomData,
            _default: PhantomData,
            _del_handlers: PhantomData,
            _success_handlers: PhantomData,
        }
    }
}

impl<
        HasDelete,
        HasSuccess,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        T: Serialize,
        CtxOptions: Clone,
    > IvoPropertyBuilder<I, O, CtxOptions>
    for SchemaBuilder<T, I, O, CtxOptions, Yes, HasDelete, HasSuccess>
{
    fn build(self) -> InternalIvoProperty<I, O, CtxOptions> {
        InternalIvoProperty {
            value: Some(match self.value {
                Some(v) => match v {
                    ComputableWithMiniSummary::Static(val) => {
                        ComputableWithMiniSummary::Static(json!(val))
                    }
                    _ => v,
                },
                _ => panic!("A constant property must have a value!"),
            }),
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

pub struct ConstantField;

impl ConstantField {
    pub fn value<I: IvoSchemaStruct, O: IvoSchemaStruct, T: Serialize, CtxOptions: Clone + Send>(
        value: T,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No> {
        SchemaBuilder {
            value: Some(ComputableWithMiniSummary::Static(json!((value)))),
            on_delete_fns: None,
            on_success_fns: None,
            ..Default::default()
        }
    }

    pub fn computed<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, F>(
        resolver: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No>
    where
        F: IntoResolverWithMiniSummary<T, I, O, CtxOptions>,
    {
        SchemaBuilder {
            value: Some(ComputableWithMiniSummary::SyncFunc(resolver.into_uniform())),
            on_delete_fns: None,
            on_success_fns: None,
            ..Default::default()
        }
    }

    pub fn computed_async<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, F>(
        resolver: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No>
    where
        F: IntoAsyncResolverWithMiniSummary<T, I, O, CtxOptions>,
    {
        SchemaBuilder {
            value: Some(ComputableWithMiniSummary::AsyncFunc(
                resolver.into_uniform(),
            )),
            on_delete_fns: None,
            on_success_fns: None,
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<HasDelete, HasSuccess, I: IvoSchemaStruct, O: IvoSchemaStruct, T, CtxOptions: Clone>
    SchemaBuilder<T, I, O, CtxOptions, Yes, HasDelete, HasSuccess>
{
    pub fn on_delete<F, Fut>(
        self,
        handler: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, HasSuccess>
    where
        F: Fn(&O, &CtxOptions) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h: DeleteHandler<O, CtxOptions> = Box::new(move |d, o| Box::pin(handler(d, o)));

        SchemaBuilder {
            value: self.value,
            on_delete_fns: Some(match self.on_delete_fns {
                Some(hs) => {
                    let mut v = Vec::from(hs);

                    v.push(h);

                    v
                }
                _ => vec![h],
            }),
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

// ON_SUCCESS is only available if HasSuccess is 'No'
impl<HasDelete, HasSuccess, I: IvoSchemaStruct, O: IvoSchemaStruct, T, CtxOptions: Clone>
    SchemaBuilder<T, I, O, CtxOptions, Yes, HasDelete, HasSuccess>
{
    pub fn on_success<F, Fut>(
        self,
        handler: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, HasDelete, Yes>
    where
        F: Fn(&IvoSummary<CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h: SuccessHandler<CtxOptions> = Box::new(move |s| Box::pin(handler(s)));

        SchemaBuilder {
            value: self.value,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: Some(match self.on_success_fns {
                Some(hs) => {
                    let mut v = Vec::from(hs);

                    v.push(h);

                    v
                }
                _ => vec![h],
            }),
            ..Default::default()
        }
    }
}
