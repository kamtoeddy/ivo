use crate::schema::fields::types::IntoUniformTimestampResolver;
use crate::schema::fields::TimestampFieldConfig;
use crate::schema::options::base::{SchemaOptions, SchemaOptionsBuilder};
use crate::schema::options::BuildableSchemaOptions;
use crate::utils::erased_value::ErasedValue;

use crate::schema::error::{DefaultErrorTool, IvoErrorTool, SchemaError};
use crate::schema::fields::base::{BuildableFieldConfig, InternalFieldConfig};
use crate::types::{IvoSchemaStruct, No, Yes};

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::marker::PhantomData;

type InternalFieldConfigs<I, O, CtxOptions, ErrorTool> =
    HashMap<String, InternalFieldConfig<I, O, CtxOptions, ErrorTool>>;

pub struct Schema<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct = I,
    CtxOptions: Clone = Option<u8>,
    ErrorTool: IvoErrorTool = DefaultErrorTool,
> {
    field_configs: InternalFieldConfigs<I, O, CtxOptions, ErrorTool>,
    _options: SchemaOptions<I, O, CtxOptions, ErrorTool>,

    _timestamp_created_at: Option<TimestampFieldConfig>,
    _timestamp_upated_at: Option<TimestampFieldConfig>,

    // contexts & values
    pub defaults: HashMap<String, ErasedValue>,
    field_names: HashSet<String>,

    // maps
    pub alias_to_virtual_map: HashMap<String, String>,
    pub dependency_map: HashMap<String, Vec<String>>,
    pub props_to_allowed_values_map: HashMap<String, HashSet<String>>,
    pub props_with_secondary_validators: HashSet<String>,
    pub virtual_to_alias_map: HashMap<String, String>,

    // post validation/onSuccess maps (simplified)
    pub post_validation_config_map: HashMap<String, ErasedValue>,
    pub prop_to_post_validation_config_ids_map: HashMap<String, HashSet<String>>,
    pub on_success_config_map: HashMap<String, ErasedValue>,
    pub prop_to_on_success_config_id_map: HashMap<String, HashSet<String>>,

    // props
    pub constants: HashSet<String>,
    pub dependents: HashSet<String>,
    pub lax_props: HashSet<String>,
    pub props: HashSet<String>,
    pub props_required_by: HashSet<String>,
    pub readonly_props: HashSet<String>,
    pub required_props: HashSet<String>,
    pub virtuals: HashSet<String>,
    // helpers
    // pub timestamp_tool: TimeStampTool,
}

