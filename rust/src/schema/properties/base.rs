use crate::types::{
    Computable, ComputableRequired, DeleteHandler, FailureHandler, FieldValidatorFn,
    ResolverWithMutSummaryFn, SuccessHandler, VirtualSanitiser,
};
use serde_json::Value;

pub struct IvoProperty<I, O, T = Value> {
    pub is_virtual: bool,
    pub alias: Option<String>,
    pub is_constant: bool,
    pub value: Option<Computable<I, O, T>>,
    pub default: Option<Computable<I, O, T>>,
    pub required: Option<ComputableRequired<I, O>>,
    pub readonly: bool,
    pub depends_on: Option<Vec<String>>,
    pub validator: Option<FieldValidatorFn<I, O, T>>,
    pub re_validator: Option<FieldValidatorFn<I, O, T>>,
    pub sanitizer: Option<VirtualSanitiser<I, O, T>>,
    pub allow_values: Option<Vec<T>>,
    pub allow_error: Option<String>,
    pub resolver: Option<ResolverWithMutSummaryFn<I, O, T>>,
    // life cycle handlers
    pub on_delete_fns: Option<Vec<DeleteHandler<O>>>,
    pub on_failure_fns: Option<Vec<FailureHandler<I, O>>>,
    pub on_success_fns: Option<Vec<SuccessHandler<I, O>>>,
}

impl<I, O, T> Default for IvoProperty<I, O, T> {
    fn default() -> Self {
        Self {
            is_virtual: false,
            alias: None,
            is_constant: false,
            value: None,
            default: None,
            required: None,
            readonly: false,
            depends_on: None,
            validator: None,
            re_validator: None,
            sanitizer: None,
            allow_values: None,
            allow_error: None,
            resolver: None,
            on_delete_fns: None,
            on_success_fns: None,
            on_failure_fns: None,
        }
    }
}
