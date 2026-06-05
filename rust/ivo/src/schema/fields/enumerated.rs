use std::{fmt::Debug, marker::PhantomData};

use crate::{
    schema::{
        error::IvoErrorTool,
        fields::base::{BuildableIvoProperty, InternalIvoProperty, IvoProperty},
    },
    traits::{
        IntoAsyncResolverWithMiniSummary, IntoDeleteHandler, IntoEnumErrorResolver,
        IntoFailureHandler, IntoResolverWithMiniSummary, IntoResolverWithMutSummaryFn,
        IntoSuccessHandler, IvoSchemaStruct,
    },
    types::{
        BooleanResolverWithMutSummary, ComputableEnumeratedError, ComputableInit,
        ComputableWithMiniSummary, DeleteHandler, FailureHandler, IvoSummary, SuccessHandler,
    },
    utils::erased_value::{erase_value, ErasedValue},
};

// Marker Types
pub struct Yes;
pub struct No;
pub struct YesComputed;

pub struct EnumFieldBuilder<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
    HasValues = No,
    HasValueError = No,
    HasDefault = No,
    HasIgnore = No,
    HasShouldInit = No,
    HasShouldUpdate = No,
    HasDelete = No,
    HasFailure = No,
    HasSuccess = No,
> {
    _t: PhantomData<T>,
    _enum_values: PhantomData<HasValues>,
    _enum_error: PhantomData<HasValueError>,
    _default: PhantomData<HasDefault>,
    _should_ignore: PhantomData<HasIgnore>,
    _should_init: PhantomData<HasShouldInit>,
    _should_update: PhantomData<HasShouldUpdate>,
    _on_delete_fns: PhantomData<HasDelete>,
    _on_failure_fns: PhantomData<HasFailure>,
    _on_success_fns: PhantomData<HasSuccess>,
    // actual data...
    enum_values: Option<Vec<ErasedValue>>,
    enum_error: Option<ComputableEnumeratedError<ErrT>>,
    default: Option<ComputableWithMiniSummary<ErasedValue, CtxOptions>>,
    should_ignore_fn: Option<BooleanResolverWithMutSummary<I, O, CtxOptions>>,
    should_init: Option<ComputableInit<I, O, CtxOptions>>,
    should_update: Option<ComputableInit<I, O, CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_failure_fns: Option<Vec<FailureHandler<I, O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<
        HasValues,
        HasValueError,
        HasDefault,
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
    EnumFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        HasValues,
        HasValueError,
        HasDefault,
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
            enum_values: None,
            enum_error: None,
            should_ignore_fn: None,
            should_init: None,
            should_update: None,
            on_delete_fns: None,
            on_failure_fns: None,
            on_success_fns: None,
            _t: PhantomData,
            _default: PhantomData,
            _enum_values: PhantomData,
            _enum_error: PhantomData,
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
        HasValues,
        HasValueError,
        HasDefault,
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
    for EnumFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        HasValues,
        HasValueError,
        HasDefault,
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
    for EnumFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        Yes,
        HasDefault,
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
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            default: self.default,
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
    > EnumFieldBuilder<T, I, O, CtxOptions, ErrT>
{
    pub fn values(self, values: Vec<T>) -> EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes> {
        EnumFieldBuilder {
            enum_values: Some(values.into_iter().map(|v| erase_value(v)).collect()),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrT: IvoErrorTool>
    EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes>
{
    pub fn error(self, error: &str) -> EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes> {
        EnumFieldBuilder {
            enum_values: self.enum_values,
            enum_error: Some(ComputableEnumeratedError::Static(error.into())),
            ..Default::default()
        }
    }

    pub fn error_fn<F>(self, error_fn: F) -> EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes>
    where
        F: IntoEnumErrorResolver<T, ErrT>,
    {
        EnumFieldBuilder {
            enum_values: self.enum_values,
            enum_error: Some(ComputableEnumeratedError::Func(error_fn.into_uniform())),
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
    > EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes>
{
    pub fn default(self, value: T) -> EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, Yes> {
        EnumFieldBuilder {
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            default: Some(ComputableWithMiniSummary::Static(erase_value(value))),
            ..Default::default()
        }
    }

    pub fn default_fn<F>(
        self,
        default_fn: F,
    ) -> EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, Yes>
    where
        F: IntoResolverWithMiniSummary<T, I, O, CtxOptions>,
    {
        EnumFieldBuilder {
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            default: Some(ComputableWithMiniSummary::SyncFunc(
                default_fn.into_uniform(),
            )),
            ..Default::default()
        }
    }

    pub fn default_async_fn<F>(
        self,
        default_fn: F,
    ) -> EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, Yes>
    where
        F: IntoAsyncResolverWithMiniSummary<I, I, O, CtxOptions>,
    {
        EnumFieldBuilder {
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            default: Some(ComputableWithMiniSummary::AsyncFunc(
                default_fn.into_uniform(),
            )),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrT: IvoErrorTool>
    EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, Yes>
{
    pub fn ignore_if<F>(
        self,
        fx: F,
    ) -> EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, Yes, Yes>
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        EnumFieldBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_ignore_fn: Some(Box::new(fx)),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrT: IvoErrorTool>
    EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, Yes>
{
    pub fn ignore_init(
        self,
    ) -> EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, Yes, No, Yes> {
        EnumFieldBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_init: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_init_if<R>(
        self,
        resolver: R,
    ) -> EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, Yes, No, YesComputed>
    where
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
    {
        EnumFieldBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_init: Some(ComputableInit::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }
}

impl<
        HasDefault,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, HasDefault>
{
    pub fn readonly(self) -> EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, HasDefault> {
        EnumFieldBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_update: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_update_if<R>(
        self,
        resolver: R,
    ) -> EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, HasDefault, No, No, YesComputed>
    where
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
    {
        EnumFieldBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_update: Some(ComputableInit::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrT: IvoErrorTool>
    EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, Yes, No, No, Yes>
{
    pub fn allow_init_if<R>(
        self,
        resolver: R,
    ) -> EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, Yes, No, YesComputed, Yes>
    where
        R: IntoResolverWithMutSummaryFn<bool, I, O, CtxOptions>,
    {
        EnumFieldBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_init: Some(ComputableInit::Func(resolver.into_resolver())),
            ..Default::default()
        }
    }
}

impl<
        HasDefault,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, HasDefault, No, Yes, YesComputed>
{
    pub fn allow_update_if<F>(
        self,
        fx: F,
    ) -> EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, HasDefault, No, Yes, YesComputed>
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        EnumFieldBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_init: self.should_init,
            should_update: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrT: IvoErrorTool>
    EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, Yes, No, No, YesComputed>
{
    pub fn ignore_init(
        self,
    ) -> EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, Yes, No, Yes, YesComputed> {
        EnumFieldBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_update: self.should_update,
            should_init: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_init_if<F>(
        self,
        fx: F,
    ) -> EnumFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, Yes, No, YesComputed, YesComputed>
    where
        F: Fn(IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
    {
        EnumFieldBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
            should_update: self.should_update,
            should_init: Some(ComputableInit::Func(Box::new(fx))),
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<
        HasDefault,
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
    EnumFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        Yes,
        HasDefault,
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
    ) -> EnumFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        Yes,
        HasDefault,
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

        EnumFieldBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
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
        HasDefault,
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
    EnumFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        Yes,
        HasDefault,
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
    ) -> EnumFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        Yes,
        HasDefault,
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

        EnumFieldBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
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
        HasDefault,
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
    EnumFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        Yes,
        HasDefault,
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
    ) -> EnumFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        Yes,
        HasDefault,
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

        EnumFieldBuilder {
            default: self.default,
            enum_values: self.enum_values,
            enum_error: self.enum_error,
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
