use std::{future::Future, marker::PhantomData};

use serde_json::Value;

use crate::{
    schema::properties::base::IvoProperty,
    traits::HasPartial,
    types::{
        ComputableInit, ComputableRequired, DeleteHandler, FailureHandler, FieldValidator,
        IvoSummary, SuccessHandler, True,
    },
    ValidatorResponse,
};

pub struct RequiredField;

// Marker Types
struct Yes;
struct No;

struct SchemaBuilder<
    T,
    I: HasPartial,
    O: HasPartial,
    CtxOptions,
    HasValidator,
    HasRevalidator,
    HasShouldUpdate,
    HasDelete,
    HasFailure,
    HasSuccess,
> {
    _validator: PhantomData<HasValidator>,
    _re_validator: PhantomData<HasRevalidator>,
    _should_update: PhantomData<HasShouldUpdate>,
    _on_delete_fns: PhantomData<HasDelete>,
    _on_failure_fns: PhantomData<HasFailure>,
    _on_success_fns: PhantomData<HasSuccess>,
    // actual data...
    validator: Option<FieldValidator<T, I, O, CtxOptions>>,
    re_validator: Option<FieldValidator<T, I, O, CtxOptions>>,
    should_update: Option<ComputableInit<I, O, CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_failure_fns: Option<Vec<FailureHandler<I, O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<
        HasValidator,
        HasRevalidator,
        HasShouldUpdate,
        HasDelete,
        HasFailure,
        HasSuccess,
        T,
        I: HasPartial,
        O: HasPartial,
        CtxOptions,
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
        I: HasPartial,
        O: HasPartial,
        CtxOptions,
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
    pub fn build(self) -> IvoProperty<T, I, O, CtxOptions> {
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
    pub fn validate<T, I: HasPartial, O: HasPartial, CtxOptions, F>(
        validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No>
    where
        F: Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<T>
            + Send
            + Sync
            + 'static,
    {
        SchemaBuilder {
            validator: Some(FieldValidator::Sync(Box::new(validator))),
            ..Default::default()
        }
    }

    pub fn validate_async<T, I: HasPartial, O: HasPartial, CtxOptions, F, Fut>(
        validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No>
    where
        F: Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ValidatorResponse<T>> + Send + 'static,
    {
        SchemaBuilder {
            validator: Some(FieldValidator::Async(Box::new(move |v, ctx| {
                Box::pin(validator(v, ctx))
            }))),
            ..Default::default()
        }
    }
}

impl<T, I: HasPartial, O: HasPartial, CtxOptions>
    SchemaBuilder<T, I, O, CtxOptions, Yes, No, No, No, No, No>
{
    pub fn re_validate<F>(
        self,
        re_validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, No, No, No, No>
    where
        F: Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> ValidatorResponse<T>
            + Send
            + Sync
            + 'static,
    {
        SchemaBuilder {
            validator: self.validator,
            re_validator: Some(FieldValidator::Sync(Box::new(re_validator))),
            ..Default::default()
        }
    }

    pub fn validate_async<F, Fut>(
        self,
        re_validator: F,
    ) -> SchemaBuilder<T, I, O, CtxOptions, Yes, Yes, Yes, No, No, No>
    where
        F: Fn(&Value, &mut IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ValidatorResponse<T>> + Send + 'static,
    {
        SchemaBuilder {
            validator: self.validator,
            re_validator: Some(FieldValidator::Async(Box::new(move |v, ctx| {
                Box::pin(re_validator(v, ctx))
            }))),
            ..Default::default()
        }
    }
}

impl<HasRevalidator, T, I: HasPartial, O: HasPartial, CtxOptions>
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
        F: Fn(&mut IvoSummary<I, O, CtxOptions>) -> bool + Send + Sync + 'static,
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
        I: HasPartial,
        O: HasPartial,
        CtxOptions,
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
    pub fn on_delete(
        self,
        handler: DeleteHandler<O, CtxOptions>,
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
            on_delete_fns: Some(vec![handler]),
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
        HasSuccess,
        T,
        I: HasPartial,
        O: HasPartial,
        CtxOptions,
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
        No,
        HasSuccess,
    >
{
    pub fn on_failure(
        self,
        handler: FailureHandler<I, O, CtxOptions>,
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
    > {
        SchemaBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            should_update: self.should_update,
            on_delete_fns: self.on_delete_fns,
            on_failure_fns: Some(vec![handler]),
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }

    pub fn on_failure_fns(
        self,
        handlers: Vec<FailureHandler<I, O, CtxOptions>>,
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
    > {
        SchemaBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            on_delete_fns: self.on_delete_fns,
            on_failure_fns: Some(handlers),
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
        T,
        I: HasPartial,
        O: HasPartial,
        CtxOptions,
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
        No,
    >
{
    pub fn on_success(
        self,
        handler: SuccessHandler<I, O, CtxOptions>,
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
    > {
        SchemaBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            should_update: self.should_update,
            on_delete_fns: self.on_delete_fns,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: Some(vec![handler]),
            ..Default::default()
        }
    }

    pub fn on_success_fns(
        self,
        handlers: Vec<SuccessHandler<I, O, CtxOptions>>,
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
    > {
        SchemaBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            should_update: self.should_update,
            on_delete_fns: self.on_delete_fns,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: Some(handlers),
            ..Default::default()
        }
    }
}
