use std::marker::PhantomData;

use ivo_types::{
    types::{erase_value, ErasedValue},
    DefaultErrorTool, IvoErrorTool,
};

use crate::{
    schema::{
        fields::{
            base::{BuildableFieldConfig, FieldConfig, FieldType, InternalFieldConfig},
            types::{
                IntoDeleteHandler, IntoSuccessHandler, IntoUniformResolver,
                IntoValueResolverWithMiniContext, IsFieldProvisionEnabled, Resolver,
                ValueResolverWithMiniContext,
            },
        },
        types::{
            DeleteHandler, IsProvided, IsProvidedButNotComputed, IvoFieldValue, No, SuccessHandler,
            Yes, YesComputed,
        },
    },
    IvoStruct,
};

pub struct DependentFieldBuilder<
    T: IvoFieldValue,
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool = DefaultErrorTool,
    HasDefault = No,
    HasParents = No,
    HasResolver = No,
    HasReadonly = No,
    HasDelete = No,
    HasSuccess = No,
> {
    _t: PhantomData<T>,
    _err: PhantomData<ErrorTool>,
    _default: PhantomData<HasDefault>,
    _depends_on: PhantomData<HasParents>,
    _resolver: PhantomData<HasResolver>,
    _del_handlers: PhantomData<HasDelete>,
    _readonly: PhantomData<HasReadonly>,
    _success_handlers: PhantomData<HasSuccess>,
    // actual data...
    default: Option<ValueResolverWithMiniContext<ErasedValue, I, CtxOptions>>,
    depends_on: Option<Vec<&'static str>>,
    resolver: Option<Resolver<ErasedValue, I, O, CtxOptions>>,
    ignore_update: Option<IsFieldProvisionEnabled<I, O, CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<
        HasDefault,
        HasParents,
        HasResolver,
        HasReadonly,
        HasDelete,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
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
        HasReadonly,
        HasDelete,
        HasSuccess,
    >
{
    pub const fn new() -> Self {
        Self {
            default: None,
            depends_on: None,
            resolver: None,
            ignore_update: None,
            on_delete_fns: None,
            on_success_fns: None,
            _t: PhantomData,
            _err: PhantomData,
            _default: PhantomData,
            _depends_on: PhantomData,
            _readonly: PhantomData,
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
        HasReadonly,
        HasDelete,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
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
        HasReadonly,
        HasDelete,
        HasSuccess,
    >
{
    fn default() -> Self {
        Self::new()
    }
}

impl<
        HasDefault: IsProvided,
        HasReadonly,
        HasDelete,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > BuildableFieldConfig<I, O, CtxOptions, ErrorTool>
    for DependentFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
        Yes,
        Yes,
        HasReadonly,
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
            ignore_update: self.ignore_update,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

impl<T: IvoFieldValue, I: IvoStruct, O: IvoStruct, CtxOptions, ErrorTool: IvoErrorTool>
    DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool>
{
    pub fn default(self, value: T) -> DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes> {
        DependentFieldBuilder {
            default: Some(ValueResolverWithMiniContext::Static(erase_value(value))),
            ..Default::default()
        }
    }

    pub fn default_fn<F>(
        self,
        default_fn: F,
    ) -> DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool, YesComputed>
    where
        F: IntoValueResolverWithMiniContext<T, I, CtxOptions>,
    {
        DependentFieldBuilder {
            default: Some(ValueResolverWithMiniContext::Func(
                default_fn.into_uniform(),
            )),
            ..Default::default()
        }
    }
}

impl<
        HasDefault: IsProvided,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasDefault>
{
    pub fn depends_on<const N: usize>(
        self,
        fields: [&'static str; N],
    ) -> DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasDefault, Yes> {
        DependentFieldBuilder {
            default: self.default,
            depends_on: Some(Vec::from(fields)),
            ..Default::default()
        }
    }
}

impl<
        HasDefault: IsProvided,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasDefault, Yes>
{
    pub fn resolve<R>(
        self,
        resolver: R,
    ) -> DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasDefault, Yes, Yes>
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
        HasDefault: IsProvidedButNotComputed,
        HasDelete,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
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
        Yes,
        Yes,
        No,
        HasDelete,
        HasSuccess,
    >
{
    /// During updates, the current value of the field is compared with it's
    /// default value. If both values are equal, this field's resolver will be called.
    ///
    /// This rule does not work with computed default values.
    pub fn readonly(
        self,
    ) -> DependentFieldBuilder<
        T,
        I,
        O,
        CtxOptions,
        ErrorTool,
        HasDefault,
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
            ignore_update: Some(IsFieldProvisionEnabled::Readonly),
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<
        HasDefault: IsProvided,
        HasReadonly,
        HasDelete,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
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
        Yes,
        Yes,
        HasReadonly,
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
        HasDefault,
        Yes,
        Yes,
        HasReadonly,
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
            ignore_update: self.ignore_update,
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
        HasDefault: IsProvided,
        HasReadonly,
        HasDelete,
        HasSuccess,
        T: IvoFieldValue,
        I: IvoStruct,
        O: IvoStruct,
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
        Yes,
        Yes,
        HasReadonly,
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
        HasDefault,
        Yes,
        Yes,
        HasReadonly,
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
            ignore_update: self.ignore_update,
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
