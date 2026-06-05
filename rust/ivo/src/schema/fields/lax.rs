use std::{fmt::Debug, marker::PhantomData};

use crate::{
    erased_value::{erase_value, ErasedValue},
    schema::{
        error::IvoErrorTool,
        fields::base::{BuildableIvoProperty, InternalIvoProperty, IvoProperty},
    },
    traits::{
        IntoAsyncFieldValidator, IntoDeleteHandler, IntoFailureHandler, IntoFieldValidator,
        IntoRequiredResolverFn, IntoResolverWithMiniSummary, IntoResolverWithMutSummaryFn,
        IntoSuccessHandler, IvoSchemaStruct,
    },
    types::{
        BooleanResolverWithMutSummary, ComputableInit, ComputableRequired,
        ComputableWithMiniSummary, DeleteHandler, FailureHandler, FieldValidator, SuccessHandler,
    },
};

// Marker Types
pub struct Yes;
pub struct No;
pub struct YesComputed;

pub struct LaxFieldBuilder<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
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
    default: Option<ComputableWithMiniSummary<ErasedValue, CtxOptions>>,
    validator: Option<FieldValidator<I, O, CtxOptions, ErrT>>,
    re_validator: Option<FieldValidator<I, O, CtxOptions, ErrT>>,
    required: Option<ComputableRequired<I, O, CtxOptions>>,
    should_ignore_fn: Option<BooleanResolverWithMutSummary<I, O, CtxOptions>>,
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
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
            required: None,
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
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > Default
    for LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > BuildableIvoProperty<I, O, CtxOptions, ErrT>
    for LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
    fn build(self) -> InternalIvoProperty<I, O, CtxOptions, ErrT> {
        IvoProperty {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
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
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > LaxFieldBuilder<T, I, O, CtxOptions, ErrT>
{
    pub fn default(self, value: T) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrT, Yes> {
        LaxFieldBuilder {
            default: Some(ComputableWithMiniSummary::Static(erase_value(value))),
            ..Default::default()
        }
    }

    pub fn default_fn<F>(self, default_fn: F) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrT, Yes>
    where
        F: IntoResolverWithMiniSummary<T, I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: Some(ComputableWithMiniSummary::SyncFunc(
                default_fn.into_uniform(),
            )),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrT: IvoErrorTool>
    LaxFieldBuilder<T, I, O, CtxOptions, ErrT, Yes>
{
    pub fn validate<F>(self, validator: F) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions, ErrT>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: Some(FieldValidator::Sync(validator.into_uniform())),
            ..Default::default()
        }
    }

    pub fn validate_async<F>(
        self,
        validator: F,
    ) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes>
    where
        F: IntoAsyncFieldValidator<T, I, O, CtxOptions, ErrT>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: Some(FieldValidator::Async(validator.into_uniform())),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrT: IvoErrorTool>
    LaxFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes>
{
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, Yes>
    where
        F: IntoFieldValidator<T, I, O, CtxOptions, ErrT>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: Some(FieldValidator::Sync(re_validator.into_uniform())),
            ..Default::default()
        }
    }

    pub fn re_validate_async<F>(
        self,
        re_validator: F,
    ) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes>
    where
        F: IntoAsyncFieldValidator<T, I, O, CtxOptions, ErrT>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: Some(FieldValidator::Async(re_validator.into_uniform())),
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
    > LaxFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, HasRevalidator>
{
    pub fn required_if<R>(
        self,
        resolver: R,
    ) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, HasRevalidator, Yes>
    where
        R: IntoRequiredResolverFn<I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: Some(ComputableRequired::Func(resolver.into_resolver())),
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
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > LaxFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, HasValidator, HasRevalidator, HasRequired>
{
    pub fn ignore_if<R>(
        self,
        resolver: R,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        Yes,
    >
    where
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
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
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > LaxFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, HasValidator, HasRevalidator, HasRequired>
{
    pub fn ignore_init(
        self,
    ) -> LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
            required: self.required,
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
        ErrT,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        YesComputed,
    >
    where
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
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
        ErrT,
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
            required: self.required,
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
        ErrT,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        No,
        YesComputed,
    >
    where
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
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
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        ErrT,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        YesComputed,
        Yes,
    >
    where
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
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
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        ErrT,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        Yes,
        YesComputed,
    >
    where
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
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
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        ErrT,
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
            required: self.required,

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
        ErrT,
        Yes,
        HasValidator,
        HasRevalidator,
        HasRequired,
        No,
        YesComputed,
        YesComputed,
    >
    where
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
    {
        LaxFieldBuilder {
            default: self.default,
            validator: self.validator,
            re_validator: self.re_validator,
            required: self.required,
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
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        ErrT,
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
            required: self.required,
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
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        ErrT,
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
            required: self.required,
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
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    >
    LaxFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
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
        ErrT,
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
            required: self.required,
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
