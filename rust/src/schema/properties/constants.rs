use crate::{
    schema::properties::base::IvoProperty,
    types::{Computable, Context, DeleteHandler, SuccessHandler},
};

pub struct ConstantField;

impl ConstantField {
    pub fn value<I, O, T>(value: T) -> Buildable<I, O, T> {
        Buildable {
            value: Computable::Static(value),
            on_delete_fns: None,
            on_success_fns: None,
        }
    }

    pub fn computed<I, O, T>(
        compute_fn: Box<dyn Fn(&Context<I, O>) -> T + Send + Sync>,
    ) -> Buildable<I, O, T> {
        Buildable {
            value: Computable::Func(compute_fn),
            on_delete_fns: None,
            on_success_fns: None,
        }
    }
}

struct Buildable<I, O, T> {
    value: Computable<I, O, T>,
    on_delete_fns: Option<Vec<DeleteHandler<O>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O>>>,
}

impl<I, O, T> Buildable<I, O, T> {
    pub fn on_delete(mut self, handler: DeleteHandler<O>) -> Self {
        let mut handlers = self.on_delete_fns.unwrap_or(vec![]);

        handlers.push(handler);

        self.on_delete_fns = Some(handlers);

        self
    }

    pub fn on_success(mut self, handler: SuccessHandler<I, O>) -> Self {
        let mut handlers = self.on_success_fns.unwrap_or(vec![]);

        handlers.push(handler);

        self.on_success_fns = Some(handlers);

        self
    }

    pub fn build(self) -> IvoProperty<I, O, T> {
        IvoProperty {
            value: Some(self.value),
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}
