use crate::{
    schema::properties::base::IvoProperty,
    types::{
        Computable, ComputableRequired, DeleteHandler, RequiredResolverFn, ResolverWithContextFn,
        ResolverWithMutSummaryFn, SuccessHandler,
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
    HasRequired,
    HasDelete,
    HasSuccess,
> {
    _default: std::marker::PhantomData<HasDefault>,
    _parents: std::marker::PhantomData<HasParents>,
    _resolver: std::marker::PhantomData<HasResolver>,
    _readonly: std::marker::PhantomData<HasReadonly>,
    _required: std::marker::PhantomData<HasRequired>,
    _del_handlers: std::marker::PhantomData<HasDelete>,
    _success_handlers: std::marker::PhantomData<HasSuccess>,
    // actual data...
    default: Option<Computable<I, O, T>>,
    depends_on: Option<Vec<String>>,
    resolver: Option<ResolverWithMutSummaryFn<I, O, T>>,
    readonly: bool,
    required: Option<ComputableRequired<I, O>>,
    on_delete_fns: Option<Vec<DeleteHandler<O>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O>>>,
}

impl<
        HasDefault,
        HasParents,
        HasResolver,
        HasReadonly,
        HasRequired,
        HasDelete,
        HasSuccess,
        I,
        O,
        T,
    > Default
    for SchemaBuilder<
        I,
        O,
        T,
        HasDefault,
        HasParents,
        HasResolver,
        HasReadonly,
        HasRequired,
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
            required: None,
            on_delete_fns: None,
            on_success_fns: None,
            _default: std::marker::PhantomData,
            _parents: std::marker::PhantomData,
            _readonly: std::marker::PhantomData,
            _resolver: std::marker::PhantomData,
            _required: std::marker::PhantomData,
            _del_handlers: std::marker::PhantomData,
            _success_handlers: std::marker::PhantomData,
        }
    }
}

impl<HasReadonly, HasRequired, HasDelete, HasSuccess, I, O, T>
    SchemaBuilder<I, O, T, Yes, Yes, Yes, HasReadonly, HasRequired, HasDelete, HasSuccess>
{
    pub fn build(self) -> IvoProperty<I, O, T> {
        IvoProperty {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            readonly: self.readonly,
            required: self.required,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

impl DependentField {
    pub fn default<I, O, T>(value: T) -> SchemaBuilder<I, O, T, Yes, No, No, No, No, No, No> {
        SchemaBuilder {
            default: Some(Computable::Static(value)),
            ..Default::default()
        }
    }

    pub fn default_fn<I, O, T>(
        default_fn: ResolverWithContextFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, No, No, No, No, No, No> {
        SchemaBuilder {
            default: Some(Computable::Func(default_fn)),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, No, No, No, No, No, No> {
    pub fn depends_on(
        self,
        parents: &[&str],
    ) -> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No, No> {
        SchemaBuilder {
            default: self.default,
            depends_on: Some(
                parents
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            ),
            ..Default::default()
        }
    }
}

impl<I, O, T> SchemaBuilder<I, O, T, Yes, Yes, No, No, No, No, No> {
    pub fn resolver(
        self,
        resolver: ResolverWithMutSummaryFn<I, O, T>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, No, No, No, No> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: Some(resolver),
            ..Default::default()
        }
    }
}

impl<HasRequired, HasDelete, HasSuccess, I, O, T>
    SchemaBuilder<I, O, T, Yes, Yes, Yes, No, HasRequired, HasDelete, HasSuccess>
{
    pub fn readonly(
        self,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, Yes, HasRequired, HasDelete, HasSuccess> {
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
impl<HasReadonly, HasRequired, HasSuccess, I, O, T>
    SchemaBuilder<I, O, T, Yes, Yes, Yes, HasReadonly, HasRequired, No, HasSuccess>
{
    pub fn on_delete(
        self,
        handler: DeleteHandler<O>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, HasReadonly, HasRequired, Yes, HasSuccess> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            readonly: self.readonly,
            required: self.required,
            on_delete_fns: Some(vec![handler]),
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }

    pub fn on_delete_fns(
        self,
        handlers: Vec<DeleteHandler<O>>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, HasReadonly, HasRequired, Yes, HasSuccess> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            readonly: self.readonly,
            required: self.required,
            on_delete_fns: Some(handlers),
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

// ON_SUCCESS is only available if HasSuccess is 'No'
impl<HasReadonly, HasRequired, HasDelete, I, O, T>
    SchemaBuilder<I, O, T, Yes, Yes, Yes, HasReadonly, HasRequired, HasDelete, No>
{
    pub fn on_success(
        self,
        handler: SuccessHandler<I, O>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, HasReadonly, HasRequired, HasDelete, Yes> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            readonly: self.readonly,
            required: self.required,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: Some(vec![handler]),
            ..Default::default()
        }
    }

    pub fn on_success_fns(
        self,
        handlers: Vec<SuccessHandler<I, O>>,
    ) -> SchemaBuilder<I, O, T, Yes, Yes, Yes, HasReadonly, HasRequired, HasDelete, Yes> {
        SchemaBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            readonly: self.readonly,
            required: self.required,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: Some(handlers),
            ..Default::default()
        }
    }
}
