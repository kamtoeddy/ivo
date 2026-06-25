use std::{collections::HashMap, future::Future};

use futures::future::BoxFuture;

use crate::{
    schema::{
        error_tool::IvoErrorTool,
        types::{IvoFieldValue, SuccessHandler},
    },
    types::{erase_value, ErasedValue},
    DefaultFieldErrorMetadata, IvoStruct, SharedIvoContext, SharedRwCtxOptions, ValidatorError,
};

pub struct IvoValues {
    pub(crate) data: HashMap<String, ErasedValue>,
}

impl IvoValues {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set<T: IvoFieldValue>(&mut self, field: &str, value: T) -> &mut Self {
        self.data.insert(field.to_owned(), erase_value(value));

        self
    }
}

impl Default for IvoValues {
    fn default() -> Self {
        Self::new()
    }
}

pub struct OnSuccessConfig<I: IvoStruct, O: IvoStruct, CtxOptions> {
    pub fields: Vec<&'static str>,
    pub handlers: Vec<SuccessHandler<I, O, CtxOptions>>,
}

pub struct PostValidationConfig<I: IvoStruct, O: IvoStruct, CtxOptions, ErrorTool: IvoErrorTool> {
    pub fields: Vec<&'static str>,
    pub pre_validator: Option<PostValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    pub validators: Vec<PostValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
}

pub trait IntoPostValidator<I: IvoStruct, O: IvoStruct, CtxOptions, ErrorTool: IvoErrorTool> {
    fn into_validator(self) -> PostValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>;
}

impl<F, Fut, I, O, CtxOptions, ErrorTool: IvoErrorTool>
    IntoPostValidator<I, O, CtxOptions, ErrorTool> for F
where
    I: IvoStruct,
    O: IvoStruct,
    F: Fn(SharedIvoContext<I, O>, SharedRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = PostValidatorResponse<ErrorTool::FieldMetadata>> + Send + Sync + 'static,
{
    fn into_validator(self) -> PostValidator<I, O, CtxOptions, ErrorTool::FieldMetadata> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub type PostValidatorError<FieldErrorMetadata = DefaultFieldErrorMetadata> =
    HashMap<String, ValidatorError<FieldErrorMetadata>>;

pub type PostValidatorResponse<FieldErrorMetadata = DefaultFieldErrorMetadata> =
    Result<IvoValues, PostValidatorError<FieldErrorMetadata>>;

pub type PostValidator<I, O, CtxOptions, FieldErrorMetadata> = Box<
    dyn Fn(
            SharedIvoContext<I, O>,
            SharedRwCtxOptions<CtxOptions>,
        ) -> BoxFuture<'static, PostValidatorResponse<FieldErrorMetadata>>
        + Send
        + Sync
        + 'static,
>;
