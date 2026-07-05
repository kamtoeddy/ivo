#![expect(type_alias_bounds)]
#![expect(clippy::borrowed_box)]

pub use futures_locks::RwLock;
use std::{
    any::Any,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use crate::{DefaultFieldErrorMetadata, IvoErrorTool};

pub trait IvoStruct:
    Send + Sync + Sized + 'static + IvoWithPartialStruct + IvoStructMethods + Into<Self::Partial>
{
    #[inline(always)]
    fn append_updates(&mut self, updates: &Self::Partial) {
        self.ivo_internal_update_with(updates)
    }

    #[inline(always)]
    fn clone_with_updates(&self, updates: &Self::Partial) -> Self {
        self.ivo_internal_clone_with_ref(updates)
    }
}

pub trait IvoWithPartialStruct {
    type Partial: PartialEq + Debug + Default + Send + Sync + Clone + IvoPartialStructMethods;
}

pub trait IvoWithPartialErrorsStruct<FieldErrorMetadata: Send + Sync> {
    type PartialErrors: Send + Sync + IvoPartialErrorsStructMethods<FieldErrorMetadata>;
}

pub trait IvoStructMethods: IvoWithPartialStruct + Clone {
    fn ivo_internal_dangerously_get_values_from_partial(partial_values: Self::Partial) -> Self;

    fn ivo_internal_get_updates_from_partial(
        &self,
        updates: &Self::Partial,
    ) -> Option<Self::Partial>;

    #[inline(always)]
    fn ivo_internal_clone_with(&self, updates: Self::Partial) -> Self {
        self.ivo_internal_clone_with_ref(&updates)
    }

    fn ivo_internal_clone_with_ref(&self, updates: &Self::Partial) -> Self {
        let mut cloned = self.clone();

        cloned.ivo_internal_update_with(updates);

        cloned
    }

    fn ivo_internal_update_with(&mut self, updates: &Self::Partial);

    fn ivo_internal_field_names() -> HashSet<String>;

    fn ivo_internal_name() -> String;
}

pub trait IvoPartialStructMethods: Clone {
    fn ivo_internal_enumerate(&self) -> Vec<(String, ErasedValue)>;

    fn ivo_internal_fields_provided(&self) -> Vec<String>;

    fn ivo_internal_get_erased_value(&self, field_name: &str) -> ErasedValue;

    fn ivo_internal_is_value_equal(&self, field_name: &str, value: &ErasedValue) -> bool;

    fn ivo_internal_set(&mut self, field_name: &str, value: &ErasedValue);

    fn ivo_internal_unset(&mut self, field_name: &str);
}

pub trait IvoPartialErrorsStructMethods<FieldErrorMetadata: Send + Sync> {
    fn ivo_internal_enumerate(self) -> Vec<(String, (String, Option<FieldErrorMetadata>))>;
}

pub type Partial<T> = <T as IvoWithPartialStruct>::Partial;

pub trait IvoFieldValue: Clone + Debug + Send + Sync + 'static {}

impl<T> IvoFieldValue for T where T: Clone + Debug + Send + Sync + 'static {}

pub trait CloneableAny: Any + Send + Sync {
    fn clone_box(&self) -> Box<dyn CloneableAny>;
    fn as_any(&self) -> &dyn Any;
    fn runtime_type_name(&self) -> &'static str;
}

impl<T> CloneableAny for T
where
    T: IvoFieldValue,
{
    fn clone_box(&self) -> Box<dyn CloneableAny> {
        Box::new(T::clone(self)) // This triggers the concrete type's clone method!
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn runtime_type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

impl Clone for Box<dyn CloneableAny> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub type ErasedValue = Box<dyn CloneableAny>;

#[inline(always)]
pub fn erase_value<T: IvoFieldValue>(value: T) -> Box<dyn CloneableAny> {
    Box::new(value)
}

#[inline(always)]
pub fn parse_value<T: IvoFieldValue>(e: &Box<dyn CloneableAny>) -> Option<T> {
    e.as_any().downcast_ref::<T>().cloned()
}

pub fn parse_or_panic<T: IvoFieldValue>(
    erased_value: &Box<dyn CloneableAny>,
    field_name: Option<&str>,
) -> T {
    let value = parse_value::<T>(erased_value);

    if let Some(actual_value) = value {
        return actual_value;
    }

    let expected_type_path = std::any::type_name::<T>();
    let actual_type_path = erased_value.runtime_type_name();
    let field_name = field_name
        .map(|n| format!("\"{n}\""))
        .unwrap_or_else(|| "value".into());

    panic!(
        "\nFailed to parse {field_name}. Expected: \"{expected_type_path}\", but got \"{actual_type_path}\"\n"
    )
}

pub type ValidatorResponse<T: IvoFieldValue, ErrorMetadata = DefaultFieldErrorMetadata> =
    Result<Option<T>, ValidatorError<ErrorMetadata>>;

pub type ValidatorError<FieldErrorMetadata> = (String, Option<FieldErrorMetadata>);

pub type PostValidatorError<FieldErrorMetadata = DefaultFieldErrorMetadata> =
    HashMap<String, ValidatorError<FieldErrorMetadata>>;

pub type PostValidatorResponse<
    I: IvoStruct + IvoWithPartialErrorsStruct<ErrorTool::FieldMetadata>,
    ErrorTool: IvoErrorTool,
> = Result<Option<I::Partial>, I::PartialErrors>;
