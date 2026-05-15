use std::{future::Future, marker::PhantomData};

use serde_json::Value;

use crate::{
    schema::properties::base::IvoProperty,
    traits::HasPartial,
    types::{
        BooleanResolverWithMutSummary, ComputableInit, ComputableRequired, FailureHandler,
        FieldValidator, IvoSummary, SuccessHandler, VirtualSanitiser,
    },
    ValidatorResponse,
};

pub struct VirtualField;

// Marker Types
struct Yes;
struct No;
struct YesComputed;

struct SchemaBuilder<
    T,
    I: HasPartial,
    O: HasPartial,
    CtxOptions,
    HasValidator,
    HasAlias,
    HasRevalidator,
    HasSanitizer,
    HasRequired,
    HasIgnore,
    HasShouldInit,
    HasShouldUpdate,
    HasFailure,
    HasSuccess,
> {
    _alias: PhantomData<HasAlias>,
    _validator: PhantomData<HasValidator>,
    _re_validator: PhantomData<HasRevalidator>,
    _required_fn: PhantomData<HasRequired>,
    _sanitizer_fn: PhantomData<HasSanitizer>,
    _should_ignore: PhantomData<HasIgnore>,
    _should_init: PhantomData<HasShouldInit>,
    _should_update: PhantomData<HasShouldUpdate>,
    _on_failure_fns: PhantomData<HasFailure>,
    _on_success_fns: PhantomData<HasSuccess>,
    // actual data...
    alias: Option<String>,
    validator: Option<FieldValidator<T, I, O, CtxOptions>>,
    re_validator: Option<FieldValidator<T, I, O, CtxOptions>>,
    required: Option<ComputableRequired<I, O, CtxOptions>>,
    sanitizer: Option<VirtualSanitiser<T, I, O, CtxOptions>>,
    should_ignore_fn: Option<BooleanResolverWithMutSummary<I, O, CtxOptions>>,
    should_init: Option<ComputableInit<I, O, CtxOptions>>,
    should_update: Option<ComputableInit<I, O, CtxOptions>>,
    on_failure_fns: Option<Vec<FailureHandler<I, O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<
        HasValidator,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
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
    > Default
    for SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        HasValidator,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasFailure,
        HasSuccess,
    >
{
    fn default() -> Self {
        Self {
            alias: None,
            validator: None,
            re_validator: None,
            required: None,
            sanitizer: None,
            should_ignore_fn: None,
            should_init: None,
            should_update: None,
            on_failure_fns: None,
            on_success_fns: None,
            _alias: PhantomData,
            _validator: PhantomData,
            _re_validator: PhantomData,
            _required_fn: PhantomData,
            _sanitizer_fn: PhantomData,
            _should_ignore: PhantomData,
            _should_init: PhantomData,
            _should_update: PhantomData,
            _on_failure_fns: PhantomData,
            _on_success_fns: PhantomData,
        }
    }
}

impl<
        HasAlias,
        HasRevalidator,
        HasSanitizer,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasFailure,
        HasSuccess,
    >
{
    pub fn build(self) -> IvoProperty<T, I, O, CtxOptions> {
        IvoProperty {
            is_virtual: true,
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_ignore: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

impl VirtualField {
    pub fn alias<T, I: HasPartial, O: HasPartial, CtxOptions>(
        name: &str,
    ) -> SchemaBuilder<T, I, O, CtxOptions, No, Yes, No, No, No, No, No, No, No, No> {
        SchemaBuilder {
            alias: Some(name.to_string()),
            ..Default::default()
        }
    }

    pub fn validate<T, I: HasPartial, O: HasPartial, CtxOptions, F>(
        validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<T>
            + Send
            + Sync
            + 'static,
    {
        SchemaBuilder {
            validator: Some(FieldValidator::Sync(Box::new(validator))),
            ..Default::default()
        }
    }

    pub fn validate_async<T, I: HasPartial, O: HasPartial, CtxOptions, F, Fut>(
        validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ValidatorResponse<T>> + Send + 'static,
    {
        SchemaBuilder {
            validator: Some(FieldValidator::Async(Box::new(move |v, ctx| {
                Box::pin(validator(v, ctx))
            }))),
            ..Default::default()
        }
    }
}

impl<HasRevalidator, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, No, HasRevalidator, No, No, No, No, No, No, No>
{
    pub fn alias(
        self,
        name: &str,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, No, No, No, No, No, No, No, No> {
        SchemaBuilder {
            alias: Some(name.to_string()),
            validator: self.validator,
            re_validator: self.re_validator,
            sanitizer: self.sanitizer,
            required: self.required,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

impl<HasAlias, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, No, HasAlias, No, No, No, No, No, No, No, No>
{
    pub fn validate<F>(
        self,
        validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, HasAlias, No, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<T>
            + Send
            + Sync
            + 'static,
    {
        SchemaBuilder {
            alias: self.alias,
            validator: Some(FieldValidator::Sync(Box::new(validator))),
            ..Default::default()
        }
    }

    pub fn validate_async<F, Fut>(
        self,
        validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, HasAlias, No, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ValidatorResponse<T>> + Send + 'static,
    {
        SchemaBuilder {
            alias: self.alias,
            validator: Some(FieldValidator::Async(Box::new(move |v, ctx| {
                Box::pin(validator(v, ctx))
            }))),
            ..Default::default()
        }
    }
}

impl<HasAlias, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, HasAlias, No, No, No, No, No, No, No, No>
{
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, HasAlias, Yes, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<T>
            + Send
            + Sync
            + 'static,
    {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: Some(FieldValidator::Sync(Box::new(re_validator))),
            ..Default::default()
        }
    }

    pub fn re_validate_async<F, Fut>(
        self,
        re_validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, HasAlias, Yes, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ValidatorResponse<T>> + Send + 'static,
    {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: Some(FieldValidator::Async(Box::new(move |v, ctx| {
                Box::pin(re_validator(v, ctx))
            }))),
            ..Default::default()
        }
    }
}

impl<HasAlias, HasRevalidator, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, HasAlias, HasRevalidator, No, No, No, No, No, No, No>
{
    pub fn required_if<F>(
        self,
        required_fn: F,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasAlias,
        HasRevalidator,
        No,
        Yes,
        No,
        No,
        No,
        No,
        No,
    >
    where
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> (bool, &str) + Send + Sync + 'static,
    {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: Some(ComputableRequired::Func(Box::new(required_fn))),
            ..Default::default()
        }
    }
}

impl<HasAlias, HasRevalidator, HasRequired, T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasAlias,
        HasRevalidator,
        No,
        HasRequired,
        No,
        No,
        No,
        No,
        No,
    >
{
    pub fn sanitize<F>(
        self,
        sanitizer: F,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasAlias,
        HasRevalidator,
        Yes,
        HasRequired,
        No,
        No,
        No,
        No,
        No,
    >
    where
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> T + Send + Sync + 'static,
    {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: Some(Box::new(sanitizer)),
            ..Default::default()
        }
    }
}

impl<
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        Yes,
        No,
        No,
        No,
        No,
    >
    where
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_ignore_fn: Some(Box::new(fx)),
            ..Default::default()
        }
    }
}

impl<
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        No,
        Yes,
        No,
        No,
        No,
    > {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
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
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_init: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }

    pub fn ignore_update(
        self,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        No,
        No,
        Yes,
        No,
        No,
    > {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        No,
        No,
        YesComputed,
        No,
        No,
    >
    where
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_init: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }
}

impl<
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        No,
        No,
        Yes,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        No,
        YesComputed,
        Yes,
        No,
        No,
    >
    where
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_init: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }
}

impl<
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        No,
        Yes,
        YesComputed,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        No,
        Yes,
        YesComputed,
        No,
        No,
    >
    where
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_init: self.should_init,
            should_update: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }
}

impl<
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        No,
        No,
        YesComputed,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        No,
        Yes,
        YesComputed,
        No,
        No,
    > {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        No,
        YesComputed,
        YesComputed,
        No,
        No,
    >
    where
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_update: self.should_update,
            should_init: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }
}

// ON_FAILURE is only available if HasFailure is 'No'
impl<
        HasAlias,
        HasRevalidator,
        HasSanitizer,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasFailure,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        Yes,
        HasSuccess,
    > {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            sanitizer: self.sanitizer,
            required: self.required,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        Yes,
        HasSuccess,
    > {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            sanitizer: self.sanitizer,
            required: self.required,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            on_failure_fns: Some(handlers),
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

// ON_SUCCESS is only available if HasSuccess is 'No'
impl<
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasFailure,
        Yes,
    > {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            sanitizer: self.sanitizer,
            required: self.required,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
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
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasFailure,
        Yes,
    > {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            sanitizer: self.sanitizer,
            required: self.required,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: Some(handlers),
            ..Default::default()
        }
    }
}
