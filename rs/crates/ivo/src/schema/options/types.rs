use std::{collections::HashMap, fmt::Debug, future::Future};

use futures::future::BoxFuture;

use crate::{
    erase_value, schema::error_tool::IvoErrorTool, types::SuccessHandler, DefaultFieldErrorMetadata,
    ErasedValue, IvoSchemaStruct, SharedIvoContext, SharedRwCtxOptions, ValidatorError,
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

    pub fn set<T: Clone + Debug + Send + Sync + 'static>(
        &mut self,
        field: &str,
        value: T,
    ) -> &mut Self {
        self.data.insert(field.to_owned(), erase_value(value));

        self
    }
}

pub struct OnSuccessConfig<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> {
    pub fields: Vec<&'static str>,
    pub handlers: Vec<SuccessHandler<I, O, CtxOptions>>,
}

pub struct PostValidationConfig<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
> {
    pub fields: Vec<&'static str>,
    pub pre_validator: Option<PostValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    pub validators: Vec<PostValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
}

pub trait IntoPostValidator<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
>
{
    fn into_validator(self) -> PostValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>;
}

impl<F, Fut, I, O, CtxOptions, ErrorTool: IvoErrorTool>
    IntoPostValidator<I, O, CtxOptions, ErrorTool> for F
where
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
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
