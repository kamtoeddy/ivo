use std::marker::PhantomData;

use crate::{
    schema::{
        error::IvoErrorTool,
        fields::{
            base::{BuildableFieldConfig, FieldConfig, InternalFieldConfig},
            types::{
                ComputableInit, ComputableRequired, ComputableRequiredError, IntoDeleteHandler,
                IntoFailureHandler, IntoFieldValidator, IntoRequiredResolverFn,
                IntoResolverWithMutSummaryFn, IntoSuccessHandler, UniformValidator,
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
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
    HasRequiredError = No,
    HasValidator = No,
    HasRevalidator = No,
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
    validator: Option<UniformValidator<I, O, CtxOptions, ErrT::FieldMetadata>>,
    re_validator: Option<UniformValidator<I, O, CtxOptions, ErrT::FieldMetadata>>,
    should_update: Option<ComputableInit<I, O, CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_failure_fns: Option<Vec<FailureHandler<I, O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<
        HasRequiredError,
        HasValidator,
        HasRevalidator,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    >
    RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        HasRequiredError,
        HasValidator,
        HasRevalidator,
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
        HasRequiredError,
        HasValidator,
        HasRevalidator,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > Default
    for RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        HasRequiredError,
        HasValidator,
        HasRevalidator,
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
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > BuildableFieldConfig<I, O, CtxOptions, ErrT>
    for RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        Yes,
        HasRevalidator,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    fn build(self) -> InternalFieldConfig<I, O, CtxOptions, ErrT> {
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

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrT: IvoErrorTool>
    RequiredFieldBuilder<T, I, O, CtxOptions, ErrT>
{
    pub fn error(
        self,
        error: &'static str,
    ) -> RequiredFieldBuilder<T, I, O, CtxOptions, ErrT, Yes> {
        RequiredFieldBuilder {
            required_error: Some(ComputableRequiredError::Static(error)),
            ..Default::default()
        }
    }

    pub fn error_fn<R>(self, resolver: R) -> RequiredFieldBuilder<T, I, O, CtxOptions, ErrT, Yes>
    where
        R: IntoRequiredResolverFn<I, O, CtxOptions>,
    {
        RequiredFieldBuilder {
            required_error: Some(ComputableRequiredError::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrT: IvoErrorTool>
    RequiredFieldBuilder<T, I, O, CtxOptions, ErrT, Yes>
{
    pub fn validate<F>(
        self,
        validator: F,
    ) -> RequiredFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions, ErrT>,
    {
        RequiredFieldBuilder {
            validator: Some(validator.into_uniform()),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrT: IvoErrorTool>
    RequiredFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes>
{
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> RequiredFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, Yes>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions, ErrT>,
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
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > RequiredFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, HasRevalidator>
{
    pub fn readonly(
        self,
    ) -> RequiredFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, HasRevalidator, Yes> {
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
    ) -> RequiredFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, HasRevalidator, Yes>
    where
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
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
        HasShouldUpdate,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    >
    RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        HasRevalidator,
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
        ErrT,
        Yes,
        HasRevalidator,
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
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    >
    RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        HasRevalidator,
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
        ErrT,
        Yes,
        HasRevalidator,
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
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    >
    RequiredFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        HasRevalidator,
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
        ErrT,
        Yes,
        HasRevalidator,
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
