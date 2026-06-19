use std::marker::PhantomData;

use crate::{
    schema::{
        error_tool::IvoErrorTool,
        fields::types::{
            BooleanResolver, ComputableInit, ComputableRequiredError, ComputableWithMiniContext,
            RequiredResolver, Resolver, TimestampResolver, UniformValidator, VirtualSanitizer,
        },
    },
    types::{DeleteHandler, ErasedValue, FailureHandler, IvoFieldValue, No, SuccessHandler, Yes},
    IvoSchemaStruct,
};

pub trait BuildableFieldConfig<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
>
{
    fn build(self) -> InternalFieldConfig<I, O, CtxOptions, ErrorTool>;
}

pub type InternalFieldConfig<I, O, CtxOptions, ErrorTool> =
    FieldConfig<ErasedValue, I, O, CtxOptions, ErrorTool>;

pub enum FieldType {
    Constant,
    Dependent,
    Lax,
    Required,
    Virtual,
}

pub struct FieldConfig<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
> {
    pub field_type: FieldType,
    pub alias: Option<String>,
    pub default: Option<ComputableWithMiniContext<T, I, CtxOptions>>,
    pub depends_on: Option<Vec<&'static str>>,
    pub is_readonly: bool,
    pub value: Option<ComputableWithMiniContext<T, I, CtxOptions>>,
    pub required_fn: Option<RequiredResolver<I, O, CtxOptions>>,
    pub required_error: Option<ComputableRequiredError<I, O, CtxOptions>>,
    pub resolver: Option<Resolver<T, I, O, CtxOptions>>,
    pub sanitizer: Option<VirtualSanitizer<T, I, O, CtxOptions>>,
    pub validator: Option<UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    pub re_validator: Option<UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    //
    pub should_ignore: Option<BooleanResolver<I, O, CtxOptions>>,
    pub should_init: Option<ComputableInit<I, O, CtxOptions>>,
    pub should_update: Option<ComputableInit<I, O, CtxOptions>>,
    // life cycle handlers
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_failure_fns: Option<Vec<FailureHandler<I, O, CtxOptions>>>,
    pub on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions, ErrorTool: IvoErrorTool> Default
    for FieldConfig<T, I, O, CtxOptions, ErrorTool>
{
    fn default() -> Self {
        Self {
            field_type: FieldType::Lax,
            alias: None,
            is_readonly: false,
            value: None,
            default: None,
            depends_on: None,
            re_validator: None,
            required_fn: None,
            required_error: None,
            resolver: None,
            sanitizer: None,
            validator: None,
            should_ignore: None,
            should_init: None,
            should_update: None,
            on_delete_fns: None,
            on_success_fns: None,
            on_failure_fns: None,
        }
    }
}

pub struct TimestampConfig<T: IvoFieldValue> {
    pub created_at: Option<&'static str>,
    pub updated_at: Option<&'static str>,
    pub resolver: TimestampResolver<T>,
    pub with_optional_updated_at: bool,
}

pub trait BuildableTimestampConfig<T: IvoFieldValue> {
    fn build(self) -> TimestampConfig<T>;
}

pub struct TimestampConfigBuilder<
    T: IvoFieldValue,
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

impl<T: IvoFieldValue> BuildableTimestampConfig<T> for TimestampConfigBuilder<T, Yes, Yes> {
    fn build(self) -> TimestampConfig<T> {
        TimestampConfig {
            created_at: self.created_at,
            updated_at: self.updated_at,
            resolver: self.resovler.unwrap(),
            with_optional_updated_at: self.with_optional_updated_at,
        }
    }
}

impl<HasCreatedAt, T: IvoFieldValue> BuildableTimestampConfig<T>
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

impl<T: IvoFieldValue> TimestampConfigBuilder<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<HasDateFn, HasCreatedAt, HasUpdatedAt, T: IvoFieldValue> Default
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

impl<T: IvoFieldValue> TimestampConfigBuilder<T> {
    pub fn date_fn<R>(self, resolver: R) -> TimestampConfigBuilder<T, Yes>
    where
        R: Fn() -> T + Send + Sync + 'static,
    {
        TimestampConfigBuilder {
            resovler: Some(Box::new(resolver)),
            ..Default::default()
        }
    }
}

impl<HasUpdatedAt, T: IvoFieldValue> TimestampConfigBuilder<T, Yes, No, HasUpdatedAt> {
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

impl<HasCreatedAt, T: IvoFieldValue> TimestampConfigBuilder<T, Yes, HasCreatedAt, No> {
    pub fn updated_at(
        self,
        custom_name: Option<&'static str>,
        is_optional: bool,
    ) -> TimestampConfigBuilder<T, Yes, HasCreatedAt, Yes> {
        TimestampConfigBuilder {
            resovler: self.resovler,
            created_at: self.created_at,
            updated_at: Some(custom_name.unwrap_or("updated_at")),
            with_optional_updated_at: is_optional,
            ..Default::default()
        }
    }
}
