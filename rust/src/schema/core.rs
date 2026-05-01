use crate::schema::definition::{DefaultValue, PropertyDefinition};
use crate::schema::utils::{SchemaError, TimeStampTool};
use crate::ValidatorResponse;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

pub struct SchemaCore {
    definitions: HashMap<String, PropertyDefinition>,
    options: Option<Value>,

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

impl SchemaCore {
    pub fn new(definitions: HashMap<String, PropertyDefinition>, options: Option<Value>) -> Self {
        let mut err_tool = SchemaError::new();

        if definitions.is_empty() {
            err_tool.add(
                "schema properties",
                "Insufficient Schema properties".to_string(),
            );

            err_tool.throw();
        }

        let timestamp_tool = TimeStampTool::new(options.as_ref());

        let mut core = Self {
            definitions,
            options,
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
        for (prop, def) in &self.definitions {
            // virtuals
            if def.virtual_prop {
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
            if def.constant {
                self.constants.insert(prop.clone());
                if def.value.is_none() {
                    err_tool.add(
                        prop,
                        "Constant properties must have a value or setter".to_string(),
                    );
                } else if def.default.is_none() {
                    if let Some(crate::schema::definition::ConstantValue::Static(v)) = &def.value {
                        self.defaults.insert(prop.clone(), v.clone());
                    }
                }

                continue;
            }

            // regular prop
            self.props.insert(prop.clone());

            if let Some(defv) = &def.default {
                if let DefaultValue::Static(v) = defv {
                    self.defaults.insert(prop.clone(), v.clone());
                }
            }

            if def.required == Some(true) {
                self.required_props.insert(prop.clone());
            }

            if def.readonly == Some(true) {
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
        for (prop, def) in &self.definitions {
            if let Some(allowed) = &def.allow_values {
                if allowed.len() < 2 {
                    err_tool.add(
                        prop,
                        "Allowed values must have at least 2 values".to_string(),
                    );
                } else {
                    let mut uniq = HashSet::new();
                    let mut ok = true;
                    for v in allowed {
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
                        let set: HashSet<String> = allowed
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

            if let Some(validators) = &def.validator {
                if validators.is_empty() || validators.len() > 2 {
                    err_tool.add(
                        prop,
                        "Validator array must contain exactly 1 or 2 functions".to_string(),
                    );
                } else if validators.len() == 2 {
                    self.props_with_secondary_validators.insert(prop.clone());
                }
            }
        }

        // Dependency analyses
        for dep in &self.dependents {
            let circular = self.get_circular_dependencies_of(dep);
            for c in circular {
                err_tool.add(dep, format!("Circular dependency identified with '{c}'"));
            }

            if let Some(def) = self.definitions.get(dep) {
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

        err_tool.throw();
    }

    pub fn get_definition(&self, prop: &str) -> Option<&PropertyDefinition> {
        self.definitions.get(prop)
    }

    pub fn get_default_value(&self, prop: &str, ctx: &HashMap<String, Value>) -> Option<Value> {
        let def = self.definitions.get(prop)?;
        match &def.default {
            Some(DefaultValue::Static(v)) => Some(v.clone()),
            Some(DefaultValue::Func(f)) => Some(f(ctx)),
            None => None,
        }
    }

    pub fn get_constant_value(&self, prop: &str, ctx: &HashMap<String, Value>) -> Option<Value> {
        let def = self.definitions.get(prop)?;
        match &def.value {
            Some(crate::schema::definition::ConstantValue::Static(v)) => Some(v.clone()),
            Some(crate::schema::definition::ConstantValue::Func(f)) => Some(f(ctx)),
            None => None,
        }
    }

    fn get_option_bool(&self, key: &str) -> bool {
        if let Some(opts) = &self.options {
            if let Some(obj) = opts.as_object() {
                if let Some(v) = obj.get(key) {
                    return v.as_bool().unwrap_or(false);
                }
            }
        }

        false
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

    /// Resolve defaults iteratively based on dependencies.
    /// It will repeatedly evaluate defaults whose dependencies are satisfied (present in `context`).
    /// If unresolved defaults remain and schema option `error_on_unresolved_defaults` is true,
    /// returns Err(SchemaError) listing the unresolved props.
    pub fn resolve_defaults(&self, context: &mut HashMap<String, Value>) {
        let mut pending: HashSet<String> = self
            .definitions
            .iter()
            .filter_map(|(k, def)| {
                if def.default.is_some() {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .filter(|k| !context.contains_key(k))
            .collect();

        let mut progress = true;

        while progress && !pending.is_empty() {
            progress = false;
            let mut resolved: Vec<String> = Vec::new();

            for prop in pending.iter() {
                if let Some(def) = self.definitions.get(prop) {
                    let deps = def
                        .depends_on
                        .as_ref()
                        .map(|v| v.as_slice())
                        .unwrap_or(&[] as &[String]);

                    // ready if all deps are present in context or are not properties
                    let ready = deps
                        .iter()
                        .all(|d| context.contains_key(d) || !self.definitions.contains_key(d));

                    if ready {
                        if let Some(val) = self.get_default_value(prop, context) {
                            context.insert(prop.clone(), val);
                            resolved.push(prop.clone());
                            progress = true;
                        }
                    }
                } else {
                    // unknown definition -> mark resolved to avoid infinite loops
                    resolved.push(prop.clone());
                }
            }

            for r in resolved {
                pending.remove(&r);
            }
        }

        if !pending.is_empty() {
            if self.get_option_bool("error_on_unresolved_defaults") {
                // find cycles among pending
                let cycles = self.find_cycles_in_pending(&pending);

                let mut payload = std::collections::HashMap::new();

                for p in pending.iter() {
                    if let Some(paths) = cycles.get(p) {
                        // report all cycle paths involving this property
                        payload.insert(p.clone(), paths.clone());
                    } else {
                        payload.insert(
                            p.clone(),
                            vec!["Unresolved default due to missing dependencies".to_string()],
                        );
                    }
                }
            }
        }
    }

    /// Resolve constants iteratively; constants may depend on other values in context.
    /// If unresolved constants remain and schema option `error_on_unresolved_constants` is true,
    /// returns Err(SchemaError) listing unresolved constants; otherwise returns Ok(())
    pub fn resolve_constants(&self, context: &mut HashMap<String, Value>) {
        let mut pending: HashSet<String> = self
            .constants
            .iter()
            .filter(|k| !context.contains_key(*k))
            .cloned()
            .collect();

        let mut progress = true;

        while progress && !pending.is_empty() {
            progress = false;
            let mut resolved = Vec::new();

            for prop in pending.iter() {
                if let Some(def) = self.definitions.get(prop) {
                    let deps = def
                        .depends_on
                        .as_ref()
                        .map(|v| v.as_slice())
                        .unwrap_or(&[] as &[String]);
                    let ready = deps
                        .iter()
                        .all(|d| context.contains_key(d) || !self.definitions.contains_key(d));

                    if ready {
                        if let Some(val) = self.get_constant_value(prop, context) {
                            context.insert(prop.clone(), val);
                            resolved.push(prop.clone());
                            progress = true;
                        }
                    }
                } else {
                    resolved.push(prop.clone());
                }
            }

            for r in resolved {
                pending.remove(&r);
            }
        }

        if !pending.is_empty() {
            if self.get_option_bool("error_on_unresolved_constants") {
                // detect cycles among pending constants
                let cycles = self.find_cycles_in_pending(&pending);

                let mut payload = std::collections::HashMap::new();

                for p in pending.iter() {
                    if let Some(paths) = cycles.get(p) {
                        payload.insert(p.clone(), paths.clone());
                    } else {
                        payload.insert(
                            p.clone(),
                            vec!["Unresolved constant due to missing dependencies".to_string()],
                        );
                    }
                }
            }
        }
    }

    pub fn run_validators(&self, prop: &str, value: &Value) -> Option<ValidatorResponse<Value>> {
        let def = self.definitions.get(prop)?;
        let validators = def.validator.as_ref()?;

        if validators.is_empty() {
            return None;
        }

        let ctx = HashMap::new();
        let primary = &validators[0];

        match primary(value, &ctx) {
            Ok(v) => {
                if validators.len() == 2 {
                    let secondary = &validators[1];
                    Some(secondary(&v, &ctx))
                } else {
                    Some(Ok(v))
                }
            }
            Err(e) => Some(Err(e)),
        }
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

    fn get_circular_dependencies_of(&self, property: &str) -> Vec<String> {
        let mut circular = Vec::new();

        if !self.dependents.contains(property) {
            return circular;
        }

        fn dfs(
            core: &SchemaCore,
            start: &str,
            node: &str,
            visited: &mut Vec<String>,
            out: &mut Vec<String>,
        ) {
            if !core.dependents.contains(node) || visited.contains(&node.to_string()) {
                return;
            }
            if start != node {
                visited.push(node.to_string());
            }

            if let Some(def) = core.definitions.get(node) {
                if let Some(deps) = &def.depends_on {
                    for s in deps {
                        if s == start {
                            out.push(node.to_string());
                        } else if core.dependents.contains(s) {
                            dfs(core, start, s, visited, out);
                        }
                    }
                }
            }
        }

        dfs(self, property, property, &mut Vec::new(), &mut circular);

        circular.sort();
        circular.dedup();
        circular
    }

    fn is_redundant_dependency_of(
        &self,
        property: &str,
        parent_prop: &str,
        other_parent: &str,
    ) -> bool {
        if !self.dependents.contains(property) {
            return false;
        }

        fn is_in_transitive(
            core: &SchemaCore,
            start: &str,
            target: &str,
            visited: &mut HashSet<String>,
        ) -> bool {
            if visited.contains(start) {
                return false;
            }
            visited.insert(start.to_string());

            if let Some(def) = core.definitions.get(start) {
                if let Some(deps) = &def.depends_on {
                    for s in deps {
                        if s == target {
                            return true;
                        }
                        if is_in_transitive(core, s, target, visited) {
                            return true;
                        }
                    }
                }
            }

            false
        }

        let mut visited = HashSet::new();
        is_in_transitive(self, other_parent, parent_prop, &mut visited)
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

        if let Some(def) = self.definitions.get(node) {
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
