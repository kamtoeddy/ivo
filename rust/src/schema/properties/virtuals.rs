use crate::{
    schema::properties::base::IvoProperty,
    types::{
        Computable, ComputableRequired, FailureHandler, FieldValidatorFn, RequiredResolverFn,
        SuccessHandler, VirtualSanitiser,
    },
};

pub struct VirtualField;

// Marker Types
pub struct Yes;
pub struct No;

struct WithValidatorBuilder<I, O, T> {
    validator: FieldValidatorFn<I, O, T>,
}

impl<I, O, T> WithValidatorBuilder<I, O, T> {
    pub fn required(
        self,
        required_fn: RequiredResolverFn<I, O>,
    ) -> WithReValidatorBuilder<I, O, T> {
        WithReValidatorBuilder {
            validator: self.validator,
            required_fn: Some(required_fn),
            re_validator: None,
            sanitizer_fn: None,
            alias_name: None,
            on_failure_fns: None,
            on_success_fns: None,
        }
    }

    pub fn re_validator(
        self,
        re_validator: FieldValidatorFn<I, O, T>,
    ) -> WithReValidatorBuilder<I, O, T> {
        WithReValidatorBuilder {
            validator: self.validator,
            re_validator: Some(re_validator),
            required_fn: None,
            sanitizer_fn: None,
            alias_name: None,
            on_failure_fns: None,
            on_success_fns: None,
        }
    }
}

struct WithReValidatorBuilder<I, O, T> {
    validator: FieldValidatorFn<I, O, T>,
    re_validator: Option<FieldValidatorFn<I, O, T>>,
    required_fn: Option<RequiredResolverFn<I, O>>,
    sanitizer_fn: Option<VirtualSanitiser<I, O, T>>,
    alias_name: Option<String>,
    on_failure_fns: Option<Vec<FailureHandler<I, O>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O>>>,
}

impl<I, O, T> WithReValidatorBuilder<I, O, T> {
    pub fn alias(mut self, name: &str) -> Self {
        self.alias_name = Some(name.to_string());

        self
    }

    pub fn required(mut self, fx: RequiredResolverFn<I, O>) -> Self {
        self.required_fn = Some(fx);

        self
    }

    pub fn sanitizer(mut self, fx: VirtualSanitiser<I, O, T>) -> Self {
        self.sanitizer_fn = Some(fx);

        self
    }

    pub fn on_success(mut self, handler: SuccessHandler<I, O>) -> Self {
        let mut handlers = self.on_success_fns.unwrap_or(vec![]);

        handlers.push(handler);

        self.on_success_fns = Some(handlers);

        self
    }

    pub fn on_failure(mut self, handler: FailureHandler<I, O>) -> Self {
        let mut handlers = self.on_failure_fns.unwrap_or(vec![]);

        handlers.push(handler);

        self.on_failure_fns = Some(handlers);

        self
    }

    pub fn build(self) -> IvoProperty<I, O, T> {
        IvoProperty {
            is_virtual: true,
            validator: Some(self.validator),
            re_validator: self.re_validator,
            required: match self.required_fn {
                Some(fx) => Some(ComputableRequired::Func(fx)),
                _ => None,
            },
            sanitizer: self.sanitizer_fn,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

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
    _alias: std::marker::PhantomData<HasAlias>,
    _validator: std::marker::PhantomData<HasValidator>,
    _re_validator: std::marker::PhantomData<HasRevalidator>,
    _required_fn: std::marker::PhantomData<HasRequired>,
    _sanitizer_fn: std::marker::PhantomData<HasSanitizer>,
    _should_ignore: std::marker::PhantomData<HasIgnore>,
    _should_init: std::marker::PhantomData<HasShouldInit>,
    _should_update: std::marker::PhantomData<HasShouldUpdate>,
    _on_failure_fns: std::marker::PhantomData<HasFailure>,
    _on_success_fns: std::marker::PhantomData<HasSuccess>,
    // actual data...
    alias: Option<String>,
    validator: Option<FieldValidatorFn<I, O, T>>,
    re_validator: Option<FieldValidatorFn<I, O, T>>,
    required: Option<ComputableRequired<I, O>>,
    sanitizer: Option<VirtualSanitiser<I, O, T>>,
    should_ignore: Option<Computable<I, O, bool>>,
    should_init: Option<Computable<I, O, bool>>,
    should_update: Option<Computable<I, O, bool>>,
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
            should_ignore: None,
            should_init: None,
            should_update: None,
            on_failure_fns: None,
            on_success_fns: None,
            _alias: std::marker::PhantomData,
            _validator: std::marker::PhantomData,
            _re_validator: std::marker::PhantomData,
            _required_fn: std::marker::PhantomData,
            _sanitizer_fn: std::marker::PhantomData,
            _should_ignore: std::marker::PhantomData,
            _should_init: std::marker::PhantomData,
            _should_update: std::marker::PhantomData,
            _on_failure_fns: std::marker::PhantomData,
            _on_success_fns: std::marker::PhantomData,
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
            should_ignore: self.should_ignore,
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

    pub fn validator<I, O, T>(
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
            should_ignore: self.should_ignore,
            should_init: self.should_init,
            should_update: self.should_update,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

impl<HasAlias, I, O, T> SchemaBuilder<I, O, T, No, HasAlias, No, No, No, No, No, No, No, No> {
    pub fn validator(
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
    pub fn re_validator(
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
            should_ignore: self.should_ignore,
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
            should_ignore: self.should_ignore,
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
            should_ignore: self.should_ignore,
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
            should_ignore: self.should_ignore,
            should_init: self.should_init,
            should_update: self.should_update,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: Some(handlers),
            ..Default::default()
        }
    }
}
