pub mod error_tool;
pub mod fields;
pub mod options;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::schema::error_tool::{DefaultErrorTool, IvoErrorTool};
use crate::schema::fields::base::{BuildableFieldConfig, FieldType, InternalFieldConfig};
use crate::schema::fields::types::IntoUniformTimestampResolver;
use crate::schema::fields::TimestampFieldConfig;
use crate::schema::options::base::{SchemaOptions, SchemaOptionsBuilder};
use crate::schema::options::BuildableSchemaOptions;
use crate::types::{IvoSchemaStruct, No, Yes};

type InternalFieldConfigs<I, O, CtxOptions, ErrorTool> =
    HashMap<String, InternalFieldConfig<I, O, CtxOptions, ErrorTool>>;

const COLOR_RED: &str = "\x1b[31m";
const STYLE_RESET: &str = "\x1b[0m";
const FONT_BOLD: &str = "\x1b[1m";

pub struct Schema<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct = I,
    CtxOptions = Option<()>,
    ErrorTool: IvoErrorTool = DefaultErrorTool,
> {
    pub(crate) field_configs: InternalFieldConfigs<I, O, CtxOptions, ErrorTool>,
    pub(crate) options: SchemaOptions<I, O, CtxOptions, ErrorTool>,

    _timestamp_created_at: Option<TimestampFieldConfig>,
    _timestamp_updated_at: Option<TimestampFieldConfig>,
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

        let s = Self {
            field_configs: Self::make_field_configs(
                fields.configs,
                &fields.timestamp_created_at,
                &fields.timestamp_upated_at,
            ),
            options: options_maker(SchemaOptions::new()).build(),
            _timestamp_created_at: fields.timestamp_created_at,
            _timestamp_updated_at: fields.timestamp_upated_at,
        };

        s.check_options();

        s
    }

    fn check_options(&self) {
        // todo!()
    }

    fn make_field_configs(
        config_tuples: Vec<(String, InternalFieldConfig<I, O, CtxOptions, ErrorTool>)>,
        timestamp_created_at: &Option<TimestampFieldConfig>,
        timestamp_updated_at: &Option<TimestampFieldConfig>,
    ) -> InternalFieldConfigs<I, O, CtxOptions, ErrorTool> {
        let input_field_names = I::ivo_internal_field_names();

        let input_struct_name = format!(
            "{FONT_BOLD}{}{STYLE_RESET}{COLOR_RED}",
            I::ivo_internal_name()
        );

        let output_struct_name = format!(
            "{FONT_BOLD}{}{STYLE_RESET}{COLOR_RED}",
            O::ivo_internal_name()
        );

        let mut field_names = HashSet::new();
        let mut alias_to_virtual = HashMap::new();

        for (field_name, config) in config_tuples.iter() {
            let field_name_str = field_name.as_str();

            if field_names.contains(field_name) {
                panic!("\n{COLOR_RED}[{field_name}]: occurs more than once, please remove duplicates{STYLE_RESET}\n");
            }

            if let Some(TimestampFieldConfig { name, .. }) = timestamp_created_at {
                if field_name == name {
                    panic!(
                        "\n{COLOR_RED}[{field_name}]: is not a valid field name. It is the creation timestamp on {output_struct_name}{STYLE_RESET}\n"
                    );
                }
            }

            if let Some(TimestampFieldConfig { name, .. }) = timestamp_updated_at {
                if field_name == name {
                    panic!(
                        "\n{COLOR_RED}[{field_name}]: is not a valid field name. It is the update timestamp on {output_struct_name}{STYLE_RESET}\n"
                    );
                }
            }

            field_names.insert(field_name);

            // virtuals
            match config {
                InternalFieldConfig {
                    field_type: FieldType::Virtual,
                    alias,
                    ..
                } => {
                    if let Some(alias) = alias {
                        if field_name == alias {
                            panic!("\n{COLOR_RED}[{field_name}]: virtual alias name must be different from field name{STYLE_RESET}\n");
                        }

                        if let Some(other_field) = alias_to_virtual.get(&alias) {
                            panic!("\n{COLOR_RED}[{field_name}]: \"{alias}\" is already the alias of \"{other_field}\"{STYLE_RESET}\n");
                        }

                        if let Some(TimestampFieldConfig { name, .. }) = timestamp_created_at {
                            if alias == name {
                                panic!(
                                    "\n{COLOR_RED}[{field_name}]: \"{alias}\" is not a valid alias. It is the creation timestamp on {output_struct_name}{STYLE_RESET}\n"
                                );
                            }
                        }

                        if let Some(TimestampFieldConfig { name, .. }) = timestamp_updated_at {
                            if alias == name {
                                panic!(
                                    "\n{COLOR_RED}[{field_name}]: \"{alias}\" is not a valid alias. It is the update timestamp on {output_struct_name}{STYLE_RESET}\n"
                                );
                            }
                        }

                        for (name, config) in config_tuples.iter() {
                            if name != alias {
                                continue;
                            }

                            if let Some(depends_on) = config.depends_on.as_ref() {
                                if !depends_on.iter().any(|parent| parent == &field_name_str) {
                                    panic!("\n{COLOR_RED}[{field_name}]: \"{alias}\" is not a valid alias for field because \"{alias}\" does not depend on \"{field_name}\"{STYLE_RESET}\n");
                                }

                                continue;
                            }

                            panic!("\n{COLOR_RED}[{field_name}]: \"{alias}\" is not a valid alias for field because it is not a dependent field{STYLE_RESET}\n");
                        }

                        if !input_field_names.contains(alias) {
                            panic!(
                                "\n{COLOR_RED}[{field_name}]: \"{alias}\" must be present on {input_struct_name}{STYLE_RESET}\n");
                        }

                        if input_field_names.contains(field_name) {
                            panic!(
                                "\n{COLOR_RED}[{field_name}]: has an alias. Hence, cannot be present on {input_struct_name}{STYLE_RESET}\n");
                        }

                        alias_to_virtual.insert(alias, field_name);
                    }

                    continue;
                }
                _ => (),
            }

            match config {
                // dependents
                InternalFieldConfig {
                    field_type: FieldType::Dependent,
                    ..
                } => {}
                _ => (),
            }
        }

        let mut field_configs = HashMap::new();

        for (field_name, config) in config_tuples {
            field_configs.insert(field_name, config);
        }

        field_configs
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
    configs: Vec<(String, InternalFieldConfig<I, O, CtxOptions, ErrorTool>)>,
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
            configs: Vec::new(),
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
        self.configs.push((name.to_owned(), config.build()));

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
                name: name.unwrap_or("updated_at"),
                resovler: resolver.into_resolver(),
                is_optional,
            }),
            ..Default::default()
        }
    }
}
