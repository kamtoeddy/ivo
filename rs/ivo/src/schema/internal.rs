use std::collections::HashSet;

use crate::{
    schema::{
        fields::base::{FieldType, InternalFieldConfig},
        options::base::SchemaOptions,
    },
    types::PartialMapOfErasedValues,
    IvoErrorTool, IvoSchemaStruct, Schema,
};

pub(super) trait SchemaInternals<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
>
{
    fn options(&self) -> &SchemaOptions<I, O, CtxOptions, ErrorTool>;
}

pub(super) struct InputFieldCollection<
    'a,
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
> {
    schema: &'a Schema<I, O, CtxOptions, ErrorTool>,
    pub schema_input_fields: Vec<String>,
    pub schema_output_fields: Vec<String>,
    config_names: HashSet<String>,
    pub fields: Vec<InputFieldInfo>,
}

impl<'a, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions, ErrorTool: IvoErrorTool>
    InputFieldCollection<'a, I, O, CtxOptions, ErrorTool>
{
    pub fn new(
        schema: &'a Schema<I, O, CtxOptions, ErrorTool>,
        erased_input_values: &PartialMapOfErasedValues,
    ) -> Self {
        let mut config_names = HashSet::new();

        let fields_names = erased_input_values
            .inner
            .keys()
            .map(|f| f.to_owned())
            .collect::<Vec<String>>();

        let schema_input_fields = I::ivo_internal_field_names();
        let schema_output_fields = O::ivo_internal_field_names();

        let mut fields = Vec::with_capacity(fields_names.len());

        for field_name in fields_names.iter() {
            if let Some(field_info) = Self::get_field_info(
                field_name,
                &schema,
                &schema_input_fields,
                &schema_output_fields,
            ) {
                config_names.insert(field_info.config_name.clone());
                fields.push(field_info);
            }
        }

        Self {
            schema,
            schema_input_fields,
            schema_output_fields,
            config_names,
            fields,
        }
    }

    pub fn from_fields(
        schema: &'a Schema<I, O, CtxOptions, ErrorTool>,
        fields: &Vec<InputFieldInfo>,
        schema_input_fields: &Vec<String>,
        schema_output_fields: &Vec<String>,
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
            fields: fields.clone(),
        }
    }

    pub fn contains(&self, field_name: &String) -> bool {
        self.config_names.contains(field_name)
    }

    fn _find(&self, field_name: &String) -> Option<InputFieldInfo> {
        self.fields.iter().find(|f| f.name == *field_name).cloned()
    }

    pub fn get(&self, field_name: &String) -> Option<InputFieldInfo> {
        self._find(field_name).or_else(|| {
            Self::get_field_info(
                field_name,
                &self.schema,
                &self.schema_input_fields,
                &self.schema_output_fields,
            )
        })
    }

    pub fn get_field_info(
        field_name: &String,
        schema: &Schema<I, O, CtxOptions, ErrorTool>,
        schema_input_fields: &Vec<String>,
        schema_output_fields: &Vec<String>,
    ) -> Option<InputFieldInfo> {
        if let Some(InternalFieldConfig {
            alias, depends_on, ..
        }) = schema.get_field_config(field_name)
        {
            if depends_on.is_none() {
                return Some(InputFieldInfo {
                    config_name: field_name.clone(),
                    is_input: schema_input_fields.contains(field_name),
                    is_output: schema_output_fields.contains(field_name),
                    name: alias.clone().unwrap_or(field_name.clone()),
                });
            }

            // otherwise, field_name is an alias for a virtual field
            // the current config depends on

            for parent_name in depends_on.as_ref().unwrap() {
                match schema.get_field_config(parent_name) {
                    Some(InternalFieldConfig {
                        alias: Some(alias),
                        field_type: FieldType::Virtual,
                        validator: Some(validator),
                        ..
                    }) if alias == field_name => {
                        return Some(InputFieldInfo {
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

impl<'a, I: IvoSchemaStruct, O: IvoSchemaStruct, CtxOptions, ErrorTool: IvoErrorTool> Clone
    for InputFieldCollection<'a, I, O, CtxOptions, ErrorTool>
{
    fn clone(&self) -> Self {
        Self {
            // cache: self.cache.clone(),
            config_names: self.config_names.clone(),
            fields: self.fields.clone(),
            schema: self.schema,
            schema_input_fields: self.schema_input_fields.clone(),
            schema_output_fields: self.schema_output_fields.clone(),
        }
    }
}

#[derive(Clone)]
pub(super) struct InputFieldInfo {
    pub name: String,
    pub config_name: String,
    pub is_input: bool,
    pub is_output: bool,
}
