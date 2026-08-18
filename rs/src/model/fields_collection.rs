use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use crate::{
    __private_types::IvoInputStruct,
    schema::fields::base::{FieldType, InternalFieldConfig},
    types::InternalFieldConfigs,
    IvoErrorSanitizer, IvoStruct,
};

pub(super) struct FieldInfoCollection<'a> {
    fields: &'a HashMap<&'static str, InputFieldInfo<'static>>,
    fields_provided: HashSet<String>,
    relevant_fields_provided: HashSet<String>,
    relevant_dependent_config_names: HashSet<String>,
    relevant_config_names: HashSet<String>,
}

impl<'a> FieldInfoCollection<'a> {
    #[inline]
    pub fn new(fields: &'a HashMap<&'static str, InputFieldInfo<'static>>) -> Self {
        Self {
            fields,
            relevant_config_names: HashSet::new(),
            fields_provided: HashSet::new(),
            relevant_fields_provided: HashSet::new(),
            relevant_dependent_config_names: HashSet::new(),
        }
    }

    pub fn new_with_fields_provided(mut self, fields_provided: HashSet<String>) -> Self {
        self.fields_provided = fields_provided;

        self
    }

    pub fn new_with_relevant_fields_provided(
        mut self,
        relevant_fields_provided: HashSet<String>,
    ) -> Self {
        let mut config_names = HashSet::new();
        let mut output_fields_changed = HashSet::new();

        for field_name in relevant_fields_provided.iter() {
            let info = self.get(field_name);

            config_names.insert(info.config_name.to_string());

            if !info.is_virtual {
                output_fields_changed.insert(field_name.clone());
            }
        }

        self.relevant_config_names = config_names;
        self.relevant_fields_provided = relevant_fields_provided;

        self
    }

    pub fn cloned_from_relevant_dependent_fields(&self) -> Self {
        let mut col = self.clone();
        let mut relevant_dependent_config_names = HashSet::new();

        for field_name in col.relevant_fields_provided.iter() {
            let info = self.get(field_name);

            relevant_dependent_config_names.insert(info.config_name.to_string());
        }

        col.relevant_dependent_config_names = relevant_dependent_config_names;

        col
    }

    pub fn new_with_dependent_fields_changed(mut self, field_names: HashSet<String>) -> Self {
        self.relevant_dependent_config_names = field_names;

        self
    }

    #[inline(always)]
    pub fn fields_provided(&'a self) -> &'a HashSet<String> {
        &self.fields_provided
    }

    #[inline(always)]
    pub fn relevant_fields_provided(&'a self) -> &'a HashSet<String> {
        &self.relevant_fields_provided
    }

    #[inline(always)]
    pub fn is_relevant_config_name(&self, config_name: &str) -> bool {
        self.relevant_config_names.contains(config_name)
    }

    #[inline(always)]
    pub fn relevant_dependent_config_names(&self) -> &HashSet<String> {
        &self.relevant_dependent_config_names
    }

    #[inline(always)]
    pub fn get(&self, field_name: &str) -> &InputFieldInfo<'static> {
        self.fields.get(field_name).unwrap()
    }
}

#[inline]
pub(crate) fn parse_field_infos<
    I: IvoInputStruct<CtxOptions, ErrorSanitizer>,
    O: IvoStruct,
    CtxOptions: Clone,
    ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
>(
    field_configs: &InternalFieldConfigs<I, O, CtxOptions, ErrorSanitizer>,
) -> HashMap<&'static str, InputFieldInfo<'static>> {
    let mut fields = HashMap::new();

    for (config_name, config) in field_configs.iter() {
        match config {
            InternalFieldConfig {
                alias,
                field_type: FieldType::Lax | FieldType::Required | FieldType::Virtual,
                ..
            } => {
                let is_virtual = matches!(config.field_type, FieldType::Virtual);

                fields.insert(
                    *config_name,
                    InputFieldInfo {
                        config_name,
                        name: config_name,
                        is_virtual,
                    },
                );

                if let Some(name) = alias {
                    fields.insert(
                        *name,
                        InputFieldInfo {
                            config_name,
                            name,
                            is_virtual,
                        },
                    );

                    // necessary for group validations and resolvers
                    fields.insert(
                        *config_name,
                        InputFieldInfo {
                            config_name,
                            name,
                            is_virtual,
                        },
                    );
                }
            }
            _ => {
                continue;
            }
        };
    }

    fields
}

impl<'a> Clone for FieldInfoCollection<'a> {
    fn clone(&self) -> Self {
        Self {
            fields: self.fields,
            fields_provided: self.fields_provided.clone(),
            relevant_config_names: self.relevant_config_names.clone(),
            relevant_fields_provided: self.relevant_fields_provided.clone(),
            relevant_dependent_config_names: self.relevant_dependent_config_names.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct InputFieldInfo<'a> {
    pub name: &'a str,
    pub config_name: &'a str,
    pub is_virtual: bool,
}
