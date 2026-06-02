use std::{future::Future, marker::PhantomData};

use serde::Serialize;
use serde_json::{json, Value};

use crate::{
    fields::base::{BuildableIvoProperty, InternalIvoProperty, IvoProperty},
    traits::{IntoAsyncResolverWithMiniSummary, IntoResolverWithMiniSummary, IvoSchemaStruct},
    types::{ComputableWithMiniSummary, DeleteHandler, IvoSummary, SuccessHandler},
};

// Marker Types
pub struct Yes;
pub struct No;

pub struct ConstantFieldBuilder<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    HasDefault = No,
    HasDelete = No,
    HasSuccess = No,
> {
    _t: PhantomData<T>,
    _default: PhantomData<HasDefault>,
    _del_handlers: PhantomData<HasDelete>,
    _success_handlers: PhantomData<HasSuccess>,
    // actual data...
    value: Option<ComputableWithMiniSummary<Value, CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<
        HasDefault,
        HasDelete,
        HasSuccess,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        T,
        CtxOptions: Clone,
    > ConstantFieldBuilder<T, I, O, CtxOptions, HasDefault, HasDelete, HasSuccess>
{
    pub const fn new() -> Self {
        Self {
            value: None,
            on_delete_fns: None,
            on_success_fns: None,
            _t: PhantomData,
            _default: PhantomData,
            _del_handlers: PhantomData,
            _success_handlers: PhantomData,
        }
    }
}

impl<
        HasDefault,
        HasDelete,
        HasSuccess,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        T,
        CtxOptions: Clone,
    > Default for ConstantFieldBuilder<T, I, O, CtxOptions, HasDefault, HasDelete, HasSuccess>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<
        HasDelete,
        HasSuccess,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        T: Serialize,
        CtxOptions: Clone,
    > BuildableIvoProperty<I, O, CtxOptions>
    for ConstantFieldBuilder<T, I, O, CtxOptions, Yes, HasDelete, HasSuccess>
{
    fn build(self) -> InternalIvoProperty<I, O, CtxOptions> {
        IvoProperty {
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

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, T: Serialize, CtxOptions: Clone + Send>
    ConstantFieldBuilder<T, I, O, CtxOptions>
{
    pub fn value(self, value: T) -> ConstantFieldBuilder<T, I, O, CtxOptions, Yes> {
        ConstantFieldBuilder {
            value: Some(ComputableWithMiniSummary::Static(json!((value)))),
            on_delete_fns: None,
            on_success_fns: None,
            ..Default::default()
        }
    }

    pub fn computed<F>(self, resolver: F) -> ConstantFieldBuilder<T, I, O, CtxOptions, Yes>
    where
        F: IntoResolverWithMiniSummary<T, I, O, CtxOptions>,
    {
        ConstantFieldBuilder {
            value: Some(ComputableWithMiniSummary::SyncFunc(resolver.into_uniform())),
            on_delete_fns: None,
            on_success_fns: None,
            ..Default::default()
        }
    }

    pub fn computed_async<F>(self, resolver: F) -> ConstantFieldBuilder<T, I, O, CtxOptions, Yes>
    where
        F: IntoAsyncResolverWithMiniSummary<T, I, O, CtxOptions>,
    {
        ConstantFieldBuilder {
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
    ConstantFieldBuilder<T, I, O, CtxOptions, Yes, HasDelete, HasSuccess>
{
    pub fn on_delete<F, Fut>(
        self,
        handler: F,
    ) -> ConstantFieldBuilder<T, I, O, CtxOptions, Yes, Yes, HasSuccess>
    where
        F: Fn(&O, &CtxOptions) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h: DeleteHandler<O, CtxOptions> = Box::new(move |d, o| Box::pin(handler(d, o)));

        ConstantFieldBuilder {
            value: self.value,
            on_delete_fns: Some(match self.on_delete_fns {
                Some(hs) => {
                    let mut v = hs;

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
    ConstantFieldBuilder<T, I, O, CtxOptions, Yes, HasDelete, HasSuccess>
{
    pub fn on_success<F, Fut>(
        self,
        handler: F,
    ) -> ConstantFieldBuilder<T, I, O, CtxOptions, Yes, HasDelete, Yes>
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h: SuccessHandler<I, O, CtxOptions> = Box::new(move |s| Box::pin(handler(s)));

        ConstantFieldBuilder {
            value: self.value,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: Some(match self.on_success_fns {
                Some(hs) => {
                    let mut v = hs;

                    v.push(h);

                    v
                }
                _ => vec![h],
            }),
            ..Default::default()
        }
    }
}
