use std::collections::HashMap;

pub type DefaultFieldErrorMetadata = ();
pub type DefaultErrorPayload = IvoErrorPayload<DefaultFieldErrorMetadata>;

#[derive(Debug, Clone)]
pub struct FieldError<FieldMetadata: Clone = DefaultFieldErrorMetadata> {
    pub reason: String,
    pub metadata: Option<FieldMetadata>,
}

pub type IvoErrorPayload<FieldMetadata: Clone> = HashMap<String, FieldError<FieldMetadata>>;

#[derive(Debug)]
pub struct DefaultErrorSanitizer<FieldMetadata: Clone = DefaultFieldErrorMetadata> {
    payload: IvoErrorPayload<FieldMetadata>,
}

impl<FieldMetadata: Clone> DefaultErrorSanitizer<FieldMetadata> {
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Self {
            payload: HashMap::new(),
        }
    }

    #[inline(always)]
    pub(crate) fn add(&mut self, field_name: &str, value: FieldError<FieldMetadata>) -> &mut Self {
        self.payload.insert(field_name.to_string(), value);

        self
    }

    #[inline(always)]
    pub(crate) fn has_errors(&self) -> bool {
        !self.payload.is_empty()
    }

    #[inline(always)]
    pub(crate) fn payload(self) -> IvoErrorPayload<FieldMetadata> {
        self.payload
    }
}

impl<FieldMetadata: Clone + Send + Sync> IvoErrorSanitizer
    for DefaultErrorSanitizer<FieldMetadata>
{
    type FieldMetadata = FieldMetadata;
    type ErrorPayload = IvoErrorPayload<Self::FieldMetadata>;

    #[inline(always)]
    fn sanitize<CtxOptions>(
        payload: IvoErrorPayload<Self::FieldMetadata>,
        _: &CtxOptions,
    ) -> Self::ErrorPayload {
        payload
    }
}

pub trait IvoErrorSanitizer {
    type FieldMetadata: Clone + Send + Sync;
    type ErrorPayload;

    fn sanitize<CtxOptions>(
        payload: IvoErrorPayload<Self::FieldMetadata>,
        ctx_options: &CtxOptions,
    ) -> Self::ErrorPayload;
}
