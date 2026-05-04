use crate::types::{
    BooleanResolverWithMutSummary, ComputableInit, ComputableRequired, ComputableWithContext,
    DeleteHandler, FailureHandler, FieldValidatorFn, ResolverWithMutSummaryFn, SuccessHandler,
    VirtualSanitiser,
};
use serde_json::Value;

pub struct IvoProperty<I, O, T = Value> {
    pub alias: Option<String>,
    pub enumerated_error: Option<String>,
    pub enumerated_values: Option<Vec<T>>,
    pub default: Option<ComputableWithContext<I, O, T>>,
    pub depends_on: Option<Vec<String>>,
    pub is_constant: bool,
    pub is_virtual: bool,
    pub value: Option<ComputableWithContext<I, O, T>>,
    pub readonly: bool,
    pub re_validator: Option<FieldValidatorFn<I, O, T>>,
    pub required: Option<ComputableRequired<I, O>>,
    pub resolver: Option<ResolverWithMutSummaryFn<I, O, T>>,
    pub sanitizer: Option<VirtualSanitiser<I, O, T>>,
    pub validator: Option<FieldValidatorFn<I, O, T>>,
    //
    pub should_ignore: Option<BooleanResolverWithMutSummary<I, O>>,
    pub should_init: Option<ComputableInit<I, O>>,
    pub should_update: Option<ComputableInit<I, O>>,
    // life cycle handlers
    pub on_delete_fns: Option<Vec<DeleteHandler<O>>>,
    pub on_failure_fns: Option<Vec<FailureHandler<I, O>>>,
    pub on_success_fns: Option<Vec<SuccessHandler<I, O>>>,
}

impl<I, O, T> Default for IvoProperty<I, O, T> {
    fn default() -> Self {
        Self {
            alias: None,
            is_constant: false,
            is_virtual: false,
            value: None,
            default: None,
            required: None,
            readonly: false,
            depends_on: None,
            validator: None,
            re_validator: None,
            sanitizer: None,
            enumerated_values: None,
            enumerated_error: None,
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
