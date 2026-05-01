use crate::error::{FieldError, IvoError};
use crate::types::{ComputedValue, Context, ImmutableSummary, MutableSummary};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[async_trait]
pub trait Validator<I, O>: Send + Sync {
    async fn validate(
        &self,
        value: &serde_json::Value,
        summary: &MutableSummary<I, O>,
    ) -> Result<serde_json::Value, String>;
}

pub struct PropertyDefinition<I, O> {
    pub virtual_prop: bool,
    pub alias: Option<String>,
    pub constant: bool,
    pub value: Option<ComputedValue<I, O>>,
    pub default: Option<ComputedValue<I, O>>,
    pub required: Option<bool>,
    pub readonly: bool,
    pub depends_on: Vec<String>,
    pub primary_validators: Vec<Box<dyn Validator<I, O>>>,
    pub secondary_validators: Vec<Box<dyn Validator<I, O>>>,
    pub allow_values: Option<Vec<Value>>,
    pub allow_error: Option<String>,
    pub resolver: Option<Box<dyn Fn(&MutableSummary<I, O>) -> Value + Send + Sync>>,
}

impl<I, O> PropertyDefinition<I, O> {
    pub fn builder() -> PropertyBuilder<I, O> {
        PropertyBuilder::default()
    }
}

pub struct PropertyBuilder<I, O> {
    virtual_prop: bool,
    alias: Option<String>,
    constant: bool,
    value: Option<ComputedValue<I, O>>,
    default: Option<ComputedValue<I, O>>,
    required: Option<bool>,
    readonly: bool,
    depends_on: Vec<String>,
    primary_validators: Vec<Box<dyn Validator<I, O>>>,
    secondary_validators: Vec<Box<dyn Validator<I, O>>>,
    allow_values: Option<Vec<Value>>,
    allow_error: Option<String>,
    resolver: Option<Box<dyn Fn(&MutableSummary<I, O>) -> Value + Send + Sync>>,
}

impl<I, O> Default for PropertyBuilder<I, O> {
    fn default() -> Self {
        Self {
            virtual_prop: false,
            alias: None,
            constant: false,
            value: None,
            default: None,
            required: None,
            readonly: false,
            depends_on: Vec::new(),
            primary_validators: Vec::new(),
            secondary_validators: Vec::new(),
            allow_values: None,
            allow_error: None,
            resolver: None,
        }
    }
}

impl<I, O> PropertyBuilder<I, O> {
    pub fn virtual_prop(mut self, v: bool) -> Self {
        self.virtual_prop = v;
        self
    }
    pub fn alias(mut self, a: impl Into<String>) -> Self {
        self.alias = Some(a.into());
        self
    }
    pub fn constant(mut self, v: ComputedValue<I, O>) -> Self {
        self.constant = true;
        self.value = Some(v);
        self
    }
    pub fn default_value(mut self, v: ComputedValue<I, O>) -> Self {
        self.default = Some(v);
        self
    }
    pub fn required(mut self, r: bool) -> Self {
        self.required = Some(r);
        self
    }
    pub fn readonly(mut self, r: bool) -> Self {
        self.readonly = r;
        self
    }
    pub fn depends_on(mut self, deps: Vec<String>) -> Self {
        self.depends_on = deps;
        self
    }
    pub fn primary_validator(mut self, v: Box<dyn Validator<I, O>>) -> Self {
        self.primary_validators.push(v);
        self
    }
    pub fn secondary_validator(mut self, v: Box<dyn Validator<I, O>>) -> Self {
        self.secondary_validators.push(v);
        self
    }
    pub fn allow_values(mut self, vals: Vec<Value>) -> Self {
        self.allow_values = Some(vals);
        self
    }
    pub fn allow_error(mut self, e: impl Into<String>) -> Self {
        self.allow_error = Some(e.into());
        self
    }
    pub fn resolver<F: Fn(&MutableSummary<I, O>) -> Value + Send + Sync + 'static>(
        mut self,
        f: F,
    ) -> Self {
        self.resolver = Some(Box::new(f));
        self
    }
    pub fn build(self) -> PropertyDefinition<I, O> {
        PropertyDefinition {
            virtual_prop: self.virtual_prop,
            alias: self.alias,
            constant: self.constant,
            value: self.value,
            default: self.default,
            required: self.required,
            readonly: self.readonly,
            depends_on: self.depends_on,
            primary_validators: self.primary_validators,
            secondary_validators: self.secondary_validators,
            allow_values: self.allow_values,
            allow_error: self.allow_error,
            resolver: self.resolver,
        }
    }
}

