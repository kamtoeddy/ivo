use std::marker::PhantomData;

use crate::{
    schema::{
        error::IvoErrorTool,
        fields::{
            base::{BuildableFieldConfig, FieldConfig, InternalFieldConfig},
            types::{
                BooleanResolverWithMutSummary, ComputableInit, ComputableRequired,
                IntoFailureHandler, IntoFieldValidator, IntoRequiredResolverFn,
                IntoResolverWithMutSummaryFn, IntoSuccessHandler, IntoVirtualSanitizer,
                UniformValidator, VirtualSanitiser,
            },
        },
    },
    types::{FailureHandler, No, SuccessHandler, Yes, YesComputed},
    utils::erased_value::ErasedValue,
    IvoSchemaStruct,
};

pub struct VirtualFieldBuilder<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
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
    validator: Option<UniformValidator<I, O, CtxOptions, ErrT::FieldMetadata>>,
    re_validator: Option<UniformValidator<I, O, CtxOptions, ErrT::FieldMetadata>>,
    required: Option<ComputableRequired<I, O, CtxOptions>>,
    sanitizer: Option<VirtualSanitiser<ErasedValue, I, O, CtxOptions>>,
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
        ErrT: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        ErrT: IvoErrorTool,
    > Default
    for VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        ErrT: IvoErrorTool,
    > BuildableFieldConfig<I, O, CtxOptions, ErrT>
    for VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
    fn build(self) -> InternalFieldConfig<I, O, CtxOptions, ErrT> {
        FieldConfig {
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
        ErrT: IvoErrorTool,
    > VirtualFieldBuilder<T, I, O, CtxOptions, ErrT, HasValidator, No, HasRevalidator>
{
    /// this can used to mask the actual name of the virtual field from the public
    /// if set, this name must replace the actual name of the virtual field on the input struct, I
    /// i.e: I cannot have both the virtual field name and it alias
    pub fn alias(
        self,
        name: &str,
    ) -> VirtualFieldBuilder<T, I, O, CtxOptions, ErrT, HasValidator, Yes> {
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

impl<
        HasAlias,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > VirtualFieldBuilder<T, I, O, CtxOptions, ErrT, No, HasAlias>
{
    pub fn validate<F>(
        self,
        validator: F,
    ) -> VirtualFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, HasAlias>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions, ErrT>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: Some(validator.into_uniform()),
            ..Default::default()
        }
    }
}

impl<
        HasAlias,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > VirtualFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, HasAlias>
{
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> VirtualFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, HasAlias, Yes>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions, ErrT>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: Some(re_validator.into_uniform()),
            ..Default::default()
        }
    }
}

impl<
        HasAlias,
        HasRevalidator,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > VirtualFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, HasAlias, HasRevalidator>
{
    pub fn required_if<R>(
        self,
        resolver: R,
    ) -> VirtualFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, HasAlias, HasRevalidator, No, Yes>
    where
        R: IntoRequiredResolverFn<I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: Some(ComputableRequired::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }
}

impl<
        HasAlias,
        HasRevalidator,
        HasRequired,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    >
    VirtualFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, HasAlias, HasRevalidator, No, HasRequired>
{
    pub fn sanitize<F>(
        self,
        sanitizer: F,
    ) -> VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        HasAlias,
        HasRevalidator,
        Yes,
        HasRequired,
    >
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
        ErrT: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
    >
{
    pub fn ignore_if<R>(
        self,
        resolver: R,
    ) -> VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        Yes,
    >
    where
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_ignore_fn: Some(resolver.into_resolver()),
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
        ErrT: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        ErrT,
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

    pub fn allow_init_if<R>(
        self,
        resolver: R,
    ) -> VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        No,
        YesComputed,
    >
    where
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_init: Some(ComputableInit::Func(resolver.into_resolver())),
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
        ErrT,
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

    pub fn allow_update_if<R>(
        self,
        resolver: R,
    ) -> VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_init: Some(ComputableInit::Func(resolver.into_resolver())),
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
        ErrT: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
    pub fn allow_init_if<R>(
        self,
        resolver: R,
    ) -> VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_init: Some(ComputableInit::Func(resolver.into_resolver())),
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
        ErrT: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
    pub fn allow_update_if<R>(
        self,
        resolver: R,
    ) -> VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_init: self.should_init,
            should_update: Some(ComputableInit::Func(resolver.into_resolver())),
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
        ErrT: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        ErrT,
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

    pub fn allow_init_if<R>(
        self,
        resolver: R,
    ) -> VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
            sanitizer: self.sanitizer,
            should_update: self.should_update,
            should_init: Some(ComputableInit::Func(resolver.into_resolver())),
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
        ErrT: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
    pub fn on_failure<H>(
        self,
        handler: H,
    ) -> VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        H: IntoFailureHandler<I, O, CtxOptions>,
    {
        let h = handler.into_handler();

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
                    let mut v = hs;

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
        ErrT: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
    pub fn on_success<H>(
        self,
        handler: H,
    ) -> VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        H: IntoSuccessHandler<I, O, CtxOptions>,
    {
        let h = handler.into_handler();

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
                    let mut v = hs;

                    v.push(h);

                    v
                }
                _ => vec![h],
            }),
            ..Default::default()
        }
    }
}
