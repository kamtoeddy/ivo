use std::{future::Future, marker::PhantomData};

use crate::{
    __private_types::types::BooleanResolver,
    schema::{
        fields::{
            base::{BuildableFieldConfig, FieldConfig, FieldType, InternalFieldConfig},
            types::{
                ComputableRequiredError, IntoDeleteHandler, IntoFailureHandler, IntoFieldValidator,
                IntoInitRequiredErrorResolver, IntoSuccessHandler, IsFieldProvisionEnabled,
                UniformValidator,
            },
        },
        types::{DeleteHandler, FailureHandler, FieldValue, No, SuccessHandler, Yes},
    },
    types::internal::IvoErrorSanitizer,
    IvoRwCtxOptions, IvoStruct,
};

pub struct RequiredFieldBuilder<
    T: FieldValue,
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    HasValidator = No,
    HasRevalidator = No,
    HasRequiredError = No,
    HasIgnoreUpdate = No,
    HasDelete = No,
    HasFailure = No,
    HasSuccess = No,
> {
    name: &'static str,
    required_error: Option<ComputableRequiredError<I, CtxOptions>>,
    validator: Option<UniformValidator<I, O, CtxOptions, ErrorSanitizer::Metadata>>,
    re_validator: Option<UniformValidator<I, O, CtxOptions, ErrorSanitizer::Metadata>>,
    ignore_update: Option<IsFieldProvisionEnabled<I, O, CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_failure_fns: Option<Vec<FailureHandler<I, O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
    // markers...
    _t: PhantomData<T>,
    _err: PhantomData<HasRequiredError>,
    _validator: PhantomData<HasValidator>,
    _re_validator: PhantomData<HasRevalidator>,
    _ignore_update: PhantomData<HasIgnoreUpdate>,
    _on_delete_fns: PhantomData<HasDelete>,
    _on_failure_fns: PhantomData<HasFailure>,
    _on_success_fns: PhantomData<HasSuccess>,
}

impl<
        HasValidator,
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    >
    RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
        HasValidator,
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
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
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    > Default
    for RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
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
        Self::new("")
    }
}

impl<
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    > BuildableFieldConfig<I, O, CtxOptions, ErrorSanitizer>
    for RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
        Yes,
        HasRevalidator,
        HasRequiredError,
        HasIgnoreUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    fn build(self) -> InternalFieldConfig<I, O, CtxOptions, ErrorSanitizer> {
        FieldConfig {
            name: self.name,
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
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    > RequiredFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, HasValidator, HasRevalidator>
{
    pub fn required_error(
        self,
        error: &'static str,
    ) -> RequiredFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, HasValidator, HasRevalidator, Yes>
    {
        RequiredFieldBuilder {
            name: self.name,
            validator: self.validator,
            re_validator: self.re_validator,
            required_error: Some(ComputableRequiredError::Static(error)),
            ..Default::default()
        }
    }

    pub fn required_error_fn<R>(
        self,
        resolver: R,
    ) -> RequiredFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, HasValidator, HasRevalidator, Yes>
    where
        R: IntoInitRequiredErrorResolver<I, O, CtxOptions>,
    {
        RequiredFieldBuilder {
            name: self.name,
            validator: self.validator,
            re_validator: self.re_validator,
            required_error: Some(ComputableRequiredError::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }
}

impl<
        HasRequiredError,
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    > RequiredFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, No, No, HasRequiredError>
{
    pub fn validate<F>(
        self,
        validator: F,
    ) -> RequiredFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, Yes, No, HasRequiredError>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions, ErrorSanitizer>,
    {
        RequiredFieldBuilder {
            name: self.name,
            validator: Some(validator.into_uniform()),
            required_error: self.required_error,
            ..Default::default()
        }
    }
}

impl<
        HasRequiredError,
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    > RequiredFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, Yes, No, HasRequiredError>
{
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> RequiredFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, Yes, Yes, HasRequiredError>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions, ErrorSanitizer>,
    {
        RequiredFieldBuilder {
            name: self.name,
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
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    >
    RequiredFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, Yes, HasRevalidator, HasRequiredError>
{
    pub fn readonly(
        self,
    ) -> RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
        Yes,
        HasRevalidator,
        HasRequiredError,
        Yes,
    > {
        RequiredFieldBuilder {
            name: self.name,
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
        ErrorSanitizer,
        Yes,
        HasRevalidator,
        HasRequiredError,
        Yes,
    >
    where
        R: IntoIgnoreUpdateResolver<I, O, CtxOptions>,
    {
        RequiredFieldBuilder {
            name: self.name,
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
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    >
    RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
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
        ErrorSanitizer,
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
            name: self.name,
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
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    >
    RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
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
        ErrorSanitizer,
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
            name: self.name,
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
        T: FieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    >
    RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
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
        ErrorSanitizer,
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
            name: self.name,
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

pub trait IntoIgnoreUpdateResolver<I: IvoStruct, O: IvoStruct, CtxOptions> {
    fn into_resolver(self) -> BooleanResolver<I, O, CtxOptions>;
}

impl<F, Fut, I: IvoStruct, O: IvoStruct, CtxOptions> IntoIgnoreUpdateResolver<I, O, CtxOptions>
    for F
where
    F: Fn(I::Partial, O, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = bool> + Send + 'static,
{
    fn into_resolver(self) -> BooleanResolver<I, O, CtxOptions> {
        Box::new(move |ctx, o| Box::pin(self(ctx.input(), ctx.full_values().unwrap(), o)))
    }
}
