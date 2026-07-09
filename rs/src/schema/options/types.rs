#![expect(type_alias_bounds)]

use std::future::Future;

use crate::__private_types::types::PartialErrorsMethods;
use crate::__private_types::{FieldInfo, IvoInputStruct};

use crate::schema::fields::types::RequiredResolver;
use crate::types::internal::PostValidatorResponse;
use crate::{schema::types::SuccessHandler, IvoContext, IvoRwCtxOptions, IvoStruct};
use crate::{FieldError, IvoErrorTool};
use futures::future::BoxFuture;

pub type IgnoreUpdateOptionResolver<I: IvoStruct, O: IvoStruct, CtxOptions> = Box<
    dyn Fn(I::Partial, O, IvoRwCtxOptions<CtxOptions>) -> BoxFuture<'static, bool>
        + Send
        + Sync
        + 'static,
>;

pub trait IntoShouldUpdateOptionResolver<I: IvoStruct, O: IvoStruct, CtxOptions> {
    fn into_resolver(self) -> IgnoreUpdateOptionResolver<I, O, CtxOptions>;
}

impl<F, Fut, I, O, CtxOptions> IntoShouldUpdateOptionResolver<I, O, CtxOptions> for F
where
    I: IvoStruct,
    O: IvoStruct,
    F: Fn(I::Partial, O, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = bool> + Send + Sync + 'static,
{
    fn into_resolver(self) -> IgnoreUpdateOptionResolver<I, O, CtxOptions> {
        Box::new(move |partial_input, output, o| Box::pin(self(partial_input, output, o)))
    }
}

pub struct OnSuccessConfig<I: IvoStruct, O: IvoStruct, CtxOptions> {
    pub fields: Vec<&'static str>,
    pub handlers: Vec<SuccessHandler<I, O, CtxOptions>>,
}

pub struct PostValidationConfig<
    I: IvoInputStruct<ErrorTool>,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
> {
    pub fields: Vec<&'static str>,
    pub pre_validator: Option<PostValidator<I, O, CtxOptions, ErrorTool>>,
    pub validators: Vec<PostValidator<I, O, CtxOptions, ErrorTool>>,
}

pub trait IntoPostValidator<
    I: IvoInputStruct<ErrorTool>,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
>
{
    fn into_validator(self) -> PostValidator<I, O, CtxOptions, ErrorTool>;
}

impl<F, Fut, I, O, CtxOptions, ErrorTool> IntoPostValidator<I, O, CtxOptions, ErrorTool> for F
where
    I: IvoInputStruct<ErrorTool>,
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

pub struct RequiredOptionConfig<
    I: IvoInputStruct<ErrorTool>,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
> {
    pub fields: Vec<&'static str>,
    pub resolver: RequiredOptionResolver<I, O, CtxOptions, ErrorTool>,
}

pub trait IntoRequiredOptionsResolver<
    I: IvoInputStruct<ErrorTool>,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
>
{
    fn into_resolver(self) -> RequiredOptionResolver<I, O, CtxOptions, ErrorTool>;
}

impl<F, Fut, I, O, CtxOptions, ErrorTool> IntoRequiredOptionsResolver<I, O, CtxOptions, ErrorTool>
    for F
where
    I: IvoInputStruct<ErrorTool>,
    O: IvoStruct,
    ErrorTool: IvoErrorTool,
    F: Fn(IvoContext<I, O>, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Option<I::PartialErrors>> + Send + Sync + 'static,
{
    fn into_resolver(self) -> RequiredOptionResolver<I, O, CtxOptions, ErrorTool> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub type RequiredOptionResolver<
    I: IvoInputStruct<ErrorTool>,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
> = Box<
    dyn Fn(
            IvoContext<I, O>,
            IvoRwCtxOptions<CtxOptions>,
        ) -> BoxFuture<'static, Option<I::PartialErrors>>
        + Send
        + Sync
        + 'static,
>;

type UniformRequiredResponse<'a, ErrorTool: IvoErrorTool> =
    BoxFuture<'a, Option<Vec<(String, FieldError<ErrorTool::FieldMetadata>)>>>;

pub trait UniformRequiredResolver<
    I: IvoInputStruct<ErrorTool>,
    O: IvoStruct,
    CtxOptions: Send + Sync,
    ErrorTool: IvoErrorTool,
>
{
    fn resolve<'a>(
        &'a self,
        fields: Vec<FieldInfo>,
        ctx: IvoContext<I, O>,
        o: IvoRwCtxOptions<CtxOptions>,
    ) -> UniformRequiredResponse<'a, ErrorTool>;
}

impl<I, O, CtxOptions: Send + Sync, ErrorTool> UniformRequiredResolver<I, O, CtxOptions, ErrorTool>
    for RequiredResolver<I, O, CtxOptions>
where
    I: IvoInputStruct<ErrorTool>,
    O: IvoStruct,
    ErrorTool: IvoErrorTool,
{
    fn resolve<'a>(
        &'a self,
        fields: Vec<FieldInfo>,
        ctx: IvoContext<I, O>,
        o: IvoRwCtxOptions<CtxOptions>,
    ) -> UniformRequiredResponse<'a, ErrorTool> {
        Box::pin(async move {
            self(ctx, o).await.map(|reason| {
                vec![(
                    fields[0].name.clone(),
                    FieldError {
                        metadata: None::<ErrorTool::FieldMetadata>,
                        reason,
                    },
                )]
            })
        })
    }
}

impl<I, O, CtxOptions: Send + Sync, ErrorTool> UniformRequiredResolver<I, O, CtxOptions, ErrorTool>
    for RequiredOptionConfig<I, O, CtxOptions, ErrorTool>
where
    I: IvoInputStruct<ErrorTool>,
    O: IvoStruct,
    ErrorTool: IvoErrorTool,
{
    fn resolve<'a>(
        &'a self,
        fields: Vec<FieldInfo>,
        ctx: IvoContext<I, O>,
        o: IvoRwCtxOptions<CtxOptions>,
    ) -> UniformRequiredResponse<'a, ErrorTool> {
        let resolver = &self.resolver;

        Box::pin(async move {
            let errors = resolver(ctx, o).await?;

            let mut results = vec![];

            for (field_name, (reason, metadata)) in errors.ivo_internal_enumerate() {
                if fields.iter().find(|info| info.name == field_name).is_some() {
                    results.push((field_name, FieldError { metadata, reason }));
                }
            }

            Some(results)
        })
    }
}
