pub mod demo;
pub mod schema;
mod traits;
pub mod types;
pub mod validators;

pub use partial_derive::MakePartial;

pub use schema::utils::TimeStampTool;
pub use types::ValidatorResponse;
