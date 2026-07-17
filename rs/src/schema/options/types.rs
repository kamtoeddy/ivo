#![expect(type_alias_bounds)]

use std::collections::HashSet;
use std::future::Future;

use crate::__private_types::types::{
    BooleanResolver, IgnoreUpdateOptionResolver, PartialErrorsMethods,
};
use crate::__private_types::{FieldError, IvoInputStruct};

use crate::schema::fields::types::RequiredResolver;
use crate::{
    schema::types::SuccessHandler, types::internal::PostValidatorResponse, IvoContext,
    IvoErrorSanitizer, IvoRwCtxOptions, IvoStruct,
};
use futures::future::BoxFuture;

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

pub struct IgnoreOptionConfig<I: IvoStruct, O: IvoStruct, CtxOptions> {
    pub fields: Vec<&'static str>,
    pub resolver: BooleanResolver<I, O, CtxOptions>,
}

pub struct IgnoreUpdateOptionConfig<I: IvoStruct, O: IvoStruct, CtxOptions> {
    pub fields: Vec<&'static str>,
    pub resolver: IgnoreUpdateOptionResolver<I, O, CtxOptions>,
}

pub struct OnSuccessConfig<I: IvoStruct, O: IvoStruct, CtxOptions> {
    pub fields: Vec<&'static str>,
    pub handlers: Vec<SuccessHandler<I, O, CtxOptions>>,
}

pub struct PostValidationConfig<
    I: IvoInputStruct<ErrorSanitizer>,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer,
> {
    pub fields: Vec<&'static str>,
    pub pre_validator: Option<PostValidator<I, O, CtxOptions, ErrorSanitizer>>,
    pub validators: Vec<PostValidator<I, O, CtxOptions, ErrorSanitizer>>,
}

pub trait IntoPostValidator<
    I: IvoInputStruct<ErrorSanitizer>,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer,
>
{
    fn into_validator(self) -> PostValidator<I, O, CtxOptions, ErrorSanitizer>;
}

impl<F, Fut, I, O, CtxOptions, ErrorSanitizer> IntoPostValidator<I, O, CtxOptions, ErrorSanitizer>
    for F
where
    I: IvoInputStruct<ErrorSanitizer>,
    O: IvoStruct,
    ErrorSanitizer: IvoErrorSanitizer,
    F: Fn(IvoContext<I, O>, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = PostValidatorResponse<I, ErrorSanitizer>> + Send + Sync + 'static,
{
    fn into_validator(self) -> PostValidator<I, O, CtxOptions, ErrorSanitizer> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub type PostValidator<I, O, CtxOptions, ErrorSanitizer> = Box<
    dyn Fn(
            IvoContext<I, O>,
            IvoRwCtxOptions<CtxOptions>,
        ) -> BoxFuture<'static, PostValidatorResponse<I, ErrorSanitizer>>
        + Send
        + Sync
        + 'static,
>;

pub struct RequiredOptionConfig<
    I: IvoInputStruct<ErrorSanitizer>,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer,
> {
    pub fields: Vec<&'static str>,
    pub resolver: RequiredOptionResolver<I, O, CtxOptions, ErrorSanitizer>,
}

pub trait IntoRequiredOptionsResolver<
    I: IvoInputStruct<ErrorSanitizer>,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer,
>
{
    fn into_resolver(self) -> RequiredOptionResolver<I, O, CtxOptions, ErrorSanitizer>;
}

impl<F, Fut, I, O, CtxOptions, ErrorSanitizer>
    IntoRequiredOptionsResolver<I, O, CtxOptions, ErrorSanitizer> for F
where
    I: IvoInputStruct<ErrorSanitizer>,
    O: IvoStruct,
    ErrorSanitizer: IvoErrorSanitizer,
    F: Fn(IvoContext<I, O>, IvoRwCtxOptions<CtxOptions>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Option<I::PartialErrors>> + Send + Sync + 'static,
{
    fn into_resolver(self) -> RequiredOptionResolver<I, O, CtxOptions, ErrorSanitizer> {
        Box::new(move |ctx, o| Box::pin(self(ctx, o)))
    }
}

pub type RequiredOptionResolver<
    I: IvoInputStruct<ErrorSanitizer>,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer,
> = Box<
    dyn Fn(
            IvoContext<I, O>,
            IvoRwCtxOptions<CtxOptions>,
        ) -> BoxFuture<'static, Option<I::PartialErrors>>
        + Send
        + Sync
        + 'static,
>;

type UniformRequiredResponse<'a, ErrorSanitizer: IvoErrorSanitizer> =
    BoxFuture<'a, Option<Vec<(String, FieldError<ErrorSanitizer::FieldMetadata>)>>>;

pub trait UniformRequiredResolver<
    I: IvoInputStruct<ErrorSanitizer>,
    O: IvoStruct,
    CtxOptions: Send + Sync,
    ErrorSanitizer: IvoErrorSanitizer,
>
{
    fn resolve<'a>(
        &'a self,
        field_names: HashSet<&'a str>,
        ctx: IvoContext<I, O>,
        o: IvoRwCtxOptions<CtxOptions>,
    ) -> UniformRequiredResponse<'a, ErrorSanitizer>;
}

impl<I, O, CtxOptions: Send + Sync, ErrorSanitizer>
    UniformRequiredResolver<I, O, CtxOptions, ErrorSanitizer> for RequiredResolver<I, O, CtxOptions>
where
    I: IvoInputStruct<ErrorSanitizer>,
    O: IvoStruct,
    ErrorSanitizer: IvoErrorSanitizer,
{
    fn resolve<'a>(
        &'a self,
        field_names: HashSet<&'a str>,
        ctx: IvoContext<I, O>,
        o: IvoRwCtxOptions<CtxOptions>,
    ) -> UniformRequiredResponse<'a, ErrorSanitizer> {
        Box::pin(async move {
            self(ctx, o).await.map(|reason| {
                vec![(
                    field_names.iter().next().unwrap().to_string(),
                    FieldError {
                        metadata: None::<ErrorSanitizer::FieldMetadata>,
                        reason,
                    },
                )]
            })
        })
    }
}

