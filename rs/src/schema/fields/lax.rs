#![expect(clippy::type_complexity)]

use std::marker::PhantomData;

use crate::{
    schema::{
        fields::{
            base::{BuildableFieldConfig, FieldConfig, FieldType, InternalFieldConfig},
            types::{
                BooleanResolver, IntoBooleanResolver, IntoDeleteHandler, IntoFailureHandler,
                IntoFieldValidator, IntoRequiredResolver, IntoSuccessHandler,
                IntoValueResolverWithSharedInput, IsFieldProvisionEnabled, RequiredResolver,
                UniformValidator, ValueResolverWithSharedInput,
            },
        },
        types::{
            DeleteHandler, FailureHandler, FieldValue, IsProvided, IsProvidedButNotComputed, No,
            SuccessHandler, Yes, YesComputed,
        },
    },
    types::internal::{
        types::{erase_value, ErasedValue},
        IvoErrorTool,
    },
    IvoStruct,
};

pub struct LaxFieldBuilder<
    T: FieldValue,
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
    HasDefault = No,
    HasValidator = No,
    HasRevalidator = No,
    HasRequired = No,
    HasIgnore = No,
    HasIgnoreInit = No,
    HasIgnoreUpdate = No,
    HasDelete = No,
    HasFailure = No,
    HasSuccess = No,
> {
    _t: PhantomData<T>,
    _default: PhantomData<HasDefault>,
    _validator: PhantomData<HasValidator>,
    _re_validator: PhantomData<HasRevalidator>,
    _required_fn: PhantomData<HasRequired>,
    _ignore: PhantomData<HasIgnore>,
    _ignore_init: PhantomData<HasIgnoreInit>,
    _ignore_update: PhantomData<HasIgnoreUpdate>,
    _on_delete_fns: PhantomData<HasDelete>,
    _on_failure_fns: PhantomData<HasFailure>,
    _on_success_fns: PhantomData<HasSuccess>,
    // actual data...
    default: Option<ValueResolverWithSharedInput<ErasedValue, I, CtxOptions>>,
    validator: Option<UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    re_validator: Option<UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    required_fn: Option<RequiredResolver<I, O, CtxOptions>>,
    should_ignore: Option<BooleanResolver<I, O, CtxOptions>>,
    ignore_init: Option<IsFieldProvisionEnabled<I, O, CtxOptions>>,
    ignore_update: Option<IsFieldProvisionEnabled<I, O, CtxOptions>>,
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
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    pub const fn new() -> Self {
        Self {
            default: None,
            validator: None,
            re_validator: None,
            required_fn: None,
            should_ignore: None,
            ignore_init: None,
            ignore_update: None,
            on_delete_fns: None,
            on_failure_fns: None,
            on_success_fns: None,
            _t: PhantomData,
            _default: PhantomData,
            _validator: PhantomData,
            _re_validator: PhantomData,
            _required_fn: PhantomData,
            _ignore: PhantomData,
            _ignore_init: PhantomData,
            _ignore_update: PhantomData,
            _on_delete_fns: PhantomData,
            _on_failure_fns: PhantomData,
            _on_success_fns: PhantomData,
        }
    }
}

impl<
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > Default
    for LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    fn default() -> Self {
        Self::new()
    }
}

impl<
        HasDefault: IsProvided,
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > BuildableFieldConfig<I, O, CtxOptions, ErrorTool>
    for LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    fn build(self) -> InternalFieldConfig<I, O, CtxOptions, ErrorTool> {
        FieldConfig {
            field_type: FieldType::Lax,
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            ignore: self.should_ignore,
            ignore_init: self.ignore_init,
            ignore_update: self.ignore_update,
            on_delete_fns: self.on_delete_fns,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

impl<T: FieldValue, I: IvoStruct, O: IvoStruct, CtxOptions, ErrorTool: IvoErrorTool>
    LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool>
{
    pub fn default(self, value: T) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes> {
        LaxFieldBuilder {
            default: Some(ValueResolverWithSharedInput::Static(erase_value(value))),
            ..Default::default()
        }
    }

    pub fn default_fn<F>(
        self,
        default_fn: F,
    ) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, YesComputed>
    where
        F: IntoValueResolverWithSharedInput<T, I, CtxOptions>,
    {
        LaxFieldBuilder {
            default: Some(ValueResolverWithSharedInput::Func(
                default_fn.into_uniform(),
            )),
            ..Default::default()
        }
    }
}

impl<
        HasDefault: IsProvided,
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasDefault>
{
    pub fn validate<F>(
        self,
        validator: F,
    ) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasDefault, Yes>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions, ErrorTool>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: Some(validator.into_uniform()),
            ..Default::default()
        }
    }
}

