pub(crate) mod base;
mod on_success;
mod post_validate;
pub(crate) mod types;

use crate::{
    schema::{
        fields::types::{IntoBooleanResolver, IntoDeleteHandler},
        options::types::{
            IgnoreOptionConfig, IgnoreUpdateOptionConfig, IntoRequiredOptionsResolver,
            IntoShouldUpdateOptionResolver, RequiredOptionConfig,
        },
        types::No,
        Yes,
    },
    IvoErrorSanitizer, IvoInputStruct, IvoStruct,
};
use base::{SchemaOptions, SchemaOptionsBuilder};
use on_success::{BuildableOnSuccess, OnSuccessOptionBuilder};
use post_validate::{BuildablePostValidator, PostValidateOptionBuilder};

pub trait BuildableSchemaOptions<
    I: IvoInputStruct<CtxOptions, ErrorSanitizer>,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
>
{
    fn build(self) -> SchemaOptions<I, O, CtxOptions, ErrorSanitizer>;
}

impl<
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnore,
        HasIgnoreUpdate,
        HasRequired,
        I: IvoInputStruct<CtxOptions, ErrorSanitizer>,
        O: IvoStruct,
        CtxOptions,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    > BuildableSchemaOptions<I, O, CtxOptions, ErrorSanitizer>
    for SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnore,
        HasIgnoreUpdate,
        HasRequired,
    >
{
    fn build(self) -> SchemaOptions<I, O, CtxOptions, ErrorSanitizer> {
        SchemaOptions {
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            post_validate: self.post_validate,
            ignore: self.ignore,
            ignore_update: self.ignore_update,
            required: self.required,
        }
    }
}

impl<
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnore,
        HasIgnoreUpdate,
        HasRequired,
        I: IvoInputStruct<CtxOptions, ErrorSanitizer>,
        O: IvoStruct,
        CtxOptions,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    >
    SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnore,
        HasIgnoreUpdate,
        HasRequired,
    >
{
    pub fn ignore<const N: usize, R>(
        self,
        fields: [&'static str; N],
        r: R,
    ) -> SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
        HasPostValidate,
        HasDelete,
        HasSuccess,
        Yes,
        HasIgnoreUpdate,
        HasRequired,
    >
    where
        R: IntoBooleanResolver<I, O, CtxOptions>,
    {
        let mut ignore_configs = self.ignore.unwrap_or_default();

        ignore_configs.push(IgnoreOptionConfig {
            fields: fields.into(),
            resolver: r.into_resolver(),
        });

        SchemaOptionsBuilder::from(
            Some(ignore_configs),
            self.ignore_update,
            self.on_delete_fns,
            self.on_success_fns,
            self.post_validate,
            self.required,
        )
    }

    pub fn on_delete<H>(
        self,
        h: H,
    ) -> SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
        HasPostValidate,
        Yes,
        HasSuccess,
        HasIgnore,
        HasIgnoreUpdate,
        HasRequired,
    >
    where
        H: IntoDeleteHandler<O, CtxOptions>,
    {
        let mut on_delete_fns = self.on_delete_fns.unwrap_or_default();
        on_delete_fns.push(h.into_handler());

        SchemaOptionsBuilder::from(
            self.ignore,
            self.ignore_update,
            Some(on_delete_fns),
            self.on_success_fns,
            self.post_validate,
            self.required,
        )
    }

    pub fn on_success<const N: usize, Builder, Buildable>(
        self,
        fields: [&'static str; N],
        b: Builder,
    ) -> SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
        HasPostValidate,
        HasDelete,
        Yes,
        HasIgnore,
        HasIgnoreUpdate,
        HasRequired,
    >
    where
        Builder: Fn(OnSuccessOptionBuilder<I, O, CtxOptions, Yes>) -> Buildable,
        Buildable: BuildableOnSuccess<I, O, CtxOptions>,
    {
        let config = b(OnSuccessOptionBuilder::<I, O, CtxOptions>::fields(fields)).build();

        let mut on_success_fns = self.on_success_fns.unwrap_or_default();
        on_success_fns.push(config);

        SchemaOptionsBuilder::from(
            self.ignore,
            self.ignore_update,
            self.on_delete_fns,
            Some(on_success_fns),
            self.post_validate,
            self.required,
        )
    }

    pub fn post_validate<const N: usize, Builder, Buildable>(
        self,
        fields: [&'static str; N],
        b: Builder,
    ) -> SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
        Yes,
        HasDelete,
        HasSuccess,
        HasIgnore,
        HasIgnoreUpdate,
        HasRequired,
    >
    where
        Builder: Fn(PostValidateOptionBuilder<I, O, CtxOptions, ErrorSanitizer, Yes>) -> Buildable,
        Buildable: BuildablePostValidator<I, O, CtxOptions, ErrorSanitizer>,
    {
        let config =
            b(PostValidateOptionBuilder::<I, O, CtxOptions, ErrorSanitizer>::fields(fields))
                .build();

        let mut post_validate = self.post_validate.unwrap_or_default();
        post_validate.push(config);

        SchemaOptionsBuilder::from(
            self.ignore,
            self.ignore_update,
            self.on_delete_fns,
            self.on_success_fns,
            Some(post_validate),
            self.required,
        )
    }

    pub fn required<const N: usize, R>(
        self,
        fields: [&'static str; N],
        r: R,
    ) -> SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
        Yes,
        HasDelete,
        HasSuccess,
        HasIgnore,
        HasIgnoreUpdate,
        Yes,
    >
    where
        R: IntoRequiredOptionsResolver<I, O, CtxOptions, ErrorSanitizer>,
    {
        let mut required = self.required.unwrap_or_default();
        required.push(RequiredOptionConfig {
            fields: fields.into(),
            resolver: r.into_resolver(),
        });

        SchemaOptionsBuilder::from(
            self.ignore,
            self.ignore_update,
            self.on_delete_fns,
            self.on_success_fns,
            self.post_validate,
            Some(required),
        )
    }
}

impl<
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnore,
        HasRequired,
        I: IvoInputStruct<CtxOptions, ErrorSanitizer>,
        O: IvoStruct,
        CtxOptions,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    >
    SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnore,
        No,
        HasRequired,
    >
{
    pub fn ignore_update<const N: usize, R>(
        self,
        fields: [&'static str; N],
        r: R,
    ) -> SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorSanitizer,
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnore,
        Yes,
        HasRequired,
    >
    where
        R: IntoShouldUpdateOptionResolver<I, O, CtxOptions>,
    {
        let mut ignore_update = self.ignore_update.unwrap_or_default();
        ignore_update.push(IgnoreUpdateOptionConfig {
            fields: fields.into(),
            resolver: r.into_resolver(),
        });

        SchemaOptionsBuilder::from(
            self.ignore,
            Some(ignore_update),
            self.on_delete_fns,
            self.on_success_fns,
            self.post_validate,
            self.required,
        )
    }
}
