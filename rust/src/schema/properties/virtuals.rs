use std::marker::PhantomData;

use crate::{
    schema::properties::base::IvoProperty,
    types::{
        BooleanResolverWithMutSummary, ComputableInit, ComputableRequired, FailureHandler,
        FieldValidatorFn, RequiredResolverFn, SuccessHandler, VirtualSanitiser,
    },
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
    validator: Option<FieldValidatorFn<I, O, T>>,
    re_validator: Option<FieldValidatorFn<I, O, T>>,
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

    pub fn validate<I, O, T>(
        validator: FieldValidatorFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, No, No, No, No, No, No, No, No, No> {
        SchemaBuilder {
            validator: Some(validator),
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
    pub fn validate(
        self,
        validator: FieldValidatorFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, HasAlias, No, No, No, No, No, No, No, No> {
        SchemaBuilder {
            alias: self.alias,
            validator: Some(validator),
            ..Default::default()
        }
    }
}

impl<HasAlias, I, O, T> SchemaBuilder<I, O, T, Yes, HasAlias, No, No, No, No, No, No, No, No> {
    pub fn re_validate(
        self,
        re_validator: FieldValidatorFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, HasAlias, Yes, No, No, No, No, No, No, No> {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: Some(re_validator),
            ..Default::default()
        }
    }
}

impl<HasAlias, HasRevalidator, I, O, T>
    SchemaBuilder<I, O, T, Yes, HasAlias, HasRevalidator, No, No, No, No, No, No, No>
{
    pub fn required_if(
        self,
        required_fn: RequiredResolverFn<I, O>,
    ) -> SchemaBuilder<I, O, T, Yes, HasAlias, HasRevalidator, No, Yes, No, No, No, No, No> {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: Some(ComputableRequired::Func(required_fn)),
            ..Default::default()
        }
    }
}

impl<HasAlias, HasRevalidator, HasRequired, I, O, T>
    SchemaBuilder<I, O, T, Yes, HasAlias, HasRevalidator, No, HasRequired, No, No, No, No, No>
{
    pub fn sanitize(
        self,
        sanitizer: VirtualSanitiser<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, HasAlias, HasRevalidator, Yes, HasRequired, No, No, No, No, No>
    {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: Some(sanitizer),
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
    pub fn ignore_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
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
    > {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_ignore_fn: Some(fx),
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

    pub fn allow_init_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
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
    > {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_init: Some(ComputableInit::Func(fx)),
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

    pub fn allow_update_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
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
    > {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_init: Some(ComputableInit::Func(fx)),
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
    pub fn allow_init_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
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
    > {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_init: Some(ComputableInit::Func(fx)),
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
    pub fn allow_update_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
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
            should_init: self.should_init,
            should_update: Some(ComputableInit::Func(fx)),
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

    pub fn allow_init_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
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
    > {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_update: self.should_update,
            should_init: Some(ComputableInit::Func(fx)),
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
