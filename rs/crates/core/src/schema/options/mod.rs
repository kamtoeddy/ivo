pub(crate) mod base;
mod on_success;
mod post_validate;
pub(crate) mod types;

use crate::{
    schema::{fields::types::IntoDeleteHandler, options::types::IntoShouldUpdateResolver, Yes},
    IvoErrorTool, IvoStruct,
};
use base::{SchemaOptions, SchemaOptionsBuilder};
use on_success::{BuildableOnSuccess, OnSuccessOptionBuilder};
use post_validate::{BuildablePostValidator, PostValidateOptionBuilder};

pub trait BuildableSchemaOptions<I: IvoStruct, O: IvoStruct, CtxOptions, ErrorTool: IvoErrorTool> {
    fn build(self) -> SchemaOptions<I, O, CtxOptions, ErrorTool>;
}

impl<
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnoreUpdate,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > BuildableSchemaOptions<I, O, CtxOptions, ErrorTool>
    for SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnoreUpdate,
    >
{
    fn build(self) -> SchemaOptions<I, O, CtxOptions, ErrorTool> {
        SchemaOptions {
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            post_validate: self.post_validate,
            ignore_update: self.ignore_update,
        }
    }
}

impl<
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnoreUpdate,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnoreUpdate,
    >
{
    pub fn on_delete<H>(
        self,
        handler: H,
    ) -> SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasPostValidate,
        Yes,
        HasSuccess,
        HasIgnoreUpdate,
    >
    where
        H: IntoDeleteHandler<O, CtxOptions>,
    {
        let mut on_delete_fns = self.on_delete_fns.unwrap_or_default();
        on_delete_fns.push(handler.into_handler());

        SchemaOptionsBuilder::from(
            Some(on_delete_fns),
            self.on_success_fns,
            self.post_validate,
            self.ignore_update,
        )
    }

    pub fn on_success<const N: usize, Builder, Buildable>(
        self,
        fields: [&'static str; N],
        builder: Builder,
    ) -> SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasPostValidate,
        HasDelete,
        Yes,
        HasIgnoreUpdate,
    >
    where
        Builder: Fn(OnSuccessOptionBuilder<I, O, CtxOptions, Yes>) -> Buildable,
        Buildable: BuildableOnSuccess<I, O, CtxOptions>,
    {
        let config = builder(OnSuccessOptionBuilder::<I, O, CtxOptions>::fields(fields)).build();

        let mut on_success_fns = self.on_success_fns.unwrap_or_default();
        on_success_fns.push(config);

        SchemaOptionsBuilder::from(
            self.on_delete_fns,
            Some(on_success_fns),
            self.post_validate,
            self.ignore_update,
        )
    }

    pub fn post_validate<const N: usize, Builder, Buildable>(
        self,
        fields: [&'static str; N],
        builder: Builder,
    ) -> SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        HasDelete,
        HasSuccess,
        HasIgnoreUpdate,
    >
    where
        Builder: Fn(PostValidateOptionBuilder<I, O, CtxOptions, ErrorTool, Yes>) -> Buildable,
        Buildable: BuildablePostValidator<I, O, CtxOptions, ErrorTool>,
    {
        let config =
            builder(PostValidateOptionBuilder::<I, O, CtxOptions, ErrorTool>::fields(fields))
                .build();

        let mut post_validate = self.post_validate.unwrap_or_default();
        post_validate.push(config);

        SchemaOptionsBuilder::from(
            self.on_delete_fns,
            self.on_success_fns,
            Some(post_validate),
            self.ignore_update,
        )
    }
}

impl<
        HasPostValidate,
        HasDelete,
        HasSuccess,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > SchemaOptionsBuilder<I, O, CtxOptions, ErrorTool, HasPostValidate, HasDelete, HasSuccess>
{
    pub fn ignore_update<R>(
        self,
        handler: R,
    ) -> SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasPostValidate,
        HasDelete,
        HasSuccess,
        Yes,
    >
    where
        R: IntoShouldUpdateResolver<I, O, CtxOptions>,
    {
        SchemaOptionsBuilder::from(
            self.on_delete_fns,
            self.on_success_fns,
            self.post_validate,
            Some(handler.into_resolver()),
        )
    }
}
