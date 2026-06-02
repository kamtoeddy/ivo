use std::{future::Future, marker::PhantomData};

use serde::Serialize;
use serde_json::{json, Value};

use crate::{
    schema::properties::base::{InternalIvoProperty, IvoProperty, BuildableIvoProperty},
    traits::{
        IntoAsyncResolverWithMiniSummary, IntoEnumErrorResolver, IntoResolverWithMiniSummary,
        IvoSchemaStruct,
    },
    types::{
        BooleanResolverWithMutSummary, ComputableEnumeratedError, ComputableInit,
        ComputableWithMiniSummary, DeleteHandler, FailureHandler, IvoSummary, SuccessHandler,
    },
};

pub struct EnumeratedField;

// Marker Types
pub struct Yes;
pub struct No;
pub struct YesComputed;

pub struct SchemaBuilder<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    HasValues,
    HasValueError,
    HasDefault,
    HasIgnore,
    HasShouldInit,
    HasShouldUpdate,
    HasDelete,
    HasFailure,
    HasSuccess,
> {
    _t: PhantomData<T>,
    _enum_values: PhantomData<HasValues>,
    _enum_error: PhantomData<HasValueError>,
    _default: PhantomData<HasDefault>,
    _should_ignore: PhantomData<HasIgnore>,
    _should_init: PhantomData<HasShouldInit>,
    _should_update: PhantomData<HasShouldUpdate>,
    _on_delete_fns: PhantomData<HasDelete>,
    _on_failure_fns: PhantomData<HasFailure>,
    _on_success_fns: PhantomData<HasSuccess>,
    // actual data...
    enum_values: Option<Vec<Value>>,
    enum_error: Option<ComputableEnumeratedError>,
    default: Option<ComputableWithMiniSummary<Value, CtxOptions>>,
    should_ignore_fn: Option<BooleanResolverWithMutSummary<I, O, CtxOptions>>,
    should_init: Option<ComputableInit<I, O, CtxOptions>>,
    should_update: Option<ComputableInit<I, O, CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_failure_fns: Option<Vec<FailureHandler<I, O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<
        HasValues,
        HasValueError,
        HasDefault,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    > Default
    for SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        HasValues,
        HasValueError,
        HasDefault,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    fn default() -> Self {
        Self {
            default: None,
            enum_values: None,
            enum_error: None,
            should_ignore_fn: None,
            should_init: None,
            should_update: None,
            on_delete_fns: None,
            on_failure_fns: None,
            on_success_fns: None,
            _t: PhantomData,
            _default: PhantomData,
            _enum_values: PhantomData,
            _enum_error: PhantomData,
            _should_ignore: PhantomData,
            _should_init: PhantomData,
            _should_update: PhantomData,
            _on_delete_fns: PhantomData,
            _on_failure_fns: PhantomData,
            _on_success_fns: PhantomData,
        }
    }
}

