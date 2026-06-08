use std::marker::PhantomData;

use crate::{
    schema::{
        error::IvoErrorTool,
        options::types::{IntoPostValidator, PostValidationConfig, PostValidatorFn},
    },
    traits::IvoSchemaStruct,
    types::{No, Yes},
};

pub struct PostValidateOptionBuilder<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
    HasFields = No,
    HasValidator = No,
    HasPreValidator = No,
> {
    _has_fields: PhantomData<HasFields>,
    _pre_v: PhantomData<HasPreValidator>,
    _validator: PhantomData<HasValidator>,
    // actual data...
    fields: Vec<&'static str>,
    pre_validator: Option<PostValidatorFn<I, O, CtxOptions, ErrT::FieldMetadata>>,
    validators: Vec<PostValidatorFn<I, O, CtxOptions, ErrT::FieldMetadata>>,
}

impl<
        HasFields,
        HasPreValidator,
        HasValidator,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > Default
    for PostValidateOptionBuilder<I, O, CtxOptions, ErrT, HasFields, HasPreValidator, HasValidator>
{
    fn default() -> Self {
        Self {
            fields: vec![],
            pre_validator: None,
            validators: vec![],
            _has_fields: PhantomData,
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
    for PostValidateOptionBuilder<I, O, CtxOptions, ErrT, Yes, Yes, HasPreValidator>
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
    PostValidateOptionBuilder<I, O, CtxOptions, ErrT>
{
    pub fn fields<const N: usize>(
        fields: [&'static str; N],
    ) -> PostValidateOptionBuilder<I, O, CtxOptions, ErrT, Yes> {
        PostValidateOptionBuilder {
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
    > PostValidateOptionBuilder<I, O, CtxOptions, ErrT, Yes, HasValidator, HasPreValidator>
{
    pub fn validate<F>(
        self,
        validator: F,
    ) -> PostValidateOptionBuilder<I, O, CtxOptions, ErrT, Yes, Yes, HasPreValidator>
    where
        F: IntoPostValidator<I, O, CtxOptions, ErrT>,
    {
        let mut validators = self.validators;
        validators.push(validator.into_validator());

        PostValidateOptionBuilder {
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
    > PostValidateOptionBuilder<I, O, CtxOptions, ErrT, Yes, HasValidator, No>
{
    pub fn pre_validate<F>(
        self,
        validator: F,
    ) -> PostValidateOptionBuilder<I, O, CtxOptions, ErrT, Yes, HasValidator, Yes>
    where
        F: IntoPostValidator<I, O, CtxOptions, ErrT>,
    {
        PostValidateOptionBuilder {
            fields: self.fields,
            validators: self.validators,
            pre_validator: Some(validator.into_validator()),
            ..Default::default()
        }
    }
}
