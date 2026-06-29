pub mod fields;
pub mod options;
mod types;

use ivo_types::{DefaultErrorTool, IvoErrorTool, IvoStruct};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::marker::PhantomData;
pub use types::IvoFieldValue;
use types::{No, Yes};

use crate::schema::fields::base::{
    BuildableFieldConfig, BuildableTimestampConfig, FieldType, InternalFieldConfig,
    TimestampConfigBuilder,
};
use crate::schema::fields::TimestampConfig;
use crate::schema::options::base::{SchemaOptions, SchemaOptionsBuilder};
use crate::schema::options::types::{OnSuccessConfig, PostValidationConfig};
use crate::schema::options::BuildableSchemaOptions;

type InternalFieldConfigs<I, O, CtxOptions, ErrorTool> =
    HashMap<String, InternalFieldConfig<I, O, CtxOptions, ErrorTool>>;

const COLOR_RED: &str = "\x1b[31m";
const STYLE_RESET: &str = "\x1b[0m";
const FONT_BOLD: &str = "\x1b[1m";

pub struct Schema<
    I: IvoStruct,
    O: IvoStruct = I,
    CtxOptions = Option<()>,
    Timestamp: Clone + Debug + Send + Sync + 'static = (),
    ErrorTool: IvoErrorTool = DefaultErrorTool,
> {
    pub(crate) field_configs: InternalFieldConfigs<I, O, CtxOptions, ErrorTool>,
    pub(crate) options: SchemaOptions<I, O, CtxOptions, ErrorTool>,
    pub(crate) timestamp_configs: Option<TimestampConfig<Timestamp>>,
}