pub enum IvoField<I, O> {
    Constant {
        value: ComputedValue<I, O>,
    },
    Dependent {
        default: ComputedValue<I, O>,
        depends_on: Vec<String>,
        resolver: Box<dyn Fn(&MutableSummary<I, O>) -> Value + Send + Sync>,
        readonly: bool,
    },
    Lax {
        default: ComputedValue<I, O>,
        validator: Option<Box<dyn Validator<I, O>>>,
    },
    ReadOnly {
        default: ComputedValue<I, O>,
        validator: Box<dyn Validator<I, O>>,
    },
    Virtual {
        alias: Option<String>,
        validator: Option<Box<dyn Validator<I, O>>>,
    },
}

impl<I, O> IvoField<I, O>
where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
{
    pub fn to_definition(self) -> PropertyDefinition<I, O> {
        match self {
            IvoField::Constant { value } => PropertyBuilder::default().constant(value).build(),
            IvoField::Dependent {
                default,
                depends_on,
                resolver,
                readonly,
            } => {
                let mut b = PropertyBuilder::default()
                    .default_value(default)
                    .depends_on(depends_on)
                    .readonly(readonly);
                b.resolver = Some(resolver);
                b.build()
            }
            IvoField::Lax { default, validator } => {
                let mut b = PropertyBuilder::default().default_value(default);
                if let Some(v) = validator {
                    b = b.primary_validator(v);
                }
                b.build()
            }
            IvoField::ReadOnly { default, validator } => PropertyBuilder::default()
                .default_value(default)
                .primary_validator(validator)
                .readonly(true)
                .build(),
            IvoField::Virtual { alias, validator } => {
                let mut b = PropertyBuilder::default().virtual_prop(true);
                if let Some(a) = alias {
                    b = b.alias(a);
                }
                if let Some(v) = validator {
                    b = b.primary_validator(v);
                }
                b.build()
            }
        }
    }
}

#[async_trait]
pub trait PostValidator<I, O>: Send + Sync {
    /// Returns a map of field names to updated values on success,
    /// or a map of field names to error messages on failure.
    async fn validate(
        &self,
        summary: &MutableSummary<I, O>,
    ) -> Result<HashMap<String, Value>, HashMap<String, String>>;
}

pub struct PostValidationConfig<I, O> {
    pub properties: Vec<String>,
    pub validator: Box<dyn PostValidator<I, O>>,
}

pub struct Schema<I: Clone + Eq + PartialEq, O: Clone + Eq + PartialEq> {
    pub definitions: HashMap<String, PropertyDefinition<I, O>>,
    pub post_validators: Vec<PostValidationConfig<I, O>>,
    // ... options
}

