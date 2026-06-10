use std::marker::PhantomData;

use crate::{
    schema::{
        error::IvoErrorTool,
        fields::{
            base::{BuildableFieldConfig, FieldConfig, InternalFieldConfig},
            types::{
                ComputableInit, ComputableRequired, ComputableRequiredError, IntoDeleteHandler,
                IntoFailureHandler, IntoFieldValidator, IntoRequiredErrorResolver, IntoResolver,
                IntoSuccessHandler, UniformValidator,
            },
        },
    },
    types::{DeleteHandler, FailureHandler, No, SuccessHandler, True, Yes},
    IvoSchemaStruct,
};

pub struct RequiredFieldBuilder<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
    HasValidator = No,
    HasRevalidator = No,
    HasRequiredError = No,
    HasShouldUpdate = No,
    HasDelete = No,
    HasFailure = No,
    HasSuccess = No,
> {
    _t: PhantomData<T>,
    _err: PhantomData<HasRequiredError>,
    _validator: PhantomData<HasValidator>,
    _re_validator: PhantomData<HasRevalidator>,
    _should_update: PhantomData<HasShouldUpdate>,
    _on_delete_fns: PhantomData<HasDelete>,
    _on_failure_fns: PhantomData<HasFailure>,
    _on_success_fns: PhantomData<HasSuccess>,
    // actual data...
    required_error: Option<ComputableRequiredError<I, O, CtxOptions>>,
    validator: Option<UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    re_validator: Option<UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    should_update: Option<ComputableInit<I, O, CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_failure_fns: Option<Vec<FailureHandler<I, O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<
        HasValidator,
        HasRevalidator,
        HasRequiredError,
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
    RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasValidator,
        HasRevalidator,
        HasRequiredError,
        HasShouldUpdate,
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
            should_update: None,
            on_delete_fns: None,
            on_failure_fns: None,
            on_success_fns: None,
            _t: PhantomData,
            _err: PhantomData,
            _validator: PhantomData,
            _re_validator: PhantomData,
            _should_update: PhantomData,
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
    for RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasValidator,
        HasRevalidator,
        HasRequiredError,
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
        HasRevalidator,
        HasRequiredError,
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
    for RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasRevalidator,
        HasRequiredError,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    fn build(self) -> InternalFieldConfig<I, O, CtxOptions, ErrorTool> {
        FieldConfig {
            required_error: self.required_error,
            validator: self.validator,
            re_validator: self.re_validator,
            required: Some(ComputableRequired::Static(True)),
            should_update: self.should_update,
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
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
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
            required_error: Some(ComputableRequiredError::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }
}

impl<
        HasRequiredError,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
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
            ..Default::default()
        }
    }
}

impl<
        HasRequiredError,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
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
            ..Default::default()
        }
    }
}

impl<
        HasRevalidator,
        HasRequiredError,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
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
            should_update: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_update_if<R>(
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
        R: IntoResolver<bool, I, O, CtxOptions>,
    {
        RequiredFieldBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            should_update: Some(ComputableInit::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<
        HasRevalidator,
        HasRequiredError,
        HasShouldUpdate,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
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
        HasShouldUpdate,
        No,
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
        HasShouldUpdate,
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
        HasRequiredError,
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
    RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasRevalidator,
        HasRequiredError,
        HasShouldUpdate,
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
        HasShouldUpdate,
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
        HasRevalidator,
        HasRequiredError,
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
    RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasRevalidator,
        HasRequiredError,
        HasShouldUpdate,
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
        HasShouldUpdate,
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
