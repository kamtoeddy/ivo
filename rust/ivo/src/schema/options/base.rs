use std::marker::PhantomData;

use crate::{
    schema::error::IvoErrorTool,
    traits::IvoSchemaStruct,
    types::{BooleanResolverWithMutSummary, ComputableInit, DeleteHandler, SuccessHandler},
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
    _err: PhantomData<ErrT>,

    //
    pub should_ignore: Option<BooleanResolverWithMutSummary<I, O, CtxOptions>>,
    pub should_update: Option<ComputableInit<I, O, CtxOptions>>,

    // life cycle handlers
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrT: IvoErrorTool> Default
    for IvoSchemaOption<I, O, CtxOptions, ErrT>
{
    fn default() -> Self {
        Self {
            should_ignore: None,
            should_update: None,
            on_delete_fns: None,
            on_success_fns: None,
            _err: PhantomData,
        }
    }
}
