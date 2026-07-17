use std::collections::HashMap;

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
    payload: IvoErrorPayload<Metadata>,
}

impl<Metadata: Clone> DefaultErrorSanitizer<Metadata> {
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Self {
            payload: HashMap::new(),
        }
    }

    #[inline(always)]
    pub(crate) fn add(&mut self, field_name: &str, value: FieldError<Metadata>) -> &mut Self {
        self.payload.insert(field_name.to_string(), value);

        self
    }

    #[inline(always)]
    pub(crate) fn has_errors(&self) -> bool {
        !self.payload.is_empty()
    }

    #[inline(always)]
    pub(crate) fn payload(self) -> IvoErrorPayload<Metadata> {
        self.payload
    }
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
