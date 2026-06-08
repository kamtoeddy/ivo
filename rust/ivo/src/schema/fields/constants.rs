use std::{fmt::Debug, marker::PhantomData};

use crate::{
    schema::{
        error::IvoErrorTool,
        fields::base::{BuildableFieldConfig, FieldConfig, InternalFieldConfig},
    },
    traits::{IntoDeleteHandler, IntoResolverWithMiniSummary, IntoSuccessHandler, IvoSchemaStruct},
    types::{ComputableWithMiniSummary, DeleteHandler, No, SuccessHandler, Yes},
    utils::erased_value::{erase_value, ErasedValue},
};

pub struct ConstantFieldBuilder<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
    HasDefault = No,
    HasDelete = No,
    HasSuccess = No,
> {
    _t: PhantomData<T>,
    _err: PhantomData<ErrT>,
    _default: PhantomData<HasDefault>,
    _del_handlers: PhantomData<HasDelete>,
    _success_handlers: PhantomData<HasSuccess>,
    // actual data...
    value: Option<ComputableWithMiniSummary<ErasedValue, I, O, CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<
        HasDefault,
        HasDelete,
        HasSuccess,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        T,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > ConstantFieldBuilder<T, I, O, CtxOptions, ErrT, HasDefault, HasDelete, HasSuccess>
{
    pub const fn new() -> Self {
        Self {
            value: None,
            on_delete_fns: None,
            on_success_fns: None,
            _t: PhantomData,
            _err: PhantomData,
            _default: PhantomData,
            _del_handlers: PhantomData,
            _success_handlers: PhantomData,
        }
    }
}

impl<
        HasDefault,
        HasDelete,
        HasSuccess,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        T,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > Default
    for ConstantFieldBuilder<T, I, O, CtxOptions, ErrT, HasDefault, HasDelete, HasSuccess>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<
        HasDelete,
        HasSuccess,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        T: Clone + Debug + Send + Sync + 'static,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > BuildableFieldConfig<I, O, CtxOptions, ErrT>
    for ConstantFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, HasDelete, HasSuccess>
{
    fn build(self) -> InternalFieldConfig<I, O, CtxOptions, ErrT> {
        FieldConfig {
            value: self.value,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

impl<
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        T: Clone + Debug + Send + Sync + 'static,
        CtxOptions: Clone + Send,
        ErrT: IvoErrorTool,
    > ConstantFieldBuilder<T, I, O, CtxOptions, ErrT>
{
    pub fn value(self, value: T) -> ConstantFieldBuilder<T, I, O, CtxOptions, ErrT, Yes> {
        ConstantFieldBuilder {
            value: Some(ComputableWithMiniSummary::Static(erase_value(value))),
            on_delete_fns: None,
            on_success_fns: None,
            ..Default::default()
        }
    }

    pub fn computed<F>(self, resolver: F) -> ConstantFieldBuilder<T, I, O, CtxOptions, ErrT, Yes>
    where
        F: IntoResolverWithMiniSummary<T, I, O, CtxOptions>,
    {
        ConstantFieldBuilder {
            value: Some(ComputableWithMiniSummary::Func(resolver.into_uniform())),
            on_delete_fns: None,
            on_success_fns: None,
            ..Default::default()
        }
    }
}

// ON_DELETE is only available if HasDelete is 'No'
impl<
        HasDelete,
        HasSuccess,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        T,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > ConstantFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, HasDelete, HasSuccess>
{
    pub fn on_delete<H>(
        self,
        handler: H,
    ) -> ConstantFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, Yes, HasSuccess>
    where
        H: IntoDeleteHandler<O, CtxOptions>,
    {
        let h = handler.into_handler();

        ConstantFieldBuilder {
            value: self.value,
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
        HasDelete,
        HasSuccess,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        T,
        CtxOptions: Clone,
        ErrT: IvoErrorTool,
    > ConstantFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, HasDelete, HasSuccess>
{
    pub fn on_success<H>(
        self,
        handler: H,
    ) -> ConstantFieldBuilder<T, I, O, CtxOptions, ErrT, Yes, HasDelete, Yes>
    where
        H: IntoSuccessHandler<I, O, CtxOptions>,
    {
        let h = handler.into_handler();

        ConstantFieldBuilder {
            value: self.value,
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
