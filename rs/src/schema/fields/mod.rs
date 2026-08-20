pub(crate) mod base;
mod constants;
mod dependents;
mod lax;
mod required;
pub(crate) mod types;
mod virtuals;

pub use base::TimestampConfig;

use crate::{
    schema::{
        fields::{
            constants::ConstantFieldBuilder, dependents::DependentFieldBuilder,
            lax::LaxFieldBuilder, required::RequiredFieldBuilder, virtuals::VirtualFieldBuilder,
        },
        types::Yes,
        FieldValue,
    },
    IvoErrorSanitizer, IvoStruct,
};

/// A constant field is one whose value never changes after an entity is created.
///
/// e.g.: ids
pub fn constant_field<
    T: FieldValue,
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
>(
    name: &'static str,
) -> ConstantFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer> {
    ConstantFieldBuilder::new(name)
}

/// A dependent field is one that belongs in the output struct O, but not in the input struct I.
///
/// This is a field that derives it values from one or more other fields.
///
/// A dependent field must:
/// - have a default value or a function to generate one
/// - one or more parent fields to react to
/// - a resolver (function) used to generate it's value, everytime the value of any of its parent fields
///   changes
pub fn dependent_field<
    T: FieldValue,
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    const N: usize,
>(
    name: &'static str,
    fields: [&'static str; N],
) -> DependentFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer, Yes> {
    DependentFieldBuilder::new(name, fields)
}

/// Lax field
pub fn lax_field<
    T: FieldValue,
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
>(
    name: &'static str,
) -> LaxFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer> {
    LaxFieldBuilder::new(name)
}

/// Required field
pub fn required_field<
    T: FieldValue,
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
>(
    name: &'static str,
) -> RequiredFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer> {
    RequiredFieldBuilder::new(name)
}

/// A virtual field is one that belongs in the input struct I, but not in the output struct O.
///
/// A virtual field must:
/// - be used in combination with dependent fields. i.e: a virtual field must be a parent to
///   one or more dependent fields
/// - have a validator
pub fn virtual_field<
    T: FieldValue,
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
>(
    name: &'static str,
) -> VirtualFieldBuilder<T, I, O, CtxOptions, ErrorSanitizer> {
    VirtualFieldBuilder::new(name)
}
