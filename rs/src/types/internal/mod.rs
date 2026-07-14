mod error_tool;
pub mod types;

pub use error_tool::{
    DefaultErrorPayload, DefaultErrorTool, DefaultFieldErrorMetadata, FieldError, IvoErrorTool,
};

pub use types::{
    FieldValue, IvoInputStruct, IvoRwLock, IvoStruct, IvoStructMethods, Partial,
    PartialStructMethods, PostValidatorError, PostValidatorResponse, ValidatorError,
    ValidatorResponse, WithPartialStruct,
};

#[derive(Clone, Debug)]
pub(crate) struct FieldInfo<'a> {
    pub name: &'a str,
    pub config_name: &'a str,
    pub is_input: bool,
    pub is_output: bool,
}
