use crate::{
    schema::properties::base::IvoProperty,
    types::{
        BooleanResolverWithMutSummary, ComputableEnumeratedError, ComputableInit,
        ComputableWithContext, DeleteHandler, EnumeratedErrorResolver, FailureHandler,
        ResolverWithContextFn, SuccessHandler,
    },
};

pub struct EnumeratedField;

// Marker Types
pub struct Yes;
pub struct No;
pub struct YesComputed;

struct SchemaBuilder<
    I,
    O,
    T,
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
    _enum_values: std::marker::PhantomData<HasValues>,
    _enum_error: std::marker::PhantomData<HasValueError>,
    _default: std::marker::PhantomData<HasDefault>,
    _should_ignore: std::marker::PhantomData<HasIgnore>,
    _should_init: std::marker::PhantomData<HasShouldInit>,
    _should_update: std::marker::PhantomData<HasShouldUpdate>,
    _on_delete_fns: std::marker::PhantomData<HasDelete>,
    _on_failure_fns: std::marker::PhantomData<HasFailure>,
    _on_success_fns: std::marker::PhantomData<HasSuccess>,
    // actual data...
    enum_values: Option<Vec<T>>,
    enum_error: Option<ComputableEnumeratedError<T>>,
    default: Option<ComputableWithContext<I, O, T>>,
    should_ignore_fn: Option<BooleanResolverWithMutSummary<I, O>>,
    should_init: Option<ComputableInit<I, O>>,
    should_update: Option<ComputableInit<I, O>>,
    on_delete_fns: Option<Vec<DeleteHandler<O>>>,
    on_failure_fns: Option<Vec<FailureHandler<I, O>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O>>>,
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
        I,
        O,
        T,
    > Default
    for SchemaBuilder<
        I,
        O,
        T,
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
            _default: std::marker::PhantomData,
            _enum_values: std::marker::PhantomData,
            _enum_error: std::marker::PhantomData,
            _should_ignore: std::marker::PhantomData,
            _should_init: std::marker::PhantomData,
            _should_update: std::marker::PhantomData,
            _on_delete_fns: std::marker::PhantomData,
            _on_failure_fns: std::marker::PhantomData,
            _on_success_fns: std::marker::PhantomData,
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
        I,
        O,
        T,
    >
    SchemaBuilder<
        I,
        O,
        T,
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
    pub fn build(self) -> IvoProperty<I, O, T> {
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
    pub fn values<I, O, T>(
        values: Vec<T>,
    ) -> SchemaBuilder<I, O, T, Yes, No, No, No, No, No, No, No, No> {
        SchemaBuilder {
            enum_values: Some(values),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, No, No, No, No, No, No, No, No> {
    pub fn error(
        self,
        error: &str,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No, No, No, No> {
        SchemaBuilder {
            enum_values: self.enum_values,
            enum_error: Some(ComputableEnumeratedError::Static(error.into())),
            ..Default::default()
        }
    }

    pub fn error_fn(
        self,
        error_fn: EnumeratedErrorResolver<T>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No, No, No, No> {
        SchemaBuilder {
            enum_values: self.enum_values,
            enum_error: Some(ComputableEnumeratedError::Func(error_fn)),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No, No, No, No> {
    pub fn default(
        self,
        value: T,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, No, No, No, No, No> {
        SchemaBuilder {
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            default: Some(ComputableWithContext::Static(value)),
            ..Default::default()
        }
    }

    pub fn default_fn(
        self,
        default_fn: ResolverWithContextFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, No, No, No, No, No> {
        SchemaBuilder {
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            default: Some(ComputableWithContext::Func(default_fn)),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, No, No, No, No, No> {
    pub fn ignore_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, Yes, No, No, No, No, No> {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_ignore_fn: Some(fx),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, No, No, No, No, No> {
    pub fn ignore_init(self) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, Yes, No, No, No, No> {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_init: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_init_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, YesComputed, No, No, No, No> {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_init: Some(ComputableInit::Func(fx)),
            ..Default::default()
        }
    }
}

impl<HasDefault, I, O, T> SchemaBuilder<I, O, T, Yes, Yes, HasDefault, No, No, No, No, No, No> {
    pub fn ignore_update(
        self,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, HasDefault, No, No, Yes, No, No, No> {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_update: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_update_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, HasDefault, No, No, YesComputed, No, No, No> {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_update: Some(ComputableInit::Func(fx)),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, No, Yes, No, No, No> {
    pub fn allow_init_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, YesComputed, Yes, No, No, No> {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_init: Some(ComputableInit::Func(fx)),
            ..Default::default()
        }
    }
}

impl<HasDefault, I, O, T>
    SchemaBuilder<I, O, T, Yes, Yes, HasDefault, No, Yes, YesComputed, No, No, No>
{
    pub fn allow_update_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, HasDefault, No, Yes, YesComputed, No, No, No> {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_init: self.should_init,
            should_update: Some(ComputableInit::Func(fx)),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, No, YesComputed, No, No, No> {
    pub fn ignore_init(
        self,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, Yes, YesComputed, No, No, No> {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_update: self.should_update,
            should_init: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_init_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, YesComputed, YesComputed, No, No, No> {
        SchemaBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_update: self.should_update,
            should_init: Some(ComputableInit::Func(fx)),
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<HasDefault, HasIgnore, HasShouldInit, HasShouldUpdate, HasFailure, HasSuccess, I, O, T>
    SchemaBuilder<
        I,
        O,
        T,
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
        handler: DeleteHandler<O>,
    ) -> SchemaBuilder<
        I,
        O,
        T,
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
        handlers: Vec<DeleteHandler<O>>,
    ) -> SchemaBuilder<
        I,
        O,
        T,
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
impl<HasDefault, HasIgnore, HasShouldInit, HasShouldUpdate, HasDelete, HasSuccess, I, O, T>
    SchemaBuilder<
        I,
        O,
        T,
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
        handler: FailureHandler<I, O>,
    ) -> SchemaBuilder<
        I,
        O,
        T,
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
        handlers: Vec<FailureHandler<I, O>>,
    ) -> SchemaBuilder<
        I,
        O,
        T,
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
impl<HasDefault, HasIgnore, HasShouldInit, HasShouldUpdate, HasDelete, HasFailure, I, O, T>
    SchemaBuilder<
        I,
        O,
        T,
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
        handler: SuccessHandler<I, O>,
    ) -> SchemaBuilder<
        I,
        O,
        T,
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
        handlers: Vec<SuccessHandler<I, O>>,
    ) -> SchemaBuilder<
        I,
        O,
        T,
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
