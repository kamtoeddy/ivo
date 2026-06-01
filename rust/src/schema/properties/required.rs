use std::{future::Future, marker::PhantomData};

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    schema::properties::base::{InternalIvoProperty, IvoProperty, IvoPropertyBuilder},
    traits::{
        IntoAsyncFieldReValidator, IntoAsyncFieldValidator, IntoFieldReValidator,
        IntoFieldValidator, IvoSchemaStruct,
    },
    types::{
        ComputableInit, ComputableRequired, DeleteHandler, FailureHandler, FieldReValidator,
        FieldValidator, IvoSummary, SuccessHandler, True,
    },
};

pub struct RequiredField;

// Marker Types
pub struct Yes;
pub struct No;

pub struct SchemaBuilder<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    HasValidator,
    HasRevalidator,
    HasShouldUpdate,
    HasDelete,
    HasFailure,
    HasSuccess,
> {
    _d: PhantomData<T>,
    _i: PhantomData<I>,
    _validator: PhantomData<HasValidator>,
    _re_validator: PhantomData<HasRevalidator>,
    _should_update: PhantomData<HasShouldUpdate>,
    _on_delete_fns: PhantomData<HasDelete>,
    _on_failure_fns: PhantomData<HasFailure>,
    _on_success_fns: PhantomData<HasSuccess>,
    // actual data...
    validator: Option<FieldValidator<CtxOptions>>,
    re_validator: Option<FieldReValidator<CtxOptions>>,
    should_update: Option<ComputableInit<CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_failure_fns: Option<Vec<FailureHandler<CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<CtxOptions>>>,
}

impl<
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
    > Default
    for SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        HasValidator,
        HasRevalidator,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    fn default() -> Self {
        Self {
            validator: None,
            re_validator: None,
            should_update: None,
            on_delete_fns: None,
            on_failure_fns: None,
            on_success_fns: None,
            _d: PhantomData,
            _i: PhantomData,
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
        HasRevalidator,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    > IvoPropertyBuilder<I, O, CtxOptions>
    for SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasRevalidator,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    fn build(self) -> InternalIvoProperty<I, O, CtxOptions> {
        IvoProperty {
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

impl RequiredField {
    pub fn validate<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, F>(
        validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No>
    where
        F: IntoFieldValidator<T, CtxOptions>,
    {
        SchemaBuilder {
            validator: Some(FieldValidator::Sync(validator.into_uniform())),
            ..Default::default()
        }
    }

    pub fn validate_async<
        F,
        T: Serialize,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    >(
        validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No>
    where
        F: IntoAsyncFieldValidator<T, CtxOptions>,
    {
        SchemaBuilder {
            validator: Some(FieldValidator::Async(validator.into_uniform())),
            ..Default::default()
        }
    }
}

impl<
        T: DeserializeOwned + Serialize,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
    > SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No>
{
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, No, No, No, No>
    where
        F: IntoFieldReValidator<T, CtxOptions>,
    {
        SchemaBuilder {
            validator: self.validator,
            re_validator: Some(FieldReValidator::Sync(re_validator.into_uniform())),
            ..Default::default()
        }
    }

    pub fn validate_async<F, Fut>(
        self,
        re_validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, No>
    where
        F: IntoAsyncFieldReValidator<T, CtxOptions>,
    {
        SchemaBuilder {
            validator: self.validator,
            re_validator: Some(FieldReValidator::Async(re_validator.into_uniform())),
            ..Default::default()
        }
    }
}

impl<HasRevalidator, T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    SchemaBuilder<T, I, O, CtxOptions, Yes, HasRevalidator, No, No, No, No>
{
    pub fn readonly(
        self,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, HasRevalidator, Yes, No, No, No> {
        SchemaBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            should_update: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_update_if<F>(
        self,
        fx: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, HasRevalidator, No, Yes, No, No>
    where
        F: Fn(IvoSummary<CtxOptions>) -> bool + Send + Sync + 'static,
    {
        SchemaBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            should_update: Some(ComputableInit::Func(Box::new(fx))),
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
    >
    SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasRevalidator,
        HasShouldUpdate,
        No,
        HasFailure,
        HasSuccess,
    >
{
    pub fn on_delete<F, Fut>(
        self,
        handler: F,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasRevalidator,
        HasShouldUpdate,
        Yes,
        HasFailure,
        HasSuccess,
    >
    where
        F: Fn(&O, &CtxOptions) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h: DeleteHandler<O, CtxOptions> = Box::new(move |d, o| Box::pin(handler(d, o)));

        SchemaBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            should_update: self.should_update,
            on_delete_fns: Some(match self.on_delete_fns {
                Some(hs) => {
                    let mut v = Vec::from(hs);

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

    pub fn on_delete_fns(
        self,
        handlers: Vec<DeleteHandler<O, CtxOptions>>,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasRevalidator,
        HasShouldUpdate,
        Yes,
        HasFailure,
        HasSuccess,
    > {
        SchemaBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            should_update: self.should_update,
            on_delete_fns: Some(handlers),
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
    >
    SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasRevalidator,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    pub fn on_failure<F, Fut>(
        self,
        handler: F,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasRevalidator,
        HasShouldUpdate,
        HasDelete,
        Yes,
        HasSuccess,
    >
    where
        F: Fn(IvoSummary<CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h: FailureHandler<CtxOptions> = Box::new(move |s| Box::pin(handler(s)));

        SchemaBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            should_update: self.should_update,
            on_delete_fns: self.on_delete_fns,
            on_failure_fns: Some(match self.on_failure_fns {
                Some(hs) => {
                    let mut v = Vec::from(hs);

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
    >
    SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasRevalidator,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
    >
{
    pub fn on_success<F, Fut>(
        self,
        handler: F,
    ) -> SchemaBuilder<
        T,
        I,
        O,
        CtxOptions,
        Yes,
        HasRevalidator,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        Yes,
    >
    where
        F: Fn(IvoSummary<CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h: SuccessHandler<CtxOptions> = Box::new(move |s| Box::pin(handler(s)));

        SchemaBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            should_update: self.should_update,
            on_delete_fns: self.on_delete_fns,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: Some(match self.on_success_fns {
                Some(hs) => {
                    let mut v = Vec::from(hs);

                    v.push(h);

                    v
                }
                _ => vec![h],
            }),
            ..Default::default()
        }
    }
}
