pub mod base;
pub mod on_success;
pub mod post_validate;
pub mod timestamp_tool;
pub(crate) mod types;

use crate::{
    schema::{
        fields::types::IntoDeleteHandler,
        options::{
            base::{SchemaOptions, SchemaOptionsBuilder},
            on_success::{BuildableOnSuccess, OnSuccessOptionBuilder},
            post_validate::{BuildablePostValidator, PostValidateOptionBuilder},
        },
    },
    types::Yes,
    IvoErrorTool, IvoSchemaStruct,
};

pub use types::{IvoValues, PostValidatorResponse};

pub trait BuildableSchemaOptions<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
>
{
    fn build(self) -> SchemaOptions<I, O, CtxOptions, ErrorTool>;
}

impl<
        HasIgnore,
        HasShouldUpdate,
        HasPostValidate,
        HasDelete,
        HasSuccess,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > BuildableSchemaOptions<I, O, CtxOptions, ErrorTool>
    for SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasIgnore,
        HasShouldUpdate,
        HasPostValidate,
        HasDelete,
        HasSuccess,
    >
{
    fn build(self) -> SchemaOptions<I, O, CtxOptions, ErrorTool> {
        SchemaOptions {
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            post_validate: self.post_validate,
            should_ignore: self.should_ignore,
            should_update: self.should_update,
        }
    }
}

impl<
        HasIgnore,
        HasShouldUpdate,
        HasPostValidate,
        HasDelete,
        HasSuccess,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasIgnore,
        HasShouldUpdate,
        HasPostValidate,
        HasDelete,
        HasSuccess,
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
        HasIgnore,
        HasShouldUpdate,
        HasPostValidate,
        Yes,
        HasSuccess,
    >
    where
        H: IntoDeleteHandler<O, CtxOptions>,
    {
        let mut on_delete_fns = self.on_delete_fns.unwrap_or_default();
        on_delete_fns.push(handler.into_handler());

        SchemaOptionsBuilder {
            on_delete_fns: Some(on_delete_fns),
            on_success_fns: self.on_success_fns,
            post_validate: self.post_validate,
            should_ignore: self.should_ignore,
            should_update: self.should_update,
            ..SchemaOptionsBuilder::default()
        }
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
        HasIgnore,
        HasShouldUpdate,
        HasPostValidate,
        HasDelete,
        Yes,
    >
    where
        Builder: Fn(OnSuccessOptionBuilder<I, O, CtxOptions, Yes>) -> Buildable,
        Buildable: BuildableOnSuccess<I, O, CtxOptions>,
    {
        let config = builder(OnSuccessOptionBuilder::<I, O, CtxOptions>::fields(fields)).build();

        let mut on_success_fns = self.on_success_fns.unwrap_or_default();
        on_success_fns.push(config);

        SchemaOptionsBuilder {
            on_delete_fns: self.on_delete_fns,
            on_success_fns: Some(on_success_fns),
            post_validate: self.post_validate,
            should_ignore: self.should_ignore,
            should_update: self.should_update,
            ..SchemaOptionsBuilder::default()
        }
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
        HasIgnore,
        HasShouldUpdate,
        Yes,
        HasDelete,
        HasSuccess,
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

        SchemaOptionsBuilder {
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            post_validate: Some(post_validate),
            should_ignore: self.should_ignore,
            should_update: self.should_update,
            ..SchemaOptionsBuilder::default()
        }
    }
}
