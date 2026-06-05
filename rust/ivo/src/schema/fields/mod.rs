use std::marker::PhantomData;

use crate::{
    schema::{
        error::IvoErrorTool,
        fields::{
            constants::ConstantFieldBuilder, dependents::DependentFieldBuilder,
            enumerated::EnumFieldBuilder, lax::LaxFieldBuilder, required::RequiredFieldBuilder,
            virtuals::VirtualFieldBuilder,
        },
    },
    traits::IvoSchemaStruct,
};

pub mod base;
mod constants;
mod dependents;
mod enumerated;
mod lax;
mod required;
mod virtuals;

pub struct IvoField<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrT: IvoErrorTool,
> {
    _t: PhantomData<T>,
    _i: PhantomData<I>,
    _o: PhantomData<O>,
    _c: PhantomData<CtxOptions>,
    _err: PhantomData<ErrT>,
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrT: IvoErrorTool>
    IvoField<T, I, O, CtxOptions, ErrT>
{
    pub const CONSTANT: ConstantFieldBuilder<T, I, O, CtxOptions, ErrT> =
        ConstantFieldBuilder::new();
    pub const DEPENDENT: DependentFieldBuilder<T, I, O, CtxOptions, ErrT> =
        DependentFieldBuilder::new();
    pub const ENUM: EnumFieldBuilder<T, I, O, CtxOptions, ErrT> = EnumFieldBuilder::new();
    pub const LAX: LaxFieldBuilder<T, I, O, CtxOptions, ErrT> = LaxFieldBuilder::new();
    pub const REQUIRED: RequiredFieldBuilder<T, I, O, CtxOptions, ErrT> =
        RequiredFieldBuilder::new();
    pub const VIRTUAL: VirtualFieldBuilder<T, I, O, CtxOptions, ErrT> = VirtualFieldBuilder::new();
}
