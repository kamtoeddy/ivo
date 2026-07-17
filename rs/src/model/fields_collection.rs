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

pub(super) struct FieldInfoCollection<
    'a,
    I: IvoInputStruct<CtxOptions, ErrorSanitizer>,
    O: IvoStruct,
    CtxOptions: Clone,
    ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
> {
    fields: HashMap<&'a str, FieldInfo<'a>>,
    fields_provided: HashSet<String>,
    relevant_fields_provided: HashSet<String>,
    relevant_dependent_config_names: HashSet<String>,
    relevant_config_names: HashSet<String>,
    field_configs: &'a InternalFieldConfigs<I, O, CtxOptions, ErrorSanitizer>,
}

impl<
        'a,
        I: IvoInputStruct<CtxOptions, ErrorSanitizer>,
        O: IvoStruct,
        CtxOptions: Clone,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    > FieldInfoCollection<'a, I, O, CtxOptions, ErrorSanitizer>
{
    #[inline]
    pub fn new(field_configs: &'a InternalFieldConfigs<I, O, CtxOptions, ErrorSanitizer>) -> Self {
        Self {
            field_configs,
            fields: Self::parse_fields(field_configs),
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

            if info.is_output {
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
    pub fn is_relevant_dependent_config_name(&self, config_name: &str) -> bool {
        self.relevant_dependent_config_names.contains(config_name)
    }

    #[inline(always)]
    pub fn get(&'a self, field_name: &str) -> &'a FieldInfo<'a> {
        self.fields.get(field_name).unwrap()
    }

    fn parse_fields(
        field_configs: &'a InternalFieldConfigs<I, O, CtxOptions, ErrorSanitizer>,
    ) -> HashMap<&'a str, FieldInfo<'a>> {
        let mut fields = HashMap::new();

        for (config_name, config) in field_configs.iter() {
            match config {
                InternalFieldConfig {
                    alias,
                    field_type: FieldType::Virtual,
                    ..
                } => {
                    fields.insert(
                        *config_name,
                        FieldInfo {
                            config_name,
                            is_input: true,
                            is_output: false,
                            name: config_name,
                        },
                    );

                    if let Some(name) = alias {
                        fields.insert(
                            *name,
                            FieldInfo {
                                config_name,
                                is_input: true,
                                is_output: false,
                                name,
                            },
                        );

                        // necessary for group validations and resolvers
                        fields.insert(
                            *config_name,
                            FieldInfo {
                                config_name,
                                is_input: true,
                                is_output: false,
                                name,
                            },
                        );
                    }
                }
                InternalFieldConfig {
                    field_type: FieldType::Lax | FieldType::Required,
                    ..
                } => {
                    fields.insert(
                        *config_name,
                        FieldInfo {
                            config_name,
                            is_input: true,
                            is_output: true,
                            name: config_name,
                        },
                    );
                }
                _ => {
                    continue;
                }
            };
        }

        fields
    }
}

impl<
        'a,
        I: IvoInputStruct<CtxOptions, ErrorSanitizer>,
        O: IvoStruct,
        CtxOptions: Clone,
        ErrorSanitizer: IvoErrorSanitizer<CtxOptions>,
    > Clone for FieldInfoCollection<'a, I, O, CtxOptions, ErrorSanitizer>
{
    fn clone(&self) -> Self {
        Self {
            field_configs: self.field_configs,
            fields: self.fields.clone(),
            fields_provided: self.fields_provided.clone(),
            relevant_config_names: self.relevant_config_names.clone(),
            relevant_fields_provided: self.relevant_fields_provided.clone(),
            relevant_dependent_config_names: self.relevant_dependent_config_names.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct FieldInfo<'a> {
    pub name: &'a str,
    pub config_name: &'a str,
    pub is_input: bool,
    pub is_output: bool,
}
