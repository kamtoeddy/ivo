use crate::{
    schema::{
        error::IvoErrorTool,
        fields::types::{
            BooleanResolverWithMutSummary, ComputableInit, ComputableRequired,
            ComputableRequiredError, ComputableWithMiniSummary, ResolverWithMutSummary,
            UniformTimestampResolver, UniformValidator, VirtualSanitiser,
        },
    },
    types::{DeleteHandler, FailureHandler, SuccessHandler},
    utils::erased_value::ErasedValue,
    IvoSchemaStruct,
};

pub trait BuildableFieldConfig<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrorTool: IvoErrorTool,
>
{
    fn build(self) -> InternalFieldConfig<I, O, CtxOptions, ErrorTool>;
}

pub type InternalFieldConfig<I, O, CtxOptions, ErrorTool> =
    FieldConfig<ErasedValue, I, O, CtxOptions, ErrorTool>;

pub struct FieldConfig<
    T,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrorTool: IvoErrorTool,
> {
    pub alias: Option<String>,
    pub default: Option<ComputableWithMiniSummary<T, I, O, CtxOptions>>,
    pub depends_on: Option<Vec<&'static str>>,
    pub is_constant: bool,
    pub is_readonly: bool,
    pub is_virtual: bool,
    pub value: Option<ComputableWithMiniSummary<T, I, O, CtxOptions>>,
    pub required: Option<ComputableRequired<I, O, CtxOptions>>,
    pub required_error: Option<ComputableRequiredError<I, O, CtxOptions>>,
    pub resolver: Option<ResolverWithMutSummary<T, I, O, CtxOptions>>,
    pub sanitizer: Option<VirtualSanitiser<T, I, O, CtxOptions>>,
    pub validator: Option<UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    pub re_validator: Option<UniformValidator<I, O, CtxOptions, ErrorTool::FieldMetadata>>,
    //
    pub should_ignore: Option<BooleanResolverWithMutSummary<I, O, CtxOptions>>,
    pub should_init: Option<ComputableInit<I, O, CtxOptions>>,
    pub should_update: Option<ComputableInit<I, O, CtxOptions>>,
    // life cycle handlers
    pub on_delete_fns: Option<Vec<DeleteHandler<O, CtxOptions>>>,
    pub on_failure_fns: Option<Vec<FailureHandler<I, O, CtxOptions>>>,
    pub on_success_fns: Option<Vec<SuccessHandler<I, O, CtxOptions>>>,
}

impl<T, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrorTool: IvoErrorTool> Default
    for FieldConfig<T, I, O, CtxOptions, ErrorTool>
{
    fn default() -> Self {
        Self {
            alias: None,
            is_constant: false,
            is_readonly: false,
            is_virtual: false,
            value: None,
            default: None,
            depends_on: None,
            re_validator: None,
            required: None,
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

#[allow(dead_code)]
pub struct TimestampFieldConfig {
    pub name: &'static str,
    pub resovler: UniformTimestampResolver,
    pub is_optional: bool,
}
