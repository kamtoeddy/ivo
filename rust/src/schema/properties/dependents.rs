use std::{future::Future, marker::PhantomData};

use crate::{
    schema::properties::base::IvoProperty,
    traits::HasPartial,
    types::{
        ComputableInit, ComputableWithMiniSummary, DeleteHandler, False, IvoMiniSummary,
        IvoSummary, ResolverWithMutSummary, SuccessHandler,
    },
};

pub struct DependentField;

// Marker Types
struct Yes;
struct No;

struct SchemaBuilder<
    T,
    I: HasPartial,
    O: HasPartial,
    CtxOptions,
    HasDefault,
    HasParents,
    HasResolver,
    HasShouldUpdate,
    HasDelete,
    HasSuccess,
> {
    _default: PhantomData<HasDefault>,
    _depends_on: PhantomData<HasParents>,
    _resolver: PhantomData<HasResolver>,
    _del_handlers: PhantomData<HasDelete>,
    _should_update: PhantomData<HasShouldUpdate>,
    _success_handlers: PhantomData<HasSuccess>,
    // actual data...
    default: Option<ComputableWithMiniSummary<T, CtxOptions>>,
    depends_on: Option<Vec<String>>,
    resolver: Option<ResolverWithMutSummary<T, I, O, CtxOptions>>,
    should_update: Option<False>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<
        HasDefault,
        HasParents,
        HasResolver,
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
        T,
        I: HasPartial,
        O: HasPartial,
        CtxOptions,
    > Default
    for SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
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
            _depends_on: PhantomData,
            _should_update: PhantomData,
            _resolver: PhantomData,
            _del_handlers: PhantomData,
            _success_handlers: PhantomData,
        }
    }
}

impl<HasShouldUpdate, HasDelete, HasSuccess, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, HasShouldUpdate, HasDelete, HasSuccess>
{
    pub fn build(self) -> IvoProperty<T, I, O, CtxOptions> {
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
    pub fn default<T, I: HasPartial, O: HasPartial, CtxOptions>(
        value: T,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No> {
        SchemaBuilder {
            default: Some(ComputableWithMiniSummary::Static(value)),
            ..Default::default()
        }
    }

    pub fn default_fn<T, I: HasPartial, O: HasPartial, CtxOptions, F>(
        default_fn: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No>
    where
        F: Fn(&mut IvoMiniSummary<CtxOptions>) -> T + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: Some(ComputableWithMiniSummary::SyncFunc(Box::new(default_fn))),
            ..Default::default()
        }
    }
}

impl<T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No>
{
    pub fn depends_on(
        self,
        fields: &[&str],
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, No, No, No, No> {
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

impl<T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, No, No, No, No>
{
    pub fn resolve<R>(
        self,
        resolver: R,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, No>
    where
        R: Fn(&mut IvoSummary<I, O, CtxOptions>) -> T + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: Some(ResolverWithMutSummary::Sync(Box::new(resolver))),
            ..Default::default()
        }
    }

    pub fn resolve_async<R, Fut>(
        self,
        resolver: R,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, No>
    where
        R: Fn(&mut IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = T> + Send + 'static,
    {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: Some(ResolverWithMutSummary::Async(Box::new(move |s| {
                Box::pin(resolver(s))
            }))),
            ..Default::default()
        }
    }
}

impl<HasDelete, HasSuccess, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, HasDelete, HasSuccess>
{
    pub fn readonly(
        self,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, Yes, HasDelete, HasSuccess> {
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
impl<HasShouldUpdate, HasSuccess, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, HasShouldUpdate, No, HasSuccess>
{
    pub fn on_delete(
        self,
        handler: DeleteHandler<O, CtxOptions>,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, HasShouldUpdate, Yes, HasSuccess> {
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
        handlers: Vec<DeleteHandler<O, CtxOptions>>,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, HasShouldUpdate, Yes, HasSuccess> {
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
impl<HasShouldUpdate, HasDelete, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, HasShouldUpdate, HasDelete, No>
{
    pub fn on_success(
        self,
        handler: SuccessHandler<I, O, CtxOptions>,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, HasShouldUpdate, HasDelete, Yes> {
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
        handlers: Vec<SuccessHandler<I, O, CtxOptions>>,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, HasShouldUpdate, HasDelete, Yes> {
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
