use crate::types::{ComputableValueWithContext, Context, FieldValidatorFn, ResolverFn};
use serde_json::Value;

pub struct IvoProperty<I, O, T = Value> {
    pub is_virtual: bool,
    pub alias: Option<String>,
    pub is_constant: bool,
    pub value: Option<ComputableValueWithContext<I, O, T>>,
    pub default: Option<ComputableValueWithContext<I, O, T>>,
    pub required: Option<bool>,
    pub readonly: bool,
    pub depends_on: Option<Vec<String>>,
    pub validator: Option<FieldValidatorFn<I, O, T>>,
    pub re_validator: Option<FieldValidatorFn<I, O, T>>,
    pub sanitizer: Option<ResolverFn<I, O, T>>,
    pub allow_values: Option<Vec<T>>,
    pub allow_error: Option<String>,
    pub resolver: Option<ResolverFn<I, O, T>>,
    // life cycle handlers
    pub on_delete_fns: Option<Vec<Box<dyn Fn(&Context<I, O>)>>>,
    pub on_failure_fns: Option<Vec<Box<dyn Fn(&Context<I, O>)>>>,
    pub on_success_fns: Option<Vec<Box<dyn Fn(&Context<I, O>)>>>,
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
