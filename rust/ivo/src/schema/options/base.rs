use std::marker::PhantomData;

use crate::{
    schema::error::IvoErrorTool,
    traits::{IvoSchemaStruct, PostValidationConfig},
    types::{BooleanResolverWithMutSummary, ComputableInit, DeleteHandler, No, SuccessHandler},
};

pub trait BuildableIvoOption<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
>
{
    fn build(self) -> IvoSchemaOption<I, O, CtxOptions, ErrT>;
}

pub struct IvoSchemaOption<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
> {
    //
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,

    pub post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrT>>>,

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

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrT: IvoErrorTool> Default
    for IvoSchemaOption<I, O, CtxOptions, ErrT>
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

pub struct IvoSchemaOptionBuilder<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
    HasIgnore = No,
    HasShouldUpdate = No,
    HasPostValidate = No,
    HasTimestamps = No,
    HasDelete = No,
    HasSuccess = No,
> {
    _on_delete_fns: PhantomData<HasDelete>,
    _on_success_fns: PhantomData<HasSuccess>,
    _post_validate: PhantomData<HasPostValidate>,
    _should_ignore: PhantomData<HasIgnore>,
    _should_update: PhantomData<HasShouldUpdate>,
    _timestaps: PhantomData<HasTimestamps>,
    //
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,

    pub post_validate: Option<Vec<PostValidationConfig<I, O, CtxOptions, ErrT>>>,

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

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrT: IvoErrorTool> Default
    for IvoSchemaOptionBuilder<I, O, CtxOptions, ErrT>
{
    fn default() -> Self {
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
