use std::{future::Future, marker::PhantomData};

use serde_json::Value;

use crate::{
    schema::properties::base::IvoProperty,
    types::{
        BooleanResolverWithMutSummary, ComputableInit, ComputableRequired, ComputableWithContext,
        Context, DeleteHandler, FailureHandler, FieldValidator, MutableSummary,
        ResolverWithContextFn, SuccessHandler,
    },
    ValidatorResponse,
};

pub struct LaxField;

// Marker Types
struct Yes;
struct No;
struct YesComputed;

struct SchemaBuilder<
    I,
    O,
    T,
    HasDefault,
    HasValidator,
    HasRevalidator,
    HasRequired,
    HasIgnore,
    HasShouldInit,
    HasShouldUpdate,
    HasDelete,
    HasFailure,
    HasSuccess,
> {
    _default: PhantomData<HasDefault>,
    _validator: PhantomData<HasValidator>,
    _re_validator: PhantomData<HasRevalidator>,
    _required_fn: PhantomData<HasRequired>,
    _should_ignore: PhantomData<HasIgnore>,
    _should_init: PhantomData<HasShouldInit>,
    _should_update: PhantomData<HasShouldUpdate>,
    _on_delete_fns: PhantomData<HasDelete>,
    _on_failure_fns: PhantomData<HasFailure>,
    _on_success_fns: PhantomData<HasSuccess>,
    // actual data...
    default: Option<ComputableWithContext<I, O, T>>,
    validator: Option<FieldValidator<I, O, T>>,
    re_validator: Option<FieldValidator<I, O, T>>,
    required: Option<ComputableRequired<I, O>>,
    should_ignore_fn: Option<BooleanResolverWithMutSummary<I, O>>,
    should_init: Option<ComputableInit<I, O>>,
    should_update: Option<ComputableInit<I, O>>,
    on_delete_fns: Option<Vec<DeleteHandler<O>>>,
    on_failure_fns: Option<Vec<FailureHandler<I, O>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O>>>,
}

impl<
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
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
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
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
            validator: None,
            re_validator: None,
            required: None,
            should_ignore_fn: None,
            should_init: None,
            should_update: None,
            on_delete_fns: None,
            on_failure_fns: None,
            on_success_fns: None,
            _default: PhantomData,
            _validator: PhantomData,
            _re_validator: PhantomData,
            _required_fn: PhantomData,
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
        HasRevalidator,
        HasRequired,
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
        HasRevalidator,
        HasDefault,
        HasRequired,
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
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
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

impl LaxField {
    pub fn default<I, O, T>(
        value: T,
    ) -> SchemaBuilder<I, O, T, Yes, No, No, No, No, No, No, No, No, No> {
        SchemaBuilder {
            default: Some(ComputableWithContext::Static(value)),
            ..Default::default()
        }
    }

