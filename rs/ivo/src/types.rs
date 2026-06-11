#![allow(type_alias_bounds)]
use std::{collections::HashMap, fmt::Debug};

use futures::future::BoxFuture;
pub use futures_locks::RwLock;
pub use std::sync::Arc;

use crate::{
    internal::InternalIvoContextMethods, schema::error::DefaultFieldErrorMetadata, ErasedValue,
};

#[derive(Debug)]
pub struct True;

// Marker Types
pub struct Yes;
pub struct No;
pub struct YesComputed;

// Optional: implement Deref to make it behave like bool
impl std::ops::Deref for True {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &true
    }
}

#[derive(Debug)]
pub struct False;

// Optional: implement Deref to make it behave like bool
impl std::ops::Deref for False {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &false
    }
}

/// This is used to show that this map does not contain all
/// the field of an ivo struct, but each erased value
/// represents an actual value, T in the struct, not
/// the Option<T>.
pub struct PartialMapOfErasedValues {
    pub inner: HashMap<String, ErasedValue>,
}

pub trait IvoSchemaStruct:
    Debug
    + Eq
    + Send
    + Sync
    + Sized
    + 'static
    + HasFields
    + HasPartial
    + FromToMap
    + WithUpdateDetails
    + Into<Self::Partial>
{
}

pub trait FromToMap {
    fn ivo_internal_from_erased_map(map: &HashMap<String, ErasedValue>) -> Self;
    fn ivo_internal_to_erased_map(&self) -> HashMap<String, ErasedValue>;
}

pub trait PartialFromToMap {
    fn ivo_internal_from_optional_erased_map(map: PartialMapOfErasedValues) -> Self;
    fn ivo_internal_from_optional_erased_map_ref(map: &PartialMapOfErasedValues) -> Self;
    fn ivo_internal_to_optional_erased_map(&self) -> PartialMapOfErasedValues;
}

pub trait HasFields {
    fn ivo_internal_field_names() -> Vec<String>;
}

pub trait HasPartial {
    type Partial: Debug + Default + Send + Sync + Clone + PartialFromToMap;
}

pub trait WithUpdateDetails: HasPartial + Clone + Sized {
    fn ivo_internal_dangerously_get_values_from_partial(partial_values: Self::Partial) -> Self;

    fn ivo_internal_get_updates_from_partial(
        &self,
        updates: &Self::Partial,
    ) -> (Self::Partial, bool);

    fn ivo_internal_get_erased_updates_from_erased_values(
        &self,
        updates: &HashMap<String, ErasedValue>,
    ) -> HashMap<String, ErasedValue>;

    fn ivo_internal_get_updates_from_erased_values(
        &self,
        updates: &HashMap<String, ErasedValue>,
    ) -> (Self::Partial, bool);

    fn ivo_internal_clone_with(&self, updates: &Self::Partial) -> Self {
        let mut cloned = self.clone();

        cloned.ivo_internal_update_with(updates);

        cloned
    }

    fn ivo_internal_update_with(&mut self, updates: &Self::Partial);
}

pub type SharedData<T> = Arc<T>;
pub type SharedCtxOptions<CtxOptions> = SharedData<CtxOptions>;
pub type SharedRwCtxOptions<CtxOptions> = SharedData<RwLock<CtxOptions>>;
pub type SharedIvoContext<I, O> = SharedData<IvoContext<I, O>>;
pub type SharedIvoMiniContext<I: IvoSchemaStruct> = SharedData<IvoMiniContext<I>>;
pub type IvoMiniContext<I: IvoSchemaStruct> = I::Partial;

pub type Partial<T> = <T as HasPartial>::Partial;

pub enum IvoContext<I: IvoSchemaStruct, O: IvoSchemaStruct> {
    Create {
        input: I::Partial,
        input_values: I::Partial,
        values: O::Partial,
    },
    Update {
        changes: O::Partial,
        input: I::Partial,
        input_values: I::Partial,
        previous_values: O,
        values: O,
    },
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct> InternalIvoContextMethods<I, O> for IvoContext<I, O> {
    fn new_create_ctx(input: I::Partial, input_values: I::Partial, values: O::Partial) -> Self {
        Self::Create {
            input,
            input_values,
            values,
        }
    }

    fn new_update_ctx(
        changes: O::Partial,
        input: I::Partial,
        input_values: I::Partial,
        previous_values: O,
        values: O,
    ) -> Self {
        Self::Update {
            changes,
            input,
            input_values,
            previous_values,
            values,
        }
    }
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct> IvoContext<I, O> {
    /// subset of output values related to which will be
    /// part of the final output of the current process
    pub fn changes(&self) -> O::Partial {
        match &self {
            IvoContext::Create { values, .. } => values,
            IvoContext::Update { changes, .. } => changes,
        }
        .clone()
    }

    pub fn is_update(&self) -> bool {
        matches!(self, IvoContext::Update { .. })
    }

    /// contains validated and up to date version of input_values
    pub fn input(&self) -> I::Partial {
        match &self {
            IvoContext::Create { input, .. } => input,
            IvoContext::Update { input, .. } => input,
        }
        .clone()
    }

    /// contains values provided at the start of the current process
    pub fn input_values(&self) -> I::Partial {
        match &self {
            IvoContext::Create { input_values, .. } => input_values,
            IvoContext::Update { input_values, .. } => input_values,
        }
        .clone()
    }

    /// subset of output values related to current process
    pub fn values(&self) -> O::Partial {
        match &self {
            IvoContext::Create { values, .. } => values.clone(),
            IvoContext::Update { values, .. } => values.clone().into(),
        }
    }
}

pub type DeleteHandler<O, CtxOptions> =
    Box<dyn Fn(Arc<O>, Arc<CtxOptions>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type FailureHandler<I, O, CtxOptions> = Box<
    dyn Fn(Arc<IvoContext<I, O>>, Arc<CtxOptions>) -> BoxFuture<'static, ()>
        + Send
        + Sync
        + 'static,
>;

pub type SuccessHandler<I, O, CtxOptions> = Box<
    dyn Fn(Arc<IvoContext<I, O>>, Arc<CtxOptions>) -> BoxFuture<'static, ()>
        + Send
        + Sync
        + 'static,
>;

pub type ValidatorResponse<T, ErrorMetadata = DefaultFieldErrorMetadata> =
    Result<T, ValidatorError<ErrorMetadata>>;

pub type ValidatorError<FieldErrorMetadata> = (String, Option<FieldErrorMetadata>);
