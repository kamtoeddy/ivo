use std::marker::PhantomData;

use crate::{
    schema::{
        error::IvoErrorTool,
        fields::types::{BooleanResolverWithMutSummary, ComputableInit},
        options::types::{OnSuccessConfig, PostValidationConfig},
    },
    types::{DeleteHandler, IvoSchemaStruct, No},
};

pub struct SchemaOptions<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrorTool: IvoErrorTool,
> {
    //
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,

    pub post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,

    pub should_ignore: Option<BooleanResolverWithMutSummary<I, O, CtxOptions>>,
    pub should_update: Option<ComputableInit<I, O, CtxOptions>>,

    // timestamps?:
    //     | boolean
    //     | {
    //         createdAt?: boolean | string;
    //         updatedAt?: boolean | string | { key?: string; nullable?: boolean };
    //     };
    pub timestamps: Option<bool>,
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrorTool: IvoErrorTool> Default
    for SchemaOptions<I, O, CtxOptions, ErrorTool>
{
    fn default() -> Self {
        Self {
            on_delete_fns: None,
            on_success_fns: None,
            post_validate: None,
            should_ignore: None,
            should_update: None,
            timestamps: None,
        }
    }
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrorTool: IvoErrorTool>
    SchemaOptions<I, O, CtxOptions, ErrorTool>
{
    pub const fn new() -> SchemaOptionsBuilder<I, O, CtxOptions, ErrorTool> {
        SchemaOptionsBuilder::new()
    }
}

pub struct SchemaOptionsBuilder<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrorTool: IvoErrorTool,
    HasIgnore = No,
    HasShouldUpdate = No,
    HasPostValidate = No,
    HasTimestamps = No,
    HasDelete = No,
    HasSuccess = No,
> {
    pub _on_delete_fns: PhantomData<HasDelete>,
    pub _on_success_fns: PhantomData<HasSuccess>,
    pub _post_validate: PhantomData<HasPostValidate>,
    pub _should_ignore: PhantomData<HasIgnore>,
    pub _should_update: PhantomData<HasShouldUpdate>,
    pub _timestaps: PhantomData<HasTimestamps>,
    //
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_success_fns: Option<Vec<OnSuccessConfig<I, O, CtxOptions>>>,

    pub post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrorTool>>>,

    pub should_ignore: Option<BooleanResolverWithMutSummary<I, O, CtxOptions>>,
    pub should_update: Option<ComputableInit<I, O, CtxOptions>>,

    // timestamps?:
    //     | boolean
    //     | {
    //         createdAt?: boolean | string;
    //         updatedAt?: boolean | string | { key?: string; nullable?: boolean };
    //     };
    pub timestamps: Option<bool>,
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
        HasTimestamps,
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
            timestamps: None,
            _on_delete_fns: PhantomData,
            _on_success_fns: PhantomData,
            _post_validate: PhantomData,
            _should_ignore: PhantomData,
            _should_update: PhantomData,
            _timestaps: PhantomData,
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
        HasTimestamps,
        HasDelete,
        HasSuccess,
    >
{
    fn default() -> Self {
        Self::new()
    }
}
