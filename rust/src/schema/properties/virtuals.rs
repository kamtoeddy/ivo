use crate::{
    schema::properties::base::IvoProperty,
    types::{Context, FieldValidatorFn, ResolverFn},
};

pub struct VirtualField;

impl VirtualField {
    pub fn validate<I, O, T>(
        validator: FieldValidatorFn<I, O, T>,
    ) -> WithValidatorBuilder<I, O, T> {
        WithValidatorBuilder { validator }
    }
}

struct WithValidatorBuilder<I, O, T> {
    validator: FieldValidatorFn<I, O, T>,
}

impl<I, O, T> WithValidatorBuilder<I, O, T> {
    pub fn re_validate(
        self,
        re_validator: FieldValidatorFn<I, O, T>,
    ) -> WithReValidatorBuilder<I, O, T> {
        WithReValidatorBuilder {
            validator: self.validator,
            re_validator: Some(re_validator),
            sanitizer_fn: None,
            alias_name: None,
            on_failure_fns: None,
            on_success_fns: None,
        }
    }
}

struct WithReValidatorBuilder<I, O, T> {
    validator: FieldValidatorFn<I, O, T>,
    re_validator: Option<FieldValidatorFn<I, O, T>>,
    sanitizer_fn: Option<ResolverFn<I, O, T>>,
    alias_name: Option<String>,
    on_failure_fns: Option<Vec<Box<dyn Fn(&Context<I, O>)>>>,
    on_success_fns: Option<Vec<Box<dyn Fn(&Context<I, O>)>>>,
}

impl<I, O, T> WithReValidatorBuilder<I, O, T> {
    pub fn alias(mut self, name: &str) -> Self {
        self.alias_name = Some(name.to_string());

        self
    }

    pub fn sanitizer(mut self, sanitizer: ResolverFn<I, O, T>) -> Self {
        self.sanitizer_fn = Some(sanitizer);

        self
    }

    pub fn on_success(mut self, handler: Box<dyn Fn(&Context<I, O>)>) -> Self {
        let mut handlers = self.on_success_fns.unwrap_or(vec![]);

        handlers.push(handler);

        self.on_success_fns = Some(handlers);

        self
    }

    pub fn on_failure(mut self, handler: Box<dyn Fn(&Context<I, O>)>) -> Self {
        let mut handlers = self.on_failure_fns.unwrap_or(vec![]);

        handlers.push(handler);

        self.on_failure_fns = Some(handlers);

        self
    }

    pub fn build(self) -> IvoProperty<I, O, T> {
        IvoProperty {
            is_virtual: true,
            validator: Some(self.validator),
            re_validator: self.re_validator,
            sanitizer: self.sanitizer_fn,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}
