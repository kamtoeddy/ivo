#![expect(type_alias_bounds)]

pub mod internal;

use std::{collections::HashMap, fmt::Debug, sync::Arc};

use crate::{
    schema::{
        fields::{base::InternalFieldConfig, TimestampConfig},
        options::base::SchemaOptions,
    },
    DefaultErrorSanitizer, IvoErrorSanitizer, IvoInputStruct,
};
use internal::{IvoRwLock, IvoStruct};

pub type IvoShared<T> = Arc<T>;
pub type IvoCtxOptions<CtxOptions> = IvoShared<CtxOptions>;
pub type IvoRwCtxOptions<CtxOptions> = IvoShared<IvoRwLock<CtxOptions>>;
pub type IvoContext<I: IvoStruct, O: IvoStruct = I> = IvoShared<InternalIvoContext<I, O>>;
pub type IvoConstantCtx<I: IvoStruct, O: IvoStruct = I> = IvoShared<IvoConstantContext<I, O>>;
pub type IvoDefaultCtx<I: IvoStruct> = IvoShared<IvoDefaultContext<I>>;

pub(crate) type InternalFieldConfigs<
    I: IvoInputStruct<CtxOptions, ErrorSanitizer>,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
> = HashMap<&'static str, InternalFieldConfig<I, O, CtxOptions, ErrorSanitizer>>;

pub struct IvoModel<
    I: IvoInputStruct<CtxOptions, ErrorSanitizer>,
    O: IvoStruct = I,
    CtxOptions: Clone + Sync + Send = Option<()>,
    Timestamp: Clone + Debug + Send + Sync + 'static = (),
    ErrorSanitizer: IvoErrorSanitizer<CtxOptions> = DefaultErrorSanitizer,
> {
    pub(crate) field_configs: InternalFieldConfigs<I, O, CtxOptions, ErrorSanitizer>,
    pub(crate) options: SchemaOptions<I, O, CtxOptions, ErrorSanitizer>,
    pub(crate) timestamp_configs: Option<TimestampConfig<Timestamp>>,
}

#[derive(Clone, Copy)]
pub enum InternalIvoContext<I: IvoStruct, O: IvoStruct> {
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

impl<I: IvoStruct, O: IvoStruct> InternalIvoContext<I, O> {
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
            InternalIvoContext::Create { values, .. } => {
                *values = changes;
            }
            InternalIvoContext::Update {
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
        if let InternalIvoContext::Update {
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
            InternalIvoContext::Create {
                input: prev_input, ..
            } => {
                *prev_input = input;
            }
            InternalIvoContext::Update {
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
            InternalIvoContext::Create { values, .. } => values,
            InternalIvoContext::Update { changes, .. } => changes,
        }
        .clone()
    }

    #[inline(always)]
    pub fn is_update(&self) -> bool {
        matches!(self, InternalIvoContext::Update { .. })
    }

    /// contains validated and up to date version of input_values
    #[inline(always)]
    pub fn input(&self) -> I::Partial {
        match &self {
            InternalIvoContext::Create { input, .. } => input,
            InternalIvoContext::Update { input, .. } => input,
        }
        .clone()
    }

    /// contains values provided at the start of the current process
    #[inline(always)]
    pub fn raw_input(&self) -> I::Partial {
        match &self {
            InternalIvoContext::Create { raw_input, .. } => raw_input,
            InternalIvoContext::Update { raw_input, .. } => raw_input,
        }
        .clone()
    }

    /// subset of output values related to current process
    #[inline(always)]
    pub fn values(&self) -> O::Partial {
        match &self {
            InternalIvoContext::Create { values, .. } => values.clone(),
            InternalIvoContext::Update { values, .. } => values.clone().into(),
        }
    }

    #[inline(always)]
    pub fn full_values(&self) -> Option<O> {
        match &self {
            InternalIvoContext::Update { values, .. } => Some(values.clone()),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn previous_values(&self) -> Option<O> {
        match &self {
            InternalIvoContext::Update {
                previous_values, ..
            } => Some(previous_values.clone()),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct IvoConstantContext<I: IvoStruct, O: IvoStruct> {
    pub input: I::Partial,
    pub raw_input: I::Partial,
    pub values: O::Partial,
}

impl<I: IvoStruct, O: IvoStruct> IvoConstantContext<I, O> {
    pub(crate) fn new(input: I::Partial, raw_input: I::Partial, values: O::Partial) -> Self {
        Self {
            input,
            raw_input,
            values,
        }
    }
}

#[derive(Clone, Copy)]
pub struct IvoDefaultContext<I: IvoStruct> {
    pub input: I::Partial,
    pub raw_input: I::Partial,
}

impl<I: IvoStruct> IvoDefaultContext<I> {
    pub(crate) fn new(input: I::Partial, raw_input: I::Partial) -> Self {
        Self { input, raw_input }
    }
}
