use std::marker::PhantomData;

use crate::{
    schema::properties::base::IvoProperty,
    types::{
        BooleanResolverWithMutSummary, ComputableInit, ComputableRequired, DeleteHandler,
        FailureHandler, FieldValidatorFn, SuccessHandler, True,
    },
};

pub struct RequiredField;

// Marker Types
struct Yes;
struct No;

struct SchemaBuilder<
    I,
    O,
    T,
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
    validator: Option<FieldValidatorFn<I, O, T>>,
    re_validator: Option<FieldValidatorFn<I, O, T>>,
    should_update: Option<ComputableInit<I, O>>,
    on_delete_fns: Option<Vec<DeleteHandler<O>>>,
    on_failure_fns: Option<Vec<FailureHandler<I, O>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O>>>,
}

impl<HasValidator, HasRevalidator, HasShouldUpdate, HasDelete, HasFailure, HasSuccess, I, O, T>
    Default
    for SchemaBuilder<
        I,
        O,
        T,
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

impl<HasRevalidator, HasShouldUpdate, HasDelete, HasFailure, HasSuccess, I, O, T>
    SchemaBuilder<I, O, T, Yes, HasRevalidator, HasShouldUpdate, HasDelete, HasFailure, HasSuccess>
{
    pub fn build(self) -> IvoProperty<I, O, T> {
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
    pub fn validate<I, O, T>(
        validator: FieldValidatorFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, No, No, No, No, No> {
        SchemaBuilder {
            validator: Some(validator),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, No, No, No, No, No> {
    pub fn re_validate(
        self,
        re_validator: FieldValidatorFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No> {
        SchemaBuilder {
            validator: self.validator,
            re_validator: Some(re_validator),
            ..Default::default()
        }
    }
}

impl<HasRevalidator, I, O, T> SchemaBuilder<I, O, T, Yes, HasRevalidator, No, No, No, No> {
    pub fn readonly(self) -> SchemaBuilder<I, O, T, Yes, HasRevalidator, Yes, No, No, No> {
        SchemaBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            should_update: Some(ComputableInit::False),
            ..Default::default()
        }
    }

    pub fn allow_update_if(
        self,
        fx: BooleanResolverWithMutSummary<I, O>,
    ) -> SchemaBuilder<I, O, T, Yes, HasRevalidator, No, Yes, No, No> {
        SchemaBuilder {
            validator: self.validator,
            re_validator: self.re_validator,
            should_update: Some(ComputableInit::Func(fx)),
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<HasRevalidator, HasShouldUpdate, HasFailure, HasSuccess, I, O, T>
    SchemaBuilder<I, O, T, Yes, HasRevalidator, HasShouldUpdate, No, HasFailure, HasSuccess>
{
    pub fn on_delete(
        self,
        handler: DeleteHandler<O>,
    ) -> SchemaBuilder<I, O, T, Yes, HasRevalidator, HasShouldUpdate, Yes, HasFailure, HasSuccess>
    {
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
        handlers: Vec<DeleteHandler<O>>,
    ) -> SchemaBuilder<I, O, T, Yes, HasRevalidator, HasShouldUpdate, Yes, HasFailure, HasSuccess>
    {
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
impl<HasRevalidator, HasShouldUpdate, HasDelete, HasSuccess, I, O, T>
    SchemaBuilder<I, O, T, Yes, HasRevalidator, HasShouldUpdate, HasDelete, No, HasSuccess>
{
    pub fn on_failure(
        self,
        handler: FailureHandler<I, O>,
    ) -> SchemaBuilder<I, O, T, Yes, HasRevalidator, HasShouldUpdate, HasDelete, Yes, HasSuccess>
    {
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
        handlers: Vec<FailureHandler<I, O>>,
    ) -> SchemaBuilder<I, O, T, Yes, HasRevalidator, HasShouldUpdate, HasDelete, Yes, HasSuccess>
    {
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
impl<HasRevalidator, HasShouldUpdate, HasDelete, HasFailure, I, O, T>
    SchemaBuilder<I, O, T, Yes, HasRevalidator, HasShouldUpdate, HasDelete, HasFailure, No>
{
    pub fn on_success(
        self,
        handler: SuccessHandler<I, O>,
    ) -> SchemaBuilder<I, O, T, Yes, HasRevalidator, HasShouldUpdate, HasDelete, HasFailure, Yes>
    {
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
        handlers: Vec<SuccessHandler<I, O>>,
    ) -> SchemaBuilder<I, O, T, Yes, HasRevalidator, HasShouldUpdate, HasDelete, HasFailure, Yes>
    {
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
