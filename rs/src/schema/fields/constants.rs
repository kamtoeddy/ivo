use std::marker::PhantomData;

use crate::{
    schema::{
        fields::{
            base::{BuildableFieldConfig, FieldConfig, FieldType, InternalFieldConfig},
            types::{
                ConstantValue, IntoConstantValueResolver, IntoDeleteHandler, IntoSuccessHandler,
            },
        },
        types::{DeleteHandler, FieldValue, No, SuccessHandler, Yes},
    },
    types::internal::{
        types::{erase_value, ErasedValue},
        IvoErrorSanitizer,
    },
    IvoStruct,
};

pub struct ConstantFieldBuilder<
    T: FieldValue,
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    HasDefault = No,
    HasDelete = No,
    HasSuccess = No,
> {
    name: &'static str,
    value: Option<ConstantValue<ErasedValue, I, O, CtxOptions>>,
    on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
    // markers...
    _t: PhantomData<T>,
    _err: PhantomData<ErrorSanitizer>,
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
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    > ConstantFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, HasDefault, HasDelete, HasSuccess>
{
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
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
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    > Default
    for ConstantFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, HasDefault, HasDelete, HasSuccess>
{
    fn default() -> Self {
        Self::new("")
    }
}

impl<
        HasDelete,
        HasSuccess,
        I: IvoStruct,
        O: IvoStruct,
        T: FieldValue,
        CtxOptions,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    > BuildableFieldConfig<I, O, CtxOptions, ErrorSanitizer>
    for ConstantFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, Yes, HasDelete, HasSuccess>
{
    fn build(self) -> InternalFieldConfig<I, O, CtxOptions, ErrorSanitizer> {
        FieldConfig {
            name: self.name,
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
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    > ConstantFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer>
{
    pub fn value(self, value: T) -> ConstantFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, Yes> {
        ConstantFieldBuilder {
            name: self.name,
            value: Some(ConstantValue::Static(erase_value(value))),
            on_delete_fns: None,
            on_success_fns: None,
            ..Default::default()
        }
    }

    pub fn value_fn<F>(
        self,
        resolver: F,
    ) -> ConstantFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, Yes>
    where
        F: IntoConstantValueResolver<T, I, O, CtxOptions>,
    {
        ConstantFieldBuilder {
            name: self.name,
            value: Some(ConstantValue::Func(resolver.into_uniform())),
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
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    > ConstantFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, Yes, HasDelete, HasSuccess>
{
    pub fn on_delete<H>(
        self,
        handler: H,
    ) -> ConstantFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, Yes, Yes, HasSuccess>
    where
        H: IntoDeleteHandler<O, CtxOptions>,
    {
        let h = handler.into_handler();

        ConstantFieldBuilder {
            name: self.name,
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
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    > ConstantFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, Yes, HasDelete, HasSuccess>
{
    pub fn on_success<H>(
        self,
        handler: H,
    ) -> ConstantFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, Yes, HasDelete, Yes>
    where
        H: IntoSuccessHandler<I, O, CtxOptions>,
    {
        let h = handler.into_handler();

        ConstantFieldBuilder {
            name: self.name,
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
