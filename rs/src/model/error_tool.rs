use std::collections::HashMap;

use crate::{__private_types::FieldError, IvoErrorPayload};

#[derive(Debug)]
pub(super) struct ErrorTool<Metadata: Clone> {
    payload: IvoErrorPayload<Metadata>,
}

impl<Metadata: Clone> ErrorTool<Metadata> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            payload: HashMap::new(),
        }
    }

    #[inline(always)]
    pub fn set(&mut self, field_name: &str, value: FieldError<Metadata>) {
        self.payload.insert(field_name.to_string(), value);
    }

    #[inline(always)]
    pub fn has_errors(&self) -> bool {
        !self.payload.is_empty()
    }

    #[inline(always)]
    pub fn payload(self) -> IvoErrorPayload<Metadata> {
        self.payload
    }
}
