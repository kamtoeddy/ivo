use std::collections::HashSet;

use crate::{schema::options::base::SchemaOptions, IvoErrorTool, IvoSchemaStruct};

pub(super) trait SchemaInternals<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
>
{
    fn options(&self) -> &SchemaOptions<I, O, CtxOptions, ErrorTool>;
}

#[derive(Clone)]
pub(super) struct InputFieldCollection {
    config_names: HashSet<String>,
    pub fields: Vec<InputFieldInfo>,
}

impl InputFieldCollection {
    pub fn new(fields: Vec<InputFieldInfo>) -> Self {
        let mut config_names = HashSet::new();

        for f in fields.iter() {
            config_names.insert(f.config_name.clone());
        }

        Self {
            config_names,
            fields,
        }
    }

    pub fn contains(&self, field_name: &String) -> bool {
        self.config_names.contains(field_name)
    }
}

#[derive(Clone)]
pub(super) struct InputFieldInfo {
    pub name: String,
    pub config_name: String,
    pub is_input: bool,
    pub is_output: bool,
}
