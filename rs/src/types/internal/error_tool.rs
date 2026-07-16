use std::collections::HashMap;

use crate::__private_types::types::DefaultCtxOptions;

pub type DefaultFieldErrorMetadata = ();
pub type DefaultErrorPayload = IvoErrorPayload<DefaultFieldErrorMetadata>;

#[derive(Debug, Clone)]
pub struct FieldError<FieldMetadata: Clone = DefaultFieldErrorMetadata> {
    pub reason: String,
    pub metadata: Option<FieldMetadata>,
}

pub type IvoErrorPayload<FieldMetadata: Clone> = HashMap<String, FieldError<FieldMetadata>>;

#[derive(Debug)]
pub struct DefaultErrorTool<FieldMetadata: Clone = DefaultFieldErrorMetadata> {
    payload: IvoErrorPayload<FieldMetadata>,
}

impl<FieldMetadata: Clone> DefaultErrorTool<FieldMetadata> {
    pub(crate) fn new() -> Self {
        Self {
            payload: HashMap::new(),
        }
    }

    pub(crate) fn add(&mut self, field_name: &str, value: FieldError<FieldMetadata>) -> &mut Self {
        self.payload.insert(field_name.to_string(), value);

        self
    }

    #[inline]
    pub(crate) fn has_errors(&self) -> bool {
        !self.payload.is_empty()
    }

    #[inline]
    pub(crate) fn payload(self) -> IvoErrorPayload<FieldMetadata> {
        self.payload
    }
}

impl<CtxOptions, FieldMetadata: Clone + Send + Sync> IvoErrorTool<CtxOptions>
    for DefaultErrorTool<FieldMetadata>
{
    type FieldMetadata = FieldMetadata;
    type ErrorPayload = IvoErrorPayload<Self::FieldMetadata>;

    fn sanitize(
        payload: IvoErrorPayload<Self::FieldMetadata>,
        _: &CtxOptions,
    ) -> Self::ErrorPayload {
        payload
    }
}

// ErrorTool trait
pub trait IvoErrorTool<CtxOptions = DefaultCtxOptions> {
    type FieldMetadata: Clone + Send + Sync;
    type ErrorPayload;

    fn sanitize(
        payload: IvoErrorPayload<Self::FieldMetadata>,
        ctx_options: &CtxOptions,
    ) -> Self::ErrorPayload;
}
