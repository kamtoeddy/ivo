use std::{future::Future, marker::PhantomData};

use crate::{
    schema::properties::base::IvoProperty,
    traits::HasPartial,
    types::{ComputableWithMiniSummary, DeleteHandler, IvoMiniSummary, SuccessHandler},
};

// Marker Types
struct Yes;
struct No;

struct SchemaBuilder<T, I: HasPartial, O: HasPartial, CtxOptions, HasDefault, HasDelete, HasSuccess>
{
    _default: PhantomData<HasDefault>,
    _del_handlers: PhantomData<HasDelete>,
    _success_handlers: PhantomData<HasSuccess>,
    // actual data...
    value: Option<ComputableWithMiniSummary<T, CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<HasDefault, HasDelete, HasSuccess, I: HasPartial, O: HasPartial, T, CtxOptions> Default
    for SchemaBuilder<T, I, O, CtxOptions, HasDefault, HasDelete, HasSuccess>
{
    fn default() -> Self {
        Self {
            value: None,
            on_delete_fns: None,
            on_success_fns: None,
            _default: PhantomData,
            _del_handlers: PhantomData,
            _success_handlers: PhantomData,
        }
    }
}

impl<HasDelete, HasSuccess, I: HasPartial, O: HasPartial, T, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, HasDelete, HasSuccess>
{
    pub fn build(self) -> IvoProperty<T, I, O, CtxOptions> {
        IvoProperty {
            value: self.value,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

pub struct ConstantField;

impl ConstantField {
    pub fn value<I: HasPartial, O: HasPartial, T, CtxOptions>(
        value: T,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No> {
        SchemaBuilder {
            value: Some(ComputableWithMiniSummary::Static(value)),
            on_delete_fns: None,
            on_success_fns: None,
            ..Default::default()
        }
    }

    pub fn computed<T, I: HasPartial, O: HasPartial, CtxOptions, F>(
        resolver: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No>
    where
        F: Fn(&mut IvoMiniSummary<CtxOptions>) -> T + Send + Sync + 'static,
    {
        SchemaBuilder {
            value: Some(ComputableWithMiniSummary::SyncFunc(Box::new(resolver))),
            on_delete_fns: None,
            on_success_fns: None,
            ..Default::default()
        }
    }

    pub fn computed_async<T, I: HasPartial, O: HasPartial, CtxOptions, F, Fut>(
        resolver: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No>
    where
        F: Fn(&mut IvoMiniSummary<CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = T> + Send + 'static,
    {
        SchemaBuilder {
            value: Some(ComputableWithMiniSummary::AsyncFunc(Box::new(move |c| {
                Box::pin(resolver(c))
            }))),
            on_delete_fns: None,
            on_success_fns: None,
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<HasSuccess, I: HasPartial, O: HasPartial, T, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, No, HasSuccess>
{
    pub fn on_delete(
        self,
        handler: DeleteHandler<O, CtxOptions>,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, HasSuccess> {
        SchemaBuilder {
            value: self.value,
            on_delete_fns: Some(vec![handler]),
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }

    pub fn on_delete_fns(
        self,
        handlers: Vec<DeleteHandler<O, CtxOptions>>,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, HasSuccess> {
        SchemaBuilder {
            value: self.value,
            on_delete_fns: Some(handlers),
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

// ON_SUCCESS is only available if HasSuccess is 'No'
impl<HasDelete, I: HasPartial, O: HasPartial, T, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, HasDelete, No>
{
    pub fn on_success(
        self,
        handler: SuccessHandler<I, O, CtxOptions>,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, HasDelete, Yes> {
        SchemaBuilder {
            value: self.value,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: Some(vec![handler]),
            ..Default::default()
        }
    }

    pub fn on_success_fns(
        self,
        handlers: Vec<SuccessHandler<I, O, CtxOptions>>,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, HasDelete, Yes> {
        SchemaBuilder {
            value: self.value,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: Some(handlers),
            ..Default::default()
        }
    }
}