impl<
        HasDefault: IsProvided,
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasDefault, Yes>
{
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasDefault, Yes>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions, ErrorTool>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: Some(re_validator.into_uniform()),
            ..Default::default()
        }
    }
}

impl<
        HasDefault: IsProvided,
        HasRevalidator,
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasDefault, Yes, HasRevalidator>
{
    pub fn required<R>(
        self,
        resolver: R,
    ) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasDefault, Yes, HasRevalidator, Yes>
    where
        R: IntoRequiredResolver<I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: Some(resolver.into_resolver()),
            ..Default::default()
        }
    }
}

impl<
        HasDefault: IsProvided,
        HasValidator,
        HasRevalidator,
        HasRequired,
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
    >
{
    pub fn ignore<R>(
        self,
        resolver: R,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
        Yes,
    >
    where
        R: IntoBooleanResolver<I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            should_ignore: Some(resolver.into_resolver()),
            ..Default::default()
        }
    }

    pub fn ignore_init(
        self,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        Yes,
    > {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            ignore_init: Some(IsFieldProvisionEnabled::False),
            ..Default::default()
        }
    }

    pub fn ignore_update(
        self,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        No,
        Yes,
    > {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            ignore_update: Some(IsFieldProvisionEnabled::False),
            ..Default::default()
        }
    }
}

impl<
        HasDefault: IsProvidedButNotComputed,
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
    >
{
    /// During updates, the current value of the field is compared with it's
    /// default value. If both values are equal, updates will be allowed.
    pub fn readonly(
        self,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        Yes,
    > {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            should_ignore: self.should_ignore,
            ignore_init: self.ignore_init,
            ignore_update: Some(IsFieldProvisionEnabled::Readonly),
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<
        HasDefault: IsProvided,
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    pub fn on_delete<H>(
        self,
        handler: H,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        Yes,
        HasFailure,
        HasSuccess,
    >
    where
        H: IntoDeleteHandler<O, CtxOptions>,
    {
        let h = handler.into_handler();

        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            should_ignore: self.should_ignore,
            ignore_init: self.ignore_init,
            ignore_update: self.ignore_update,
            on_delete_fns: Some(match self.on_delete_fns {
                Some(hs) => {
                    let mut v = hs;

                    v.push(h);

                    v
                }
                _ => vec![h],
            }),
            on_failure_fns: self.on_failure_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

// ON_FAILURE is only available if HasFailure is 'No'
impl<
        HasDefault: IsProvided,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        Yes,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    pub fn on_failure<H>(
        self,
        handler: H,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        Yes,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasDelete,
        Yes,
        HasSuccess,
    >
    where
        H: IntoFailureHandler<I, O, CtxOptions>,
    {
        let h = handler.into_handler();

        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            should_ignore: self.should_ignore,
            ignore_init: self.ignore_init,
            ignore_update: self.ignore_update,
            on_delete_fns: self.on_delete_fns,
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
        HasDefault: IsProvided,
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    pub fn on_success<H>(
        self,
        handler: H,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        Yes,
    >
    where
        H: IntoSuccessHandler<I, O, CtxOptions>,
    {
        let h = handler.into_handler();

        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            should_ignore: self.should_ignore,
            ignore_init: self.ignore_init,
            ignore_update: self.ignore_update,
            on_delete_fns: self.on_delete_fns,
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
