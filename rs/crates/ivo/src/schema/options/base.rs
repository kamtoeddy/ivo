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
    pub should_update: Option<IsFieldProvisionEnabled<I, O, CtxOptions>>,
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
            should_update: None,
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
    HasShouldUpdate = No,
    HasPostValidate = No,
    HasDelete = No,
    HasSuccess = No,
> {
    pub _on_delete_fns: PhantomData<HasDelete>,
    pub _on_success_fns: PhantomData<HasSuccess>,
    pub _post_validate: PhantomData<HasPostValidate>,
    pub _should_ignore: PhantomData<HasIgnore>,
    pub _should_update: PhantomData<HasShouldUpdate>,
    //
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,

    pub post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,

    pub should_ignore: Option<BooleanResolver<I, O, CtxOptions>>,
    pub should_update: Option<IsFieldProvisionEnabled<I, O, CtxOptions>>,
}

impl<
        HasIgnore,
        HasShouldUpdate,
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
        HasShouldUpdate,
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
            should_update: None,
            _on_delete_fns: PhantomData,
            _on_success_fns: PhantomData,
            _post_validate: PhantomData,
            _should_ignore: PhantomData,
            _should_update: PhantomData,
        }
    }
}

impl<
        HasIgnore,
        HasShouldUpdate,
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
        HasShouldUpdate,
        HasPostValidate,
        HasDelete,
        HasSuccess,
    >
{
    fn default() -> Self {
        Self::new()
    }
}
