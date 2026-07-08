#![expect(clippy::new_ret_no_self)]

use std::marker::PhantomData;

use crate::schema::options::types::RequiredOptionConfig;
use crate::types::internal::IvoStruct;
use crate::{
    __private_types::types::IvoWithPartialErrorsStruct,
    schema::{
        options::types::{IgnoreUpdateOptionResolver, OnSuccessConfig, PostValidationConfig},
        types::DeleteHandler,
        No,
    },
    IvoErrorTool,
};

pub struct SchemaOptions<
    I: IvoStruct + IvoWithPartialErrorsStruct<ErrorTool::FieldMetadata>,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
> {
    pub ignore_update: Option<IgnoreUpdateOptionResolver<I, O, CtxOptions>>,
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,
    pub post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,
    pub required: Option<Vec<RequiredOptionConfig<I, O, CtxOptions, ErrorTool>>>,
}

impl<
        I: IvoStruct + IvoWithPartialErrorsStruct<ErrorTool::FieldMetadata>,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > SchemaOptions<I, O, CtxOptions, ErrorTool>
{
    pub const fn new() -> SchemaOptionsBuilder<I, O, CtxOptions, ErrorTool> {
        SchemaOptionsBuilder::new()
    }
}

pub struct SchemaOptionsBuilder<
    I: IvoStruct + IvoWithPartialErrorsStruct<ErrorTool::FieldMetadata>,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
    HasPostValidate = No,
    HasDelete = No,
    HasSuccess = No,
    HasIgnoreUpdate = No,
    HasRequired = No,
> {
    _on_delete_fns: PhantomData<HasDelete>,
    _on_success_fns: PhantomData<HasSuccess>,
    _post_validate: PhantomData<HasPostValidate>,
    _ignore_update: PhantomData<HasIgnoreUpdate>,
    _required: PhantomData<HasRequired>,
    //
    pub(crate) ignore_update: Option<IgnoreUpdateOptionResolver<I, O, CtxOptions>>,
    pub(crate) on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub(crate) on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,
    pub(crate) post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,
    pub(crate) required: Option<Vec<RequiredOptionConfig<I, O, CtxOptions, ErrorTool>>>,
}

impl<
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnoreUpdate,
        HasRequired,
        I: IvoStruct + IvoWithPartialErrorsStruct<ErrorTool::FieldMetadata>,
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
        HasRequired,
    >
{
    pub const fn new() -> Self {
        Self {
            ignore_update: None,
            on_delete_fns: None,
            on_success_fns: None,
            post_validate: None,
            required: None,
            _on_delete_fns: PhantomData,
            _on_success_fns: PhantomData,
            _post_validate: PhantomData,
            _ignore_update: PhantomData,
            _required: PhantomData,
        }
    }

    pub fn from(
        ignore_update: Option<IgnoreUpdateOptionResolver<I, O, CtxOptions>>,
        on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
        on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,
        post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,
        required: Option<Vec<RequiredOptionConfig<I, O, CtxOptions, ErrorTool>>>,
    ) -> Self {
        Self {
            ignore_update,
            on_delete_fns,
            on_success_fns,
            post_validate,
            required,
            ..Default::default()
        }
    }
}

impl<
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnoreUpdate,
        HasRequired,
        I: IvoStruct + IvoWithPartialErrorsStruct<ErrorTool::FieldMetadata>,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > Default
    for SchemaOptionsBuilder<
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnoreUpdate,
        HasRequired,
    >
{
    fn default() -> Self {
        Self::new()
    }
}
