use std::marker::PhantomData;

use serde::Serialize;
use serde_json::{json, Value};

use crate::{
    fields::base::{BuildableIvoProperty, InternalIvoProperty, IvoProperty},
    traits::{
        IntoAsyncResolverWithMutSummary, IntoDeleteHandler, IntoResolverWithMiniSummary,
        IntoResolverWithMutSummary, IntoSuccessHandler, IvoSchemaStruct,
    },
    types::{
        ComputableInit, ComputableWithMiniSummary, DeleteHandler, False, ResolverWithMutSummary,
        SuccessHandler,
    },
};

// Marker Types
pub struct Yes;
pub struct No;

pub struct DependentFieldBuilder<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    HasDefault = No,
    HasParents = No,
    HasResolver = No,
    HasShouldUpdate = No,
    HasDelete = No,
    HasSuccess = No,
> {
    _t: PhantomData<T>,
    _default: PhantomData<HasDefault>,
    _depends_on: PhantomData<HasParents>,
    _resolver: PhantomData<HasResolver>,
    _del_handlers: PhantomData<HasDelete>,
    _should_update: PhantomData<HasShouldUpdate>,
    _success_handlers: PhantomData<HasSuccess>,
    // actual data...
    default: Option<ComputableWithMiniSummary<Value, CtxOptions>>,
    depends_on: Option<Vec<String>>,
    resolver: Option<ResolverWithMutSummary<Value, I, O, CtxOptions>>,
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    >
    DependentFieldBuilder<
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
    pub const fn new() -> Self {
        Self {
            default: None,
            depends_on: None,
            resolver: None,
            should_update: None,
            on_delete_fns: None,
            on_success_fns: None,
            _t: PhantomData,
            _default: PhantomData,
            _depends_on: PhantomData,
            _should_update: PhantomData,
            _resolver: PhantomData,
            _del_handlers: PhantomData,
            _success_handlers: PhantomData,
        }
    }
}

impl<
        HasDefault,
        HasParents,
        HasResolver,
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    > Default
    for DependentFieldBuilder<
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
        Self::new()
    }
}

impl<
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    > BuildableIvoProperty<I, O, CtxOptions>
    for DependentFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        Yes,
        Yes,
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
    >
{
    fn build(self) -> InternalIvoProperty<I, O, CtxOptions> {
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

impl<T: Serialize, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    DependentFieldBuilder<T, I, O, CtxOptions>
{
    pub fn default(self, value: T) -> DependentFieldBuilder<T, I, O, CtxOptions, Yes> {
        DependentFieldBuilder {
            default: Some(ComputableWithMiniSummary::Static(json!(value))),
            ..Default::default()
        }
    }

    pub fn default_fn<F>(self, default_fn: F) -> DependentFieldBuilder<T, I, O, CtxOptions, Yes>
    where
        F: IntoResolverWithMiniSummary<T, I, O, CtxOptions>,
    {
        DependentFieldBuilder {
            default: Some(ComputableWithMiniSummary::SyncFunc(
                default_fn.into_uniform(),
            )),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    DependentFieldBuilder<T, I, O, CtxOptions, Yes>
{
    pub fn depends_on(
        self,
        fields: Vec<&str>,
    ) -> DependentFieldBuilder<T, I, O, CtxOptions, Yes, Yes> {
        DependentFieldBuilder {
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

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    DependentFieldBuilder<T, I, O, CtxOptions, Yes, Yes>
{
    pub fn resolve<R>(
        self,
        resolver: R,
    ) -> DependentFieldBuilder<T, I, O, CtxOptions, Yes, Yes, Yes>
    where
        R: IntoResolverWithMutSummary<T, I, O, CtxOptions>,
    {
        DependentFieldBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: Some(ResolverWithMutSummary::Sync(resolver.into_uniform())),
            ..Default::default()
        }
    }

    pub fn resolve_async<R>(
        self,
        resolver: R,
    ) -> DependentFieldBuilder<T, I, O, CtxOptions, Yes, Yes, Yes>
    where
        R: IntoAsyncResolverWithMutSummary<T, I, O, CtxOptions>,
    {
        DependentFieldBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: Some(ResolverWithMutSummary::Async(resolver.into_uniform())),
            ..Default::default()
        }
    }
}

impl<HasDelete, HasSuccess, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    DependentFieldBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, HasDelete, HasSuccess>
{
    pub fn readonly(
        self,
    ) -> DependentFieldBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, Yes, HasDelete, HasSuccess> {
        DependentFieldBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            should_update: Some(False),
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    >
    DependentFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        Yes,
        Yes,
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
    >
{
    pub fn on_delete<H>(
        self,
        handler: H,
    ) -> DependentFieldBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, HasShouldUpdate, Yes, HasSuccess>
    where
        H: IntoDeleteHandler<O, CtxOptions>,
    {
        let h = handler.into_handler();

        DependentFieldBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            should_update: self.should_update,
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
impl<
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    >
    DependentFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        Yes,
        Yes,
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
    >
{
    pub fn on_success<H>(
        self,
        handler: H,
    ) -> DependentFieldBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, HasShouldUpdate, HasDelete, Yes>
    where
        H: IntoSuccessHandler<I, O, CtxOptions>,
    {
        let h = handler.into_handler();

        DependentFieldBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            should_update: self.should_update,
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
