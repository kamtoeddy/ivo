use std::{future::Future, marker::PhantomData};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::{
    schema::properties::base::{InternalIvoProperty, IvoProperty, IvoPropertyBuilder},
    traits::{
        IntoAsyncFieldReValidator, IntoAsyncFieldValidator, IntoFieldReValidator,
        IntoFieldValidator, IntoVirtualSanitizer, IvoSchemaStruct,
    },
    types::{
        BooleanResolverWithMutSummary, ComputableInit, ComputableRequired, FailureHandler,
        FieldReValidator, FieldValidator, IvoSummary, SuccessHandler, VirtualSanitiser,
    },
};

pub struct VirtualField;

// Marker Types
pub struct Yes;
pub struct No;
pub struct YesComputed;

pub struct SchemaBuilder<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
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
    > IvoPropertyBuilder<I, O, CtxOptions>
    for SchemaBuilder<
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

impl VirtualField {
    pub fn alias<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>(
        name: &str,
    ) -> SchemaBuilder<T, I, O, CtxOptions, No, Yes, No, No, No, No, No, No, No, No> {
        SchemaBuilder {
            alias: Some(name.to_string()),
            ..Default::default()
        }
    }

    pub fn validate<F, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>(
        validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No, No, No, No, No>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions>,
    {
        SchemaBuilder {
            validator: Some(FieldValidator::Sync(validator.into_uniform())),
            ..Default::default()
        }
    }

    pub fn validate_async<F, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>(
        validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No, No, No, No, No>
    where
        F: IntoAsyncFieldValidator<T, I, O, CtxOptions>,
    {
        SchemaBuilder {
            validator: Some(FieldValidator::Async(validator.into_uniform())),
            ..Default::default()
        }
    }
}

impl<HasRevalidator, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
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

impl<HasAlias, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    SchemaBuilder<T, I, O, CtxOptions, No, HasAlias, No, No, No, No, No, No, No, No>
{
    pub fn validate<F>(
        self,
        validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, HasAlias, No, No, No, No, No, No, No, No>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions>,
    {
        SchemaBuilder {
            alias: self.alias,
            validator: Some(FieldValidator::Sync(validator.into_uniform())),
            ..Default::default()
        }
    }

    pub fn validate_async<F>(
        self,
        validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, HasAlias, No, No, No, No, No, No, No, No>
    where
        F: IntoAsyncFieldValidator<T, I, O, CtxOptions>,
    {
        SchemaBuilder {
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
    > SchemaBuilder<T, I, O, CtxOptions, Yes, HasAlias, No, No, No, No, No, No, No, No>
{
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, HasAlias, Yes, No, No, No, No, No, No, No>
    where
        F: IntoFieldReValidator<T, I, O, CtxOptions>,
    {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: Some(FieldReValidator::Sync(re_validator.into_uniform())),
            ..Default::default()
        }
    }

    pub fn re_validate_async<F>(
        self,
        re_validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, HasAlias, Yes, No, No, No, No, No, No, No>
    where
        F: IntoAsyncFieldReValidator<T, I, O, CtxOptions>,
    {
        SchemaBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: Some(FieldReValidator::Async(re_validator.into_uniform())),
            ..Default::default()
        }
    }
}

impl<HasAlias, HasRevalidator, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    SchemaBuilder<T, I, O, CtxOptions, Yes, HasAlias, HasRevalidator, No, No, No, No, No, No, No>
{
    pub fn required_if<F, Fut>(
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
        F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = (bool, &'static str)> + Send + 'static,
    {
        SchemaBuilder {
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
    >
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
        F: IntoVirtualSanitizer<T, I, O, CtxOptions>,
    {
        SchemaBuilder {
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
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
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
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
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
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
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
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
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
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
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
    pub fn on_failure<F, Fut>(
        self,
        handler: F,
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
    >
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h: FailureHandler<I, O, CtxOptions> = Box::new(move |s| Box::pin(handler(s)));

        SchemaBuilder {
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
    pub fn on_success<F, Fut>(
        self,
        handler: F,
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
    >
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h: SuccessHandler<I, O, CtxOptions> = Box::new(move |s| Box::pin(handler(s)));

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
