use std::{fmt::Debug, marker::PhantomData};

use crate::{
    schema::{
        error::{DefaultErrorTool, IvoErrorTool},
        fields::{
            base::{BuildableFieldConfig, FieldConfig, FieldType, InternalFieldConfig},
            types::{
                ComputableInit, ComputableWithMiniContext, IntoDeleteHandler,
                IntoResolverWithMiniContext, IntoSuccessHandler, IntoUniformResolver, Resolver,
            },
        },
    },
    types::{DeleteHandler, False, No, SuccessHandler, Yes},
    utils::erased_value::{erase_value, ErasedValue},
    IvoSchemaStruct,
};

pub struct DependentFieldBuilder<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool = DefaultErrorTool,
    HasDefault = No,
    HasParents = No,
    HasResolver = No,
    HasShouldUpdate = No,
    HasDelete = No,
    HasSuccess = No,
> {
    _t: PhantomData<T>,
    _err: PhantomData<ErrorTool>,
    _default: PhantomData<HasDefault>,
    _depends_on: PhantomData<HasParents>,
    _resolver: PhantomData<HasResolver>,
    _del_handlers: PhantomData<HasDelete>,
    _should_update: PhantomData<HasShouldUpdate>,
    _success_handlers: PhantomData<HasSuccess>,
    // actual data...
    default: Option<ComputableWithMiniContext<ErasedValue, I, CtxOptions>>,
    depends_on: Option<Vec<&'static str>>,
    resolver: Option<Resolver<ErasedValue, I, O, CtxOptions>>,
    should_update: Option<False>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<
        HasDefault,
        HasParents,
        HasResolver,
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    DependentFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        HasParents,
        HasResolver,
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
    >
{
    pub const fn new() -> Self {
        Self {
            default: None,
            depends_on: None,
            resolver: None,
            should_update: None,
            on_delete_fns: None,
            on_success_fns: None,
            _t: PhantomData,
            _err: PhantomData,
            _default: PhantomData,
            _depends_on: PhantomData,
            _should_update: PhantomData,
            _resolver: PhantomData,
            _del_handlers: PhantomData,
            _success_handlers: PhantomData,
        }
    }
}

impl<
        HasDefault,
        HasParents,
        HasResolver,
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > Default
    for DependentFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        HasParents,
        HasResolver,
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
    >
{
    fn default() -> Self {
        Self::new()
    }
}

impl<
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > BuildableFieldConfig<I, O, CtxOptions, ErrorTool>
    for DependentFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        Yes,
        Yes,
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
    >
{
    fn build(self) -> InternalFieldConfig<I, O, CtxOptions, ErrorTool> {
        FieldConfig {
            field_type: FieldType::Dependent,
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            should_update: if self.should_update.is_some() {
                Some(ComputableInit::False)
            } else {
                None
            },
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

impl<
        T: Clone + Debug + Send + Sync + 'static,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool>
{
    pub fn default(self, value: T) -> DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes> {
        DependentFieldBuilder {
            default: Some(ComputableWithMiniContext::Static(erase_value(value))),
            ..Default::default()
        }
    }

    pub fn default_fn<F>(
        self,
        default_fn: F,
    ) -> DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes>
    where
        F: IntoResolverWithMiniContext<T, I, CtxOptions>,
    {
        DependentFieldBuilder {
            default: Some(ComputableWithMiniContext::Func(default_fn.into_uniform())),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions, ErrorTool: IvoErrorTool>
    DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes>
{
    pub fn depends_on<const N: usize>(
        self,
        fields: [&'static str; N],
    ) -> DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, Yes> {
        DependentFieldBuilder {
            default: self.default,
            depends_on: Some(Vec::from(fields)),
            ..Default::default()
        }
    }
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions, ErrorTool: IvoErrorTool>
    DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, Yes>
{
    pub fn resolve<R>(
        self,
        resolver: R,
    ) -> DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, Yes, Yes>
    where
        R: IntoUniformResolver<T, I, O, CtxOptions>,
    {
        DependentFieldBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: Some(resolver.into_uniform()),
            ..Default::default()
        }
    }
}

impl<
        HasDelete,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, Yes, Yes, No, HasDelete, HasSuccess>
{
    pub fn readonly(
        self,
    ) -> DependentFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        Yes,
        Yes,
        Yes,
        HasDelete,
        HasSuccess,
    > {
        DependentFieldBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            should_update: Some(False),
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    DependentFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        Yes,
        Yes,
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
    >
{
    pub fn on_delete<H>(
        self,
        handler: H,
    ) -> DependentFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        Yes,
        Yes,
        HasShouldUpdate,
        Yes,
        HasSuccess,
    >
    where
        H: IntoDeleteHandler<O, CtxOptions>,
    {
        let h = handler.into_handler();

        DependentFieldBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            should_update: self.should_update,
            on_delete_fns: Some(match self.on_delete_fns {
                Some(hs) => {
                    let mut v = hs;

                    v.push(h);

                    v
                }
                _ => vec![h],
            }),
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

// ON_SUCCESS is only available if HasSuccess is 'No'
impl<
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
        T,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    >
    DependentFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        Yes,
        Yes,
        HasShouldUpdate,
        HasDelete,
        HasSuccess,
    >
{
    pub fn on_success<H>(
        self,
        handler: H,
    ) -> DependentFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        Yes,
        Yes,
        Yes,
        HasShouldUpdate,
        HasDelete,
        Yes,
    >
    where
        H: IntoSuccessHandler<I, O, CtxOptions>,
    {
        let h = handler.into_handler();

        DependentFieldBuilder {
            default: self.default,
            depends_on: self.depends_on,
            resolver: self.resolver,
            should_update: self.should_update,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: Some(match self.on_success_fns {
                Some(hs) => {
                    let mut v = hs;

                    v.push(h);

                    v
                }
                _ => vec![h],
            }),
            ..Default::default()
        }
    }
}