impl<I, O, CtxOptions: Send + Sync, ErrorSanitizer>
    UniformRequiredResolver<I, O, CtxOptions, ErrorSanitizer>
    for RequiredOptionConfig<I, O, CtxOptions, ErrorSanitizer>
where
    I: IvoInputStruct<ErrorSanitizer>,
    O: IvoStruct,
    ErrorSanitizer: IvoErrorSanitizer,
{
    fn resolve<'a>(
        &'a self,
        field_names: HashSet<&'a str>,
        ctx: IvoContext<I, O>,
        o: IvoRwCtxOptions<CtxOptions>,
    ) -> UniformRequiredResponse<'a, ErrorSanitizer> {
        let resolver = &self.resolver;

        Box::pin(async move {
            let errors = resolver(ctx, o).await?;

            let mut results = vec![];

            for (field_name, (reason, metadata)) in errors.entries() {
                if field_names.contains(field_name.as_str()) {
                    results.push((field_name.to_owned(), FieldError { metadata, reason }));
                }
            }

            Some(results)
        })
    }
}

type UniformIgnoreResponse<'a> = BoxFuture<'a, bool>;

pub trait UniformIgnoreResolver<
    I: IvoInputStruct<ErrorSanitizer>,
    O: IvoStruct,
    CtxOptions: Send + Sync,
    ErrorSanitizer: IvoErrorSanitizer,
>
{
    fn resolve<'a>(
        &'a self,
        ctx: IvoContext<I, O>,
        o: IvoRwCtxOptions<CtxOptions>,
    ) -> UniformIgnoreResponse<'a>;
}

impl<I, O, CtxOptions: Send + Sync, ErrorSanitizer>
    UniformIgnoreResolver<I, O, CtxOptions, ErrorSanitizer> for BooleanResolver<I, O, CtxOptions>
where
    I: IvoInputStruct<ErrorSanitizer>,
    O: IvoStruct,
    ErrorSanitizer: IvoErrorSanitizer,
{
    fn resolve<'a>(
        &'a self,
        ctx: IvoContext<I, O>,
        o: IvoRwCtxOptions<CtxOptions>,
    ) -> UniformIgnoreResponse<'a> {
        Box::pin(self(ctx, o))
    }
}

impl<I, O, CtxOptions: Send + Sync, ErrorSanitizer>
    UniformIgnoreResolver<I, O, CtxOptions, ErrorSanitizer>
    for IgnoreUpdateOptionResolver<I, O, CtxOptions>
where
    I: IvoInputStruct<ErrorSanitizer>,
    O: IvoStruct,
    ErrorSanitizer: IvoErrorSanitizer,
{
    fn resolve<'a>(
        &'a self,
        ctx: IvoContext<I, O>,
        o: IvoRwCtxOptions<CtxOptions>,
    ) -> UniformIgnoreResponse<'a> {
        Box::pin(self(ctx.raw_input(), ctx.full_values().unwrap(), o))
    }
}
