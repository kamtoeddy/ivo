use std::marker::PhantomData;

use serde_json::Value;

use crate::{
    traits::IvoSchemaStruct,
    types::{
        BooleanResolverWithMutSummary, ComputableEnumeratedError, ComputableInit,
        ComputableRequired, ComputableWithMiniSummary, DeleteHandler, FailureHandler,
        FieldReValidator, FieldValidator, ResolverWithMutSummary, SuccessHandler, VirtualSanitiser,
    },
};

pub trait IvoPropertyBuilder<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn build(self) -> InternalIvoProperty<I, O, CtxOptions>;
}

pub type InternalIvoProperty<I, O, CtxOptions> = IvoProperty<Value, I, O, CtxOptions>;

pub struct IvoProperty<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    // _d: PhantomData<T>,
    pub _i: PhantomData<I>,
    pub alias: Option<String>,
    pub enum_error: Option<ComputableEnumeratedError>,
    pub enum_values: Option<Vec<T>>,
    pub default: Option<ComputableWithMiniSummary<T, CtxOptions>>,
    pub depends_on: Option<Vec<String>>,
    pub is_constant: bool,
    pub is_readonly: bool,
    pub is_virtual: bool,
    pub value: Option<ComputableWithMiniSummary<T, CtxOptions>>,
    pub required: Option<ComputableRequired<CtxOptions>>,
    pub resolver: Option<ResolverWithMutSummary<T, CtxOptions>>,
    pub sanitizer: Option<VirtualSanitiser<T, CtxOptions>>,
    pub validator: Option<FieldValidator<CtxOptions>>,
    pub re_validator: Option<FieldReValidator<CtxOptions>>,
    //
    pub should_ignore: Option<BooleanResolverWithMutSummary<CtxOptions>>,
    pub should_init: Option<ComputableInit<CtxOptions>>,
    pub should_update: Option<ComputableInit<CtxOptions>>,
    // life cycle handlers
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_failure_fns: Option<Vec<FailureHandler<CtxOptions>>>,
    pub on_success_fns: Option<Vec<SuccessHandler<CtxOptions>>>,
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> Default
    for IvoProperty<T, I, O, CtxOptions>
{
    fn default() -> Self {
        Self {
            alias: None,
            is_constant: false,
            is_virtual: false,
            value: None,
            default: None,
            required: None,
            is_readonly: false,
            depends_on: None,
            validator: None,
            re_validator: None,
            sanitizer: None,
            enum_values: None,
            enum_error: None,
            resolver: None,
            should_ignore: None,
            should_init: None,
            should_update: None,
            on_delete_fns: None,
            on_success_fns: None,
            on_failure_fns: None,
            _i: PhantomData,
        }
    }
}
