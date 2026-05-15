use std::{future::Future, marker::PhantomData};

use serde_json::Value;

use crate::{
    schema::properties::base::IvoProperty,
    traits::HasPartial,
    types::{
        BooleanResolverWithMutSummary, ComputableInit, ComputableRequired,
        ComputableWithMiniSummary, DeleteHandler, FailureHandler, FieldValidator, IvoSummary,
        ResolverWithMiniSummaryFn, SuccessHandler,
    },
    ValidatorResponse,
};

pub struct LaxField;

// Marker Types
struct Yes;
struct No;
struct YesComputed;

struct SchemaBuilder<
    T,
    I: HasPartial,
    O: HasPartial,
    CtxOptions,
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
    default: Option<ComputableWithMiniSummary<T, CtxOptions>>,
    validator: Option<FieldValidator<T, I, O, CtxOptions>>,
    re_validator: Option<FieldValidator<T, I, O, CtxOptions>>,
    required: Option<ComputableRequired<I, O, CtxOptions>>,
    should_ignore_fn: Option<BooleanResolverWithMutSummary<I, O, CtxOptions>>,
    should_init: Option<ComputableInit<I, O, CtxOptions>>,
    should_update: Option<ComputableInit<I, O, CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_failure_fns: Option<Vec<FailureHandler<I, O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
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
    pub fn build(self) -> IvoProperty<T, I, O, CtxOptions> {
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
    pub fn default<T, I: HasPartial, O: HasPartial, CtxOptions>(
        value: T,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No, No, No, No, No> {
        SchemaBuilder {
            default: Some(ComputableWithMiniSummary::Static(value)),
            ..Default::default()
        }
    }

    pub fn default_fn<T, I: HasPartial, O: HasPartial, CtxOptions>(
        default_fn: ResolverWithMiniSummaryFn<T, CtxOptions>,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No, No, No, No, No> {
        SchemaBuilder {
            default: Some(ComputableWithMiniSummary::SyncFunc(default_fn)),
            ..Default::default()
        }
    }
}

impl<T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No, No, No, No, No>
{
    pub fn validate<F>(
        self,
        validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, No, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<T>
            + Send
            + Sync
            + 'static,
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
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, No, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ValidatorResponse<T>> + Send + 'static,
    {
        SchemaBuilder {
            default: self.default,
            validator: Some(FieldValidator::Async(Box::new(move |v, s| {
                Box::pin(validator(v, s))
            }))),
            ..Default::default()
        }
    }
}

impl<T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, No, No, No, No, No, No, No, No>
{
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<T>
            + Send
            + Sync
            + 'static,
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
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, No, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ValidatorResponse<T>> + Send + 'static,
    {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: Some(FieldValidator::Async(Box::new(move |v, s| {
                Box::pin(re_validator(v, s))
            }))),
            ..Default::default()
        }
    }
}

impl<HasRevalidator, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, HasRevalidator, No, No, No, No, No, No, No>
{
    pub fn required_if<F>(
        self,
        required_fn: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, HasRevalidator, Yes, No, No, No, No, No, No>
    where
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> (bool, &str) + Send + Sync + 'static,
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

impl<HasValidator, HasRevalidator, HasRequired, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        No,
        No,
        No,
        No,
        No,
    >
{
    pub fn ignore_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
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
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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

impl<HasValidator, HasRevalidator, HasRequired, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        No,
        No,
        No,
        No,
        No,
    >
{
    pub fn ignore_init(
        self,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
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
        T,
        I,
        O,
        CtxOptions,
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
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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
        T,
        I,
        O,
        CtxOptions,
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
        T,
        I,
        O,
        CtxOptions,
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
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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

impl<HasValidator, HasRevalidator, HasRequired, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
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
    >
{
    pub fn allow_init_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
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
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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

impl<HasValidator, HasRevalidator, HasRequired, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
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
        T,
        I,
        O,
        CtxOptions,
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
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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

impl<HasValidator, HasRevalidator, HasRequired, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
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
        T,
        I,
        O,
        CtxOptions,
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
        T,
        I,
        O,
        CtxOptions,
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
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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
        handler: DeleteHandler<O, CtxOptions>,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
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
        handlers: Vec<DeleteHandler<O, CtxOptions>>,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
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
        handler: FailureHandler<I, O, CtxOptions>,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
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
        handlers: Vec<FailureHandler<I, O, CtxOptions>>,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
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
        handler: SuccessHandler<I, O, CtxOptions>,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
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
        handlers: Vec<SuccessHandler<I, O, CtxOptions>>,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
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
