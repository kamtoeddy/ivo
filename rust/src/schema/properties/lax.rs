use crate::{
    schema::properties::base::IvoProperty,
    types::{
        BooleanResolverWithMutSummary, ComputableInit, ComputableRequired, ComputableWithContext,
        DeleteHandler, FailureHandler, FieldValidatorFn, RequiredResolverFn, ResolverWithContextFn,
        SuccessHandler,
    },
};

pub struct LaxField;

// Marker Types
pub struct Yes;
pub struct No;
pub struct YesComputed;

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
    _default: std::marker::PhantomData<HasDefault>,
    _validator: std::marker::PhantomData<HasValidator>,
    _re_validator: std::marker::PhantomData<HasRevalidator>,
    _required_fn: std::marker::PhantomData<HasRequired>,
    _should_ignore: std::marker::PhantomData<HasIgnore>,
    _should_init: std::marker::PhantomData<HasShouldInit>,
    _should_update: std::marker::PhantomData<HasShouldUpdate>,
    _on_delete_fns: std::marker::PhantomData<HasDelete>,
    _on_failure_fns: std::marker::PhantomData<HasFailure>,
    _on_success_fns: std::marker::PhantomData<HasSuccess>,
    // actual data...
    default: Option<ComputableWithContext<I, O, T>>,
    validator: Option<FieldValidatorFn<I, O, T>>,
    re_validator: Option<FieldValidatorFn<I, O, T>>,
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
            _default: std::marker::PhantomData,
            _validator: std::marker::PhantomData,
            _re_validator: std::marker::PhantomData,
            _required_fn: std::marker::PhantomData,
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
            default: Some(ComputableWithContext::Func(default_fn)),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, No, No, No, No, No, No, No, No, No> {
    pub fn validate(
        self,
        validator: FieldValidatorFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No, No, No, No, No> {
        SchemaBuilder {
            default: self.default,
            validator: Some(validator),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No, No, No, No, No> {
    pub fn re_validate(
        self,
        re_validator: FieldValidatorFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, No, No, No, No, No, No> {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: Some(re_validator),
            ..Default::default()
        }
    }
}

impl<HasRevalidator, I, O, T>
    SchemaBuilder<I, O, T, Yes, Yes, HasRevalidator, No, No, No, No, No, No, No>
{
    pub fn required_if(
        self,
        required_fn: RequiredResolverFn<I, O>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, HasRevalidator, Yes, No, No, No, No, No, No> {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: Some(ComputableRequired::Func(required_fn)),
            ..Default::default()
        }
    }
}

impl<HasValidator, HasRevalidator, HasRequired, I, O, T>
    SchemaBuilder<I, O, T, Yes, HasValidator, HasRevalidator, HasRequired, No, No, No, No, No, No>
{
    pub fn ignore_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
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
    > {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            should_ignore_fn: Some(fx),
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

    pub fn allow_init_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
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
    > {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
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

    pub fn allow_update_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
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
    > {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            should_init: Some(ComputableInit::Func(fx)),
            ..Default::default()
        }
    }
}

impl<HasValidator, HasRevalidator, HasRequired, I, O, T>
    SchemaBuilder<I, O, T, Yes, HasValidator, HasRevalidator, HasRequired, No, No, Yes, No, No, No>
{
    pub fn allow_init_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
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
    > {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            should_init: Some(ComputableInit::Func(fx)),
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
    pub fn allow_update_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
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
            should_init: self.should_init,
            should_update: Some(ComputableInit::Func(fx)),
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

    pub fn allow_init_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
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
    > {
        SchemaBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            should_update: self.should_update,
            should_init: Some(ComputableInit::Func(fx)),
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