impl<
        HasDefault,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    > BuildableIvoProperty<I, O, CtxOptions>
    for SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        Yes,
        HasDefault,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    fn build(self) -> InternalIvoProperty<I, O, CtxOptions> {
        IvoProperty {
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            default: self.default,
            should_ignore: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
            on_delete_fns: self.on_delete_fns,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

impl EnumeratedField {
    pub fn values<T: Serialize, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>(
        values: Vec<T>,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No, No, No, No> {
        SchemaBuilder {
            enum_values: Some(values.into_iter().map(|v| json!(v)).collect()),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No, No, No, No>
{
    pub fn error(
        self,
        error: &str,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, No, No, No, No, No, No, No> {
        SchemaBuilder {
            enum_values: self.enum_values,
            enum_error: Some(ComputableEnumeratedError::Static(error.into())),
            ..Default::default()
        }
    }

    pub fn error_fn<F>(
        self,
        error_fn: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, No, No, No, No, No, No, No>
    where
        F: IntoEnumErrorResolver<T>,
    {
        SchemaBuilder {
            enum_values: self.enum_values,
            enum_error: Some(ComputableEnumeratedError::Func(error_fn.into_uniform())),
            ..Default::default()
        }
    }
}

impl<T: Serialize, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, No, No, No, No, No, No, No>
{
    pub fn default(
        self,
        value: T,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, No, No, No, No> {
        SchemaBuilder {
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            default: Some(ComputableWithMiniSummary::Static(json!(value))),
            ..Default::default()
        }
    }

    pub fn default_fn<F>(
        self,
        default_fn: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, No, No, No, No>
    where
        F: IntoResolverWithMiniSummary<T, I, O, CtxOptions>,
    {
        SchemaBuilder {
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            default: Some(ComputableWithMiniSummary::SyncFunc(
                default_fn.into_uniform(),
            )),
            ..Default::default()
        }
    }

    pub fn default_async_fn<F>(
        self,
        default_fn: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, No, No, No, No>
    where
        F: IntoAsyncResolverWithMiniSummary<I, I, O, CtxOptions>,
    {
        SchemaBuilder {
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            default: Some(ComputableWithMiniSummary::AsyncFunc(
                default_fn.into_uniform(),
            )),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, No, No, No, No>
{
    pub fn ignore_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, Yes, No, No, No, No, No>
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_ignore_fn: Some(Box::new(fx)),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, No, No, No, No>
{
    pub fn ignore_init(
        self,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, Yes, No, No, No, No> {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_init: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_init_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, YesComputed, No, No, No, No>
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_init: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }
}

impl<HasDefault, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, HasDefault, No, No, No, No, No, No>
{
    pub fn readonly(
        self,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, HasDefault, No, No, Yes, No, No, No> {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_update: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_update_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, HasDefault, No, No, YesComputed, No, No, No>
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_update: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, Yes, No, No, No>
{
    pub fn allow_init_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, YesComputed, Yes, No, No, No>
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_init: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }
}

impl<HasDefault, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, HasDefault, No, Yes, YesComputed, No, No, No>
{
    pub fn allow_update_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, HasDefault, No, Yes, YesComputed, No, No, No>
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_init: self.should_init,
            should_update: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, YesComputed, No, No, No>
{
    pub fn ignore_init(
        self,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, Yes, YesComputed, No, No, No> {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_update: self.should_update,
            should_init: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_init_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, YesComputed, YesComputed, No, No, No>
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_update: self.should_update,
            should_init: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<
        HasDefault,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    >
    SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        Yes,
        HasDefault,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        No,
        HasFailure,
        HasSuccess,
    >
{
    pub fn on_delete<F, Fut>(
        self,
        handler: F,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        Yes,
        HasDefault,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        Yes,
        HasFailure,
        HasSuccess,
    >
    where
        F: Fn(&O, &CtxOptions) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h: DeleteHandler<O, CtxOptions> = Box::new(move |d, o| Box::pin(handler(d, o)));

        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
            on_delete_fns: Some(match self.on_delete_fns {
                Some(hs) => {
                    let mut v = Vec::from(hs);

                    v.push(h);

                    v
                }
                _ => vec![h],
            }),
            on_failure_fns: self.on_failure_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

// ON_FAILURE is only available if HasFailure is 'No'
impl<
        HasDefault,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    >
    SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        Yes,
        HasDefault,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    pub fn on_failure<F, Fut>(
        self,
        handler: F,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        Yes,
        HasDefault,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        Yes,
        HasSuccess,
    >
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h: FailureHandler<I, O, CtxOptions> = Box::new(move |s| Box::pin(handler(s)));

        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
            on_delete_fns: self.on_delete_fns,
            on_failure_fns: Some(match self.on_failure_fns {
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
impl<
        HasDefault,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    >
    SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        Yes,
        HasDefault,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    pub fn on_success<F, Fut>(
        self,
        handler: F,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        Yes,
        HasDefault,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        Yes,
    >
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h: SuccessHandler<I, O, CtxOptions> = Box::new(move |s| Box::pin(handler(s)));

        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
            on_delete_fns: self.on_delete_fns,
            on_failure_fns: self.on_failure_fns,
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
