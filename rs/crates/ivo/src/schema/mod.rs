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
        f: FieldMaker,
        o: OptionsMaker,
    ) -> Self
    where
        FieldMaker: Fn(
            FieldBuilder<I, O, CtxOptions, ErrorTool>,
        )
            -> FieldBuilder<I, O, CtxOptions, ErrorTool, HasCreatedAt, HasUpdatedAt>,
        OptionsMaker: Fn(SchemaOptionsBuilder<I, O, CtxOptions, ErrorTool>) -> BuildableOptions,
        BuildableOptions: BuildableSchemaOptions<I, O, CtxOptions, ErrorTool>,
    {
        let fields = f(FieldBuilder::new());

        Self {
            field_configs: Self::make_field_configs(
                fields.configs,
                &fields.timestamp_created_at,
                &fields.timestamp_upated_at,
            ),
            options: Self::make_options(o(SchemaOptions::new()).build()),
            _timestamp_created_at: fields.timestamp_created_at,
            _timestamp_updated_at: fields.timestamp_upated_at,
        }
    }

    fn make_field_configs(
        config_tuples: Vec<(String, InternalFieldConfig<I, O, CtxOptions, ErrorTool>)>,
        timestamp_created_at: &Option<TimestampFieldConfig>,
        timestamp_updated_at: &Option<TimestampFieldConfig>,
    ) -> InternalFieldConfigs<I, O, CtxOptions, ErrorTool> {
        let input_field_names = I::ivo_internal_field_names();
        let output_field_names = O::ivo_internal_field_names();
        let input_struct_name = format!(
            "{FONT_BOLD}{}{STYLE_RESET}{COLOR_RED}",
            I::ivo_internal_name()
        );
        let output_struct_name = format!(
            "{FONT_BOLD}{}{STYLE_RESET}{COLOR_RED}",
            O::ivo_internal_name()
        );

        if let Some(TimestampFieldConfig { name, .. }) = timestamp_created_at {
            let name_owned = name.to_string();

            if !output_field_names.contains(&name_owned) {
                panic!(
                    "\n{COLOR_RED}[{name}]: is a purely output field. It must be present on {output_struct_name}{STYLE_RESET}\n"
                );
            }

            if input_field_names.contains(&name_owned) {
                panic!(
                    "\n{COLOR_RED}[{name}]: is a purely output field. It should not be present on {input_struct_name}{STYLE_RESET}\n"
                );
            }
        }

        if let Some(TimestampFieldConfig { name, .. }) = timestamp_updated_at {
            let name_owned = name.to_string();

            if !output_field_names.contains(&name_owned) {
                panic!(
                    "\n{COLOR_RED}[{name}]: is a purely output field. It must be present on {output_struct_name}{STYLE_RESET}\n"
                );
            }

            if input_field_names.contains(&name_owned) {
                panic!(
                    "\n{COLOR_RED}[{name}]: is a purely output field. It should not be present on {input_struct_name}{STYLE_RESET}\n"
                );
            }
        }

        let mut constant_field_names = HashSet::new();
        let mut dependent_field_to_parent_fields = HashMap::new();
        let mut field_names = HashSet::new();
        let mut alias_to_virtual = HashMap::new();
        let mut dependent_configs = Vec::new();

        for (field_name, config) in config_tuples.iter() {
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

            match config {
                InternalFieldConfig {
                    field_type: FieldType::Constant,
                    ..
                } => {
                    if !output_field_names.contains(field_name) {
                        panic!(
                            "\n{COLOR_RED}[{field_name}]: is a purely output field. It must be present on {output_struct_name}{STYLE_RESET}\n"
                        );
                    }

                    if input_field_names.contains(field_name) {
                        panic!(
                            "\n{COLOR_RED}[{field_name}]: is a purely output field. It should not be present on {input_struct_name}{STYLE_RESET}\n"
                        );
                    }

                    constant_field_names.insert(field_name);

                    continue;
                }
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

                        let field_name_str = field_name.as_str();

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
                                "\n{COLOR_RED}[{field_name}]: is an input field. Hence, \"{alias}\" must be present on {input_struct_name}{STYLE_RESET}\n");
                        }

                        if input_field_names.contains(field_name) {
                            panic!(
                                "\n{COLOR_RED}[{field_name}]: has an alias. Only its alias must be present on {input_struct_name}{STYLE_RESET}\n");
                        }

                        alias_to_virtual.insert(alias, field_name);

                        continue;
                    }

                    if !input_field_names.contains(field_name) {
                        panic!(
                                "\n{COLOR_RED}[{field_name}]: is an input field. It must be present on {input_struct_name}{STYLE_RESET}\n");
                    }

                    continue;
                }
                _ => (),
            }

            if !output_field_names.contains(field_name) {
                panic!(
                    "\n{COLOR_RED}[{field_name}]: is an output field. It must be present on {output_struct_name}{STYLE_RESET}\n");
            }

            match config {
                InternalFieldConfig {
                    field_type: FieldType::Dependent,
                    depends_on,
                    ..
                } => {
                    dependent_configs.push((field_name, config));
                    dependent_field_to_parent_fields
                        .insert(field_name, depends_on.as_ref().unwrap());
                }
                InternalFieldConfig {
                    field_type: FieldType::Lax | FieldType::Required,
                    ..
                } => {
                    if !input_field_names.contains(field_name) {
                        panic!(
                        "\n{COLOR_RED}[{field_name}]: is an input field. It must be present on {input_struct_name}{STYLE_RESET}\n");
                    }
                }
                _ => (),
            }
        }

        for (field_name, InternalFieldConfig { depends_on, .. }) in dependent_configs {
            let parent_fields = depends_on.as_ref().unwrap();

            if parent_fields.is_empty() {
                panic!("\n{COLOR_RED}[{field_name}]: must depend on at least one lax, required, virtual or other dependent field on your schema{STYLE_RESET}\n");
            }

            let mut parent_fields_provided = HashSet::new();

            for parent_field in parent_fields {
                let parent_field_string = parent_field.to_string();

                if let Some(TimestampFieldConfig { name, .. }) = timestamp_created_at {
                    if parent_field == name {
                        panic!(
                                    "\n{COLOR_RED}[{field_name}]: cannot depend on \"{parent_field}\" because it is the creation timestamp on {output_struct_name}{STYLE_RESET}\n"
                                );
                    }
                }

                if let Some(TimestampFieldConfig { name, .. }) = timestamp_updated_at {
                    if parent_field == name {
                        panic!(
                                    "\n{COLOR_RED}[{field_name}]: cannot depend on \"{parent_field}\" because it is the update timestamp on {output_struct_name}{STYLE_RESET}\n"
                                );
                    }
                }

                if !field_names.contains(&parent_field_string) {
                    panic!(
                                "\n{COLOR_RED}[{field_name}]: cannot depend on \"{parent_field}\" because it is not a field on your schema{STYLE_RESET}\n"
                            );
                }

                if parent_field == field_name {
                    panic!("\n{COLOR_RED}[{field_name}]: cannot depend on itself{STYLE_RESET}\n");
                }

                if parent_fields_provided.contains(parent_field) {
                    panic!(
                                "\n{COLOR_RED}[{field_name}]: \"{parent_field}\" has been provided as a parent field multiple times. remove all duplicates to proceed{STYLE_RESET}\n"
                            );
                }

                if constant_field_names.contains(&parent_field_string) {
                    panic!(
                                "\n{COLOR_RED}[{field_name}]: cannot depend on \"{parent_field}\" because it is a constant{STYLE_RESET}\n"
                            );
                }

                parent_fields_provided.insert(parent_field);
            }

            if let Some((parent_field, redundant_field, depth)) =
                Self::get_redundant_dependency_on_parent(
                    field_name,
                    &dependent_field_to_parent_fields,
                )
            {
                if depth == 0 {
                    panic!(
                            "\n{COLOR_RED}[{field_name}]: should not depend on \"{parent_field}\" and \"{redundant_field}\" because \"{parent_field}\" depends on \"{redundant_field}\"{STYLE_RESET}\n"
               );
                }

                panic!(
                           "\n{COLOR_RED}[{field_name}]: should not depend on \"{parent_field}\" and \"{redundant_field}\" because \"{parent_field}\" indirectly depends on \"{redundant_field}\"{STYLE_RESET}\n"
                       );
            }
        }

        let mut field_configs = HashMap::new();

        for (field_name, config) in config_tuples {
            field_configs.insert(field_name, config);
        }

        field_configs
    }

    fn make_options(
        options: SchemaOptions<I, O, CtxOptions, ErrorTool>,
    ) -> SchemaOptions<I, O, CtxOptions, ErrorTool> {
        options
    }

    /// Given fields a, b, c, d such that a -> \[b, c\]  **&**  b -> \[c\]  **&**  c -> \[d\]
    ///
    /// => redundancy(a, b) = Some(c)  **&**  redundancy(a, c) = None
    ///
    /// => a -> \[b\] is the only valid config for a
    ///
    /// Given fields a, b, c, d such that a -> \[b, d\]  **&**  b -> \[c\]  **&**  c -> \[d\]
    ///
    /// => redundancy(a, b) = Some(d)  **&**  redundancy(a, d) = None
    ///
    /// => a -> \[b\] is the only valid config for a
    fn get_redundant_dependency_on_parent<'r>(
        field_name: &String,
        dependent_field_to_parent_fields: &HashMap<&String, &Vec<&'r str>>,
    ) -> Option<(&'r str, &'r str, i32)> {
        if let Some(parent_deps) = dependent_field_to_parent_fields.get(field_name).as_ref() {
            for parent_name in parent_deps.iter() {
                for field_name in parent_deps.iter() {
                    if field_name == parent_name {
                        continue;
                    }

                    if let Some((_, r, d)) = Self::is_field_redundantly_dependent_on_parent(
                        field_name,
                        parent_name,
                        dependent_field_to_parent_fields,
                        0,
                    ) {
                        return Some((field_name, r, d));
                    }
                }
            }

            return None;
        }

        None
    }

    fn is_field_redundantly_dependent_on_parent<'r>(
        field_name: &'r str,
        parent_name: &'r str,
        dependent_field_to_parent_fields: &HashMap<&String, &Vec<&'r str>>,
        depth: i32,
    ) -> Option<(&'r str, &'r str, i32)> {
        if let Some(parent_deps) = dependent_field_to_parent_fields
            .get(&field_name.to_string())
            .as_ref()
        {
            if parent_deps.contains(&parent_name) {
                return Some((field_name, parent_name, depth));
            }

            for field_name in parent_deps.iter() {
                let r = Self::is_field_redundantly_dependent_on_parent(
                    field_name,
                    parent_name,
                    dependent_field_to_parent_fields,
                    depth + 1,
                );

                if r.is_some() {
                    return r;
                }
            }

            return None;
        }

        None
    }

    // Given fields a, b, c, d

    // if a -> \[b\]  **&**  b -> \[a\]

    // => circular_dependency_chain(a) = Some(vec!\[a, b\])

    // if a -> \[b\]  **&**  b -> \[c\]  **&**  c -> \[a\]

    // => circular_dependency_chain(a) = Some(vec!\[a, b, c\])

    // if a -> \[b\]  **OR**  a -> \[b\]  **&**  b -> \[c, d\]  **OR**  a -> \[b\]  **&**  b -> \[c\]  **&**  c -> \[d\]

    // => circular_dependency_chain(a) = None
    // fn get_circular_dependency_chain<'r>(
    //     field_name: &String,
    //     parent_name: &String,
    //     dependent_field_to_parent_fields: &HashMap<&String, &Vec<&'r str>>,
    // ) -> Option<Vec<&'r str>> {
    //     if let Some(parent_deps) = dependent_field_to_parent_fields.get(parent_name).as_ref() {
    //         let deps = dependent_field_to_parent_fields.get(field_name).unwrap();

    //         // for parent_dep in parent_deps.iter() {
    //         //     if deps.contains(parent_dep) {
    //         //         return Some(parent_dep);
    //         //     }
    //         // }

    //         return None;
    //     }

    //     None
    // }
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
