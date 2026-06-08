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

pub mod base;
pub mod on_success;
pub mod post_validate;
pub mod timestamp_tool;
mod types;

pub use types::IvoValues;

pub trait BuildableSchemaOptions<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
>
{
    fn build(self) -> SchemaOptions<I, O, CtxOptions, ErrT>;
}

impl<
        HasIgnore,
        HasShouldUpdate,
        HasPostValidate,
        HasTimestamps,
        HasDelete,
        HasSuccess,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > BuildableSchemaOptions<I, O, CtxOptions, ErrT>
    for SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrT,
        HasIgnore,
        HasShouldUpdate,
        HasPostValidate,
        HasTimestamps,
        HasDelete,
        HasSuccess,
    >
{
    fn build(self) -> SchemaOptions<I, O, CtxOptions, ErrT> {
        SchemaOptions {
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            post_validate: self.post_validate,
            should_ignore: self.should_ignore,
            should_update: self.should_update,
            timestamps: self.timestamps,
        }
    }
}

impl<
        HasIgnore,
        HasShouldUpdate,
        HasPostValidate,
        HasTimestamps,
        HasDelete,
        HasSuccess,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    >
    SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrT,
        HasIgnore,
        HasShouldUpdate,
        HasPostValidate,
        HasTimestamps,
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
        ErrT,
        HasIgnore,
        HasShouldUpdate,
        HasPostValidate,
        HasTimestamps,
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
            timestamps: self.timestamps,
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
        ErrT,
        HasIgnore,
        HasShouldUpdate,
        HasPostValidate,
        HasTimestamps,
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
            timestamps: self.timestamps,
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
        ErrT,
        HasIgnore,
        HasShouldUpdate,
        Yes,
        HasTimestamps,
        HasDelete,
        HasSuccess,
    >
    where
        Builder: Fn(PostValidateOptionBuilder<I, O, CtxOptions, ErrT, Yes>) -> Buildable,
        Buildable: BuildablePostValidator<I, O, CtxOptions, ErrT>,
    {
        let config = builder(PostValidateOptionBuilder::<I, O, CtxOptions, ErrT>::fields(
            fields,
        ))
        .build();

        let mut post_validate = self.post_validate.unwrap_or_default();
        post_validate.push(config);

        SchemaOptionsBuilder {
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            post_validate: Some(post_validate),
            should_ignore: self.should_ignore,
            should_update: self.should_update,
            timestamps: self.timestamps,
            ..SchemaOptionsBuilder::default()
        }
    }
}
