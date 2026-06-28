#![expect(clippy::type_complexity)]

use std::marker::PhantomData;

use crate::{
    schema::{
        error_tool::IvoErrorTool,
        fields::{
            base::{BuildableFieldConfig, FieldConfig, FieldType, InternalFieldConfig},
            types::{
                BooleanResolver, IntoBooleanResolver, IntoFailureHandler, IntoFieldValidator,
                IntoRequiredResolver, IntoResolver, IntoSuccessHandler, IntoVirtualSanitizer,
                IsFieldProvisionEnabled, RequiredResolver, UniformValidator, VirtualSanitizer,
            },
        },
        types::{FailureHandler, IvoFieldValue, No, SuccessHandler, Yes, YesComputed},
    },
    types::ErasedValue,
    IvoStruct,
};

pub struct VirtualFieldBuilder<
    T: IvoFieldValue,
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
    HasValidator = No,
    HasAlias = No,
    HasRevalidator = No,
    HasSanitizer = No,
    HasRequired = No,
    HasIgnore = No,
    HasIgnoreInit = No,
    HasIgnoreUpdate = No,
    HasFailure = No,
    HasSuccess = No,
> {
    _t: PhantomData<T>,
    _alias: PhantomData<HasAlias>,
    _validator: PhantomData<HasValidator>,
    _re_validator: PhantomData<HasRevalidator>,
    _required_fn: PhantomData<HasRequired>,
    _sanitizer_fn: PhantomData<HasSanitizer>,
    _ignore: PhantomData<HasIgnore>,
    _ignore_init: PhantomData<HasIgnoreInit>,
    _ignore_update: PhantomData<HasIgnoreUpdate>,
    _on_failure_fns: PhantomData<HasFailure>,
    _on_success_fns: PhantomData<HasSuccess>,
    // actual data...
    alias: Option<String>,
    validator: Option<UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    re_validator: Option<UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    required_fn: Option<RequiredResolver<I, O, CtxOptions>>,
    sanitizer: Option<VirtualSanitizer<ErasedValue, I, O, CtxOptions>>,
    ignore: Option<BooleanResolver<I, O, CtxOptions>>,
    ignore_init: Option<IsFieldProvisionEnabled<I, O, CtxOptions>>,
    ignore_update: Option<IsFieldProvisionEnabled<I, O, CtxOptions>>,
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
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasFailure,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasValidator,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasFailure,
        HasSuccess,
    >
{
    pub const fn new() -> Self {
        Self {
            alias: None,
            validator: None,
            re_validator: None,
            required_fn: None,
            sanitizer: None,
            ignore: None,
            ignore_init: None,
            ignore_update: None,
            on_failure_fns: None,
            on_success_fns: None,
            _t: PhantomData,
            _alias: PhantomData,
            _validator: PhantomData,
            _re_validator: PhantomData,
            _required_fn: PhantomData,
            _sanitizer_fn: PhantomData,
            _ignore: PhantomData,
            _ignore_init: PhantomData,
            _ignore_update: PhantomData,
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
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasFailure,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > Default
    for VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasValidator,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
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
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasFailure,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > BuildableFieldConfig<I, O, CtxOptions, ErrorTool>
    for VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasFailure,
        HasSuccess,
    >
{
    fn build(self) -> InternalFieldConfig<I, O, CtxOptions, ErrorTool> {
        FieldConfig {
            field_type: FieldType::Virtual,
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            sanitizer: self.sanitizer,
            ignore: self.ignore,
            ignore_init: self.ignore_init,
            ignore_update: self.ignore_update,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

impl<
        HasValidator,
        HasRevalidator,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > VirtualFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasValidator, No, HasRevalidator>
{
    /// this can used to mask the actual name of the virtual field from the public
    /// if set, this name must replace the actual name of the virtual field on the input struct, I
    /// i.e: I cannot have both the virtual field name and it alias
    pub fn alias(
        self,
        name: &str,
    ) -> VirtualFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasValidator, Yes> {
        VirtualFieldBuilder {
            alias: Some(name.to_string()),
            validator: self.validator,
            re_validator: self.re_validator,
            sanitizer: self.sanitizer,
            required_fn: self.required_fn,
            ignore: self.ignore,
            ignore_init: self.ignore_init,
            ignore_update: self.ignore_update,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

impl<
        HasAlias,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > VirtualFieldBuilder<T, I, O, CtxOptions, ErrorTool, No, HasAlias>
{
    pub fn validate<F>(
        self,
        validator: F,
    ) -> VirtualFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, HasAlias>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions, ErrorTool>,
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
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > VirtualFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, HasAlias>
{
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> VirtualFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, HasAlias, Yes>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions, ErrorTool>,
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
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > VirtualFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, HasAlias, HasRevalidator>
{
    pub fn required<R>(
        self,
        resolver: R,
    ) -> VirtualFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, HasAlias, HasRevalidator, No, Yes>
    where
        R: IntoRequiredResolver<I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: Some(resolver.into_resolver()),
            ..Default::default()
        }
    }
}

impl<
        HasAlias,
        HasRevalidator,
        HasRequired,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasAlias,
        HasRevalidator,
        No,
        HasRequired,
    >
{
    pub fn sanitize<F>(
        self,
        sanitizer: F,
    ) -> VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
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
            required_fn: self.required_fn,
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
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
    >
{
    pub fn ignore<R>(
        self,
        resolver: R,
    ) -> VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        Yes,
    >
    where
        R: IntoResolver<bool, I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            sanitizer: self.sanitizer,
            ignore: Some(resolver.into_resolver()),
            ..Default::default()
        }
    }

    pub fn ignore_init(
        self,
    ) -> VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
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
            required_fn: self.required_fn,
            sanitizer: self.sanitizer,
            ignore_init: Some(IsFieldProvisionEnabled::False),
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
        ErrorTool,
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
            required_fn: self.required_fn,
            sanitizer: self.sanitizer,
            ignore_update: Some(IsFieldProvisionEnabled::False),
            ..Default::default()
        }
    }
}

impl<
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
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
    pub fn ignore<R>(
        self,
        resolver: R,
    ) -> VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
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
        R: IntoBooleanResolver<I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            sanitizer: self.sanitizer,
            ignore_init: Some(IsFieldProvisionEnabled::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }
}

impl<
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
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
        ErrorTool,
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
        R: IntoResolver<bool, I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            sanitizer: self.sanitizer,
            ignore_init: self.ignore_init,
            ignore_update: Some(IsFieldProvisionEnabled::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }
}

impl<
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
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
        ErrorTool,
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
            required_fn: self.required_fn,
            sanitizer: self.sanitizer,
            ignore_update: self.ignore_update,
            ignore_init: Some(IsFieldProvisionEnabled::False),
            ..Default::default()
        }
    }

    pub fn ignore<R>(
        self,
        resolver: R,
    ) -> VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
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
        R: IntoBooleanResolver<I, O, CtxOptions>,
    {
        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            sanitizer: self.sanitizer,
            ignore_update: self.ignore_update,
            ignore_init: Some(IsFieldProvisionEnabled::Func(resolver.into_resolver())),
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
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasFailure,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
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
        ErrorTool,
        Yes,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
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
            required_fn: self.required_fn,
            ignore: self.ignore,
            ignore_init: self.ignore_init,
            ignore_update: self.ignore_update,
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
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasFailure,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
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
        ErrorTool,
        Yes,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
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
            required_fn: self.required_fn,
            ignore: self.ignore,
            ignore_init: self.ignore_init,
            ignore_update: self.ignore_update,
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
