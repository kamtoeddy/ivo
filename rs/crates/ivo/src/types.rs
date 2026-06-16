#![allow(type_alias_bounds)]

use futures::future::BoxFuture;
pub use futures_locks::RwLock;
pub use std::sync::Arc;
use std::{any::Any, collections::HashMap, fmt::Debug};

use crate::schema::error_tool::DefaultFieldErrorMetadata;

// Marker Types
pub struct Yes;
pub struct No;
pub struct YesComputed;

#[derive(Debug)]
pub(crate) struct False;

// Optional: implement Deref to make it behave like bool
impl std::ops::Deref for False {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &false
    }
}

/// This is used to show that this map does not contain all
/// the fields of an ivo struct, but each erased value
/// represents an actual value, T in the struct, not
/// the Option<T>.
pub struct PartialMapOfErasedValues {
    pub inner: HashMap<String, ErasedValue>,
}

impl PartialMapOfErasedValues {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

pub trait IvoSchemaStruct:
    Debug
    + Send
    + Sync
    + Sized
    + 'static
    + WithIvoStructPartial
    + IvoStructFromToErasedMap
    + IvoStructMethods
    + Into<Self::Partial>
{
}

pub trait IvoStructFromToErasedMap: WithIvoStructPartial {
    fn ivo_internal_from_erased_map(map: &HashMap<String, ErasedValue>) -> Self;
    fn ivo_internal_to_erased_map(&self) -> HashMap<String, ErasedValue>;
}

pub trait IvoStructPartialFromToErasedMap {
    fn ivo_internal_from_optional_erased_map(map: PartialMapOfErasedValues) -> Self;
    fn ivo_internal_from_optional_erased_map_ref(map: &PartialMapOfErasedValues) -> Self;
    fn ivo_internal_to_optional_erased_map(&self) -> PartialMapOfErasedValues;
}

pub trait WithIvoStructPartial {
    type Partial: PartialEq
        + Debug
        + Default
        + Send
        + Sync
        + Clone
        + IvoStructPartialFromToErasedMap
        + IvoStructPartialMethods;
}

pub trait IvoStructMethods: WithIvoStructPartial + Clone {
    fn ivo_internal_dangerously_get_values_from_partial(partial_values: Self::Partial) -> Self;

    fn ivo_internal_get_updates_from_partial(
        &self,
        updates: &Self::Partial,
    ) -> (Self::Partial, bool);

    fn ivo_internal_clone_with(&self, updates: Self::Partial) -> Self {
        self.ivo_internal_clone_with_ref(&updates)
    }

    fn ivo_internal_clone_with_ref(&self, updates: &Self::Partial) -> Self {
        let mut cloned = self.clone();

        cloned.ivo_internal_update_with(updates);

        cloned
    }

    fn ivo_internal_update_with(&mut self, updates: &Self::Partial);

    fn ivo_internal_field_names() -> Vec<String>;

    fn ivo_internal_name() -> String;
}

pub trait IvoStructPartialMethods: Clone {
    fn ivo_internal_clone_with_erased_updates(
        &self,
        updates: &HashMap<String, ErasedValue>,
    ) -> (Self, bool);

    fn ivo_internal_fields_provided(&self) -> Vec<String>;

    fn ivo_internal_is_value_equal(&self, field_name: &String, value: &ErasedValue) -> bool;
}

pub type SharedData<T> = Arc<T>;
pub type SharedCtxOptions<CtxOptions> = SharedData<CtxOptions>;
pub type SharedRwCtxOptions<CtxOptions> = SharedData<RwLock<CtxOptions>>;
pub type SharedIvoContext<I: IvoSchemaStruct, O: IvoSchemaStruct> = SharedData<IvoContext<I, O>>;
pub type SharedIvoMiniContext<I: IvoSchemaStruct> = SharedData<IvoMiniContext<I>>;
pub type IvoMiniContext<I: IvoSchemaStruct> = I::Partial;

pub type Partial<T> = <T as WithIvoStructPartial>::Partial;

pub trait CloneableAny: Any + Send + Sync {
    fn clone_box(&self) -> Box<dyn CloneableAny>;
    fn as_any(&self) -> &dyn Any;
}

impl<T> CloneableAny for T
where
    T: Clone + Debug + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableAny> {
        Box::new(T::clone(self)) // This triggers the concrete type's clone method!
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for Box<dyn CloneableAny> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub type ErasedValue = Box<dyn CloneableAny>;

pub fn erase_value<T: Clone + Debug + Send + Sync + 'static>(value: T) -> Box<dyn CloneableAny> {
    Box::new(value)
}

pub fn parse_value<T: Clone + Debug + Send + Sync + 'static>(
    e: &Box<dyn CloneableAny>,
) -> Option<T> {
    e.as_any().downcast_ref::<T>().cloned()
}

pub fn parse_or_panic<T: Clone + Debug + Send + Sync + 'static>(e: &Box<dyn CloneableAny>) -> T {
    parse_value::<T>(e).expect("Failed to parse value").clone()
}

#[derive(Clone, Copy)]
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

impl<I: IvoSchemaStruct, O: IvoSchemaStruct> IvoContext<I, O> {
    pub(crate) fn new_create_ctx(
        input: I::Partial,
        input_values: I::Partial,
        values: O::Partial,
    ) -> Self {
        Self::Create {
            input,
            input_values,
            values,
        }
    }

    pub(crate) fn new_update_ctx(
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

pub type DeleteHandler<O: IvoSchemaStruct, CtxOptions> =
    Box<dyn Fn(Arc<O>, Arc<CtxOptions>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type FailureHandler<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> = Box<
    dyn Fn(Arc<IvoContext<I, O>>, Arc<CtxOptions>) -> BoxFuture<'static, ()>
        + Send
        + Sync
        + 'static,
>;

pub type SuccessHandler<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions> = Box<
    dyn Fn(Arc<IvoContext<I, O>>, Arc<CtxOptions>) -> BoxFuture<'static, ()>
        + Send
        + Sync
        + 'static,
>;

pub type ValidatorResponse<T, ErrorMetadata = DefaultFieldErrorMetadata> =
    Result<T, ValidatorError<ErrorMetadata>>;

pub type ValidatorError<FieldErrorMetadata> = (String, Option<FieldErrorMetadata>);