impl<I, O> Schema<I, O>
where
    I: Clone + Eq + PartialEq + Serialize + for<'de> Deserialize<'de> + Send + Sync,
    O: Clone
        + Eq
        + PartialEq
        + Serialize
        + for<'de> Deserialize<'de>
        + Send
        + Sync
        + Default
        + 'static,
{
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            post_validators: Vec::new(),
        }
    }

    pub async fn create(&self, input: I) -> Result<O, IvoError> {
        let mut input_val =
            serde_json::to_value(&input).map_err(|e| IvoError::InvalidSchema(e.to_string()))?;
        let mut values = serde_json::Map::new();
        let mut errors = HashMap::new();

        // Prepare context
        let context_options = Arc::new(Mutex::new(HashMap::new()));

        // Handle aliases: if alias is provided, map it to the target property
        let mut input_obj = input_val
            .as_object_mut()
            .ok_or_else(|| IvoError::InvalidSchema("Input must be an object".to_string()))?
            .clone();
        for (name, def) in &self.definitions {
            if let Some(alias) = &def.alias {
                if let Some(val) = input_obj.get(alias) {
                    input_obj.insert(name.clone(), val.clone());
                }
            }
        }
        input_val = Value::Object(input_obj.clone());
        let input_arc = Arc::new(input.clone());

        // 1. Primary Validation
        for (name, def) in &self.definitions {
            let provided_val = input_obj.get(name);

            // Determine initial value: provided, constant, or default
            let mut current_val = match provided_val {
                Some(val) => val.clone(),
                None => {
                    if def.constant {
                        if let Some(comp) = &def.value {
                            let summary =
                                self.create_summary(&input_arc, &values, &context_options, false);
                            match comp {
                                ComputedValue::Static(v) => v.clone(),
                                ComputedValue::Func(f) => f(&summary),
                            }
                        } else {
                            Value::Null
                        }
                    } else if let Some(comp) = &def.default {
                        let summary =
                            self.create_summary(&input_arc, &values, &context_options, false);
                        match comp {
                            ComputedValue::Static(v) => v.clone(),
                            ComputedValue::Func(f) => f(&summary),
                        }
                    } else if def.required == Some(true) {
                        errors
                            .entry(name.clone())
                            .or_insert_with(Vec::new)
                            .push(FieldError {
                                reason: format!("{} is required!", name),
                                metadata: None,
                            });
                        continue;
                    } else {
                        Value::Null
                    }
                }
            };

            // Allow values check
            if let Some(allowed) = &def.allow_values {
                if !allowed.contains(&current_val) {
                    errors
                        .entry(name.clone())
                        .or_insert_with(Vec::new)
                        .push(FieldError {
                            reason: def
                                .allow_error
                                .clone()
                                .unwrap_or_else(|| "Value not allowed".to_string()),
                            metadata: Some(serde_json::to_value(allowed).unwrap()),
                        });
                    continue;
                }
            }

            // Run validators
            for validator in &def.primary_validators {
                let summary = self.create_summary(&input_arc, &values, &context_options, false);
                match validator.validate(&current_val, &summary).await {
                    Ok(validated) => current_val = validated,
                    Err(reason) => {
                        errors
                            .entry(name.clone())
                            .or_insert_with(Vec::new)
                            .push(FieldError {
                                reason,
                                metadata: None,
                            });
                        break;
                    }
                }
            }
            values.insert(name.clone(), current_val);
        }

        if !errors.is_empty() {
            return Err(IvoError::ValidationError(errors));
        }

        // 2. Secondary Validation
        for (name, def) in &self.definitions {
            if let Some(current_val) = values.get(name) {
                let mut current_val = current_val.clone();
                for validator in &def.secondary_validators {
                    let summary = self.create_summary(&input_arc, &values, &context_options, false);
                    match validator.validate(&current_val, &summary).await {
                        Ok(validated) => current_val = validated,
                        Err(reason) => {
                            errors
                                .entry(name.clone())
                                .or_insert_with(Vec::new)
                                .push(FieldError {
                                    reason,
                                    metadata: None,
                                });
                            break;
                        }
                    }
                }
                values.insert(name.clone(), current_val);
            }
        }

        if !errors.is_empty() {
            return Err(IvoError::ValidationError(errors));
        }

        // 3. Post Validation
        for config in &self.post_validators {
            let summary = self.create_summary(&input_arc, &values, &context_options, false);
            match config.validator.validate(&summary).await {
                Ok(updates) => {
                    for (name, val) in updates {
                        if config.properties.contains(&name) {
                            values.insert(name, val);
                        }
                    }
                }
                Err(post_errors) => {
                    for (name, reason) in post_errors {
                        errors
                            .entry(name)
                            .or_insert_with(Vec::new)
                            .push(FieldError {
                                reason,
                                metadata: None,
                            });
                    }
                }
            }
        }

        if !errors.is_empty() {
            return Err(IvoError::ValidationError(errors));
        }

        // 4. Resolvers (Dependent properties)
        for (name, def) in &self.definitions {
            if let Some(resolver) = &def.resolver {
                // If any dependency changed or it's a create, run resolver
                // For create, we always run it.
                let summary = self.create_summary(&input_arc, &values, &context_options, false);
                let resolved_val = resolver(&summary);
                values.insert(name.clone(), resolved_val);
            }
        }

        // Filter out virtual properties before returning
        let mut final_values = serde_json::Map::new();
        for (name, def) in &self.definitions {
            if !def.virtual_prop {
                if let Some(val) = values.get(name) {
                    final_values.insert(name.clone(), val.clone());
                }
            }
        }

        let output: O = serde_json::from_value(Value::Object(final_values))
            .map_err(|e| IvoError::InvalidSchema(e.to_string()))?;
        Ok(output)
    }

    fn create_summary(
        &self,
        input: &Arc<I>,
        current_values: &serde_json::Map<String, Value>,
        options: &Arc<Mutex<HashMap<String, Value>>>,
        is_update: bool,
    ) -> MutableSummary<I, O> {
        // This is a naive conversion from Map to O, might be slow but works for now to provide a 'values' object
        let values_obj: O =
            serde_json::from_value(Value::Object(current_values.clone())).unwrap_or_default();

        MutableSummary {
            summary: ImmutableSummary {
                changes: None,
                context: Context {
                    input: Arc::clone(input),
                    values: Arc::new(values_obj.clone()),
                    options: Arc::clone(options),
                },
                input_values: HashMap::new(), // TODO: populate
                is_update,
                previous_values: None,
                values: Arc::new(values_obj),
            },
        }
    }

    pub async fn update(&self, existing: O, changes: I) -> Result<O, IvoError> {
        let mut changes_val =
            serde_json::to_value(&changes).map_err(|e| IvoError::InvalidSchema(e.to_string()))?;
        let existing_val =
            serde_json::to_value(&existing).map_err(|e| IvoError::InvalidSchema(e.to_string()))?;
        let mut values = existing_val.as_object().unwrap().clone();
        let mut errors = HashMap::new();

        // Prepare context
        let context_options = Arc::new(Mutex::new(HashMap::new()));
        let mut changes_obj = changes_val
            .as_object_mut()
            .ok_or_else(|| IvoError::InvalidSchema("Changes must be an object".to_string()))?
            .clone();

        // Handle aliases
        for (name, def) in &self.definitions {
            if let Some(alias) = &def.alias {
                if let Some(val) = changes_obj.get(alias) {
                    changes_obj.insert(name.clone(), val.clone());
                }
            }
        }
        changes_val = Value::Object(changes_obj.clone());
        let changes_arc = Arc::new(changes.clone());

        let mut changed_fields = Vec::new();

        // 1. Primary Validation
        for (name, def) in &self.definitions {
            if let Some(val) = changes_obj.get(name) {
                // Ignore constant and readonly
                if def.constant || def.readonly {
                    continue;
                }

                let mut current_val = val.clone();

                // Allow values check
                if let Some(allowed) = &def.allow_values {
                    if !allowed.contains(&current_val) {
                        errors
                            .entry(name.clone())
                            .or_insert_with(Vec::new)
                            .push(FieldError {
                                reason: def
                                    .allow_error
                                    .clone()
                                    .unwrap_or_else(|| "Value not allowed".to_string()),
                                metadata: Some(serde_json::to_value(allowed).unwrap()),
                            });
                        continue;
                    }
                }

                for validator in &def.primary_validators {
                    let summary =
                        self.create_summary(&changes_arc, &values, &context_options, true);
                    match validator.validate(&current_val, &summary).await {
                        Ok(validated) => current_val = validated,
                        Err(reason) => {
                            errors
                                .entry(name.clone())
                                .or_insert_with(Vec::new)
                                .push(FieldError {
                                    reason,
                                    metadata: None,
                                });
                            break;
                        }
                    }
                }

                // Check if value actually changed (depth 1 comparison)
                if values.get(name) != Some(&current_val) {
                    values.insert(name.clone(), current_val);
                    changed_fields.push(name.clone());
                }
            }
        }

        if !errors.is_empty() {
            return Err(IvoError::ValidationError(errors));
        }

        if changed_fields.is_empty() {
            return Err(IvoError::NothingToUpdate);
        }

        // 2. Secondary Validation
        for name in &changed_fields {
            if let Some(def) = self.definitions.get(name) {
                if let Some(current_val) = values.get(name) {
                    let mut current_val = current_val.clone();
                    for validator in &def.secondary_validators {
                        let summary =
                            self.create_summary(&changes_arc, &values, &context_options, true);
                        match validator.validate(&current_val, &summary).await {
                            Ok(validated) => current_val = validated,
                            Err(reason) => {
                                errors.entry(name.clone()).or_insert_with(Vec::new).push(
                                    FieldError {
                                        reason,
                                        metadata: None,
                                    },
                                );
                                break;
                            }
                        }
                    }
                    values.insert(name.clone(), current_val);
                }
            }
        }

        if !errors.is_empty() {
            return Err(IvoError::ValidationError(errors));
        }

        // 3. Post Validation
        for config in &self.post_validators {
            // Only run if any affected property changed
            if config.properties.iter().any(|p| changed_fields.contains(p)) {
                let summary = self.create_summary(&changes_arc, &values, &context_options, true);
                match config.validator.validate(&summary).await {
                    Ok(updates) => {
                        for (name, val) in updates {
                            if config.properties.contains(&name) {
                                values.insert(name, val);
                            }
                        }
                    }
                    Err(post_errors) => {
                        for (name, reason) in post_errors {
                            errors
                                .entry(name)
                                .or_insert_with(Vec::new)
                                .push(FieldError {
                                    reason,
                                    metadata: None,
                                });
                        }
                    }
                }
            }
        }

        if !errors.is_empty() {
            return Err(IvoError::ValidationError(errors));
        }

        // 4. Resolvers
        for (name, def) in &self.definitions {
            if let Some(resolver) = &def.resolver {
                // Run resolver if any dependency changed
                if def.depends_on.iter().any(|d| changed_fields.contains(d)) {
                    let summary =
                        self.create_summary(&changes_arc, &values, &context_options, true);

                    let resolved_val = resolver(&summary);
                    values.insert(name.clone(), resolved_val);
                }
            }
        }

        // Filter out virtual properties
        let mut final_values = serde_json::Map::new();
        for (name, def) in &self.definitions {
            if !def.virtual_prop {
                if let Some(val) = values.get(name) {
                    final_values.insert(name.clone(), val.clone());
                }
            }
        }

        let output: O = serde_json::from_value(Value::Object(final_values))
            .map_err(|e| IvoError::InvalidSchema(e.to_string()))?;

        Ok(output)
    }
}
