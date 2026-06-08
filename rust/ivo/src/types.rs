use std::{collections::HashMap, fmt::Debug};

use futures::future::BoxFuture;

use crate::{schema::error::DefaultFieldErrorMetadata, ErasedValue};

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

pub struct OptionalErasedMap {
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
    fn ivo_internal_from_optional_erased_map(map: &OptionalErasedMap) -> Self;
    fn ivo_internal_to_optional_erased_map(&self) -> OptionalErasedMap;
}

pub trait HasFields {
    fn ivo_internal_fields() -> Vec<String>;
}

pub trait HasPartial {
    type Partial: Debug + Default + Send + Sync + Clone + PartialFromToMap;
}

pub trait WithUpdateDetails: HasPartial + Clone + Sized {
    fn ivo_internal_get_updates(
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

pub type Partial<T> = <T as HasPartial>::Partial;

pub struct IvoMiniSummary<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    input: I::Partial,
    input_values: I::Partial,
    values: O::Partial,
    options: CtxOptions,
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> IvoMiniSummary<I, O, CtxOptions> {
    pub fn new(
        input: I::Partial,
        input_values: I::Partial,
        values: O::Partial,
        options: CtxOptions,
    ) -> Self {
        Self {
            input,
            input_values,
            values,
            options,
        }
    }

    /// contains validated and up to date version of input_values
    pub fn input(&self) -> I::Partial {
        self.input.clone()
    }

    /// contains values provided at the start of the process
    pub fn input_values(&self) -> I::Partial {
        self.input_values.clone()
    }

    /// subset of output values related to current process
    pub fn values(&self) -> O::Partial {
        self.values.clone()
    }

    pub fn options(&self) -> &CtxOptions {
        &self.options
    }

    pub fn update_options(&mut self) {
        todo!()
    }
}

pub enum IvoSummary<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> {
    Create {
        input: I::Partial,
        input_values: I::Partial,
        values: O::Partial,
        options: CtxOptions,
    },
    Update {
        changes: O::Partial,
        input: I::Partial,
        input_values: I::Partial,
        previous_values: O,
        values: O,
        options: CtxOptions,
    },
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone> IvoSummary<I, O, CtxOptions> {
    pub fn for_new(
        input: I::Partial,
        input_values: I::Partial,
        values: O::Partial,
        options: CtxOptions,
    ) -> Self {
        Self::Create {
            input,
            input_values,
            values,
            options,
        }
    }

    pub fn for_update(
        changes: O::Partial,
        input: I::Partial,
        input_values: I::Partial,
        previous_values: O,
        values: O,
        options: CtxOptions,
    ) -> Self {
        Self::Update {
            changes,
            input,
            input_values,
            previous_values,
            values,
            options,
        }
    }

    /// subset of output values related to which will be
    /// part of the final output of the current process
    pub fn changes(&self) -> O::Partial {
        match &self {
            IvoSummary::Create { values, .. } => values,
            IvoSummary::Update { changes, .. } => changes,
        }
        .clone()
    }

    pub fn is_update(&self) -> bool {
        matches!(self, IvoSummary::Update { .. })
    }

    /// contains validated and up to date version of input_values
    pub fn input(&self) -> I::Partial {
        match &self {
            IvoSummary::Create { input, .. } => input,
            IvoSummary::Update { input, .. } => input,
        }
        .clone()
    }

    /// contains values provided at the start of the current process
    pub fn input_values(&self) -> I::Partial {
        match &self {
            IvoSummary::Create { input_values, .. } => input_values,
            IvoSummary::Update { input_values, .. } => input_values,
        }
        .clone()
    }

    /// subset of output values related to current process
    pub fn values(&self) -> O::Partial {
        match &self {
            IvoSummary::Create { values, .. } => values.clone(),
            IvoSummary::Update { values, .. } => values.clone().into(),
        }
    }

    pub fn get_options(&self) -> &CtxOptions {
        match &self {
            IvoSummary::Create { options, .. } => options,
            IvoSummary::Update { options, .. } => options,
        }
    }

    pub fn get_options_mut(&self) -> CtxOptions {
        match &self {
            IvoSummary::Create { options, .. } => options.clone(),
            IvoSummary::Update { options, .. } => options.clone(),
        }
    }

    pub fn update_options(&mut self) {
        // todo!()
    }
}

pub type DeleteHandler<O, CtxOptions> =
    Box<dyn Fn(O, CtxOptions) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type FailureHandler<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type SuccessHandler<I, O, CtxOptions> =
    Box<dyn Fn(IvoSummary<I, O, CtxOptions>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub type ValidatorResponse<T, ErrorMetadata = DefaultFieldErrorMetadata> =
    Result<T, ValidatorError<ErrorMetadata>>;

pub type ValidatorError<FieldErrorMetadata> = (String, Option<FieldErrorMetadata>);
