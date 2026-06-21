pub mod base;
mod constants;
mod dependents;
mod lax;
mod required;
pub mod types;
mod virtuals;

use std::marker::PhantomData;

use crate::{
    schema::{
        error_tool::IvoErrorTool,
        fields::{
            constants::ConstantFieldBuilder, dependents::DependentFieldBuilder,
            lax::LaxFieldBuilder, required::RequiredFieldBuilder, virtuals::VirtualFieldBuilder,
        },
        types::IvoFieldValue,
    },
    IvoSchemaStruct,
};

pub use base::TimestampConfig;

pub struct IvoField<
    T: IvoFieldValue,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
> {
    _t: PhantomData<T>,
    _i: PhantomData<I>,
    _o: PhantomData<O>,
    _c: PhantomData<CtxOptions>,
    _err: PhantomData<ErrorTool>,
}

impl<
        T: IvoFieldValue,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > IvoField<T, I, O, CtxOptions, ErrorTool>
{
    /// A constant field is one whose value never changes after an entity is created.
    ///
    /// e.g.: ids
    pub const CONSTANT: ConstantFieldBuilder<T, I, O, CtxOptions, ErrorTool> =
        ConstantFieldBuilder::new();

    /// A dependent field is one that belongs in the output struct O, but not in the input struct I.
    ///
    /// This is a field that derives it values from one or more other fields.
    ///
    /// A dependent field must:
    /// - have a default value or a function to generate one
    /// - one or more parent fields to react to
    /// - a resolver (function) used to generate it's value, everytime the value of any of its parent fields
    ///     changes
    pub const DEPENDENT: DependentFieldBuilder<T, I, O, CtxOptions, ErrorTool> =
        DependentFieldBuilder::new();

    /// Lax field
    pub const LAX: LaxFieldBuilder<T, I, O, CtxOptions, ErrorTool> = LaxFieldBuilder::new();

    /// Required field
    pub const REQUIRED: RequiredFieldBuilder<T, I, O, CtxOptions, ErrorTool> =
        RequiredFieldBuilder::new();

    /// A virtual field is one that belongs in the input struct I, but not in the output struct O.
    ///
    /// A virtual field must:
    /// - be used in combination with dependent fields. i.e: a virtual field must be a parent to
    /// one or more dependent fields
    /// - have a validator
    pub const VIRTUAL: VirtualFieldBuilder<T, I, O, CtxOptions, ErrorTool> =
        VirtualFieldBuilder::new();
}
