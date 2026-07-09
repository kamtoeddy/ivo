mod error_tool;
pub mod types;

pub use error_tool::{
    DefaultErrorPayload, DefaultFieldErrorMetadata, IvoDefaultErrorTool, IvoErrorTool,
    IvoFieldError, IvoUpdateError,
};

pub use types::{
    IvoFieldValue, IvoInputStruct, IvoPartialStructMethods, IvoRwLock, IvoStruct, IvoStructMethods,
    IvoWithPartialStruct, Partial, PostValidatorError, PostValidatorResponse, ValidatorError,
    ValidatorResponse,
};

#[derive(Clone, Debug)]
pub(crate) struct FieldInfo {
    pub name: String,
    pub config_name: String,
    pub is_input: bool,
    pub is_output: bool,
}
