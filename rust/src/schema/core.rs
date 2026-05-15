use crate::schema::utils::TimeStampTool;
use crate::schema::{error::SchemaError, properties::base::IvoProperty};
use crate::traits::HasPartial;
use crate::types::ComputableWithMiniSummary;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

type PropertyDefinitions<I, O, CtxOptions> = HashMap<String, IvoProperty<Value, I, O, CtxOptions>>;

pub struct SchemaCore<I: HasPartial, O: HasPartial, CtxOptions> {
    _definitions: PropertyDefinitions<I, O, CtxOptions>,
    _options: Option<Value>,

    // contexts & values
    pub context: HashMap<String, Value>,
    pub context_options: HashMap<String, Value>,
    pub defaults: HashMap<String, Value>,
    pub partial_context: HashMap<String, Value>,
    pub values: HashMap<String, Value>,

    // maps
    pub alias_to_virtual_map: HashMap<String, String>,
    pub dependency_map: HashMap<String, Vec<String>>,
    pub props_to_allowed_values_map: HashMap<String, HashSet<String>>,
    pub props_with_secondary_validators: HashSet<String>,
    pub virtual_to_alias_map: HashMap<String, String>,

    // post validation/onSuccess maps (simplified)
    pub post_validation_config_map: HashMap<String, Value>,
    pub prop_to_post_validation_config_ids_map: HashMap<String, HashSet<String>>,
    pub on_success_config_map: HashMap<String, Value>,
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
    pub timestamp_tool: TimeStampTool,
}

impl<I: HasPartial, O: HasPartial, CtxOptions> SchemaCore<I, O, CtxOptions> {
    pub fn new(definitions: PropertyDefinitions<I, O, CtxOptions>, options: Option<Value>) -> Self {
        let mut err_tool = SchemaError::new();

        if definitions.is_empty() {
            err_tool
                .add(
                    "schema properties",
                    "Insufficient Schema properties".to_string(),
                )
                .throw();
        }

        let timestamp_tool = TimeStampTool::new(options.as_ref());

        let mut core = Self {
            _definitions: definitions,
            _options: options,
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
            timestamp_tool,
        };

        core.check_prop_definitions();
        core.check_options();

        core
    }

    fn check_options(&self) {
        todo!()
    }

    fn check_prop_definitions(&mut self) {
        let mut err_tool = SchemaError::new();

        // First pass: register prop kinds and simple attributes
        for (prop, def) in &self._definitions {
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

            // regular prop
            self.props.insert(prop.clone());

            if let Some(defv) = &def.default {
                if let ComputableWithMiniSummary::Static(v) = defv {
                    self.defaults.insert(prop.clone(), v.clone());
                }
            }

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
                            .entry(p.clone())
                            .or_default()
                            .push(prop.clone());
                    }
                }
            }
        }

        // Second pass: checks that require knowledge of all props
        for (prop, def) in &self._definitions {
            if let Some(enum_values) = &def.enum_values {
                if enum_values.len() < 2 {
                    err_tool.add(
                        prop,
                        "Allowed values must have at least 2 values".to_string(),
                    );
                } else {
                    let mut uniq = HashSet::new();
                    let mut ok = true;

                    for v in enum_values {
                        let s = serde_json::to_string(v).unwrap_or_default();
                        if !uniq.insert(s) {
                            ok = false;
                            break;
                        }
                    }

                    if !ok {
                        err_tool.add(
                            prop,
                            "Allowed values must be an array of unique values".to_string(),
                        );
                    } else {
                        let set: HashSet<String> = enum_values
                            .iter()
                            .map(|v| serde_json::to_string(v).unwrap_or_default())
                            .collect();
                        self.props_to_allowed_values_map.insert(prop.clone(), set);

                        if let Some(default_val) = self.defaults.get(prop) {
                            let s = serde_json::to_string(default_val).unwrap_or_default();
                            if !self
                                .props_to_allowed_values_map
                                .get(prop)
                                .unwrap()
                                .contains(&s)
                            {
                                err_tool.add(
                                    prop,
                                    "The default value must be an allowed value".to_string(),
                                );
                            }
                        }
                    }
                }
            }
        }

        // Dependency analyses
        for dep in &self.dependents {
            let circular = self.get_circular_dependencies_of(dep);

            for c in circular {
                err_tool.add(dep, format!("Circular dependency identified with '{c}'"));
            }

            if let Some(def) = self._definitions.get(dep) {
                if let Some(parents) = &def.depends_on {
                    for parent in parents {
                        for other in parents {
                            if parent == other {
                                continue;
                            }

                            if self.is_redundant_dependency_of(dep, parent, other) {
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

    pub fn get_definition(&self, prop: &str) -> Option<&IvoProperty<Value, I, O, CtxOptions>> {
        self._definitions.get(prop)
    }

    pub fn get_definitions(&self) -> &PropertyDefinitions<I, O, CtxOptions> {
        &self._definitions
    }

    /// Resolve defaults iteratively based on dependencies.
    /// It will repeatedly evaluate defaults whose dependencies are satisfied (present in `context`).
    /// If there are unresolved defaults at the end and the schema option `error_on_unresolved_defaults` is true,
    /// this function returns Err(SchemaError) containing the unresolved properties; otherwise returns Ok(()).
    /// Return whether a name is a defined property
    pub fn is_prop(&self, prop: &str) -> bool {
        self.props.contains(prop)
    }

    /// Return whether a name is a defined virtual
    pub fn is_virtual(&self, prop: &str) -> bool {
        self.virtuals.contains(prop)
    }

    /// Return whether a name is a defined constant
    pub fn is_constant(&self, prop: &str) -> bool {
        self.constants.contains(prop)
    }

    pub fn get_reserved_keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.props.iter().cloned().collect();

        keys.extend(self.virtuals.iter().cloned());

        if let Some(k) = &self.timestamp_tool.get_keys().created_at {
            if !k.is_empty() {
                keys.push(k.clone());
            }
        }

        if let Some(k) = &self.timestamp_tool.get_keys().updated_at {
            if !k.is_empty() {
                keys.push(k.clone());
            }
        }

        keys.sort();
        keys
    }

    fn get_circular_dependencies_of(&self, _property: &str) -> Vec<String> {
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

    fn is_redundant_dependency_of(
        &self,
        property: &str,
        _parent_prop: &str,
        _other_parent: &str,
    ) -> bool {
        if !self.dependents.contains(property) {
            return false;
        }

        return false;

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
    fn find_cycles_in_pending(&self, pending: &HashSet<String>) -> HashMap<String, Vec<String>> {
        let mut cycles_map: HashMap<String, Vec<String>> = HashMap::new();

        for start in pending.iter() {
            let mut stack: Vec<String> = Vec::new();
            let mut visited: HashSet<String> = HashSet::new();
            self.dfs_find_cycles(
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

    fn dfs_find_cycles(
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

        if let Some(def) = self._definitions.get(node) {
            if let Some(deps) = &def.depends_on {
                for dep in deps.iter() {
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
                        self.dfs_find_cycles(start, dep, stack, visited, pending, cycles_map);
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
