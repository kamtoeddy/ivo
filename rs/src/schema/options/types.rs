#![expect(type_alias_bounds)]

use std::future::Future;

use crate::__private_types::types::IvoWithPartialErrorsStruct;
use crate::types::internal::PostValidatorResponse;
use crate::IvoErrorTool;
use crate::{schema::types::SuccessHandler, IvoContext, IvoRwCtxOptions, IvoStruct};
use futures::future::BoxFuture;

pub type ShouldUpdateOptionResolver<I: IvoStruct, O: IvoStruct, CtxOptions> = Box<
    dyn Fn(I::Partial, O, IvoRwCtxOptions<CtxOptions>) -> BoxFuture<'static, bool>
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
    F: Fn(I::Partial, O, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = bool> + Send + Sync + 'static,
{
    fn into_resolver(self) -> ShouldUpdateOptionResolver<I, O, CtxOptions> {
        Box::new(move |partial_input, output, o| Box::pin(self(partial_input, output, o)))
    }
}

pub struct OnSuccessConfig<I: IvoStruct, O: IvoStruct, CtxOptions> {
    pub fields: Vec<&'static str>,
    pub handlers: Vec<SuccessHandler<I, O, CtxOptions>>,
}

pub struct PostValidationConfig<
    I: IvoStruct + IvoWithPartialErrorsStruct<ErrorTool::FieldMetadata>,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
> {
    pub fields: Vec<&'static str>,
    pub pre_validator: Option<PostValidator<I, O, CtxOptions, ErrorTool>>,
    pub validators: Vec<PostValidator<I, O, CtxOptions, ErrorTool>>,
}

pub trait IntoPostValidator<
    I: IvoStruct + IvoWithPartialErrorsStruct<ErrorTool::FieldMetadata>,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
>
{
    fn into_validator(self) -> PostValidator<I, O, CtxOptions, ErrorTool>;
}

impl<F, Fut, I, O, CtxOptions, ErrorTool> IntoPostValidator<I, O, CtxOptions, ErrorTool> for F
where
    I: IvoStruct + IvoWithPartialErrorsStruct<ErrorTool::FieldMetadata>,
    O: IvoStruct,
    ErrorTool: IvoErrorTool,
    F: Fn(IvoContext<I, O>, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = PostValidatorResponse<I, ErrorTool>> + Send + Sync + 'static,
{
    fn into_validator(self) -> PostValidator<I, O, CtxOptions, ErrorTool> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub type PostValidator<I, O, CtxOptions, ErrorTool> = Box<
    dyn Fn(
            IvoContext<I, O>,
            IvoRwCtxOptions<CtxOptions>,
        ) -> BoxFuture<'static, PostValidatorResponse<I, ErrorTool>>
        + Send
        + Sync
        + 'static,
>;
