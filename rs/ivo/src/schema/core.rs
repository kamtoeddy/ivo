use crate::schema::fields::types::IntoUniformTimestampResolver;
use crate::schema::fields::TimestampFieldConfig;

use crate::schema::internal::SchemaInternals;
use crate::schema::options::base::{SchemaOptions, SchemaOptionsBuilder};
use crate::schema::options::BuildableSchemaOptions;

use crate::schema::error::{DefaultErrorTool, IvoErrorTool};
use crate::schema::fields::base::{BuildableFieldConfig, FieldType, InternalFieldConfig};
use crate::types::{IvoSchemaStruct, No, Yes};

use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

type InternalFieldConfigs<I, O, CtxOptions, ErrorTool> =
    HashMap<String, InternalFieldConfig<I, O, CtxOptions, ErrorTool>>;

pub struct Schema<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct = I,
    CtxOptions = Option<()>,
    ErrorTool: IvoErrorTool = DefaultErrorTool,
> {
    field_configs: InternalFieldConfigs<I, O, CtxOptions, ErrorTool>,
    options: SchemaOptions<I, O, CtxOptions, ErrorTool>,

    _timestamp_created_at: Option<TimestampFieldConfig>,
    _timestamp_upated_at: Option<TimestampFieldConfig>,
}

impl<'a, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions, ErrorTool: IvoErrorTool>
    Schema<I, O, CtxOptions, ErrorTool>
{
    pub fn new<FieldMaker, OptionsMaker, BuildableOptions, HasCreatedAt, HasUpdatedAt>(
        fields_maker: FieldMaker,
        options_maker: OptionsMaker,
    ) -> Self
    where
        FieldMaker: Fn(
            FieldBuilder<I, O, CtxOptions, ErrorTool>,
        )
            -> FieldBuilder<I, O, CtxOptions, ErrorTool, HasCreatedAt, HasUpdatedAt>,
        OptionsMaker: Fn(SchemaOptionsBuilder<I, O, CtxOptions, ErrorTool>) -> BuildableOptions,
        BuildableOptions: BuildableSchemaOptions<I, O, CtxOptions, ErrorTool>,
    {
        let fields = fields_maker(FieldBuilder::new());

        let mut s = Self {
            field_configs: fields.configs,
            options: options_maker(SchemaOptions::new()).build(),
            _timestamp_created_at: fields.timestamp_created_at,
            _timestamp_upated_at: fields.timestamp_upated_at,
        };

        s.check_field_configs();
        s.check_options();

        s
    }

    pub fn get_field_config(
        &self,
        prop: &str,
    ) -> Option<&InternalFieldConfig<I, O, CtxOptions, ErrorTool>> {
        self.field_configs.get(prop)
    }

    pub fn get_field_configs(&self) -> &InternalFieldConfigs<I, O, CtxOptions, ErrorTool> {
        &self.field_configs
    }

    fn check_options(&self) {
        // todo!()
    }

    fn check_field_configs(&mut self) {
        // todo!()
        // let mut err_tool = SchemaError::new();

        // First pass: register prop kinds and simple attributes
        for (_, def) in &self.field_configs {
            // virtuals
            if matches!(def.field_type, FieldType::Virtual) {
                continue;
            }

            // dependents
            if matches!(def.field_type, FieldType::Dependent) {
                continue;
            }
        }

        // if err_tool.has_errors() {
        //     err_tool.throw();
        // }
    }

    pub fn get_reserved_keys(&self) -> Vec<String> {
        todo!()
        // let mut keys: Vec<String> = self.props.iter().cloned().collect();

        // keys.extend(self.virtuals.iter().cloned());

        // if let Some(k) = &self.timestamp_tool.get_keys().created_at {
        //     if !k.is_empty() {
        //         keys.push(k.clone());
        //     }
        // }

        // if let Some(k) = &self.timestamp_tool.get_keys().updated_at {
        //     if !k.is_empty() {
        //         keys.push(k.clone());
        //     }
        // }

        // keys.sort();
        // keys
    }
}

pub struct FieldBuilder<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
    HasCreatedAt = No,
    HasUpdatedAt = No,
> {
    _c: PhantomData<HasCreatedAt>,
    _u: PhantomData<HasUpdatedAt>,
    configs: InternalFieldConfigs<I, O, CtxOptions, ErrorTool>,
    timestamp_created_at: Option<TimestampFieldConfig>,
    timestamp_upated_at: Option<TimestampFieldConfig>,
}

impl<
        HasCreatedAt,
        HasUpdatedAt,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > FieldBuilder<I, O, CtxOptions, ErrorTool, HasCreatedAt, HasUpdatedAt>
{
    fn new() -> Self {
        Self {
            configs: HashMap::new(),
            timestamp_created_at: None,
            timestamp_upated_at: None,
            _c: PhantomData,
            _u: PhantomData,
        }
    }

    pub fn set<Config>(mut self, name: &str, config: Config) -> Self
    where
        Config: BuildableFieldConfig<I, O, CtxOptions, ErrorTool>,
    {
        self.configs.insert(name.to_owned(), config.build());

        self
    }
}

impl<
        HasCreatedAt,
        HasUpdatedAt,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
    > Default for FieldBuilder<I, O, CtxOptions, ErrorTool, HasCreatedAt, HasUpdatedAt>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<HasUpdatedAt, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions, ErrorTool: IvoErrorTool>
    FieldBuilder<I, O, CtxOptions, ErrorTool, No, HasUpdatedAt>
{
    pub fn created_at<T, R>(
        self,
        resolver: R,
        name: Option<&'static str>,
    ) -> FieldBuilder<I, O, CtxOptions, ErrorTool, Yes, HasUpdatedAt>
    where
        T: Clone + Debug + Send + Sync + 'static,
        R: IntoUniformTimestampResolver<T>,
    {
        FieldBuilder {
            configs: self.configs,
            timestamp_created_at: Some(TimestampFieldConfig {
                name: name.unwrap_or("created_at"),
                resovler: resolver.into_resolver(),
                is_optional: false,
            }),
            timestamp_upated_at: self.timestamp_upated_at,
            ..Default::default()
        }
    }
}

impl<HasCreatedAt, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions, ErrorTool: IvoErrorTool>
    FieldBuilder<I, O, CtxOptions, ErrorTool, HasCreatedAt>
{
    pub fn updated_at<T, R>(
        self,
        resolver: R,
        name: Option<&'static str>,
        is_optional: bool,
    ) -> FieldBuilder<I, O, CtxOptions, ErrorTool, HasCreatedAt, Yes>
    where
        T: Clone + Debug + Send + Sync + 'static,
        R: IntoUniformTimestampResolver<T>,
    {
        FieldBuilder {
            configs: self.configs,
            timestamp_created_at: self.timestamp_created_at,
            timestamp_upated_at: Some(TimestampFieldConfig {
                name: name.unwrap_or("created_at"),
                resovler: resolver.into_resolver(),
                is_optional,
            }),
            ..Default::default()
        }
    }
}

impl<'a, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions, ErrorTool: IvoErrorTool>
    SchemaInternals<I, O, CtxOptions, ErrorTool> for Schema<I, O, CtxOptions, ErrorTool>
{
    fn options(&self) -> &SchemaOptions<I, O, CtxOptions, ErrorTool> {
        &self.options
    }
}
