use crate::schema::options::base::{SchemaOptions, SchemaOptionsBuilder};
use crate::schema::options::BuildableSchemaOptions;
use crate::utils::erased_value::ErasedValue;

use crate::schema::error::{DefaultErrorTool, IvoErrorTool, SchemaError};
use crate::schema::fields::base::{BuildableFieldConfig, InternalFieldConfig};
use crate::traits::IvoSchemaStruct;

use std::collections::{HashMap, HashSet};

type InternalFieldConfigs<I, O, CtxOptions, ErrT> =
    HashMap<String, InternalFieldConfig<I, O, CtxOptions, ErrT>>;

pub struct Schema<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct = I,
    CtxOptions: Clone = Option<u8>,
    ErrorTool: IvoErrorTool = DefaultErrorTool,
> {
    field_configs: InternalFieldConfigs<I, O, CtxOptions, ErrorTool>,
    _options: SchemaOptions<I, O, CtxOptions, ErrorTool>,

    // contexts & values
    pub context: HashMap<String, ErasedValue>,
    pub context_options: HashMap<String, ErasedValue>,
    pub defaults: HashMap<String, ErasedValue>,
    pub partial_context: HashMap<String, ErasedValue>,
    pub values: HashMap<String, ErasedValue>,
    fields_set: HashSet<String>,

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
    pub fn new<FieldBuilder, OptionsBuilder, BuildableOptions>(
        fields_builder: FieldBuilder,
        options_builder: OptionsBuilder,
    ) -> Self
    where
        FieldBuilder: Fn(
            SchemaFields<I, O, CtxOptions, ErrorTool>,
        ) -> SchemaFields<I, O, CtxOptions, ErrorTool>,
        OptionsBuilder: Fn(SchemaOptionsBuilder<I, O, CtxOptions, ErrorTool>) -> BuildableOptions,
        BuildableOptions: BuildableSchemaOptions<I, O, CtxOptions, ErrorTool>,
    {
        let mut s = Self {
            field_configs: fields_builder(SchemaFields::new()).configs,
            _options: options_builder(SchemaOptions::new()).build(),
            fields_set: {
                let mut all_fields = O::ivo_internal_fields();
                all_fields.extend(I::ivo_internal_fields());
                all_fields.into_iter().collect()
            },
            context: HashMap::new(),
            context_options: HashMap::new(),
            defaults: HashMap::new(),
            partial_context: HashMap::new(),
            values: HashMap::new(),
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
        &self.fields_set
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

        // Second pass: checks that require knowledge of all props
        for (prop, def) in &self.field_configs {
            if let Some(enum_values) = &def.enum_values {
                if enum_values.len() < 2 {
                    err_tool.add(
                        prop,
                        "Allowed values must have at least 2 values".to_string(),
                    );
                } else {
                    // let mut uniq = HashSet::new();
                    // let mut ok = true;

                    // for v in enum_values {
                    //     if !uniq.insert(v) {
                    //         ok = false;
                    //         break;
                    //     }
                    // }

                    // if !ok {
                    //     err_tool.add(
                    //         prop,
                    //         "Allowed values must be an array of unique values".to_string(),
                    //     );
                    // } else {
                    // let set: HashSet<ErasedValue> =
                    //     enum_values.iter().map(|v| v.clone()).collect();
                    // self.props_to_allowed_values_map.insert(prop.clone(), set);

                    // if let Some(default_val) = self.defaults.get(prop) {
                    //     if !self
                    //         .props_to_allowed_values_map
                    //         .get(prop)
                    //         .unwrap()
                    //         .contains(default_val)
                    //     {
                    //         err_tool.add(
                    //             prop,
                    //             "The default value must be an allowed value".to_string(),
                    //         );
                    //     }
                    // }
                    // }
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

pub struct SchemaFields<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions: Clone,
    ErrorTool: IvoErrorTool,
> {
    configs: InternalFieldConfigs<I, O, CtxOptions, ErrorTool>,
}

impl<I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions: Clone, ErrorTool: IvoErrorTool>
    SchemaFields<I, O, CtxOptions, ErrorTool>
{
    fn new() -> Self {
        Self {
            configs: HashMap::new(),
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
