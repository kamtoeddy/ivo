use std::{fmt::Debug, marker::PhantomData};

use crate::{
    schema::{
        error::IvoErrorTool,
        fields::{
            base::{BuildableFieldConfig, FieldConfig, FieldType, InternalFieldConfig},
            types::{
                BooleanResolver, ComputableInit, ComputableWithMiniContext, IntoDeleteHandler,
                IntoFailureHandler, IntoFieldValidator, IntoRequiredResolver, IntoResolver,
                IntoResolverWithMiniContext, IntoSuccessHandler, RequiredResolver,
                UniformValidator,
            },
        },
    },
    types::{
        erase_value, DeleteHandler, ErasedValue, FailureHandler, No, SuccessHandler, Yes,
        YesComputed,
    },
    IvoSchemaStruct,
};

pub struct LaxFieldBuilder<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
    HasDefault = No,
    HasValidator = No,
    HasRevalidator = No,
    HasRequired = No,
    HasIgnore = No,
    HasShouldInit = No,
    HasShouldUpdate = No,
    HasDelete = No,
    HasFailure = No,
    HasSuccess = No,
> {
    _t: PhantomData<T>,
    _default: PhantomData<HasDefault>,
    _validator: PhantomData<HasValidator>,
    _re_validator: PhantomData<HasRevalidator>,
    _required_fn: PhantomData<HasRequired>,
    _should_ignore: PhantomData<HasIgnore>,
    _should_init: PhantomData<HasShouldInit>,
    _should_update: PhantomData<HasShouldUpdate>,
    _on_delete_fns: PhantomData<HasDelete>,
    _on_failure_fns: PhantomData<HasFailure>,
    _on_success_fns: PhantomData<HasSuccess>,
    // actual data...
    default: Option<ComputableWithMiniContext<ErasedValue, I, CtxOptions>>,
    validator: Option<UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    re_validator: Option<UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    required_fn: Option<RequiredResolver<I, O, CtxOptions>>,
    should_ignore_fn: Option<BooleanResolver<I, O, CtxOptions>>,
    should_init: Option<ComputableInit<I, O, CtxOptions>>,
    should_update: Option<ComputableInit<I, O, CtxOptions>>,
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
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
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
        HasShouldInit,
        HasShouldUpdate,
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
            should_ignore_fn: None,
            should_init: None,
            should_update: None,
            on_delete_fns: None,
            on_failure_fns: None,
            on_success_fns: None,
            _t: PhantomData,
            _default: PhantomData,
            _validator: PhantomData,
            _re_validator: PhantomData,
            _required_fn: PhantomData,
            _should_ignore: PhantomData,
            _should_init: PhantomData,
            _should_update: PhantomData,
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
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
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
        HasShouldInit,
        HasShouldUpdate,
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
        HasDefault,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > BuildableFieldConfig<I, O, CtxOptions, ErrorTool>
    for LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
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
    fn build(self) -> InternalFieldConfig<I, O, CtxOptions, ErrorTool> {
        FieldConfig {
            field_type: FieldType::Lax,
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
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

impl<
        T: Clone + Debug + Send + Sync + 'static,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool>
{
    pub fn default(self, value: T) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes> {
        LaxFieldBuilder {
            default: Some(ComputableWithMiniContext::Static(erase_value(value))),
            ..Default::default()
        }
    }

    pub fn default_fn<F>(
        self,
        default_fn: F,
    ) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes>
    where
        F: IntoResolverWithMiniContext<T, I, CtxOptions>,
    {
        LaxFieldBuilder {
            default: Some(ComputableWithMiniContext::Func(default_fn.into_uniform())),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions, ErrorTool: IvoErrorTool>
    LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes>
{
    pub fn validate<F>(
        self,
        validator: F,
    ) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, Yes>
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

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions, ErrorTool: IvoErrorTool>
    LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, Yes>
{
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, Yes>
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
        HasRevalidator,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, Yes, HasRevalidator>
{
    pub fn required_if<R>(
        self,
        resolver: R,
    ) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, Yes, HasRevalidator, Yes>
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
        HasValidator,
        HasRevalidator,
        HasRequired,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, HasValidator, HasRevalidator, HasRequired>
{
    pub fn ignore_if<R>(
        self,
        resolver: R,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        Yes,
    >
    where
        R: IntoResolver<bool, I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            should_ignore_fn: Some(resolver.into_resolver()),
            ..Default::default()
        }
    }
}

impl<
        HasValidator,
        HasRevalidator,
        HasRequired,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, HasValidator, HasRevalidator, HasRequired>
{
    pub fn ignore_init(
        self,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
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
            should_init: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_init_if<R>(
        self,
        resolver: R,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        YesComputed,
    >
    where
        R: IntoResolver<bool, I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            should_init: Some(ComputableInit::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }

    pub fn readonly(
        self,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
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
            should_update: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_update_if<R>(
        self,
        resolver: R,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        No,
        YesComputed,
    >
    where
        R: IntoResolver<bool, I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            should_update: Some(ComputableInit::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }
}

impl<
        HasValidator,
        HasRevalidator,
        HasRequired,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        No,
        Yes,
    >
{
    pub fn allow_init_if<R>(
        self,
        resolver: R,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        YesComputed,
        Yes,
    >
    where
        R: IntoResolver<bool, I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            should_init: Some(ComputableInit::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }
}

impl<
        HasValidator,
        HasRevalidator,
        HasRequired,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        Yes,
        YesComputed,
    >
{
    pub fn allow_update_if<R>(
        self,
        resolver: R,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        Yes,
        YesComputed,
    >
    where
        R: IntoResolver<bool, I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            should_init: self.should_init,
            should_update: Some(ComputableInit::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }
}

impl<
        HasValidator,
        HasRevalidator,
        HasRequired,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        No,
        YesComputed,
    >
{
    pub fn ignore_init(
        self,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        Yes,
        YesComputed,
    > {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,

            should_update: self.should_update,
            should_init: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_init_if<R>(
        self,
        resolver: R,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        YesComputed,
        YesComputed,
    >
    where
        R: IntoResolver<bool, I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required_fn: self.required_fn,
            should_update: self.should_update,
            should_init: Some(ComputableInit::Func(resolver.into_resolver())),
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
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
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
    pub fn on_delete<H>(
        self,
        handler: H,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
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
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
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
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        Yes,
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
    pub fn on_failure<H>(
        self,
        handler: H,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
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
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
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
        HasValidator,
        HasRevalidator,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
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
    pub fn on_success<H>(
        self,
        handler: H,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
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
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
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
