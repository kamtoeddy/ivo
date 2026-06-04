use std::marker::PhantomData;

use crate::{
    fields::{
        constants::ConstantFieldBuilder, dependents::DependentFieldBuilder,
        enumerated::EnumFieldBuilder, lax::LaxFieldBuilder, required::RequiredFieldBuilder,
        virtuals::VirtualFieldBuilder,
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

pub struct IvoField<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    _t: PhantomData<T>,
    _i: PhantomData<I>,
    _o: PhantomData<O>,
    _c: PhantomData<CtxOptions>,
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> IvoField<T, I, O, CtxOptions> {
    pub const CONSTANT: ConstantFieldBuilder<T, I, O, CtxOptions> = ConstantFieldBuilder::new();
    pub const DEPENDENT: DependentFieldBuilder<T, I, O, CtxOptions> = DependentFieldBuilder::new();
    pub const ENUM: EnumFieldBuilder<T, I, O, CtxOptions> = EnumFieldBuilder::new();
    pub const LAX: LaxFieldBuilder<T, I, O, CtxOptions> = LaxFieldBuilder::new();
    pub const REQUIRED: RequiredFieldBuilder<T, I, O, CtxOptions> = RequiredFieldBuilder::new();
    pub const VIRTUAL: VirtualFieldBuilder<T, I, O, CtxOptions> = VirtualFieldBuilder::new();
}
