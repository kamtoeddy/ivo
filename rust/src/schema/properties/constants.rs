use crate::{
    schema::properties::base::IvoProperty,
    types::{Computable, DeleteHandler, ResolverWithContextFn, SuccessHandler},
};

// Marker Types
pub struct Yes;
pub struct No;

struct SchemaBuilder<I, O, T, HasDefault, HasDelete, HasSuccess> {
    _default: std::marker::PhantomData<HasDefault>,
    _del_handlers: std::marker::PhantomData<HasDelete>,
    _success_handlers: std::marker::PhantomData<HasSuccess>,
    // actual data...
    value: Computable<I, O, T>,
    on_delete_fns: Option<Vec<DeleteHandler<O>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O>>>,
}

impl<HasDelete, HasSuccess, I, O, T> SchemaBuilder<I, O, T, Yes, HasDelete, HasSuccess> {
    pub fn build(self) -> IvoProperty<I, O, T> {
        IvoProperty {
            value: Some(self.value),
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
            value: Computable::Static(value),
            on_delete_fns: None,
            on_success_fns: None,
            _default: std::marker::PhantomData,
            _del_handlers: std::marker::PhantomData,
            _success_handlers: std::marker::PhantomData,
        }
    }

    pub fn computed<I, O, T>(
        resolver: ResolverWithContextFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, No, No> {
        SchemaBuilder {
            value: Computable::Func(resolver),
            on_delete_fns: None,
            on_success_fns: None,
            _default: std::marker::PhantomData,
            _del_handlers: std::marker::PhantomData,
            _success_handlers: std::marker::PhantomData,
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
            _default: std::marker::PhantomData,
            _del_handlers: std::marker::PhantomData,
            _success_handlers: std::marker::PhantomData,
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
            _default: std::marker::PhantomData,
            _del_handlers: std::marker::PhantomData,
            _success_handlers: std::marker::PhantomData,
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
            _default: std::marker::PhantomData,
            _del_handlers: std::marker::PhantomData,
            _success_handlers: std::marker::PhantomData,
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
            _default: std::marker::PhantomData,
            _del_handlers: std::marker::PhantomData,
            _success_handlers: std::marker::PhantomData,
        }
    }
}
