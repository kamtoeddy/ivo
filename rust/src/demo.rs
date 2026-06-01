// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::schema::{
    properties::{
        constants::ConstantField, dependents::DependentField, enumerated::EnumeratedField,
        required::RequiredField,
    },
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

#[derive(Debug, Clone, Deserialize, Serialize, IvoStruct)]
pub struct User {
    // pub created_at: DateWithTz,
    // pub id: String,
    pub email: String,
    pub username: String,
    pub role: UserRole,
    // pub username_updated_at: Option<DateWithTz>,
    // pub updated_at: Option<DateWithTz>,
}

#[derive(Clone, Deserialize, Serialize, IvoStruct)]
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
        let resolver = || String::from("full name");

        SchemaCore::new()
            .field("id", ConstantField::value(1234))
            .field("email", RequiredField::validate(|_, __| Ok("Hello")))
            .field("username", RequiredField::validate(|_, __| Ok("john_doe")))
            .field(
                "username_last_updated_at",
                DependentField::default(json!(Some("default value")))
                    .depends_on(&["username"])
                    .resolve(|_| json!(Some("resolved value"))),
            )
            // general demo to make sure all fields work as expected
            .field(
                "c",
                ConstantField::value(String::from("String"))
                    .on_success(|_| async {})
                    .on_delete(|_, __| async {}),
            )
            .field(
                "c1",
                ConstantField::value(Some(String::from("Option<String>")))
                    .on_success(|_| async {})
                    .on_delete(|_, __| async {}),
            )
            .field(
                "c2",
                ConstantField::computed(|s| "computed &str")
                    .on_delete(|_, __| async {})
                    .on_success(|_| async { println!("on success 1") })
                    .on_success(|_| async { println!("on success 2") }),
            )
            .field(
                "c3",
                ConstantField::computed_async(|s| async { "computed &str" })
                    .on_delete(|_, __| async {})
                    .on_success(|_| async { println!("on success 1") })
                    .on_success(|_| async { println!("on success 2") }),
            )
            .field(
                "enum",
                EnumeratedField::values(vec!["hello", "hi", "greeting"])
                    // .error_fn(|_| "")
                    .error("invalid option provided")
                    .default_fn(|_| "true")
                    .readonly()
                    .on_delete(|_, __| async {})
                    .on_failure(|_| async {})
                    .on_success(|_| async {}),
            )
            .field(
                "d",
                DependentField::default(String::from("Hello"))
                    .depends_on(&["first_name", "last_name"])
                    .resolve(|_| resolver())
                    .on_delete(|_, __| async {})
                    .on_success(|_| async {}),
            )
            .field(
                "d1",
                DependentField::default_fn(|_| true)
                    .depends_on(&["first_name", "last_name"])
                    .resolve_async(|_| async {
                        resolver();

                        false
                    })
                    .readonly()
                    .on_delete(|_, __| async {})
                    .on_success(|_| async {}),
            )
    }
}
