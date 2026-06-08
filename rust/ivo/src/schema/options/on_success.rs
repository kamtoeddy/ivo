use std::marker::PhantomData;

use crate::{
    schema::options::types::OnSuccessConfig,
    traits::{IntoSuccessHandler, IvoSchemaStruct},
    types::{No, SuccessHandler, Yes},
};

pub struct OnSuccessOptionBuilder<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    HasFields = No,
    HasHandlers = No,
> {
    _f: PhantomData<HasFields>,
    _h: PhantomData<HasHandlers>,
    // actual data...
    pub fields: Vec<&'static str>,
    pub handlers: Vec<SuccessHandler<I, O, CtxOptions>>,
}

impl<HasFields, HasHandlers, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> Default
    for OnSuccessOptionBuilder<I, O, CtxOptions, HasFields, HasHandlers>
{
    fn default() -> Self {
        Self {
            fields: vec![],
            handlers: vec![],
            _f: PhantomData,
            _h: PhantomData,
        }
    }
}

pub trait BuildableOnSuccess<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    fn build(self) -> OnSuccessConfig<I, O, CtxOptions>;
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> BuildableOnSuccess<I, O, CtxOptions>
    for OnSuccessOptionBuilder<I, O, CtxOptions, Yes, Yes>
{
    fn build(self) -> OnSuccessConfig<I, O, CtxOptions> {
        OnSuccessConfig {
            fields: self.fields,
            handlers: self.handlers,
        }
    }
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    OnSuccessOptionBuilder<I, O, CtxOptions>
{
    pub fn fields<const N: usize>(
        fields: [&'static str; N],
    ) -> OnSuccessOptionBuilder<I, O, CtxOptions, Yes> {
        OnSuccessOptionBuilder {
            fields: Vec::from(fields),
            ..Default::default()
        }
    }
}

impl<HasHandlers, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone>
    OnSuccessOptionBuilder<I, O, CtxOptions, Yes, HasHandlers>
{
    pub fn handle<H>(self, handler: H) -> OnSuccessOptionBuilder<I, O, CtxOptions, Yes, Yes>
    where
        H: IntoSuccessHandler<I, O, CtxOptions>,
    {
        let mut handlers = self.handlers;
        handlers.push(handler.into_handler());

        OnSuccessOptionBuilder {
            fields: self.fields,
            handlers,
            ..Default::default()
        }
    }
}
