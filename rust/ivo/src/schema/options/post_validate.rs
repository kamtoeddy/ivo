use std::marker::PhantomData;

use crate::{
    schema::error::IvoErrorTool,
    traits::{IntoPostValidator, IvoSchemaStruct, PostValidationConfig},
    types::{No, PostValidatorFn, Yes},
};

pub struct PostValidateBuilder<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
    HasValidator = No,
    HasPreValidator = No,
> {
    _pre_v: PhantomData<HasPreValidator>,
    _validator: PhantomData<HasValidator>,
    // actual data...
    pub fields: Vec<&'static str>,
    pub pre_validator: Option<PostValidatorFn<I, O, CtxOptions, ErrT::FieldMetadata>>,
    pub validators: Vec<PostValidatorFn<I, O, CtxOptions, ErrT::FieldMetadata>>,
}

impl<
        HasPreValidator,
        HasValidator,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > Default for PostValidateBuilder<I, O, CtxOptions, ErrT, HasPreValidator, HasValidator>
{
    fn default() -> Self {
        Self {
            fields: vec![],
            pre_validator: None,
            validators: vec![],
            _pre_v: PhantomData,
            _validator: PhantomData,
        }
    }
}

pub trait BuildablePostValidator<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
>
{
    fn build(self) -> PostValidationConfig<I, O, CtxOptions, ErrT>;
}

impl<
        HasPreValidator,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > BuildablePostValidator<I, O, CtxOptions, ErrT>
    for PostValidateBuilder<I, O, CtxOptions, ErrT, Yes, HasPreValidator>
{
    fn build(self) -> PostValidationConfig<I, O, CtxOptions, ErrT> {
        PostValidationConfig {
            fields: self.fields,
            validators: self.validators,
            pre_validator: self.pre_validator,
        }
    }
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrT: IvoErrorTool>
    PostValidateBuilder<I, O, CtxOptions, ErrT>
{
    pub fn fields<const N: usize>(
        fields: [&'static str; N],
    ) -> PostValidateBuilder<I, O, CtxOptions, ErrT, Yes> {
        PostValidateBuilder {
            fields: Vec::from(fields),
            ..Default::default()
        }
    }
}

impl<
        HasValidator,
        HasPreValidator,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone + Send,
        ErrT: IvoErrorTool,
    > PostValidateBuilder<I, O, CtxOptions, ErrT, HasValidator, HasPreValidator>
{
    pub fn validate<F>(self, validator: F) -> PostValidateBuilder<I, O, CtxOptions, ErrT, Yes>
    where
        F: IntoPostValidator<I, O, CtxOptions, ErrT>,
    {
        let mut validators = self.validators;
        validators.push(validator.into_validator());

        PostValidateBuilder {
            fields: self.fields,
            validators,
            pre_validator: self.pre_validator,
            ..Default::default()
        }
    }
}

impl<
        HasValidator,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone + Send,
        ErrT: IvoErrorTool,
    > PostValidateBuilder<I, O, CtxOptions, ErrT, HasValidator, No>
{
    pub fn pre_validate<F>(
        self,
        validator: F,
    ) -> PostValidateBuilder<I, O, CtxOptions, ErrT, HasValidator, Yes>
    where
        F: IntoPostValidator<I, O, CtxOptions, ErrT>,
    {
        PostValidateBuilder {
            fields: self.fields,
            validators: self.validators,
            pre_validator: Some(validator.into_validator()),
            ..Default::default()
        }
    }
}