impl<'a, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrorTool: IvoErrorTool>
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
            _options: options_maker(SchemaOptions::new()).build(),
            field_names: {
                let mut names = O::ivo_internal_field_names();
                names.extend(I::ivo_internal_field_names());
                names.into_iter().collect()
            },
            defaults: HashMap::new(),
            alias_to_virtual_map: HashMap::new(),
            dependency_map: HashMap::new(),
            props_to_allowed_values_map: HashMap::new(),
            props_with_secondary_validators: HashSet::new(),
            virtual_to_alias_map: HashMap::new(),
            post_validation_config_map: HashMap::new(),
            prop_to_post_validation_config_ids_map: HashMap::new(),
            on_success_config_map: HashMap::new(),
            prop_to_on_success_config_id_map: HashMap::new(),
            constants: HashSet::new(),
            dependents: HashSet::new(),
            lax_props: HashSet::new(),
            props: HashSet::new(),
            props_required_by: HashSet::new(),
            readonly_props: HashSet::new(),
            required_props: HashSet::new(),
            virtuals: HashSet::new(),
            _timestamp_created_at: fields.timestamp_created_at,
            _timestamp_upated_at: fields.timestamp_upated_at,
        };

        s.check_field_configs();
        s.check_options();

        s
    }

    pub fn fields(&self) -> Vec<String> {
        let mut all_fields: Vec<_> = self.fields_set().clone().into_iter().collect();

        all_fields.sort();
        all_fields
    }

    pub fn fields_set(&self) -> &HashSet<String> {
        &self.field_names
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
        let mut err_tool = SchemaError::new();

        // First pass: register prop kinds and simple attributes
        for (prop, def) in &self.field_configs {
            // virtuals
            if def.is_virtual {
                self.virtuals.insert(prop.clone());

                if let Some(alias) = &def.alias {
                    if alias == prop {
                        err_tool.add(
                            prop,
                            "An alias cannot be the same as the virtual property".to_string(),
                        );
                    } else if self.alias_to_virtual_map.contains_key(alias) {
                        let existing = self.alias_to_virtual_map.get(alias).unwrap().clone();
                        err_tool.add(
                            prop,
                            format!(
                                "Sorry, alias provided '{alias}' already belongs to property '{existing}'"
                            ),
                        );
                    } else {
                        self.alias_to_virtual_map
                            .insert(alias.clone(), prop.clone());

                        self.virtual_to_alias_map
                            .insert(prop.clone(), alias.clone());
                    }
                }

                continue;
            }

            // constants
            if def.is_constant {
                self.constants.insert(prop.clone());

                continue;
            }

            // regular props
            self.props.insert(prop.clone());

            // if let Some(ComputableWithMiniSummary::Static(v)) = &def.default {
            //     self.defaults.insert(prop.clone(), v);
            // }

            if def.required.is_some() {
                self.required_props.insert(prop.clone());
            }

            if def.is_readonly {
                self.readonly_props.insert(prop.clone());
            }

            if let Some(depends) = &def.depends_on {
                if depends.is_empty() {
                    err_tool.add(
                        prop,
                        "Dependent properties must depend on at least one property".to_string(),
                    );
                } else {
                    self.dependents.insert(prop.clone());

                    for p in depends {
                        self.dependency_map
                            .entry(p.to_string())
                            .or_default()
                            .push(prop.clone());
                    }
                }
            }
        }

        // Dependency analyses
        for dep in &self.dependents {
            let circular = self._get_circular_dependencies_of(dep);

            for c in circular {
                err_tool.add(dep, format!("Circular dependency identified with '{c}'"));
            }

            if let Some(def) = self.field_configs.get(dep) {
                if let Some(parents) = &def.depends_on {
                    for parent in parents {
                        for other in parents {
                            if parent == other {
                                continue;
                            }

                            if self._is_redundant_dependency_of(dep, parent, other) {
                                err_tool.add(dep, format!("Dependency on '{parent}' is redundant because of dependency on '{other}'" ));
                            }
                        }
                    }
                }
            }
        }

        if err_tool.is_payload_loaded() {
            err_tool.throw();
        }
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

    fn _get_circular_dependencies_of(&self, _property: &str) -> Vec<String> {
        let circular = Vec::new();

        // if !self.dependents.contains(property) {
        //     return circular;
        // }

        // let dfs =
        //     move |start: &str, node: &str, visited: &mut Vec<String>, out: &mut Vec<String>| {
        //         if !&self.dependents.contains(node) || visited.contains(&node.to_string()) {
        //             return;
        //         }

        //         if start != node {
        //             visited.push(node.to_string());
        //         }

        //         if let Some(def) = &self._definitions.get(node) {
        //             if let Some(deps) = &def.depends_on {
        //                 for s in deps {
        //                     if s == start {
        //                         out.push(node.to_string());
        //                     } else if &self.dependents.contains(s) {
        //                         dfs(start, s, visited, out);
        //                     }
        //                 }
        //             }
        //         }
        //     };

        // dfs(property, property, &mut Vec::new(), &mut circular);

        // circular.sort();
        // circular.dedup();
        circular
    }

    fn _is_redundant_dependency_of(
        &self,
        property: &str,
        _parent_prop: &str,
        _other_parent: &str,
    ) -> bool {
        if !self.dependents.contains(property) {
            return false;
        }

        false

        // fn is_in_transitive(
        //     core: &SchemaCore,
        //     start: &str,
        //     target: &str,
        //     visited: &mut HashSet<String>,
        // ) -> bool {
        //     if visited.contains(start) {
        //         return false;
        //     }
        //     visited.insert(start.to_string());

        //     if let Some(def) = core._definitions.get(start) {
        //         if let Some(deps) = &def.depends_on {
        //             for s in deps {
        //                 if s == target {
        //                     return true;
        //                 }
        //                 if is_in_transitive(core, s, target, visited) {
        //                     return true;
        //                 }
        //             }
        //         }
        //     }

        //     false
        // }

        // let mut visited = HashSet::new();
        // is_in_transitive(self, other_parent, parent_prop, &mut visited)
    }

    /// Find cycles among a set of pending nodes. Returns a map from node -> list of cycle path strings
    fn _find_cycles_in_pending(&self, pending: &HashSet<String>) -> HashMap<String, Vec<String>> {
        let mut cycles_map: HashMap<String, Vec<String>> = HashMap::new();

        for start in pending.iter() {
            let mut stack: Vec<String> = Vec::new();
            let mut visited: HashSet<String> = HashSet::new();
            self._dfs_find_cycles(
                start,
                start,
                &mut stack,
                &mut visited,
                pending,
                &mut cycles_map,
            );
        }

        cycles_map
    }

    fn _dfs_find_cycles(
        &self,
        start: &str,
        node: &str,
        stack: &mut Vec<String>,
        visited: &mut HashSet<String>,
        pending: &HashSet<String>,
        cycles_map: &mut HashMap<String, Vec<String>>,
    ) {
        if visited.contains(node) {
            // already explored this path for start
            return;
        }

        visited.insert(node.to_string());
        stack.push(node.to_string());

        if let Some(def) = self.field_configs.get(node) {
            if let Some(deps) = &def.depends_on {
                for dep in deps.iter() {
                    let dep = &dep.to_string();

                    if !pending.contains(dep) {
                        continue; // only consider edges within pending set
                    }

                    if dep == start {
                        // found cycle: stack + start
                        let mut path = stack.clone();
                        path.push(start.to_string());
                        let path_str = path.join(" -> ");

                        // record this path for all nodes in the cycle
                        for n in path.iter() {
                            cycles_map
                                .entry(n.clone())
                                .or_default()
                                .push(path_str.clone());
                        }

                        continue;
                    }

                    if !stack.contains(dep) {
                        self._dfs_find_cycles(start, dep, stack, visited, pending, cycles_map);
                    } else {
                        // encountered a back-edge not to start: record cycle portion
                        let pos = stack.iter().position(|s| s == dep).unwrap_or(0);
                        let mut path = stack[pos..].to_vec();
                        path.push(dep.clone());
                        let path_str = path.join(" -> ");
                        for n in path.iter() {
                            cycles_map
                                .entry(n.clone())
                                .or_default()
                                .push(path_str.clone());
                        }
                    }
                }
            }
        }

        stack.pop();
    }
}

pub struct FieldBuilder<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
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
        CtxOptions: Clone,
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
        CtxOptions: Clone,
        ErrorTool: IvoErrorTool,
    > Default for FieldBuilder<I, O, CtxOptions, ErrorTool, HasCreatedAt, HasUpdatedAt>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<
        HasUpdatedAt,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrorTool: IvoErrorTool,
    > FieldBuilder<I, O, CtxOptions, ErrorTool, No, HasUpdatedAt>
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

impl<
        HasCreatedAt,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions: Clone,
        ErrorTool: IvoErrorTool,
    > FieldBuilder<I, O, CtxOptions, ErrorTool, HasCreatedAt>
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
