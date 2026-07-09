mod error_tool;
pub mod types;

pub use error_tool::{
    DefaultErrorPayload, DefaultFieldErrorMetadata, IvoDefaultErrorTool, IvoErrorTool,
    IvoFieldError,
};

pub use types::{
    FieldValue, IvoInputStruct, IvoRwLock, IvoStruct, IvoStructMethods, Partial,
    PartialStructMethods, PostValidatorError, PostValidatorResponse, ValidatorError,
    ValidatorResponse, WithPartialStruct,
};

#[derive(Clone, Debug)]
pub(crate) struct FieldInfo {
    pub name: String,
    pub config_name: String,
    pub is_input: bool,
    pub is_output: bool,
}
