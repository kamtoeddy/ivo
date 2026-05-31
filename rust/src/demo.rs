// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::schema::{
    properties::{constants::ConstantField, dependents::DependentField, required::RequiredField},
    SchemaCore,
};
use crate::IvoStruct;

// type DateWithTz = DateTime<Utc>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum UserRole {
    Admin,
    User,
    Moderator,
}

#[derive(Debug, Deserialize, Serialize, IvoStruct)]
pub struct User {
    // pub created_at: DateWithTz,
    // pub id: String,
    pub email: String,
    pub username: String,
    pub role: UserRole,
    // pub username_updated_at: Option<DateWithTz>,
    // pub updated_at: Option<DateWithTz>,
}

#[derive(Deserialize, Serialize, IvoStruct)]
pub struct UserInput {
    pub email: String,
    pub username: String,
    pub role: UserRole,
}

// type CtxOptions = HashMap<String, Value>;
// type CtxOptions = Option<String>;

pub struct DEMO;

impl DEMO {
    pub fn get_schema() -> SchemaCore<UserInput, User> {
        SchemaCore::new(
            vec![
                ("id", ConstantField::value(json!(1)).build()),
                (
                    "email",
                    RequiredField::validate(|_, __| Ok(json!("Hello"))).build(),
                ),
                (
                    "username",
                    RequiredField::validate(|_, __| Ok(json!("john_doe"))).build(),
                ),
                (
                    "username_last_updated_at",
                    DependentField::default(json!(Some("default value")))
                        .depends_on(&["username"])
                        .resolve(|_| json!(Some("resolved value")))
                        .build(),
                ),
            ],
            None,
        )
    }
}
