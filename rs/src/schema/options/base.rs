#![expect(clippy::new_ret_no_self)]

use std::marker::PhantomData;

use crate::__private_types::IvoInputStruct;
use crate::schema::options::types::{
    IgnoreOptionConfig, IgnoreUpdateOptionConfig, RequiredOptionConfig,
};
use crate::types::internal::IvoStruct;
use crate::{
    schema::{
        options::types::{OnSuccessConfig, PostValidationConfig},
        types::DeleteHandler,
        No,
    },
    IvoErrorTool,
};

pub struct SchemaOptions<
    I: IvoInputStruct<ErrorTool>,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
> {
    pub ignore: Option<Vec<IgnoreOptionConfig<I, O, CtxOptions>>>,
    pub ignore_update: Option<Vec<IgnoreUpdateOptionConfig<I, O, CtxOptions>>>,
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,
    pub post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,
    pub required: Option<Vec<RequiredOptionConfig<I, O, CtxOptions, ErrorTool>>>,
}

impl<I: IvoInputStruct<ErrorTool>, O: IvoStruct, CtxOptions, ErrorTool: IvoErrorTool>
    SchemaOptions<I, O, CtxOptions, ErrorTool>
{
    pub const fn new() -> SchemaOptionsBuilder<I, O, CtxOptions, ErrorTool> {
        SchemaOptionsBuilder::new()
    }
}

pub struct SchemaOptionsBuilder<
    I: IvoInputStruct<ErrorTool>,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
    HasPostValidate = No,
    HasDelete = No,
    HasSuccess = No,
    HasIgnore = No,
    HasIgnoreUpdate = No,
    HasRequired = No,
> {
    _on_delete_fns: PhantomData<HasDelete>,
    _on_success_fns: PhantomData<HasSuccess>,
    _post_validate: PhantomData<HasPostValidate>,
    _ignore: PhantomData<HasIgnore>,
    _ignore_update: PhantomData<HasIgnoreUpdate>,
    _required: PhantomData<HasRequired>,
    //
    pub(crate) ignore: Option<Vec<IgnoreOptionConfig<I, O, CtxOptions>>>,
    pub(crate) ignore_update: Option<Vec<IgnoreUpdateOptionConfig<I, O, CtxOptions>>>,
    pub(crate) on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub(crate) on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,
    pub(crate) post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,
    pub(crate) required: Option<Vec<RequiredOptionConfig<I, O, CtxOptions, ErrorTool>>>,
}

impl<
        HasPostValidate,
        HasDelete,
        HasSuccess,
        HasIgnore,
        HasIgnoreUpdate,
        HasRequired,
        I: IvoInputStruct<ErrorTool>,
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
        HasRequired,
    >
{
    pub const fn new() -> Self {
        Self {
            ignore: None,
            ignore_update: None,
            on_delete_fns: None,
            on_success_fns: None,
            post_validate: None,
            required: None,
            _on_delete_fns: PhantomData,
            _on_success_fns: PhantomData,
            _post_validate: PhantomData,
            _ignore: PhantomData,
            _ignore_update: PhantomData,
            _required: PhantomData,
        }
    }

    pub fn from(
        ignore: Option<Vec<IgnoreOptionConfig<I, O, CtxOptions>>>,
        ignore_update: Option<Vec<IgnoreUpdateOptionConfig<I, O, CtxOptions>>>,
        on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
        on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,
        post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,
        required: Option<Vec<RequiredOptionConfig<I, O, CtxOptions, ErrorTool>>>,
    ) -> Self {
        Self {
            ignore,
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
        HasIgnore,
        HasIgnoreUpdate,
        HasRequired,
        I: IvoInputStruct<ErrorTool>,
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
        HasRequired,
    >
{
    fn default() -> Self {
        Self::new()
    }
}
