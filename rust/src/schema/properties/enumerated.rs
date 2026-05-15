use std::{future::Future, marker::PhantomData};

use serde_json::Value;

use crate::{
    schema::properties::base::IvoProperty,
    traits::HasPartial,
    types::{
        BooleanResolverWithMutSummary, ComputableEnumeratedError, ComputableInit,
        ComputableWithMiniSummary, DeleteHandler, FailureHandler, IvoMiniSummary, IvoSummary,
        SuccessHandler,
    },
};

pub struct EnumeratedField;

// Marker Types
struct Yes;
struct No;
struct YesComputed;

struct SchemaBuilder<
    T,
    I: HasPartial,
    O: HasPartial,
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
> {
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
    enum_values: Option<Vec<T>>,
    enum_error: Option<ComputableEnumeratedError<T>>,
    default: Option<ComputableWithMiniSummary<T, CtxOptions>>,
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
        I: HasPartial,
        O: HasPartial,
        CtxOptions,
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
        I: HasPartial,
        O: HasPartial,
        CtxOptions,
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
    pub fn build(self) -> IvoProperty<T, I, O, CtxOptions> {
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
    pub fn values<T, I: HasPartial, O: HasPartial, CtxOptions>(
        values: Vec<T>,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No, No, No, No> {
        SchemaBuilder {
            enum_values: Some(values),
            ..Default::default()
        }
    }
}

impl<T, I: HasPartial, O: HasPartial, CtxOptions>
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
        F: Fn((Value, &Vec<T>)) -> &str + Send + Sync + 'static,
    {
        SchemaBuilder {
            enum_values: self.enum_values,
            enum_error: Some(ComputableEnumeratedError::Func(Box::new(error_fn))),
            ..Default::default()
        }
    }
}

impl<T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, No, No, No, No, No, No, No>
{
    pub fn default(
        self,
        value: T,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, No, No, No, No> {
        SchemaBuilder {
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            default: Some(ComputableWithMiniSummary::Static(value)),
            ..Default::default()
        }
    }

    pub fn default_fn<F>(
        self,
        default_fn: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, No, No, No, No>
    where
        F: Fn(&mut IvoMiniSummary<CtxOptions>) -> T + Send + Sync + 'static,
    {
        SchemaBuilder {
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            default: Some(ComputableWithMiniSummary::SyncFunc(Box::new(default_fn))),
            ..Default::default()
        }
    }

    pub fn default_async_fn<F, Fut>(
        self,
        default_fn: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, No, No, No, No>
    where
        F: Fn(&mut IvoMiniSummary<CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = T> + Send + 'static,
    {
        SchemaBuilder {
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            default: Some(ComputableWithMiniSummary::AsyncFunc(Box::new(move |c| {
                Box::pin(default_fn(c))
            }))),
            ..Default::default()
        }
    }
}

impl<T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, No, No, No, No>
{
    pub fn ignore_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, Yes, No, No, No, No, No>
    where
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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

impl<T, I: HasPartial, O: HasPartial, CtxOptions>
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
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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

impl<HasDefault, T, I: HasPartial, O: HasPartial, CtxOptions>
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
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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

impl<T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, Yes, No, No, No>
{
    pub fn allow_init_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, YesComputed, Yes, No, No, No>
    where
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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

impl<HasDefault, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, HasDefault, No, Yes, YesComputed, No, No, No>
{
    pub fn allow_update_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, HasDefault, No, Yes, YesComputed, No, No, No>
    where
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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

impl<T, I: HasPartial, O: HasPartial, CtxOptions>
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
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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
        I: HasPartial,
        O: HasPartial,
        CtxOptions,
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
    pub fn on_delete(
        self,
        handler: DeleteHandler<O, CtxOptions>,
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
    > {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
            on_delete_fns: Some(vec![handler]),
            on_failure_fns: self.on_failure_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }

    pub fn on_delete_fns(
        self,
        handlers: Vec<DeleteHandler<O, CtxOptions>>,
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
    > {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
            on_delete_fns: Some(handlers),
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
        HasSuccess,
        T,
        I: HasPartial,
        O: HasPartial,
        CtxOptions,
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
        No,
        HasSuccess,
    >
{
    pub fn on_failure(
        self,
        handler: FailureHandler<I, O, CtxOptions>,
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
    > {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
            on_delete_fns: self.on_delete_fns,
            on_failure_fns: Some(vec![handler]),
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }

    pub fn on_failure_fns(
        self,
        handlers: Vec<FailureHandler<I, O, CtxOptions>>,
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
    > {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            on_delete_fns: self.on_delete_fns,
            on_failure_fns: Some(handlers),
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
        T,
        I: HasPartial,
        O: HasPartial,
        CtxOptions,
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
        No,
    >
{
    pub fn on_success(
        self,
        handler: SuccessHandler<I, O, CtxOptions>,
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
    > {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
            on_delete_fns: self.on_delete_fns,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: Some(vec![handler]),
            ..Default::default()
        }
    }

    pub fn on_success_fns(
        self,
        handlers: Vec<SuccessHandler<I, O, CtxOptions>>,
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
    > {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
            on_delete_fns: self.on_delete_fns,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: Some(handlers),
            ..Default::default()
        }
    }
}
