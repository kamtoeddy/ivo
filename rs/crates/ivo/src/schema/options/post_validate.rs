use std::marker::PhantomData;

use crate::{
    schema::{
        error_tool::IvoErrorTool,
        options::types::{IntoPostValidator, PostValidationConfig, PostValidator},
        No, Yes,
    },
    IvoSchemaStruct,
};

pub struct PostValidateOptionBuilder<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
    IvoFieldNames = No,
    HasValidator = No,
    HasPreValidator = No,
> {
    _has_fields: PhantomData<IvoFieldNames>,
    _pre_v: PhantomData<HasPreValidator>,
    _validator: PhantomData<HasValidator>,
    // actual data...
    fields: Vec<&'static str>,
    pre_validator: Option<PostValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    validators: Vec<PostValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
}

impl<
        IvoFieldNames,
        HasPreValidator,
        HasValidator,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > Default
    for PostValidateOptionBuilder<
        I,
        O,
        CtxOptions,
        ErrorTool,
        IvoFieldNames,
        HasPreValidator,
        HasValidator,
    >
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
    CtxOptions,
    ErrorTool: IvoErrorTool,
>
{
    fn build(self) -> PostValidationConfig<I, O, CtxOptions, ErrorTool>;
}

impl<
        HasPreValidator,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > BuildablePostValidator<I, O, CtxOptions, ErrorTool>
    for PostValidateOptionBuilder<I, O, CtxOptions, ErrorTool, Yes, Yes, HasPreValidator>
{
    fn build(self) -> PostValidationConfig<I, O, CtxOptions, ErrorTool> {
        PostValidationConfig {
            fields: self.fields,
            validators: self.validators,
            pre_validator: self.pre_validator,
        }
    }
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions, ErrorTool: IvoErrorTool>
    PostValidateOptionBuilder<I, O, CtxOptions, ErrorTool>
{
    pub fn fields<const N: usize>(
        fields: [&'static str; N],
    ) -> PostValidateOptionBuilder<I, O, CtxOptions, ErrorTool, Yes> {
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
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > PostValidateOptionBuilder<I, O, CtxOptions, ErrorTool, Yes, HasValidator, HasPreValidator>
{
    pub fn validate<F>(
        self,
        validator: F,
    ) -> PostValidateOptionBuilder<I, O, CtxOptions, ErrorTool, Yes, Yes, HasPreValidator>
    where
        F: IntoPostValidator<I, O, CtxOptions, ErrorTool>,
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

impl<HasValidator, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions, ErrorTool: IvoErrorTool>
    PostValidateOptionBuilder<I, O, CtxOptions, ErrorTool, Yes, HasValidator, No>
{
    pub fn pre_validate<F>(
        self,
        validator: F,
    ) -> PostValidateOptionBuilder<I, O, CtxOptions, ErrorTool, Yes, HasValidator, Yes>
    where
        F: IntoPostValidator<I, O, CtxOptions, ErrorTool>,
    {
        PostValidateOptionBuilder {
            fields: self.fields,
            validators: self.validators,
            pre_validator: Some(validator.into_validator()),
            ..Default::default()
        }
    }
}
