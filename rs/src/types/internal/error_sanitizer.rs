use std::{collections::HashMap, marker::PhantomData};

pub type DefaultFieldErrorMetadata = ();
pub type DefaultErrorPayload = IvoErrorPayload<DefaultFieldErrorMetadata>;

#[derive(Debug, Clone)]
pub struct FieldError<Metadata: Clone = DefaultFieldErrorMetadata> {
    pub reason: String,
    pub metadata: Option<Metadata>,
}

pub type IvoErrorPayload<Metadata: Clone> = HashMap<String, FieldError<Metadata>>;

#[derive(Debug)]
pub struct DefaultErrorSanitizer<Metadata: Clone = DefaultFieldErrorMetadata> {
    _metadata: PhantomData<Metadata>,
}

impl<Metadata: Clone + Send + Sync> IvoErrorSanitizer for DefaultErrorSanitizer<Metadata> {
    type Metadata = Metadata;
    type Payload = IvoErrorPayload<Self::Metadata>;

    #[inline(always)]
    fn sanitize<CtxOptions>(
        payload: IvoErrorPayload<Self::Metadata>,
        _: &CtxOptions,
    ) -> Self::Payload {
        payload
    }
}

pub trait IvoErrorSanitizer {
    type Metadata: Clone + Send + Sync;
    type Payload;

    fn sanitize<CtxOptions>(
        payload: IvoErrorPayload<Self::Metadata>,
        ctx_options: &CtxOptions,
    ) -> Self::Payload;
}
