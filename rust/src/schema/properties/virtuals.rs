use crate::{
    schema::properties::base::IvoProperty,
    types::{
        ComputableRequired, FailureHandler, FieldValidatorFn, RequiredResolverFn, SuccessHandler,
        VirtualSanitiser,
    },
};

pub struct VirtualField;

impl VirtualField {
    pub fn validator<I, O, T>(
        validator: FieldValidatorFn<I, O, T>,
    ) -> WithValidatorBuilder<I, O, T> {
        WithValidatorBuilder { validator }
    }
}

struct WithValidatorBuilder<I, O, T> {
    validator: FieldValidatorFn<I, O, T>,
}

impl<I, O, T> WithValidatorBuilder<I, O, T> {
    pub fn required(
        self,
        required_fn: RequiredResolverFn<I, O>,
    ) -> WithReValidatorBuilder<I, O, T> {
        WithReValidatorBuilder {
            validator: self.validator,
            required_fn: Some(required_fn),
            re_validator: None,
            sanitizer_fn: None,
            alias_name: None,
            on_failure_fns: None,
            on_success_fns: None,
        }
    }

    pub fn re_validator(
        self,
        re_validator: FieldValidatorFn<I, O, T>,
    ) -> WithReValidatorBuilder<I, O, T> {
        WithReValidatorBuilder {
            validator: self.validator,
            re_validator: Some(re_validator),
            required_fn: None,
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
    required_fn: Option<RequiredResolverFn<I, O>>,
    sanitizer_fn: Option<VirtualSanitiser<I, O, T>>,
    alias_name: Option<String>,
    on_failure_fns: Option<Vec<FailureHandler<I, O>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O>>>,
}

impl<I, O, T> WithReValidatorBuilder<I, O, T> {
    pub fn alias(mut self, name: &str) -> Self {
        self.alias_name = Some(name.to_string());

        self
    }

    pub fn required(mut self, fx: RequiredResolverFn<I, O>) -> Self {
        self.required_fn = Some(fx);

        self
    }

    pub fn sanitizer(mut self, fx: VirtualSanitiser<I, O, T>) -> Self {
        self.sanitizer_fn = Some(fx);

        self
    }

    pub fn on_success(mut self, handler: SuccessHandler<I, O>) -> Self {
        let mut handlers = self.on_success_fns.unwrap_or(vec![]);

        handlers.push(handler);

        self.on_success_fns = Some(handlers);

        self
    }

    pub fn on_failure(mut self, handler: FailureHandler<I, O>) -> Self {
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
            required: match self.required_fn {
                Some(fx) => Some(ComputableRequired::Func(fx)),
                _ => None,
            },
            sanitizer: self.sanitizer_fn,
            on_failure_fns: self.on_failure_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}
