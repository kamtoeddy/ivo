use std::marker::PhantomData;

use crate::{
    schema::{
        fields::{
            base::{BuildableFieldConfig, FieldConfig, FieldType, InternalFieldConfig},
            types::{
                IntoDeleteHandler, IntoSuccessHandler, IntoValueResolverWithSharedInput,
                ValueResolverWithSharedInput,
            },
        },
        types::{DeleteHandler, FieldValue, No, SuccessHandler, Yes},
    },
    types::internal::{
        types::{erase_value, ErasedValue},
        IvoErrorTool,
    },
    IvoStruct,
};

pub struct ConstantFieldBuilder<
    T: FieldValue,
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool<CtxOptions>,
    HasDefault = No,
    HasDelete = No,
    HasSuccess = No,
> {
    value: Option<ValueResolverWithSharedInput<ErasedValue, I, CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
    // markers...
    _t: PhantomData<T>,
    _err: PhantomData<ErrorTool>,
    _default: PhantomData<HasDefault>,
    _del_handlers: PhantomData<HasDelete>,
    _success_handlers: PhantomData<HasSuccess>,
}

impl<
        HasDefault,
        HasDelete,
        HasSuccess,
        I: IvoStruct,
        O: IvoStruct,
        T: FieldValue,
        CtxOptions,
        ErrorTool: IvoErrorTool<CtxOptions>,
    > ConstantFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasDefault, HasDelete, HasSuccess>
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
        I: IvoStruct,
        O: IvoStruct,
        T: FieldValue,
        CtxOptions,
        ErrorTool: IvoErrorTool<CtxOptions>,
    > Default
    for ConstantFieldBuilder<T, I, O, CtxOptions, ErrorTool, HasDefault, HasDelete, HasSuccess>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<
        HasDelete,
        HasSuccess,
        I: IvoStruct,
        O: IvoStruct,
        T: FieldValue,
        CtxOptions,
        ErrorTool: IvoErrorTool<CtxOptions>,
    > BuildableFieldConfig<I, O, CtxOptions, ErrorTool>
    for ConstantFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, HasDelete, HasSuccess>
{
    fn build(self) -> InternalFieldConfig<I, O, CtxOptions, ErrorTool> {
        FieldConfig {
            field_type: FieldType::Constant,
            value: self.value,
            on_delete_fns: self.on_delete_fns,
            on_success_fns: self.on_success_fns,
            ..Default::default()
        }
    }
}

impl<
        I: IvoStruct,
        O: IvoStruct,
        T: FieldValue,
        CtxOptions,
        ErrorTool: IvoErrorTool<CtxOptions>,
    > ConstantFieldBuilder<T, I, O, CtxOptions, ErrorTool>
{
    pub fn value(self, value: T) -> ConstantFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes> {
        ConstantFieldBuilder {
            value: Some(ValueResolverWithSharedInput::Static(erase_value(value))),
            on_delete_fns: None,
            on_success_fns: None,
            ..Default::default()
        }
    }

    pub fn value_fn<F>(
        self,
        resolver: F,
    ) -> ConstantFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes>
    where
        F: IntoValueResolverWithSharedInput<T, I, CtxOptions>,
    {
        ConstantFieldBuilder {
            value: Some(ValueResolverWithSharedInput::Func(resolver.into_uniform())),
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
        I: IvoStruct,
        O: IvoStruct,
        T: FieldValue,
        CtxOptions,
        ErrorTool: IvoErrorTool<CtxOptions>,
    > ConstantFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, HasDelete, HasSuccess>
{
    pub fn on_delete<H>(
        self,
        handler: H,
    ) -> ConstantFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, Yes, HasSuccess>
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
        I: IvoStruct,
        O: IvoStruct,
        T: FieldValue,
        CtxOptions,
        ErrorTool: IvoErrorTool<CtxOptions>,
    > ConstantFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, HasDelete, HasSuccess>
{
    pub fn on_success<H>(
        self,
        handler: H,
    ) -> ConstantFieldBuilder<T, I, O, CtxOptions, ErrorTool, Yes, HasDelete, Yes>
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
