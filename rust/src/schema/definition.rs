use serde_json::Value;
use std::collections::HashMap;

pub type Context = HashMap<String, Value>;

pub enum DefaultValue {
    Static(Value),
    Func(Box<dyn Fn(&Context) -> Value + Send + Sync>),
}

pub enum ConstantValue {
    Static(Value),
    Func(Box<dyn Fn(&Context) -> Value + Send + Sync>),
}

pub type ValidatorFn =
    Box<dyn Fn(&Value, &Context) -> crate::validators::ValidationResponse<Value> + Send + Sync>;

pub struct PropertyDefinition {
    pub virtual_prop: bool,
    pub alias: Option<String>,
    pub constant: bool,
    pub value: Option<ConstantValue>,
    pub default: Option<DefaultValue>,
    pub required: Option<bool>,
    pub readonly: Option<bool>,
    pub depends_on: Option<Vec<String>>,
    pub validator: Option<Vec<ValidatorFn>>,
    pub allow_values: Option<Vec<Value>>,
    pub allow_error: Option<String>,
}

impl PropertyDefinition {
    pub fn builder() -> PropertyBuilder {
        PropertyBuilder::default()
    }
}

#[derive(Default)]
pub struct PropertyBuilder {
    virtual_prop: bool,
    alias: Option<String>,
    constant: bool,
    value: Option<ConstantValue>,
    default: Option<DefaultValue>,
    required: Option<bool>,
    readonly: Option<bool>,
    depends_on: Option<Vec<String>>,
    validator: Option<Vec<ValidatorFn>>,
    allow_values: Option<Vec<Value>>,
    allow_error: Option<String>,
}

impl PropertyBuilder {
    pub fn virtual_prop(mut self, v: bool) -> Self {
        self.virtual_prop = v;
        self
    }
    pub fn alias(mut self, a: impl Into<String>) -> Self {
        self.alias = Some(a.into());
        self
    }
    pub fn constant_static(mut self, v: Value) -> Self {
        self.constant = true;
        self.value = Some(ConstantValue::Static(v));
        self
    }
    pub fn constant_func<F: Fn(&Context) -> Value + Send + Sync + 'static>(mut self, f: F) -> Self {
        self.constant = true;
        self.value = Some(ConstantValue::Func(Box::new(f)));
        self
    }
    pub fn default_static(mut self, v: Value) -> Self {
        self.default = Some(DefaultValue::Static(v));
        self
    }
    pub fn default_func<F: Fn(&Context) -> Value + Send + Sync + 'static>(mut self, f: F) -> Self {
        self.default = Some(DefaultValue::Func(Box::new(f)));
        self
    }
    pub fn required(mut self, r: bool) -> Self {
        self.required = Some(r);
        self
    }
    pub fn readonly(mut self, r: bool) -> Self {
        self.readonly = Some(r);
        self
    }
    pub fn depends_on(mut self, deps: Vec<String>) -> Self {
        self.depends_on = Some(deps);
        self
    }
    pub fn validator(mut self, v: Vec<ValidatorFn>) -> Self {
        self.validator = Some(v);
        self
    }

    pub fn allow_values(mut self, vals: Vec<Value>) -> Self {
        self.allow_values = Some(vals);
        self
    }

    pub fn allow_error(mut self, e: impl Into<String>) -> Self {
        self.allow_error = Some(e.into());
        self
    }

    pub fn build(self) -> PropertyDefinition {
        PropertyDefinition {
            virtual_prop: self.virtual_prop,
            alias: self.alias,
            constant: self.constant,
            value: self.value,
            default: self.default,
            required: self.required,
            readonly: self.readonly,
            depends_on: self.depends_on,
            validator: self.validator,
            allow_values: self.allow_values,
            allow_error: self.allow_error,
        }
    }
}
