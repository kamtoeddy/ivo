use std::collections::HashMap;

// use chrono::{DateTime, Utc};
use partial_derive::MakePartial;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    schema::{
        properties::{constants::ConstantField, required::RequiredField},
        SchemaCore,
    },
    traits::IvoSchemaStruct,
};

// type DateWithTz = DateTime<Utc>;

#[derive(Debug, Deserialize, Serialize, MakePartial)]
pub struct User {
    // pub created_at: DateWithTz,
    // pub id: String,
    pub email: String,
    pub username: String,
    // pub username_updated_at: Option<DateWithTz>,
    // pub updated_at: Option<DateWithTz>,
}

impl IvoSchemaStruct for User {}

#[derive(Deserialize, Serialize, MakePartial)]
pub struct UserInput {
    pub email: String,
    pub username: String,
}

impl IvoSchemaStruct for UserInput {}

// type CtxOptions = HashMap<String, Value>;
type CtxOptions = Option<String>;

pub struct DEMO;

impl DEMO {
    pub fn get_schema() -> SchemaCore<UserInput, User, CtxOptions> {
        let mut props = HashMap::new();

        props.insert(String::from("id"), ConstantField::value(json!(1)).build());

        props.insert(
            String::from("email"),
            RequiredField::validate(|_, __| Ok(json!("Hello"))).build(),
        );

        props.insert(
            String::from("username"),
            RequiredField::validate(|_, __| Ok(json!("john_doe"))).build(),
        );

        SchemaCore::new(props, None)
    }
}
