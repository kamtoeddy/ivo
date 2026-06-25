#![expect(type_alias_bounds)]
#![expect(clippy::borrowed_box)]

pub use futures_locks::RwLock;
use std::{any::Any, collections::HashSet, fmt::Debug, sync::Arc};

use crate::schema::{error_tool::DefaultFieldErrorMetadata, IvoFieldValue};

pub trait IvoStruct:
    Send + Sync + Sized + 'static + IvoWithPartialStruct + IvoStructMethods + Into<Self::Partial>
{
}

pub trait IvoWithPartialStruct {
    type Partial: PartialEq + Debug + Default + Send + Sync + Clone + IvoPartialStructMethods;
}

pub trait IvoStructMethods: IvoWithPartialStruct + Clone {
    fn ivo_internal_dangerously_get_values_from_partial(partial_values: Self::Partial) -> Self;

    fn ivo_internal_get_updates_from_partial(
        &self,
        updates: &Self::Partial,
    ) -> Option<Self::Partial>;

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

    fn ivo_internal_remove_value(&mut self, field_name: &str);
}

pub type SharedData<T> = Arc<T>;
pub type SharedCtxOptions<CtxOptions> = SharedData<CtxOptions>;
pub type SharedRwCtxOptions<CtxOptions> = SharedData<RwLock<CtxOptions>>;
pub type SharedIvoContext<I: IvoStruct, O: IvoStruct> = SharedData<IvoContext<I, O>>;
pub type SharedIvoInput<I: IvoStruct> = SharedData<I::Partial>;

pub type Partial<T> = <T as IvoWithPartialStruct>::Partial;

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

pub fn erase_value<T: IvoFieldValue>(value: T) -> Box<dyn CloneableAny> {
    Box::new(value)
}

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

#[derive(Clone, Copy)]
pub enum IvoContext<I: IvoStruct, O: IvoStruct> {
    Create {
        input: I::Partial,
        raw_input: I::Partial,
        values: O::Partial,
    },
    Update {
        changes: O::Partial,
        input: I::Partial,
        raw_input: I::Partial,
        previous_values: O,
        values: O,
    },
}

impl<I: IvoStruct, O: IvoStruct> IvoContext<I, O> {
    #[inline]
    pub(crate) fn new_create_ctx(
        input: I::Partial,
        raw_input: I::Partial,
        values: O::Partial,
    ) -> Self {
        Self::Create {
            input,
            raw_input,
            values,
        }
    }

    #[inline]
    pub(crate) fn new_update_ctx(
        changes: O::Partial,
        input: I::Partial,
        raw_input: I::Partial,
        previous_values: O,
        values: O,
    ) -> Self {
        Self::Update {
            changes,
            input,
            raw_input,
            previous_values,
            values,
        }
    }

    #[inline(always)]
    pub(crate) fn set_changes(&mut self, changes: O::Partial) -> &mut Self {
        match self {
            IvoContext::Create { values, .. } => {
                *values = changes;
            }
            IvoContext::Update {
                changes: prev_changes,
                ..
            } => {
                *prev_changes = changes;
            }
        };

        self
    }

    #[inline(always)]
    pub(crate) fn set_full_values(&mut self, values: O) -> &mut Self {
        if let IvoContext::Update {
            values: prev_values,
            ..
        } = self
        {
            *prev_values = values;
        };

        self
    }

    #[inline(always)]
    pub(crate) fn set_input(&mut self, input: I::Partial) -> &mut Self {
        match self {
            IvoContext::Create {
                input: prev_input, ..
            } => {
                *prev_input = input;
            }
            IvoContext::Update {
                input: prev_input, ..
            } => {
                *prev_input = input;
            }
        };

        self
    }

    /// part of the final output of the current process
    #[inline(always)]
    pub fn changes(&self) -> O::Partial {
        match &self {
            IvoContext::Create { values, .. } => values,
            IvoContext::Update { changes, .. } => changes,
        }
        .clone()
    }

    #[inline(always)]
    pub fn is_update(&self) -> bool {
        matches!(self, IvoContext::Update { .. })
    }

    /// contains validated and up to date version of input_values
    #[inline(always)]
    pub fn input(&self) -> I::Partial {
        match &self {
            IvoContext::Create { input, .. } => input,
            IvoContext::Update { input, .. } => input,
        }
        .clone()
    }

    /// contains values provided at the start of the current process
    #[inline(always)]
    pub fn raw_input(&self) -> I::Partial {
        match &self {
            IvoContext::Create { raw_input, .. } => raw_input,
            IvoContext::Update { raw_input, .. } => raw_input,
        }
        .clone()
    }

    /// subset of output values related to current process
    #[inline(always)]
    pub fn values(&self) -> O::Partial {
        match &self {
            IvoContext::Create { values, .. } => values.clone(),
            IvoContext::Update { values, .. } => values.clone().into(),
        }
    }

    #[inline(always)]
    pub fn full_values(&self) -> Option<O> {
        match &self {
            IvoContext::Update { values, .. } => Some(values.clone()),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn previous_values(&self) -> Option<O> {
        match &self {
            IvoContext::Update {
                previous_values, ..
            } => Some(previous_values.clone()),
            _ => None,
        }
    }
}

pub type ValidatorResponse<T: IvoFieldValue, ErrorMetadata = DefaultFieldErrorMetadata> =
    Result<T, ValidatorError<ErrorMetadata>>;

pub type ValidatorError<FieldErrorMetadata> = (String, Option<FieldErrorMetadata>);
