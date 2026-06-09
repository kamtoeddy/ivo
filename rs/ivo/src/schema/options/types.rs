use std::{collections::HashMap, fmt::Debug, future::Future};

use futures::future::BoxFuture;

use crate::{
    erase_value,
    schema::error::IvoErrorTool,
    types::{IvoSummary, SuccessHandler},
    ErasedValue, IvoSchemaStruct, ValidatorError,
};

pub struct IvoValues {
    data: HashMap<String, ErasedValue>,
}

impl IvoValues {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set<T: Clone + Debug + Send + Sync + 'static>(
        &mut self,
        field: &str,
        value: T,
    ) -> &mut Self {
        self.data.insert(field.to_owned(), erase_value(value));

        self
    }
}

pub struct OnSuccessConfig<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    pub fields: Vec<&'static str>,
    pub handlers: Vec<SuccessHandler<I, O, CtxOptions>>,
}

pub struct PostValidationConfig<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
> {
    pub fields: Vec<&'static str>,
    pub pre_validator: Option<PostValidator<I, O, CtxOptions, ErrT::FieldMetadata>>,
    pub validators: Vec<PostValidator<I, O, CtxOptions, ErrT::FieldMetadata>>,
}

pub trait IntoPostValidator<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
>
{
    fn into_validator(self) -> PostValidator<I, O, CtxOptions, ErrT::FieldMetadata>;
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
    fn into_validator(self) -> PostValidator<I, O, CtxOptions, ErrT::FieldMetadata> {
        Box::new(move |s| Box::pin(self(s)))
    }
}

pub type PostValidatorError<FieldErrorMetadata> =
    HashMap<String, ValidatorError<FieldErrorMetadata>>;

pub type PostValidator<I, O, CtxOptions, FieldErrorMetadata> = Box<
    dyn Fn(
            IvoSummary<I, O, CtxOptions>,
        ) -> BoxFuture<'static, Result<IvoValues, PostValidatorError<FieldErrorMetadata>>>
        + Send
        + Sync
        + 'static,
>;
