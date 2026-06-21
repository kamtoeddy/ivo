use std::marker::PhantomData;

use crate::{
    schema::{
        fields::types::IntoSuccessHandler, options::types::OnSuccessConfig, types::SuccessHandler,
        No, Yes,
    },
    IvoSchemaStruct,
};

pub struct OnSuccessOptionBuilder<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    IvoFieldNames = No,
    HasHandlers = No,
> {
    _f: PhantomData<IvoFieldNames>,
    _h: PhantomData<HasHandlers>,
    // actual data...
    fields: Vec<&'static str>,
    handlers: Vec<SuccessHandler<I, O, CtxOptions>>,
}

impl<IvoFieldNames, HasHandlers, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> Default
    for OnSuccessOptionBuilder<I, O, CtxOptions, IvoFieldNames, HasHandlers>
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

pub trait BuildableOnSuccess<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> {
    fn build(self) -> OnSuccessConfig<I, O, CtxOptions>;
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> BuildableOnSuccess<I, O, CtxOptions>
    for OnSuccessOptionBuilder<I, O, CtxOptions, Yes, Yes>
{
    fn build(self) -> OnSuccessConfig<I, O, CtxOptions> {
        OnSuccessConfig {
            fields: self.fields,
            handlers: self.handlers,
        }
    }
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> OnSuccessOptionBuilder<I, O, CtxOptions> {
    pub fn fields<const N: usize>(
        fields: [&'static str; N],
    ) -> OnSuccessOptionBuilder<I, O, CtxOptions, Yes> {
        OnSuccessOptionBuilder {
            fields: Vec::from(fields),
            ..Default::default()
        }
    }
}

impl<HasHandlers, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions>
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
