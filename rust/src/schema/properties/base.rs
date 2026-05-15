use crate::{
    traits::HasPartial,
    types::{
        BooleanResolverWithMutSummary, ComputableEnumeratedError, ComputableInit,
        ComputableRequired, ComputableWithMiniSummary, DeleteHandler, FailureHandler,
        FieldValidator, ResolverWithMutSummary, SuccessHandler, VirtualSanitiser,
    },
};

pub struct IvoProperty<T, I: HasPartial, O: HasPartial, CtxOptions> {
    pub alias: Option<String>,
    pub enum_error: Option<ComputableEnumeratedError<T>>,
    pub enum_values: Option<Vec<T>>,
    pub default: Option<ComputableWithMiniSummary<T, CtxOptions>>,
    pub depends_on: Option<Vec<String>>,
    pub is_constant: bool,
    pub is_readonly: bool,
    pub is_virtual: bool,
    pub value: Option<ComputableWithMiniSummary<T, CtxOptions>>,
    pub required: Option<ComputableRequired<I, O, CtxOptions>>,
    pub resolver: Option<ResolverWithMutSummary<T, I, O, CtxOptions>>,
    pub sanitizer: Option<VirtualSanitiser<T, I, O, CtxOptions>>,
    pub validator: Option<FieldValidator<T, I, O, CtxOptions>>,
    pub re_validator: Option<FieldValidator<T, I, O, CtxOptions>>,
    //
    pub should_ignore: Option<BooleanResolverWithMutSummary<I, O, CtxOptions>>,
    pub should_init: Option<ComputableInit<I, O, CtxOptions>>,
    pub should_update: Option<ComputableInit<I, O, CtxOptions>>,
    // life cycle handlers
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_failure_fns: Option<Vec<FailureHandler<I, O, CtxOptions>>>,
    pub on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<T, I: HasPartial, O: HasPartial, CtxOptions> Default for IvoProperty<T, I, O, CtxOptions> {
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
        }
    }
}
