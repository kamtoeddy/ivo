#![expect(clippy::new_ret_no_self)]

use std::marker::PhantomData;

use crate::{
    schema::{
        error_tool::IvoErrorTool,
        fields::types::BooleanResolver,
        options::types::{OnSuccessConfig, PostValidationConfig},
        types::DeleteHandler,
        No,
    },
    types::IvoStruct,
};

pub struct SchemaOptions<I: IvoStruct, O: IvoStruct, CtxOptions, ErrorTool: IvoErrorTool> {
    //
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,

    pub post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,

    pub ignore: Option<BooleanResolver<I, O, CtxOptions>>,
    pub ignore_update: Option<bool>,
}

impl<I: IvoStruct, O: IvoStruct, CtxOptions, ErrorTool: IvoErrorTool> Default
    for SchemaOptions<I, O, CtxOptions, ErrorTool>
{
    fn default() -> Self {
        Self {
            on_delete_fns: None,
            on_success_fns: None,
            post_validate: None,
            ignore: None,
            ignore_update: None,
        }
    }
}

impl<I: IvoStruct, O: IvoStruct, CtxOptions, ErrorTool: IvoErrorTool>
    SchemaOptions<I, O, CtxOptions, ErrorTool>
{
    pub const fn new() -> SchemaOptionsBuilder<I, O, CtxOptions, ErrorTool> {
        SchemaOptionsBuilder::new()
    }
}

pub struct SchemaOptionsBuilder<
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
    HasPostValidate = No,
    HasDelete = No,
    HasSuccess = No,
    HasIgnore = No,
    HasIgnoreUpdate = No,
> {
    _on_delete_fns: PhantomData<HasDelete>,
    _on_success_fns: PhantomData<HasSuccess>,
    _post_validate: PhantomData<HasPostValidate>,
    _ignore: PhantomData<HasIgnore>,
    _ignore_update: PhantomData<HasIgnoreUpdate>,
    //
    pub(crate) on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub(crate) on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,

    pub(crate) post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,

    pub(crate) ignore: Option<BooleanResolver<I, O, CtxOptions>>,
    pub(crate) ignore_update: Option<bool>,
}

impl<
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnore,
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
        HasIgnore,
        HasIgnoreUpdate,
    >
{
    pub const fn new() -> Self {
        Self {
            on_delete_fns: None,
            on_success_fns: None,
            post_validate: None,
            ignore: None,
            ignore_update: None,
            _on_delete_fns: PhantomData,
            _on_success_fns: PhantomData,
            _post_validate: PhantomData,
            _ignore: PhantomData,
            _ignore_update: PhantomData,
        }
    }

    pub fn from(
        on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
        on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,
        post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,
        ignore: Option<BooleanResolver<I, O, CtxOptions>>,
        ignore_update: Option<bool>,
    ) -> Self {
        Self {
            on_delete_fns,
            on_success_fns,
            post_validate,
            ignore,
            ignore_update,
            ..Default::default()
        }
    }
}

impl<
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnore,
        HasIgnoreUpdate,
        I: IvoStruct,
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
        HasIgnore,
        HasIgnoreUpdate,
    >
{
    fn default() -> Self {
        Self::new()
    }
}
