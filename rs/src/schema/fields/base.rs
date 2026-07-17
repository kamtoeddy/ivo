use std::marker::PhantomData;

use crate::__private_types::types::{BooleanResolver, Resolver};
use crate::types::internal::{types::ErasedValue, IvoErrorSanitizer};
use crate::{
    schema::{
        fields::types::{
            ComputableRequiredError, IsFieldProvisionEnabled, RequiredResolver, TimestampResolver,
            UniformValidator, ValueResolverWithSharedInput, VirtualSanitizer,
        },
        types::{DeleteHandler, FailureHandler, FieldValue, No, SuccessHandler, Yes},
    },
    IvoStruct,
};

pub trait BuildableFieldConfig<
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    ErrorSanitizer: IvoErrorSanitizer,
>
{
    fn build(self) -> InternalFieldConfig<I, O, CtxOptions, ErrorSanitizer>;
}

pub type InternalFieldConfig<I, O, CtxOptions, ErrorSanitizer> =
    FieldConfig<ErasedValue, I, O, CtxOptions, ErrorSanitizer>;

pub enum FieldType {
    Constant,
    Dependent,
    Lax,
    Required,
    Virtual,
}

pub struct FieldConfig<T, I: IvoStruct, O: IvoStruct, CtxOptions, ErrorSanitizer: IvoErrorSanitizer>
{
    pub field_type: FieldType,
    pub alias: Option<&'static str>,
    pub default: Option<ValueResolverWithSharedInput<T, I, CtxOptions>>,
    pub depends_on: Option<Vec<&'static str>>,
    pub value: Option<ValueResolverWithSharedInput<T, I, CtxOptions>>,
    pub required_fn: Option<RequiredResolver<I, O, CtxOptions>>,
    pub required_error: Option<ComputableRequiredError<I, O, CtxOptions>>,
    pub resolver: Option<Resolver<T, I, O, CtxOptions>>,
    pub sanitizer: Option<VirtualSanitizer<T, I, O, CtxOptions>>,
    pub validator: Option<UniformValidator<I, O, CtxOptions, ErrorSanitizer::FieldMetadata>>,
    pub re_validator: Option<UniformValidator<I, O, CtxOptions, ErrorSanitizer::FieldMetadata>>,
    //
    pub ignore: Option<BooleanResolver<I, O, CtxOptions>>,
    pub ignore_init: Option<IsFieldProvisionEnabled<I, O, CtxOptions>>,
    pub ignore_update: Option<IsFieldProvisionEnabled<I, O, CtxOptions>>,
    // life cycle handlers
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_failure_fns: Option<Vec<FailureHandler<I, O, CtxOptions>>>,
    pub on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<T, I: IvoStruct, O: IvoStruct, CtxOptions, ErrorSanitizer: IvoErrorSanitizer> Default
    for FieldConfig<T, I, O, CtxOptions, ErrorSanitizer>
{
    fn default() -> Self {
        Self {
            field_type: FieldType::Lax,
            alias: None,
            value: None,
            default: None,
            depends_on: None,
            re_validator: None,
            required_fn: None,
            required_error: None,
            resolver: None,
            sanitizer: None,
            validator: None,
            ignore: None,
            ignore_init: None,
            ignore_update: None,
            on_delete_fns: None,
            on_success_fns: None,
            on_failure_fns: None,
        }
    }
}

pub struct TimestampConfig<T: FieldValue> {
    pub created_at: Option<&'static str>,
    pub updated_at: Option<&'static str>,
    pub resolver: TimestampResolver<T>,
    pub with_optional_updated_at: bool,
}

pub trait BuildableTimestampConfig<T: FieldValue> {
    fn build(self) -> TimestampConfig<T>;
}

pub struct TimestampConfigBuilder<
    T: FieldValue,
    HasDateFn = No,
    HasCreatedAt = No,
    HasUpdatedAt = No,
> {
    created_at: Option<&'static str>,
    updated_at: Option<&'static str>,
    resovler: Option<TimestampResolver<T>>,
    with_optional_updated_at: bool,
    _c: PhantomData<HasCreatedAt>,
    _r: PhantomData<HasDateFn>,
    _u: PhantomData<HasUpdatedAt>,
}

impl<T: FieldValue> BuildableTimestampConfig<T> for TimestampConfigBuilder<T, Yes, Yes> {
    fn build(self) -> TimestampConfig<T> {
        TimestampConfig {
            created_at: self.created_at,
            updated_at: self.updated_at,
            resolver: self.resovler.unwrap(),
            with_optional_updated_at: self.with_optional_updated_at,
        }
    }
}

impl<HasCreatedAt, T: FieldValue> BuildableTimestampConfig<T>
    for TimestampConfigBuilder<T, Yes, HasCreatedAt, Yes>
{
    fn build(self) -> TimestampConfig<T> {
        TimestampConfig {
            created_at: self.created_at,
            updated_at: self.updated_at,
            resolver: self.resovler.unwrap(),
            with_optional_updated_at: self.with_optional_updated_at,
        }
    }
}

impl<T: FieldValue> TimestampConfigBuilder<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<HasDateFn, HasCreatedAt, HasUpdatedAt, T: FieldValue> Default
    for TimestampConfigBuilder<T, HasDateFn, HasCreatedAt, HasUpdatedAt>
{
    fn default() -> Self {
        Self {
            created_at: None,
            updated_at: None,
            resovler: None,
            with_optional_updated_at: false,
            _c: PhantomData,
            _r: PhantomData,
            _u: PhantomData,
        }
    }
}

impl<T: FieldValue> TimestampConfigBuilder<T> {
    pub fn resolve<R>(self, resolver: R) -> TimestampConfigBuilder<T, Yes>
    where
        R: Fn() -> T + Send + Sync + 'static,
    {
        TimestampConfigBuilder {
            resovler: Some(Box::new(resolver)),
            ..Default::default()
        }
    }
}

impl<HasUpdatedAt, T: FieldValue> TimestampConfigBuilder<T, Yes, No, HasUpdatedAt> {
    pub fn created_at(
        self,
        custom_name: Option<&'static str>,
    ) -> TimestampConfigBuilder<T, Yes, Yes, HasUpdatedAt> {
        TimestampConfigBuilder {
            resovler: self.resovler,
            created_at: Some(custom_name.unwrap_or("created_at")),
            updated_at: self.updated_at,
            with_optional_updated_at: self.with_optional_updated_at,
            ..Default::default()
        }
    }
}

impl<HasCreatedAt, T: FieldValue> TimestampConfigBuilder<T, Yes, HasCreatedAt, No> {
    pub fn updated_at(
        self,
        custom_name: Option<&'static str>,
    ) -> TimestampConfigBuilder<T, Yes, HasCreatedAt, Yes> {
        TimestampConfigBuilder {
            resovler: self.resovler,
            created_at: self.created_at,
            updated_at: Some(custom_name.unwrap_or("updated_at")),
            with_optional_updated_at: false,
            ..Default::default()
        }
    }

    pub fn optional_updated_at(
        self,
        custom_name: Option<&'static str>,
    ) -> TimestampConfigBuilder<T, Yes, HasCreatedAt, Yes> {
        TimestampConfigBuilder {
            resovler: self.resovler,
            created_at: self.created_at,
            updated_at: Some(custom_name.unwrap_or("updated_at")),
            with_optional_updated_at: true,
            ..Default::default()
        }
    }
}
