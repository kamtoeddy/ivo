use std::marker::PhantomData;

use crate::{
    schema::error::IvoErrorTool,
    traits::{IntoPostValidator, IvoSchemaStruct, PostValidationConfig},
    types::{No, PostValidatorFn, Yes},
};

pub struct OnSuccessOptionBuilder<
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
    pub fields: Vec<&'static str>,
    pub pre_validator: Option<PostValidatorFn<I, O, CtxOptions, ErrT::FieldMetadata>>,
    pub validators: Vec<PostValidatorFn<I, O, CtxOptions, ErrT::FieldMetadata>>,
    // pub on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
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
    for OnSuccessOptionBuilder<I, O, CtxOptions, ErrT, HasFields, HasPreValidator, HasValidator>
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
    for OnSuccessOptionBuilder<I, O, CtxOptions, ErrT, Yes, Yes, HasPreValidator>
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
    OnSuccessOptionBuilder<I, O, CtxOptions, ErrT>
{
    pub fn fields<const N: usize>(
        fields: [&'static str; N],
    ) -> OnSuccessOptionBuilder<I, O, CtxOptions, ErrT, Yes> {
        OnSuccessOptionBuilder {
            fields: Vec::from(fields),
            ..Default::default()
        }
    }
}

// ON_SUCCESS is only available if HasSuccess is 'No'
impl<
        HasAlias,
        HasRevalidator,
        HasSanitizer,
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
    VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasFailure,
        HasSuccess,
    >
{
    pub fn on_success<H>(
        self,
        handler: H,
    ) -> VirtualFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrT,
        Yes,
        HasAlias,
        HasRevalidator,
        HasSanitizer,
        HasRequired,
        HasIgnore,
        HasShouldInit,
        HasShouldUpdate,
        HasFailure,
        Yes,
    >
    where
        H: IntoSuccessHandler<I, O, CtxOptions>,
    {
        let h = handler.into_handler();

        VirtualFieldBuilder {
            alias: self.alias,
            validator: self.validator,
            re_validator: self.re_validator,
            sanitizer: self.sanitizer,
            required: self.required,
            should_ignore_fn: self.should_ignore_fn,
            should_init: self.should_init,
            should_update: self.should_update,
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
