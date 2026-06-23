use std::{collections::HashSet, fmt::Debug};

use crate::{
    schema::fields::base::{FieldType, InternalFieldConfig},
    IvoErrorTool, IvoSchemaStruct, Schema,
};

pub(super) struct FieldInfoCollection<
    'a,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    Timestamp: Clone + Debug + Send + Sync + 'static,
    ErrorTool: IvoErrorTool,
> {
    config_names: HashSet<String>,
    schema: &'a Schema<I, O, CtxOptions, Timestamp, ErrorTool>,
    pub fields: Vec<FieldInfo>,
    pub schema_input_fields: HashSet<String>,
    pub schema_output_fields: HashSet<String>,
}

impl<
        'a,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        Timestamp: Clone + Debug + Send + Sync + 'static,
        ErrorTool: IvoErrorTool,
    > FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>
{
    #[inline]
    pub fn new(schema: &'a Schema<I, O, CtxOptions, Timestamp, ErrorTool>) -> Self {
        Self {
            schema,
            schema_input_fields: I::ivo_internal_field_names(),
            schema_output_fields: O::ivo_internal_field_names(),
            config_names: HashSet::new(),
            fields: Vec::new(),
        }
    }

    #[inline]
    pub fn add(&mut self, field_info: FieldInfo) {
        if self.config_names.insert(field_info.config_name.clone()) {
            self.fields.push(field_info);
        }
    }

    pub fn set_fields(&mut self, fields: Vec<FieldInfo>) {
        let mut config_names = HashSet::new();

        for field_info in fields.iter() {
            config_names.insert(field_info.config_name.clone());
        }

        self.config_names = config_names;
        self.fields = fields;
    }

    pub fn from_fields(
        schema: &'a Schema<I, O, CtxOptions, Timestamp, ErrorTool>,
        fields: Vec<FieldInfo>,
        schema_input_fields: &HashSet<String>,
        schema_output_fields: &HashSet<String>,
    ) -> Self {
        let mut config_names = HashSet::new();

        for field_info in fields.iter() {
            config_names.insert(field_info.config_name.clone());
        }

        Self {
            schema,
            schema_input_fields: schema_input_fields.clone(),
            schema_output_fields: schema_output_fields.clone(),
            config_names,
            fields,
        }
    }

    #[inline(always)]
    pub fn contains(&self, field_name: &String) -> bool {
        self.config_names.contains(field_name)
    }

    #[inline(always)]
    fn _find(&self, field_name: &String) -> Option<FieldInfo> {
        self.fields.iter().find(|f| f.name == *field_name).cloned()
    }

    pub fn get(&self, field_name: &String) -> Option<FieldInfo> {
        self._find(field_name).or_else(|| {
            Self::get_field_info(
                field_name,
                &self.schema,
                &self.schema_input_fields,
                &self.schema_output_fields,
            )
        })
    }

    fn get_field_info(
        field_name: &String,
        schema: &Schema<I, O, CtxOptions, Timestamp, ErrorTool>,
        schema_input_fields: &HashSet<String>,
        schema_output_fields: &HashSet<String>,
    ) -> Option<FieldInfo> {
        if let Some(InternalFieldConfig {
            alias, depends_on, ..
        }) = schema.field_configs.get(field_name)
        {
            if depends_on.is_none() {
                return Some(FieldInfo {
                    config_name: field_name.clone(),
                    is_input: schema_input_fields.contains(field_name),
                    is_output: schema_output_fields.contains(field_name),
                    name: alias.clone().unwrap_or_else(|| field_name.clone()),
                });
            }

            // otherwise, field_name is an alias for a virtual field
            // the current config depends on

            for parent_name in depends_on.as_ref().unwrap() {
                match schema.field_configs.get(&parent_name.to_string()) {
                    Some(InternalFieldConfig {
                        alias: Some(alias),
                        field_type: FieldType::Virtual,
                        validator: Some(validator),
                        ..
                    }) if alias == field_name => {
                        return Some(FieldInfo {
                            config_name: parent_name.to_string(),
                            is_input: true,
                            is_output: false,
                            name: field_name.clone(),
                        });
                    }
                    _ => {}
                }
            }
        }

        None
    }
}

impl<
        'a,
        I: IvoSchemaStruct,
        O: IvoSchemaStruct,
        CtxOptions,
        ErrorTool: IvoErrorTool,
        Timestamp: Clone + Debug + Send + Sync + 'static,
    > Clone for FieldInfoCollection<'a, I, O, CtxOptions, Timestamp, ErrorTool>
{
    fn clone(&self) -> Self {
        Self {
            config_names: self.config_names.clone(),
            fields: self.fields.clone(),
            schema: self.schema,
            schema_input_fields: self.schema_input_fields.clone(),
            schema_output_fields: self.schema_output_fields.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct FieldInfo {
    pub name: String,
    pub config_name: String,
    pub is_input: bool,
    pub is_output: bool,
}
