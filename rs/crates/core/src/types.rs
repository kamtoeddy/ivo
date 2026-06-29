#![expect(type_alias_bounds)]

use std::sync::Arc;

use ivo_types::{IvoStruct, RwLock};

pub type SharedIvoData<T> = Arc<T>;
pub type SharedCtxOptions<CtxOptions> = SharedIvoData<CtxOptions>;
pub type SharedRwCtxOptions<CtxOptions> = SharedIvoData<RwLock<CtxOptions>>;
pub type SharedIvoContext<I: IvoStruct, O: IvoStruct> = SharedIvoData<IvoContext<I, O>>;
pub type SharedIvoInput<I: IvoStruct> = SharedIvoData<I::Partial>;
pub type UpdateResolverData<I: IvoStruct, O: IvoStruct> = (I::Partial, O);

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
