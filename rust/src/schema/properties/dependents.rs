use crate::{
    schema::properties::base::IvoProperty,
    types::{ComputableValueWithContext, Context, ResolverFn},
};

pub struct DependentField;

impl DependentField {
    pub fn default<I, O, T>(value: T) -> WithoutParentsBuilder<I, O, T> {
        WithoutParentsBuilder {
            default: ComputableValueWithContext::Static(value),
        }
    }

    pub fn default_fn<I, O, T>(
        default_fn: Box<dyn Fn(&Context<I, O>) -> T + Send + Sync>,
    ) -> WithoutParentsBuilder<I, O, T> {
        WithoutParentsBuilder {
            default: ComputableValueWithContext::Func(default_fn),
        }
    }
}

struct WithoutParentsBuilder<I, O, T> {
    default: ComputableValueWithContext<I, O, T>,
}

impl<I, O, T> WithoutParentsBuilder<I, O, T> {
    pub fn depends_on(self, parents: Vec<&str>) -> WithoutResolverBuilder<I, O, T> {
        WithoutResolverBuilder {
            default: self.default,
            parents: parents
                .clone()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        }
    }
}

struct WithoutResolverBuilder<I, O, T> {
    default: ComputableValueWithContext<I, O, T>,
    parents: Vec<String>,
}

impl<I, O, T> WithoutResolverBuilder<I, O, T> {
    pub fn resolver(self, resolver: ResolverFn<I, O, T>) -> Buildable<I, O, T> {
        Buildable {
            default: self.default,
            parents: self.parents,
            resolver,
            on_delete_fns: None,
            on_success_fns: None,
        }
    }
}

struct Buildable<I, O, T> {
    default: ComputableValueWithContext<I, O, T>,
    parents: Vec<String>,
    resolver: ResolverFn<I, O, T>,
    on_delete_fns: Option<Vec<Box<dyn Fn(&Context<I, O>)>>>,
    on_success_fns: Option<Vec<Box<dyn Fn(&Context<I, O>)>>>,
}

impl<I, O, T> Buildable<I, O, T> {
    pub fn on_delete(mut self, handler: Box<dyn Fn(&Context<I, O>)>) -> Self {
        let mut handlers = self.on_delete_fns.unwrap_or(vec![]);

        handlers.push(handler);

        self.on_delete_fns = Some(handlers);

        self
    }

    pub fn on_success(mut self, handler: Box<dyn Fn(&Context<I, O>)>) -> Self {
        let mut handlers = self.on_success_fns.unwrap_or(vec![]);

        handlers.push(handler);

        self.on_success_fns = Some(handlers);

        self
    }

    pub fn build(self) -> IvoProperty<I, O, T> {
        IvoProperty {
            default: Some(self.default),
            depends_on: Some(self.parents),
            resolver: Some(self.resolver),
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}
