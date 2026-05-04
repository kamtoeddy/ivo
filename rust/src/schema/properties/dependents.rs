use crate::{
    schema::properties::base::IvoProperty,
    types::{
        ComputableWithContext, DeleteHandler, ResolverWithContextFn, ResolverWithMutSummaryFn,
        SuccessHandler,
    },
};

pub struct DependentField;

// Marker Types
pub struct Yes;
pub struct No;

struct SchemaBuilder<
    I,
    O,
    T,
    HasDefault,
    HasParents,
    HasResolver,
    HasReadonly,
    HasDelete,
    HasSuccess,
> {
    _default: std::marker::PhantomData<HasDefault>,
    _parents: std::marker::PhantomData<HasParents>,
    _resolver: std::marker::PhantomData<HasResolver>,
    _readonly: std::marker::PhantomData<HasReadonly>,
    _del_handlers: std::marker::PhantomData<HasDelete>,
    _success_handlers: std::marker::PhantomData<HasSuccess>,
    // actual data...
    default: Option<ComputableWithContext<I, O, T>>,
    depends_on: Option<Vec<String>>,
    resolver: Option<ResolverWithMutSummaryFn<I, O, T>>,
    readonly: bool,
    on_delete_fns: Option<Vec<DeleteHandler<O>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O>>>,
}

impl<HasDefault, HasParents, HasResolver, HasReadonly, HasDelete, HasSuccess, I, O, T> Default
    for SchemaBuilder<
        I,
        O,
        T,
        HasDefault,
        HasParents,
        HasResolver,
        HasReadonly,
        HasDelete,
        HasSuccess,
    >
{
    fn default() -> Self {
        Self {
            default: None,
            depends_on: None,
            resolver: None,
            readonly: false,
            on_delete_fns: None,
            on_success_fns: None,
            _default: std::marker::PhantomData,
            _parents: std::marker::PhantomData,
            _readonly: std::marker::PhantomData,
            _resolver: std::marker::PhantomData,
            _del_handlers: std::marker::PhantomData,
            _success_handlers: std::marker::PhantomData,
        }
    }
}

impl<HasReadonly, HasDelete, HasSuccess, I, O, T>
    SchemaBuilder<I, O, T, Yes, Yes, Yes, HasReadonly, HasDelete, HasSuccess>
{
    pub fn build(self) -> IvoProperty<I, O, T> {
        IvoProperty {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            readonly: self.readonly,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

impl DependentField {
    pub fn default<I, O, T>(value: T) -> SchemaBuilder<I, O, T, Yes, No, No, No, No, No> {
        SchemaBuilder {
            default: Some(ComputableWithContext::Static(value)),
            ..Default::default()
        }
    }

    pub fn default_fn<I, O, T>(
        default_fn: ResolverWithContextFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, No, No, No, No, No> {
        SchemaBuilder {
            default: Some(ComputableWithContext::Func(default_fn)),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, No, No, No, No, No> {
    pub fn depends_on(self, fields: &[&str]) -> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No> {
        SchemaBuilder {
            default: self.default,
            depends_on: Some(
                fields
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            ),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No> {
    pub fn resolve(
        self,
        resolver: ResolverWithMutSummaryFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, No, No> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: Some(resolver),
            ..Default::default()
        }
    }
}

impl<HasDelete, HasSuccess, I, O, T>
    SchemaBuilder<I, O, T, Yes, Yes, Yes, No, HasDelete, HasSuccess>
{
    pub fn readonly(self) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, Yes, HasDelete, HasSuccess> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            readonly: true,
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<HasReadonly, HasSuccess, I, O, T>
    SchemaBuilder<I, O, T, Yes, Yes, Yes, HasReadonly, No, HasSuccess>
{
    pub fn on_delete(
        self,
        handler: DeleteHandler<O>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, HasReadonly, Yes, HasSuccess> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            readonly: self.readonly,
            on_delete_fns: Some(vec![handler]),
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }

    pub fn on_delete_fns(
        self,
        handlers: Vec<DeleteHandler<O>>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, HasReadonly, Yes, HasSuccess> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            readonly: self.readonly,
            on_delete_fns: Some(handlers),
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

// ON_SUCCESS is only available if HasSuccess is 'No'
impl<HasReadonly, HasDelete, I, O, T>
    SchemaBuilder<I, O, T, Yes, Yes, Yes, HasReadonly, HasDelete, No>
{
    pub fn on_success(
        self,
        handler: SuccessHandler<I, O>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, HasReadonly, HasDelete, Yes> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            readonly: self.readonly,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: Some(vec![handler]),
            ..Default::default()
        }
    }

    pub fn on_success_fns(
        self,
        handlers: Vec<SuccessHandler<I, O>>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, HasReadonly, HasDelete, Yes> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            readonly: self.readonly,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: Some(handlers),
            ..Default::default()
        }
    }
}
