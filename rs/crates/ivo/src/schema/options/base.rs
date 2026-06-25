#![expect(clippy::new_ret_no_self)]

use std::marker::PhantomData;

use crate::{
    schema::{
        error_tool::IvoErrorTool,
        fields::types::{BooleanResolver, IsFieldProvisionEnabled},
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

    pub should_ignore: Option<BooleanResolver<I, O, CtxOptions>>,
    pub ignore_update: Option<IsFieldProvisionEnabled<I, O, CtxOptions>>,
}

impl<I: IvoStruct, O: IvoStruct, CtxOptions, ErrorTool: IvoErrorTool> Default
    for SchemaOptions<I, O, CtxOptions, ErrorTool>
{
    fn default() -> Self {
        Self {
            on_delete_fns: None,
            on_success_fns: None,
            post_validate: None,
            should_ignore: None,
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
    HasIgnore = No,
    HasIgnoreUpdate = No,
    HasPostValidate = No,
    HasDelete = No,
    HasSuccess = No,
> {
    pub _on_delete_fns: PhantomData<HasDelete>,
    pub _on_success_fns: PhantomData<HasSuccess>,
    pub _post_validate: PhantomData<HasPostValidate>,
    pub _should_ignore: PhantomData<HasIgnore>,
    pub _ignore_update: PhantomData<HasIgnoreUpdate>,
    //
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,

    pub post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,

    pub should_ignore: Option<BooleanResolver<I, O, CtxOptions>>,
    pub ignore_update: Option<IsFieldProvisionEnabled<I, O, CtxOptions>>,
}

impl<
        HasIgnore,
        HasIgnoreUpdate,
        HasPostValidate,
        HasDelete,
        HasSuccess,
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
        HasIgnore,
        HasIgnoreUpdate,
        HasPostValidate,
        HasDelete,
        HasSuccess,
    >
{
    pub const fn new() -> Self {
        Self {
            on_delete_fns: None,
            on_success_fns: None,
            post_validate: None,
            should_ignore: None,
            ignore_update: None,
            _on_delete_fns: PhantomData,
            _on_success_fns: PhantomData,
            _post_validate: PhantomData,
            _should_ignore: PhantomData,
            _ignore_update: PhantomData,
        }
    }
}

impl<
        HasIgnore,
        HasIgnoreUpdate,
        HasPostValidate,
        HasDelete,
        HasSuccess,
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
        HasIgnore,
        HasIgnoreUpdate,
        HasPostValidate,
        HasDelete,
        HasSuccess,
    >
{
    fn default() -> Self {
        Self::new()
    }
}
