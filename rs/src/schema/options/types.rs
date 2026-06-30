use std::future::Future;

use crate::types::internal::{IvoErrorTool, PostValidatorResponse};
use crate::{
    schema::types::SuccessHandler, IvoContext, IvoRwCtxOptions, IvoStruct, IvoUpdateData,
};
use futures::future::BoxFuture;

pub type ShouldUpdateOptionResolver<I, O, CtxOptions> = Box<
    dyn Fn(IvoUpdateData<I, O>, IvoRwCtxOptions<CtxOptions>) -> BoxFuture<'static, bool>
        + Send
        + Sync
        + 'static,
>;

pub trait IntoShouldUpdateOptionResolver<I: IvoStruct, O: IvoStruct, CtxOptions> {
    fn into_resolver(self) -> ShouldUpdateOptionResolver<I, O, CtxOptions>;
}

impl<F, Fut, I, O, CtxOptions> IntoShouldUpdateOptionResolver<I, O, CtxOptions> for F
where
    I: IvoStruct,
    O: IvoStruct,
    F: Fn(IvoUpdateData<I, O>, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = bool> + Send + Sync + 'static,
{
    fn into_resolver(self) -> ShouldUpdateOptionResolver<I, O, CtxOptions> {
        Box::new(move |data, o| Box::pin(self(data, o)))
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
    F: Fn(IvoContext<I, O>, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut:
        Future<Output = PostValidatorResponse<I, ErrorTool::FieldMetadata>> + Send + Sync + 'static,
{
    fn into_validator(self) -> PostValidator<I, O, CtxOptions, ErrorTool::FieldMetadata> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub type PostValidator<I, O, CtxOptions, FieldErrorMetadata> = Box<
    dyn Fn(
            IvoContext<I, O>,
            IvoRwCtxOptions<CtxOptions>,
        ) -> BoxFuture<'static, PostValidatorResponse<I, FieldErrorMetadata>>
        + Send
        + Sync
        + 'static,
>;