impl<
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        Timestamp: Clone + Debug + Send + Sync + 'static,
        ErrorTool: IvoErrorTool,
    > Schema<I, O, CtxOptions, Timestamp, ErrorTool>
{
    #[track_caller]
    pub fn new<FieldMaker, OptionsMaker, BuildableOptions, WithTimestamps>(
        f: FieldMaker,
        o: OptionsMaker,
    ) -> Self
    where
        FieldMaker: Fn(
            FieldBuilder<I, O, CtxOptions, Timestamp, ErrorTool>,
        )
            -> FieldBuilder<I, O, CtxOptions, Timestamp, ErrorTool, WithTimestamps>,
        OptionsMaker: Fn(SchemaOptionsBuilder<I, O, CtxOptions, ErrorTool>) -> BuildableOptions,
        BuildableOptions: BuildableSchemaOptions<I, O, CtxOptions, ErrorTool>,
    {
        let fields = f(FieldBuilder::new());

        let input_field_names = I::ivo_internal_field_names();
        let output_field_names = O::ivo_internal_field_names();

        let (field_configs, alias_to_virtual_map) = Self::make_field_configs(
            fields.configs,
            &fields.timestamp_config,
            &input_field_names,
            &output_field_names,
        );

        let options = Self::make_options(
            o(SchemaOptions::new()).build(),
            &field_configs,
            &alias_to_virtual_map,
            &input_field_names,
            &output_field_names,
        );

        Self {
            field_configs,
            options,
            timestamp_configs: fields.timestamp_config,
        }
    }

    #[track_caller]
    fn make_field_configs(
        config_tuples: Vec<(String, InternalFieldConfig<I, O, CtxOptions, ErrorTool>)>,
        timestamp_configs: &Option<TimestampConfig<Timestamp>>,
        input_field_names: &HashSet<String>,
        output_field_names: &HashSet<String>,
    ) -> (
        InternalFieldConfigs<I, O, CtxOptions, ErrorTool>,
        HashMap<String, String>,
    ) {
        let input_struct_name = format!(
            "{FONT_BOLD}{}{STYLE_RESET}{COLOR_RED}",
            I::ivo_internal_name()
        );
        let output_struct_name = format!(
            "{FONT_BOLD}{}{STYLE_RESET}{COLOR_RED}",
            O::ivo_internal_name()
        );

        if let Some(TimestampConfig {
            created_at: Some(name),
            ..
        }) = timestamp_configs
        {
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

        if let Some(TimestampConfig {
            updated_at: Some(name),
            ..
        }) = timestamp_configs
        {
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

            if let Some(TimestampConfig {
                created_at: Some(name),
                ..
            }) = timestamp_configs
            {
                if field_name == name {
                    panic!(
                        "\n{COLOR_RED}[{field_name}]: is not a valid field name. It is the creation timestamp on {output_struct_name}{STYLE_RESET}\n"
                    );
                }
            }

            if let Some(TimestampConfig {
                updated_at: Some(name),
                ..
            }) = timestamp_configs
            {
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

                        if let Some(other_field) = alias_to_virtual.get(alias) {
                            panic!("\n{COLOR_RED}[{field_name}]: \"{alias}\" is already the alias of \"{other_field}\"{STYLE_RESET}\n");
                        }

                        if let Some(TimestampConfig {
                            created_at: Some(name),
                            ..
                        }) = timestamp_configs
                        {
                            if alias == name {
                                panic!(
                                    "\n{COLOR_RED}[{field_name}]: \"{alias}\" is not a valid alias. It is the creation timestamp on {output_struct_name}{STYLE_RESET}\n"
                                );
                            }
                        }

                        if let Some(TimestampConfig {
                            updated_at: Some(name),
                            ..
                        }) = timestamp_configs
                        {
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

                        alias_to_virtual.insert(alias.clone(), field_name.clone());

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
                } if !input_field_names.contains(field_name) => {
                    panic!(
                        "\n{COLOR_RED}[{field_name}]: is an input field. It must be present on {input_struct_name}{STYLE_RESET}\n");
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

                if let Some(TimestampConfig {
                    created_at: Some(name),
                    ..
                }) = timestamp_configs
                {
                    if parent_field == name {
                        panic!(
                                    "\n{COLOR_RED}[{field_name}]: cannot depend on \"{parent_field}\" because it is the creation timestamp on {output_struct_name}{STYLE_RESET}\n"
                                );
                    }
                }

                if let Some(TimestampConfig {
                    updated_at: Some(name),
                    ..
                }) = timestamp_configs
                {
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
                Self::get_redundant_dependency(parent_fields, &dependent_field_to_parent_fields)
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

            if let Some(chain) = Self::get_circular_dependency_chain(
                field_name,
                parent_fields,
                &dependent_field_to_parent_fields,
            ) {
                let chain = chain.join(" <-> ");

                panic!(
                           "\n{COLOR_RED}[{field_name}]: circular dependency identified between \"{chain}\"{STYLE_RESET}\n"
                       );
            }
        }

        let mut field_configs = HashMap::new();

        for (field_name, config) in config_tuples {
            field_configs.insert(field_name, config);
        }

        (field_configs, alias_to_virtual)
    }

    #[track_caller]
    fn make_options(
        options: SchemaOptions<I, O, CtxOptions, ErrorTool>,
        field_configs: &InternalFieldConfigs<I, O, CtxOptions, ErrorTool>,
        alias_to_virtual_map: &HashMap<String, String>,
        input_field_names: &HashSet<String>,
        output_field_names: &HashSet<String>,
    ) -> SchemaOptions<I, O, CtxOptions, ErrorTool> {
        if let Some(configs) = options.on_success_fns.as_ref() {
            let option_name = "options.on_success";
            let mut field_names = HashSet::new();

            for OnSuccessConfig { fields, .. } in configs {
                for field_name in fields {
                    if field_names.contains(field_name) {
                        panic!(
                            "\n{COLOR_RED}[{option_name}]: remove duplicates of \"{field_name}\" in grouped on_success config{STYLE_RESET}\n"
                        );
                    }

                    let owned_field_name = field_name.to_string();

                    if let Some(virtual_field) = alias_to_virtual_map.get(&owned_field_name) {
                        panic!(
                            "\n{COLOR_RED}[{option_name}]: \"{field_name}\" is an alias; use \"{virtual_field}\" instead{STYLE_RESET}\n"
                        );
                    };

                    if input_field_names.contains(&owned_field_name) {
                        field_names.insert(field_name);

                        continue;
                    };

                    if let Some(InternalFieldConfig {
                        field_type: FieldType::Virtual,
                        ..
                    }) = field_configs.get(&owned_field_name)
                    {
                        field_names.insert(field_name);

                        continue;
                    };

                    if output_field_names.contains(&owned_field_name) {
                        panic!(
                        "\n{COLOR_RED}[{option_name}]: timestamps are not allowed in on_success. remove \"{field_name}\"{STYLE_RESET}\n"
                    );
                    }

                    panic!(
                            "\n{COLOR_RED}[{option_name}]: \"{field_name}\" does not exist on your schema{STYLE_RESET}\n"
                        );
                }
            }
        }

        if let Some(configs) = options.post_validate.as_ref() {
            let option_name = "options.post_validate";
            let mut field_names = HashSet::new();

            for PostValidationConfig { fields, .. } in configs {
                if fields.len() < 2 {
                    panic!(
                        "\n{COLOR_RED}[{option_name}]: post-validation expects at least 2 fields {STYLE_RESET}\n"
                    );
                }

                for field_name in fields {
                    if field_names.contains(field_name) {
                        panic!(
                            "\n{COLOR_RED}[{option_name}]: remove duplicates of \"{field_name}\" in your post-validation config{STYLE_RESET}\n"
                        );
                    }

                    let owned_field_name = field_name.to_string();

                    if let Some(virtual_field) = alias_to_virtual_map.get(&owned_field_name) {
                        panic!(
                            "\n{COLOR_RED}[{option_name}]: \"{field_name}\" is an alias; use \"{virtual_field}\" instead{STYLE_RESET}\n"
                        );
                    };

                    if input_field_names.contains(&owned_field_name) {
                        field_names.insert(field_name);

                        continue;
                    };

                    if output_field_names.contains(&owned_field_name) {
                        panic!(
                        "\n{COLOR_RED}[{option_name}]: \"{field_name}\" cannot be post_validated{STYLE_RESET}\n"
                    );
                    } else if !field_configs.contains_key(&owned_field_name) {
                        panic!(
                            "\n{COLOR_RED}[{option_name}]: \"{field_name}\" does not exist on your schema{STYLE_RESET}\n"
                        );
                    };
                }
            }
        }

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
    fn get_redundant_dependency<'r>(
        parent_fields: &Vec<&'r str>,
        dependent_field_to_parent_fields: &HashMap<&String, &Vec<&'r str>>,
    ) -> Option<(&'r str, &'r str, i32)> {
        for parent_name in parent_fields.iter() {
            for field_name in parent_fields.iter() {
                if field_name == parent_name {
                    continue;
                }

                if let Some((redundant_field, depth)) =
                    Self::is_field_redundantly_dependent_on_parent(
                        field_name,
                        parent_name,
                        dependent_field_to_parent_fields,
                        0,
                    )
                {
                    return Some((field_name, redundant_field, depth));
                }
            }
        }

        None
    }

    fn is_field_redundantly_dependent_on_parent<'r>(
        field_name: &'r str,
        parent_name: &'r str,
        dependent_field_to_parent_fields: &HashMap<&String, &Vec<&'r str>>,
        depth: i32,
    ) -> Option<(&'r str, i32)> {
        if let Some(parent_deps) = dependent_field_to_parent_fields
            .get(&field_name.to_string())
            .as_ref()
        {
            if parent_deps.contains(&parent_name) {
                return Some((parent_name, depth));
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

    /// Given fields a, b, c, d
    ///
    /// if a -> \[b\]  **&**  b -> \[a\]
    ///
    /// => circular_dependency_chain(a) = Some(vec!\[a, b\])
    ///
    /// if a -> \[b\]  **&**  b -> \[c\]  **&**  c -> \[a\]
    ///
    /// => circular_dependency_chain(a) = Some(vec!\[a, b, c\])
    ///
    /// if a -> \[b\]  **OR**  a -> \[b\]  **&**  b -> \[c, d\]  **OR**  a -> \[b\]  **&**  b -> \[c\]  **&**  c -> \[d\]
    ///
    /// => circular_dependency_chain(a) = None
    fn get_circular_dependency_chain<'c>(
        dependent_field_name: &'c str,
        parent_fields: &Vec<&'c str>,
        dependent_field_to_parent_fields: &HashMap<&String, &Vec<&'c str>>,
    ) -> Option<Vec<&'c str>> {
        for parent_name in parent_fields.iter() {
            if let Some(chain) = Self::is_field_circularly_dependent_on_parent(
                dependent_field_name,
                parent_name,
                dependent_field_to_parent_fields,
                vec![dependent_field_name],
            ) {
                return Some(chain);
            }
        }

        None
    }

    fn is_field_circularly_dependent_on_parent<'c>(
        dependent_field_name: &'c str,
        parent_name: &'c str,
        dependent_field_to_parent_fields: &HashMap<&String, &Vec<&'c str>>,
        mut visited_nodes: Vec<&'c str>,
    ) -> Option<Vec<&'c str>> {
        if let Some(parent_deps) = dependent_field_to_parent_fields
            .get(&parent_name.to_string())
            .as_ref()
        {
            visited_nodes.push(parent_name);

            if parent_deps.contains(&dependent_field_name) {
                return Some(visited_nodes);
            }

            for field_name in parent_deps.iter() {
                let r = Self::is_field_circularly_dependent_on_parent(
                    dependent_field_name,
                    field_name,
                    dependent_field_to_parent_fields,
                    visited_nodes.clone(),
                );

                if r.is_some() {
                    return r;
                }
            }

            return None;
        }

        None
    }
}

pub struct FieldBuilder<
    I: IvoStruct,
    O: IvoStruct,
    CtxOptions,
    T: IvoFieldValue,
    ErrorTool: IvoErrorTool,
    WithTimestamps = No,
> {
    _t: PhantomData<WithTimestamps>,
    configs: Vec<(String, InternalFieldConfig<I, O, CtxOptions, ErrorTool>)>,
    timestamp_config: Option<TimestampConfig<T>>,
}

impl<
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        Timestamp: Clone + Debug + Send + Sync + 'static,
        ErrorTool: IvoErrorTool,
    > FieldBuilder<I, O, CtxOptions, Timestamp, ErrorTool>
{
    fn new() -> Self {
        Self {
            configs: Vec::new(),
            timestamp_config: None,
            _t: PhantomData,
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
        I: IvoStruct,
        O: IvoStruct,
        CtxOptions,
        Timestamp: Clone + Debug + Send + Sync + 'static,
        ErrorTool: IvoErrorTool,
    > FieldBuilder<I, O, CtxOptions, Timestamp, ErrorTool>
{
    pub fn timestamps<BuildableConfig, R>(
        self,
        t: R,
    ) -> FieldBuilder<I, O, CtxOptions, Timestamp, ErrorTool, Yes>
    where
        BuildableConfig: BuildableTimestampConfig<Timestamp>,
        R: Fn(TimestampConfigBuilder<Timestamp>) -> BuildableConfig,
    {
        FieldBuilder {
            configs: self.configs,
            timestamp_config: Some(t(TimestampConfigBuilder::new()).build()),
            _t: PhantomData,
        }
    }
}
