use std::marker::PhantomData;

use crate::{schema::properties::constants::ConstantFieldBuilder, traits::IvoSchemaStruct};

pub mod base;
pub mod constants;
pub mod dependents;
pub mod enumerated;
pub mod lax;
pub mod required;
pub mod virtuals;

pub struct IvoField<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    _i: PhantomData<I>,
    _o: PhantomData<O>,
    _c: PhantomData<CtxOptions>,
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> IvoField<I, O, CtxOptions> {
    pub fn constant<T>() -> ConstantFieldBuilder<T, I, O, CtxOptions> {
        Default::default()
    }

    // pub fn dependent() -> DependentField {
    //     Default::default()
    // }

    // pub fn enumerated() -> EnumeratedField {
    //     Default::default()
    // }

    // pub fn lax() -> LaxField {
    //     Default::default()
    // }

    // pub fn required() -> RequiredField {
    //     Default::default()
    // }

    // pub fn virtualized() -> VirtualField {
    //     Default::default()
    // }
}
