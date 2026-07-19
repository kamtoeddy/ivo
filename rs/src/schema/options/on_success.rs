use std::marker::PhantomData;

use crate::{
    schema::{
        fields::types::IntoSuccessHandler, options::types::OnSuccessConfig, types::SuccessHandler,
        No, Yes,
    },
    IvoStruct,
};

pub struct OnSuccessOptionBuilder<
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    IvoFieldNames = No,
    HasHandlers = No,
> {
    fields: Vec<&'static str>,
    handlers: Vec<SuccessHandler<I, O, CtxOptions>>,
    // markers...
    _f: PhantomData<IvoFieldNames>,
    _h: PhantomData<HasHandlers>,
}

impl<IvoFieldNames, HasHandlers, I: IvoStruct, O: IvoStruct, CtxOptions> Default
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

pub trait BuildableOnSuccess<I: IvoStruct, O: IvoStruct, CtxOptions> {
    fn build(self) -> OnSuccessConfig<I, O, CtxOptions>;
}

impl<I: IvoStruct, O: IvoStruct, CtxOptions> BuildableOnSuccess<I, O, CtxOptions>
    for OnSuccessOptionBuilder<I, O, CtxOptions, Yes, Yes>
{
    fn build(self) -> OnSuccessConfig<I, O, CtxOptions> {
        OnSuccessConfig {
            fields: self.fields,
            handlers: self.handlers,
        }
    }
}

impl<I: IvoStruct, O: IvoStruct, CtxOptions> OnSuccessOptionBuilder<I, O, CtxOptions> {
    pub fn fields<const N: usize>(
        fields: [&'static str; N],
    ) -> OnSuccessOptionBuilder<I, O, CtxOptions, Yes> {
        OnSuccessOptionBuilder {
            fields: Vec::from(fields),
            ..Default::default()
        }
    }
}

impl<HasHandlers, I: IvoStruct, O: IvoStruct, CtxOptions>
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
