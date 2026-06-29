use std::marker::PhantomData;

use ivo_types::IvoErrorTool;

use crate::{
    schema::{
        fields::{
            base::{BuildableFieldConfig, FieldConfig, FieldType, InternalFieldConfig},
            types::{
                ComputableRequiredError, IntoDeleteHandler, IntoFailureHandler, IntoFieldValidator,
                IntoIgnoreUpdateResolver, IntoRequiredErrorResolver, IntoSuccessHandler,
                IsFieldProvisionEnabled, UniformValidator,
            },
        },
        types::{DeleteHandler, FailureHandler, IvoFieldValue, No, SuccessHandler, Yes},
    },
    IvoStruct,
};

pub struct RequiredFieldBuilder<
    T: IvoFieldValue,
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
    HasValidator = No,
    HasRevalidator = No,
    HasRequiredError = No,
    HasIgnoreUpdate = No,
    HasDelete = No,
    HasFailure = No,
    HasSuccess = No,
> {
    _t: PhantomData<T>,
    _err: PhantomData<HasRequiredError>,
    _validator: PhantomData<HasValidator>,
    _re_validator: PhantomData<HasRevalidator>,
    _ignore_update: PhantomData<HasIgnoreUpdate>,
    _on_delete_fns: PhantomData<HasDelete>,
    _on_failure_fns: PhantomData<HasFailure>,
    _on_success_fns: PhantomData<HasSuccess>,
    // actual data...
    required_error: Option<ComputableRequiredError<I, O, CtxOptions>>,
    validator: Option<UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    re_validator: Option<UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    ignore_update: Option<IsFieldProvisionEnabled<I, O, CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_failure_fns: Option<Vec<FailureHandler<I, O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<
        HasValidator,
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasValidator,
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    pub const fn new() -> Self {
        Self {
            required_error: None,
            validator: None,
            re_validator: None,
            ignore_update: None,
            on_delete_fns: None,
            on_failure_fns: None,
            on_success_fns: None,
            _t: PhantomData,
            _err: PhantomData,
            _validator: PhantomData,
            _re_validator: PhantomData,
            _ignore_update: PhantomData,
            _on_delete_fns: PhantomData,
            _on_failure_fns: PhantomData,
            _on_success_fns: PhantomData,
        }
    }
}

impl<
        HasValidator,
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > Default
    for RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasValidator,
        HasRevalidator,
        HasRequiredError,
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
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > BuildableFieldConfig<I, O, CtxOptions, ErrorTool>
    for RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    fn build(self) -> InternalFieldConfig<I, O, CtxOptions, ErrorTool> {
        FieldConfig {
            field_type: FieldType::Required,
            required_error: self.required_error,
            validator: self.validator,
            re_validator: self.re_validator,
            ignore_update: self.ignore_update,
            on_delete_fns: self.on_delete_fns,
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
    > RequiredFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasValidator, HasRevalidator>
{
    pub fn required_error(
        self,
        error: &'static str,
    ) -> RequiredFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasValidator, HasRevalidator, Yes>
    {
        RequiredFieldBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            required_error: Some(ComputableRequiredError::Static(error)),
            ..Default::default()
        }
    }

    pub fn required_error_fn<R>(
        self,
        resolver: R,
    ) -> RequiredFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasValidator, HasRevalidator, Yes>
    where
        R: IntoRequiredErrorResolver<I, O, CtxOptions>,
    {
        RequiredFieldBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            required_error: Some(ComputableRequiredError::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }
}

impl<
        HasRequiredError,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > RequiredFieldBuilder<T, I, O, CtxOptions, ErrorTool, No, No, HasRequiredError>
{
    pub fn validate<F>(
        self,
        validator: F,
    ) -> RequiredFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, No, HasRequiredError>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions, ErrorTool>,
    {
        RequiredFieldBuilder {
            validator: Some(validator.into_uniform()),
            required_error: self.required_error,
            ..Default::default()
        }
    }
}

impl<
        HasRequiredError,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > RequiredFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, No, HasRequiredError>
{
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> RequiredFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, Yes, HasRequiredError>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions, ErrorTool>,
    {
        RequiredFieldBuilder {
            validator: self.validator,
            re_validator: Some(re_validator.into_uniform()),
            required_error: self.required_error,
            ..Default::default()
        }
    }
}

impl<
        HasRevalidator,
        HasRequiredError,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > RequiredFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, HasRevalidator, HasRequiredError>
{
    pub fn readonly(
        self,
    ) -> RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasRevalidator,
        HasRequiredError,
        Yes,
    > {
        RequiredFieldBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            required_error: self.required_error,
            ignore_update: Some(IsFieldProvisionEnabled::Readonly),
            ..Default::default()
        }
    }

    pub fn ignore_update<R>(
        self,
        resolver: R,
    ) -> RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasRevalidator,
        HasRequiredError,
        Yes,
    >
    where
        R: IntoIgnoreUpdateResolver<I, O, CtxOptions>,
    {
        RequiredFieldBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            required_error: self.required_error,
            ignore_update: Some(IsFieldProvisionEnabled::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    pub fn on_delete<H>(
        self,
        handler: H,
    ) -> RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        Yes,
        HasFailure,
        HasSuccess,
    >
    where
        H: IntoDeleteHandler<O, CtxOptions>,
    {
        let h = handler.into_handler();

        RequiredFieldBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            required_error: self.required_error,
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
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    pub fn on_failure<H>(
        self,
        handler: H,
    ) -> RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        Yes,
        HasSuccess,
    >
    where
        H: IntoFailureHandler<I, O, CtxOptions>,
    {
        let h = handler.into_handler();

        RequiredFieldBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            required_error: self.required_error,
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
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    pub fn on_success<H>(
        self,
        handler: H,
    ) -> RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        Yes,
    >
    where
        H: IntoSuccessHandler<I, O, CtxOptions>,
    {
        let h = handler.into_handler();

        RequiredFieldBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            required_error: self.required_error,
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
