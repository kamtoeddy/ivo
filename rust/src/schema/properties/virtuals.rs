use std::{future::Future, marker::PhantomData};

use serde_json::Value;

use crate::{
    schema::properties::base::IvoProperty,
    types::{
        BooleanResolverWithMutSummary, ComputableInit, ComputableRequired, Context, FailureHandler,
        FieldValidator, MutableSummary, SuccessHandler, VirtualSanitiser,
    },
    ValidatorResponse,
};

pub struct VirtualField;

// Marker Types
struct Yes;
struct No;
struct YesComputed;

struct SchemaBuilder<
    I,
    O,
    T,
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
    validator: Option<FieldValidator<I, O, T>>,
    re_validator: Option<FieldValidator<I, O, T>>,
    required: Option<ComputableRequired<I, O>>,
    sanitizer: Option<VirtualSanitiser<I, O, T>>,
    should_ignore_fn: Option<BooleanResolverWithMutSummary<I, O>>,
    should_init: Option<ComputableInit<I, O>>,
    should_update: Option<ComputableInit<I, O>>,
    on_failure_fns: Option<Vec<FailureHandler<I, O>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O>>>,
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
        I,
        O,
        T,
    > Default
    for SchemaBuilder<
        I,
        O,
        T,
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
        I,
        O,
        T,
    >
    SchemaBuilder<
        I,
        O,
        T,
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
    pub fn build(self) -> IvoProperty<I, O, T> {
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
    pub fn alias<I, O, T>(
        name: &str,
    ) -> SchemaBuilder<I, O, T, No, Yes, No, No, No, No, No, No, No, No> {
        SchemaBuilder {
            alias: Some(name.to_string()),
            ..Default::default()
        }
    }

    pub fn validate<I, O, T, F>(
        validator: F,
    ) -> SchemaBuilder<I, O, T, Yes, No, No, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &Context<I, O>) -> ValidatorResponse<T> + Send + Sync + 'static,
    {
        SchemaBuilder {
            validator: Some(FieldValidator::Sync(Box::new(validator))),
            ..Default::default()
        }
    }

    pub fn validate_async<I, O, T, F, Fut>(
        validator: F,
    ) -> SchemaBuilder<I, O, T, Yes, No, No, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &Context<I, O>) -> Fut + Send + Sync + 'static,
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

impl<HasRevalidator, I, O, T>
    SchemaBuilder<I, O, T, Yes, No, HasRevalidator, No, No, No, No, No, No, No>
{
    pub fn alias(
        self,
        name: &str,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No, No, No, No, No> {
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

impl<HasAlias, I, O, T> SchemaBuilder<I, O, T, No, HasAlias, No, No, No, No, No, No, No, No> {
    pub fn validate<F>(
        self,
        validator: F,
    ) -> SchemaBuilder<I, O, T, Yes, HasAlias, No, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &Context<I, O>) -> ValidatorResponse<T> + Send + Sync + 'static,
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
    ) -> SchemaBuilder<I, O, T, Yes, HasAlias, No, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &Context<I, O>) -> Fut + Send + Sync + 'static,
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

impl<HasAlias, I, O, T> SchemaBuilder<I, O, T, Yes, HasAlias, No, No, No, No, No, No, No, No> {
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> SchemaBuilder<I, O, T, Yes, HasAlias, Yes, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &Context<I, O>) -> ValidatorResponse<T> + Send + Sync + 'static,
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
    ) -> SchemaBuilder<I, O, T, Yes, HasAlias, Yes, No, No, No, No, No, No, No>
    where
        F: Fn(&Value, &Context<I, O>) -> Fut + Send + Sync + 'static,
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

impl<HasAlias, HasRevalidator, I, O, T>
    SchemaBuilder<I, O, T, Yes, HasAlias, HasRevalidator, No, No, No, No, No, No, No>
{
    pub fn required_if<F>(
        self,
        required_fn: F,
    ) -> SchemaBuilder<I, O, T, Yes, HasAlias, HasRevalidator, No, Yes, No, No, No, No, No>
    where
        F: Fn(&Context<I, O>) -> (bool, &str) + Send + Sync + 'static,
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

impl<HasAlias, HasRevalidator, HasRequired, I, O, T>
    SchemaBuilder<I, O, T, Yes, HasAlias, HasRevalidator, No, HasRequired, No, No, No, No, No>
{
    pub fn sanitize<F>(
        self,
        sanitizer: F,
    ) -> SchemaBuilder<I, O, T, Yes, HasAlias, HasRevalidator, Yes, HasRequired, No, No, No, No, No>
    where
        F: Fn(&MutableSummary<I, O>) -> T + Send + Sync + 'static,
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

impl<HasAlias, HasRevalidator, HasSanitizer, HasRequired, I, O, T>
    SchemaBuilder<
        I,
        O,
        T,
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
        I,
        O,
        T,
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
        F: Fn(&MutableSummary<I, O>) -> bool + Send + Sync + 'static,
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

impl<HasAlias, HasRevalidator, HasSanitizer, HasRequired, I, O, T>
    SchemaBuilder<
        I,
        O,
        T,
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
        I,
        O,
        T,
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
        I,
        O,
        T,
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
        F: Fn(&MutableSummary<I, O>) -> bool + Send + Sync + 'static,
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
        I,
        O,
        T,
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
        I,
        O,
        T,
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
        F: Fn(&MutableSummary<I, O>) -> bool + Send + Sync + 'static,
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

impl<HasAlias, HasRevalidator, HasSanitizer, HasRequired, I, O, T>
    SchemaBuilder<
        I,
        O,
        T,
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
        I,
        O,
        T,
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
        F: Fn(&MutableSummary<I, O>) -> bool + Send + Sync + 'static,
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

impl<HasAlias, HasRevalidator, HasSanitizer, HasRequired, I, O, T>
    SchemaBuilder<
        I,
        O,
        T,
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
        I,
        O,
        T,
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
        F: Fn(&MutableSummary<I, O>) -> bool + Send + Sync + 'static,
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

impl<HasAlias, HasRevalidator, HasSanitizer, HasRequired, I, O, T>
    SchemaBuilder<
        I,
        O,
        T,
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
        I,
        O,
        T,
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
        I,
        O,
        T,
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
        F: Fn(&MutableSummary<I, O>) -> bool + Send + Sync + 'static,
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
        I,
        O,
        T,
    >
    SchemaBuilder<
        I,
        O,
        T,
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
        handler: FailureHandler<I, O>,
    ) -> SchemaBuilder<
        I,
        O,
        T,
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
        handlers: Vec<FailureHandler<I, O>>,
    ) -> SchemaBuilder<
        I,
        O,
        T,
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
        I,
        O,
        T,
    >
    SchemaBuilder<
        I,
        O,
        T,
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
        handler: SuccessHandler<I, O>,
    ) -> SchemaBuilder<
        I,
        O,
        T,
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
        handlers: Vec<SuccessHandler<I, O>>,
    ) -> SchemaBuilder<
        I,
        O,
        T,
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
