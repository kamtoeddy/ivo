use std::marker::PhantomData;

use crate::{
    schema::properties::base::IvoProperty,
    types::{ComputableWithContext, DeleteHandler, ResolverWithContextFn, SuccessHandler},
};

// Marker Types
struct Yes;
struct No;

struct SchemaBuilder<I, O, T, HasDefault, HasDelete, HasSuccess> {
    _default: PhantomData<HasDefault>,
    _del_handlers: PhantomData<HasDelete>,
    _success_handlers: PhantomData<HasSuccess>,
    // actual data...
    value: Option<ComputableWithContext<I, O, T>>,
    on_delete_fns: Option<Vec<DeleteHandler<O>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O>>>,
}

impl<HasDefault, HasDelete, HasSuccess, I, O, T> Default
    for SchemaBuilder<I, O, T, HasDefault, HasDelete, HasSuccess>
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

impl<HasDelete, HasSuccess, I, O, T> SchemaBuilder<I, O, T, Yes, HasDelete, HasSuccess> {
    pub fn build(self) -> IvoProperty<I, O, T> {
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
    pub fn value<I, O, T>(value: T) -> SchemaBuilder<I, O, T, Yes, No, No> {
        SchemaBuilder {
            value: Some(ComputableWithContext::Static(value)),
            on_delete_fns: None,
            on_success_fns: None,
            ..Default::default()
        }
    }

    pub fn computed<I, O, T>(
        resolver: ResolverWithContextFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, No, No> {
        SchemaBuilder {
            value: Some(ComputableWithContext::Func(resolver)),
            on_delete_fns: None,
            on_success_fns: None,
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<HasSuccess, I, O, T> SchemaBuilder<I, O, T, Yes, No, HasSuccess> {
    pub fn on_delete(
        self,
        handler: DeleteHandler<O>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, HasSuccess> {
        SchemaBuilder {
            value: self.value,
            on_delete_fns: Some(vec![handler]),
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }

    pub fn on_delete_fns(
        self,
        handlers: Vec<DeleteHandler<O>>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, HasSuccess> {
        SchemaBuilder {
            value: self.value,
            on_delete_fns: Some(handlers),
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

// ON_SUCCESS is only available if HasSuccess is 'No'
impl<HasDelete, I, O, T> SchemaBuilder<I, O, T, Yes, HasDelete, No> {
    pub fn on_success(
        self,
        handler: SuccessHandler<I, O>,
    ) -> SchemaBuilder<I, O, T, Yes, HasDelete, Yes> {
        SchemaBuilder {
            value: self.value,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: Some(vec![handler]),
            ..Default::default()
        }
    }

    pub fn on_success_fns(
        self,
        handlers: Vec<SuccessHandler<I, O>>,
    ) -> SchemaBuilder<I, O, T, Yes, HasDelete, Yes> {
        SchemaBuilder {
            value: self.value,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: Some(handlers),
            ..Default::default()
        }
    }
}
