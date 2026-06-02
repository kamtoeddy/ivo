use std::{future::Future, marker::PhantomData};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::{
    fields::base::{BuildableIvoProperty, InternalIvoProperty, IvoProperty},
    traits::{
        IntoAsyncFieldReValidator, IntoAsyncFieldValidator, IntoFieldReValidator,
        IntoFieldValidator, IntoVirtualSanitizer, IvoSchemaStruct,
    },
    types::{
        BooleanResolverWithMutSummary, ComputableInit, ComputableRequired, FailureHandler,
        FieldReValidator, FieldValidator, IvoSummary, SuccessHandler, VirtualSanitiser,
    },
};

// Marker Types
pub struct Yes;
pub struct No;
pub struct YesComputed;

pub struct VirtualFieldBuilder<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    HasValidator = No,
    HasAlias = No,
    HasRevalidator = No,
    HasSanitizer = No,
    HasRequired = No,
    HasIgnore = No,
    HasShouldInit = No,
    HasShouldUpdate = No,
    HasFailure = No,
    HasSuccess = No,
> {
    _t: PhantomData<T>,
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
    validator: Option<FieldValidator<I, O, CtxOptions>>,
    re_validator: Option<FieldReValidator<I, O, CtxOptions>>,
    required: Option<ComputableRequired<I, O, CtxOptions>>,
    sanitizer: Option<VirtualSanitiser<Value, I, O, CtxOptions>>,
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    >
    VirtualFieldBuilder<
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
    pub const fn new() -> Self {
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
            _t: PhantomData,
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    > Default
    for VirtualFieldBuilder<
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
        Self::new()
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    > BuildableIvoProperty<I, O, CtxOptions>
    for VirtualFieldBuilder<
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
    fn build(self) -> InternalIvoProperty<I, O, CtxOptions> {
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

impl<
        HasValidator,
        HasRevalidator,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    > VirtualFieldBuilder<T, I, O, CtxOptions, HasValidator, No, HasRevalidator>
{
    pub fn alias(self, name: &str) -> VirtualFieldBuilder<T, I, O, CtxOptions, HasValidator, Yes> {
        VirtualFieldBuilder {
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

impl<HasAlias, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    VirtualFieldBuilder<T, I, O, CtxOptions, No, HasAlias>
{
    pub fn validate<F>(
        self,
        validator: F,
    ) -> VirtualFieldBuilder<T, I, O, CtxOptions, Yes, HasAlias>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: Some(FieldValidator::Sync(validator.into_uniform())),
            ..Default::default()
        }
    }

    pub fn validate_async<F>(
        self,
        validator: F,
    ) -> VirtualFieldBuilder<T, I, O, CtxOptions, Yes, HasAlias>
    where
        F: IntoAsyncFieldValidator<T, I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: Some(FieldValidator::Async(validator.into_uniform())),
            ..Default::default()
        }
    }
}

impl<
        HasAlias,
        T: DeserializeOwned + Serialize,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    > VirtualFieldBuilder<T, I, O, CtxOptions, Yes, HasAlias>
{
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> VirtualFieldBuilder<T, I, O, CtxOptions, Yes, HasAlias, Yes>
    where
        F: IntoFieldReValidator<T, I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: Some(FieldReValidator::Sync(re_validator.into_uniform())),
            ..Default::default()
        }
    }

    pub fn re_validate_async<F>(
        self,
        re_validator: F,
    ) -> VirtualFieldBuilder<T, I, O, CtxOptions, Yes, HasAlias, Yes>
    where
        F: IntoAsyncFieldReValidator<T, I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: Some(FieldReValidator::Async(re_validator.into_uniform())),
            ..Default::default()
        }
    }
}

impl<HasAlias, HasRevalidator, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    VirtualFieldBuilder<T, I, O, CtxOptions, Yes, HasAlias, HasRevalidator>
{
    pub fn required_if<F, Fut>(
        self,
        required_fn: F,
    ) -> VirtualFieldBuilder<T, I, O, CtxOptions, Yes, HasAlias, HasRevalidator, No, Yes>
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = (bool, String)> + Send + 'static,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: Some(ComputableRequired::Func(Box::new(move |s| {
                Box::pin(required_fn(s))
            }))),
            ..Default::default()
        }
    }
}

impl<
        HasAlias,
        HasRevalidator,
        HasRequired,
        T: Serialize,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    > VirtualFieldBuilder<T, I, O, CtxOptions, Yes, HasAlias, HasRevalidator, No, HasRequired>
{
    pub fn sanitize<F>(
        self,
        sanitizer: F,
    ) -> VirtualFieldBuilder<T, I, O, CtxOptions, Yes, HasAlias, HasRevalidator, Yes, HasRequired>
    where
        F: IntoVirtualSanitizer<T, I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: Some(sanitizer.into_uniform()),
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
    >
{
    pub fn ignore_if<F>(
        self,
        fx: F,
    ) -> VirtualFieldBuilder<
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
    >
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        VirtualFieldBuilder {
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
    >
{
    pub fn ignore_init(
        self,
    ) -> VirtualFieldBuilder<
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
    > {
        VirtualFieldBuilder {
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
    ) -> VirtualFieldBuilder<
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
    >
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        VirtualFieldBuilder {
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
    ) -> VirtualFieldBuilder<
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
    > {
        VirtualFieldBuilder {
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
    ) -> VirtualFieldBuilder<
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
    >
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        VirtualFieldBuilder {
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    >
    VirtualFieldBuilder<
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
    >
{
    pub fn allow_init_if<F>(
        self,
        fx: F,
    ) -> VirtualFieldBuilder<
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
    >
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        VirtualFieldBuilder {
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    >
    VirtualFieldBuilder<
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
    >
{
    pub fn allow_update_if<F>(
        self,
        fx: F,
    ) -> VirtualFieldBuilder<
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
    >
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        VirtualFieldBuilder {
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    >
    VirtualFieldBuilder<
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
    >
{
    pub fn ignore_init(
        self,
    ) -> VirtualFieldBuilder<
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
    > {
        VirtualFieldBuilder {
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
    ) -> VirtualFieldBuilder<
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
    >
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        VirtualFieldBuilder {
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    >
    VirtualFieldBuilder<
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
    pub fn on_failure<F, Fut>(
        self,
        handler: F,
    ) -> VirtualFieldBuilder<
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
    >
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h: FailureHandler<I, O, CtxOptions> = Box::new(move |s| Box::pin(handler(s)));

        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            sanitizer: self.sanitizer,
            required: self.required,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    >
    VirtualFieldBuilder<
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
    pub fn on_success<F, Fut>(
        self,
        handler: F,
    ) -> VirtualFieldBuilder<
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
    >
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h: SuccessHandler<I, O, CtxOptions> = Box::new(move |s| Box::pin(handler(s)));

        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            sanitizer: self.sanitizer,
            required: self.required,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
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
