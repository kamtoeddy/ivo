use std::future::Future;

use crate::{
    schema::error::IvoErrorTool,
    types::{IvoSummary, IvoValues, PostValidatorError, PostValidatorFn},
    IvoSchemaStruct,
};

// pub trait IntoDeleteHandler<O: IvoSchemaStruct, CtxOptions: Clone> {
//     fn into_handler(self) -> DeleteHandler<O, CtxOptions>;
// }

// impl<F, Fut, O, CtxOptions: Clone> IntoDeleteHandler<O, CtxOptions> for F
// where
//     O: IvoSchemaStruct,
//     F: Fn(O, CtxOptions) -> Fut + Send + Sync + 'static,
//     Fut: Future<Output = ()> + Send + Sync + 'static,
// {
//     fn into_handler(self) -> DeleteHandler<O, CtxOptions> {
//         Box::new(move |o, s| Box::pin(self(o, s)))
//     }
// }

// pub trait IntoSuccessHandler<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
//     fn into_handler(self) -> SuccessHandler<I, O, CtxOptions>;
// }

// impl<F, Fut, I, O, CtxOptions: Clone> IntoSuccessHandler<I, O, CtxOptions> for F
// where
//     I: IvoSchemaStruct,
//     O: IvoSchemaStruct,
//     F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
//     Fut: Future<Output = ()> + Send + Sync + 'static,
// {
//     fn into_handler(self) -> SuccessHandler<I, O, CtxOptions> {
//         Box::new(move |s| Box::pin(self(s)))
//     }
// }

pub struct PostValidationConfig<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
> {
    pub fields: Vec<&'static str>,
    pub pre_validator: Option<PostValidatorFn<I, O, CtxOptions, ErrT::FieldMetadata>>,
    pub validators: Vec<PostValidatorFn<I, O, CtxOptions, ErrT::FieldMetadata>>,
}

pub trait IntoPostValidator<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
>
{
    fn into_validator(self) -> PostValidatorFn<I, O, CtxOptions, ErrT::FieldMetadata>;
}

impl<F, Fut, I, O, CtxOptions: Clone, ErrT: IvoErrorTool> IntoPostValidator<I, O, CtxOptions, ErrT>
    for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    F: Fn(IvoSummary<I, O, CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<IvoValues, PostValidatorError<ErrT::FieldMetadata>>>
        + Send
        + Sync
        + 'static,
{
    fn into_validator(self) -> PostValidatorFn<I, O, CtxOptions, ErrT::FieldMetadata> {
        Box::new(move |s| Box::pin(self(s)))
    }
}