    pub fn default_fn<I, O, T>(
        default_fn: ResolverWithContextFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, No, No, No, No, No, No, No, No, No> {
        SchemaBuilder {
            default: Some(ComputableWithContext::SyncFunc(default_fn)),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, No, No, No, No, No, No, No, No, No> {
    pub fn validate<F>(
        self,
        validator: F,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &Context<I, O>) -> ValidatorResponse<T> + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            validator: Some(FieldValidator::Sync(Box::new(validator))),
            ..Default::default()
        }
    }

    pub fn validate_async<F, Fut>(
        self,
        validator: F,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &Context<I, O>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ValidatorResponse<T>> + Send + 'static,
    {
        SchemaBuilder {
            default: self.default,
            validator: Some(FieldValidator::Async(Box::new(move |v, ctx| {
                Box::pin(validator(v, ctx))
            }))),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No, No, No, No, No> {
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &Context<I, O>) -> ValidatorResponse<T> + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: Some(FieldValidator::Sync(Box::new(re_validator))),
            ..Default::default()
        }
    }

    pub fn re_validate_async<F, Fut>(
        self,
        re_validator: F,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &Context<I, O>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ValidatorResponse<T>> + Send + 'static,
    {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: Some(FieldValidator::Async(Box::new(move |v, ctx| {
                Box::pin(re_validator(v, ctx))
            }))),
            ..Default::default()
        }
    }
}

impl<HasRevalidator, I, O, T>
    SchemaBuilder<I, O, T, Yes, Yes, HasRevalidator, No, No, No, No, No, No, No>
{
    pub fn required_if<F>(
        self,
        required_fn: F,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, HasRevalidator, Yes, No, No, No, No, No, No>
    where
        F: Fn(&Context<I, O>) -> (bool, &str) + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: Some(ComputableRequired::Func(Box::new(required_fn))),
            ..Default::default()
        }
    }
}

impl<HasValidator, HasRevalidator, HasRequired, I, O, T>
    SchemaBuilder<I, O, T, Yes, HasValidator, HasRevalidator, HasRequired, No, No, No, No, No, No>
{
    pub fn ignore_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<
        I,
        O,
        T,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        Yes,
        No,
        No,
        No,
        No,
        No,
    >
    where
        F: Fn(&MutableSummary<I, O>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            should_ignore_fn: Some(Box::new(fx)),
            ..Default::default()
        }
    }
}

impl<HasValidator, HasRevalidator, HasRequired, I, O, T>
    SchemaBuilder<I, O, T, Yes, HasValidator, HasRevalidator, HasRequired, No, No, No, No, No, No>
{
    pub fn ignore_init(
        self,
    ) -> SchemaBuilder<
        I,
        O,
        T,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        Yes,
        No,
        No,
        No,
        No,
    > {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            should_init: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_init_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<
        I,
        O,
        T,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        YesComputed,
        No,
        No,
        No,
        No,
    >
    where
        F: Fn(&MutableSummary<I, O>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            should_init: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }

    pub fn readonly(
        self,
    ) -> SchemaBuilder<
        I,
        O,
        T,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        No,
        Yes,
        No,
        No,
        No,
    > {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            should_update: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_update_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<
        I,
        O,
        T,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        No,
        YesComputed,
        No,
        No,
        No,
    >
    where
        F: Fn(&MutableSummary<I, O>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            should_update: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }
}

impl<HasValidator, HasRevalidator, HasRequired, I, O, T>
    SchemaBuilder<I, O, T, Yes, HasValidator, HasRevalidator, HasRequired, No, No, Yes, No, No, No>
{
    pub fn allow_init_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<
        I,
        O,
        T,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        YesComputed,
        Yes,
        No,
        No,
        No,
    >
    where
        F: Fn(&MutableSummary<I, O>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            should_init: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }
}

impl<HasValidator, HasRevalidator, HasRequired, I, O, T>
    SchemaBuilder<
        I,
        O,
        T,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        Yes,
        YesComputed,
        No,
        No,
        No,
    >
{
    pub fn allow_update_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<
        I,
        O,
        T,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        Yes,
        YesComputed,
        No,
        No,
        No,
    >
    where
        F: Fn(&MutableSummary<I, O>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            should_init: self.should_init,
            should_update: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }
}

impl<HasValidator, HasRevalidator, HasRequired, I, O, T>
    SchemaBuilder<
        I,
        O,
        T,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        No,
        YesComputed,
        No,
        No,
        No,
    >
{
    pub fn ignore_init(
        self,
    ) -> SchemaBuilder<
        I,
        O,
        T,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        Yes,
        YesComputed,
        No,
        No,
        No,
    > {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,

            should_update: self.should_update,
            should_init: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_init_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<
        I,
        O,
        T,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        YesComputed,
        YesComputed,
        No,
        No,
        No,
    >
    where
        F: Fn(&MutableSummary<I, O>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            should_update: self.should_update,
            should_init: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
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
        HasValidator,
        HasRevalidator,
        HasRequired,
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
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        Yes,
        HasFailure,
        HasSuccess,
    > {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
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
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        Yes,
        HasFailure,
        HasSuccess,
    > {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
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
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
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
        HasRevalidator,
        HasRequired,
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
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        Yes,
        HasSuccess,
    > {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
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
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        Yes,
        HasSuccess,
    > {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
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
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        I,
        O,
        T,
    >
    SchemaBuilder<
        I,
        O,
        T,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
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
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        Yes,
    > {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
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
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        Yes,
    > {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
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
