#![expect(clippy::new_ret_no_self)]

use std::marker::PhantomData;

use crate::schema::{
    options::types::{OnSuccessConfig, PostValidationConfig, ShouldUpdateOptionResolver},
    types::DeleteHandler,
    No,
};
use crate::types::internal::{IvoErrorTool, IvoStruct};

pub struct SchemaOptions<I: IvoStruct, O: IvoStruct, CtxOptions, ErrorTool: IvoErrorTool> {
    pub ignore_update: Option<ShouldUpdateOptionResolver<I, O, CtxOptions>>,
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,
    pub post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,
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
    HasIgnoreUpdate = No,
> {
    _on_delete_fns: PhantomData<HasDelete>,
    _on_success_fns: PhantomData<HasSuccess>,
    _post_validate: PhantomData<HasPostValidate>,
    _ignore_update: PhantomData<HasIgnoreUpdate>,
    //
    pub(crate) ignore_update: Option<ShouldUpdateOptionResolver<I, O, CtxOptions>>,
    pub(crate) on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub(crate) on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,
    pub(crate) post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,
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
    pub const fn new() -> Self {
        Self {
            ignore_update: None,
            on_delete_fns: None,
            on_success_fns: None,
            post_validate: None,
            _on_delete_fns: PhantomData,
            _on_success_fns: PhantomData,
            _post_validate: PhantomData,
            _ignore_update: PhantomData,
        }
    }

    pub fn from(
        ignore_update: Option<ShouldUpdateOptionResolver<I, O, CtxOptions>>,
        on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
        on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,
        post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,
    ) -> Self {
        Self {
            ignore_update,
            on_delete_fns,
            on_success_fns,
            post_validate,
            ..Default::default()
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
    >
{
    fn default() -> Self {
        Self::new()
    }
}
