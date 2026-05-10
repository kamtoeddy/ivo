use std::marker::PhantomData;

use crate::{
    schema::properties::base::IvoProperty,
    types::{
        ComputableInit, ComputableWithContext, DeleteHandler, False, ResolverWithContextFn,
        ResolverWithMutSummaryFn, SuccessHandler,
    },
};

pub struct DependentField;

// Marker Types
struct Yes;
struct No;

struct SchemaBuilder<
    I,
    O,
    T,
    HasDefault,
    HasParents,
    HasResolver,
    HasShouldUpdate,
    HasDelete,
    HasSuccess,
> {
    _default: PhantomData<HasDefault>,
    _parents: PhantomData<HasParents>,
    _resolver: PhantomData<HasResolver>,
    _del_handlers: PhantomData<HasDelete>,
    _should_update: PhantomData<HasShouldUpdate>,
    _success_handlers: PhantomData<HasSuccess>,
    // actual data...
    default: Option<ComputableWithContext<I, O, T>>,
    depends_on: Option<Vec<String>>,
    resolver: Option<ResolverWithMutSummaryFn<I, O, T>>,
    should_update: Option<False>,
    on_delete_fns: Option<Vec<DeleteHandler<O>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O>>>,
}

impl<HasDefault, HasParents, HasResolver, HasShouldUpdate, HasDelete, HasSuccess, I, O, T> Default
    for SchemaBuilder<
        I,
        O,
        T,
        HasDefault,
        HasParents,
        HasResolver,
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
    >
{
    fn default() -> Self {
        Self {
            default: None,
            depends_on: None,
            resolver: None,
            should_update: None,
            on_delete_fns: None,
            on_success_fns: None,
            _default: PhantomData,
            _parents: PhantomData,
            _should_update: PhantomData,
            _resolver: PhantomData,
            _del_handlers: PhantomData,
            _success_handlers: PhantomData,
        }
    }
}

impl<HasShouldUpdate, HasDelete, HasSuccess, I, O, T>
    SchemaBuilder<I, O, T, Yes, Yes, Yes, HasShouldUpdate, HasDelete, HasSuccess>
{
    pub fn build(self) -> IvoProperty<I, O, T> {
        IvoProperty {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            should_update: if self.should_update.is_some() {
                Some(ComputableInit::False)
            } else {
                None
            },
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

impl DependentField {
    pub fn default<I, O, T>(value: T) -> SchemaBuilder<I, O, T, Yes, No, No, No, No, No> {
        SchemaBuilder {
            default: Some(ComputableWithContext::Static(value)),
            ..Default::default()
        }
    }

    pub fn default_fn<I, O, T>(
        default_fn: ResolverWithContextFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, No, No, No, No, No> {
        SchemaBuilder {
            default: Some(ComputableWithContext::Func(default_fn)),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, No, No, No, No, No> {
    pub fn depends_on(self, fields: &[&str]) -> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No> {
        SchemaBuilder {
            default: self.default,
            depends_on: Some(
                fields
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            ),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No> {
    pub fn resolve(
        self,
        resolver: ResolverWithMutSummaryFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, No, No> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: Some(resolver),
            ..Default::default()
        }
    }
}

impl<HasDelete, HasSuccess, I, O, T>
    SchemaBuilder<I, O, T, Yes, Yes, Yes, No, HasDelete, HasSuccess>
{
    pub fn readonly(self) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, Yes, HasDelete, HasSuccess> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            should_update: Some(False),
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<HasShouldUpdate, HasSuccess, I, O, T>
    SchemaBuilder<I, O, T, Yes, Yes, Yes, HasShouldUpdate, No, HasSuccess>
{
    pub fn on_delete(
        self,
        handler: DeleteHandler<O>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, HasShouldUpdate, Yes, HasSuccess> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            should_update: self.should_update,
            on_delete_fns: Some(vec![handler]),
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }

    pub fn on_delete_fns(
        self,
        handlers: Vec<DeleteHandler<O>>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, HasShouldUpdate, Yes, HasSuccess> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            should_update: self.should_update,
            on_delete_fns: Some(handlers),
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

// ON_SUCCESS is only available if HasSuccess is 'No'
impl<HasShouldUpdate, HasDelete, I, O, T>
    SchemaBuilder<I, O, T, Yes, Yes, Yes, HasShouldUpdate, HasDelete, No>
{
    pub fn on_success(
        self,
        handler: SuccessHandler<I, O>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, HasShouldUpdate, HasDelete, Yes> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            should_update: self.should_update,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: Some(vec![handler]),
            ..Default::default()
        }
    }

    pub fn on_success_fns(
        self,
        handlers: Vec<SuccessHandler<I, O>>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, HasShouldUpdate, HasDelete, Yes> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            should_update: self.should_update,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: Some(handlers),
            ..Default::default()
        }
    }
}
