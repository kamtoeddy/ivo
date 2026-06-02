// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::schema::{
    properties::{
        constants::ConstantField, dependents::DependentField, enumerated::EnumeratedField,
        lax::LaxField, required::RequiredField, virtuals::VirtualField,
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
    pub is_admin: Option<bool>,
}

// type CtxOptions = HashMap<String, Value>;
// type CtxOptions = Option<String>;

pub struct DEMO;

impl DEMO {
    pub fn get_schema() -> SchemaCore<UserInput, User> {
        let resolver = || String::from("full name");

        SchemaCore::new()
            .field("id", ConstantField::value(1234))
            .field(
                "email",
                RequiredField::validate(|_, _| Ok(String::from("Hello"))),
            )
            .field("username", RequiredField::validate(|_, _| Ok(true)))
            .field(
                "username_last_updated_at",
                DependentField::default(Some("default value"))
                    .depends_on(vec!["username"])
                    .resolve(|_| Some("resolved value")),
            )
            // general demo to make sure all fields work as expected
            .field(
                "c",
                ConstantField::value(String::from("String"))
                    .on_success(|_| async {})
                    .on_delete(|_, _| async {}),
            )
            .field(
                "c1",
                ConstantField::value(Some(String::from("Option<String>")))
                    .on_success(|_| async {})
                    .on_delete(|_, _| async {}),
            )
            .field(
                "c2",
                ConstantField::computed(|_| true)
                    .on_delete(|_, _| async {})
                    .on_success(|_| async { println!("on success 1") })
                    .on_success(|_| async { println!("on success 2") }),
            )
            .field(
                "c3",
                ConstantField::computed_async(|_| async { false })
                    .on_delete(|_, _| async {})
                    .on_success(|_| async { println!("on success 1") })
                    .on_success(|_| async { println!("on success 2") }),
            )
            .field(
                "enum",
                EnumeratedField::values(vec![true, false])
                    .error_fn(|_| "")
                    // .error("invalid option provided")
                    .default_fn(|_| true)
                    .readonly()
                    .on_delete(|_, _| async {})
                    .on_failure(|_| async {})
                    .on_success(|_| async {}),
            )
            .field(
                "d",
                DependentField::default(String::from("Hello"))
                    .depends_on(vec!["first_name", "last_name"])
                    .resolve(move |_| resolver())
                    .on_delete(|_, _| async {})
                    .on_success(|_| async {}),
            )
            .field(
                "d1",
                DependentField::default_fn(|_| true)
                    .depends_on(vec!["first_name", "last_name"])
                    .resolve_async(move |_| async move {
                        resolver();
                        false
                    })
                    .readonly()
                    .on_delete(|_, _| async {})
                    .on_success(|_| async {}),
            )
            .field(
                "l",
                LaxField::default(false)
                    .validate(|_, _| Ok(true))
                    .readonly()
                    .on_delete(|_, _| async {})
                    .on_failure(|_| async {})
                    .on_success(|_| async {}),
            )
            .field(
                "l1",
                LaxField::default_fn(|_| None)
                    .validate_async(|_, _| async { Ok(Some(1)) })
                    .re_validate(|_, _| Ok(Some(2)))
                    .readonly()
                    .on_delete(|_, _| async {})
                    .on_failure(|_| async {})
                    .on_success(|_| async {}),
            )
            .field(
                "l2",
                LaxField::default(None)
                    .validate_async(|_, _| async { Ok(Some(true)) })
                    .re_validate_async(|v, _| async move { Ok(v) })
                    .readonly()
                    .on_delete(|_, _| async {})
                    .on_failure(|_| async {})
                    .on_success(|_| async {}),
            )
            .field(
                "r",
                RequiredField::validate(|_, _| Err(("lol", None)))
                    .re_validate(|_, _| Ok(true))
                    .readonly()
                    .on_failure(|_| async {})
                    .on_success(|_| async {})
                    .on_delete(|_, _| async {}),
            )
            .field(
                "v",
                VirtualField::alias("lol")
                    .validate(|_, _| Ok(true))
                    .re_validate_async(|_, _| async { Ok(true) })
                    .required_if(|_| async { (true, "lol") })
                    .sanitize(|_| async { false })
                    .on_failure(|_| async {})
                    .on_success(|_| async {}),
            )
            .field(
                "v1",
                VirtualField::validate_async(|_, _| async {
                    if true {
                        Ok(true)
                    } else {
                        Err(("lol", None))
                    }
                })
                .re_validate(|_, _| Ok(true))
                .alias("lol")
                .required_if(|_| async { (true, "lol") })
                .sanitize(|_| async { false })
                .on_failure(|_| async {})
                .on_success(|_| async {}),
            )
            .field(
                "v2",
                VirtualField::validate(|_, _| Ok(true))
                    .re_validate(|_, _| Ok(true))
                    .alias("lol")
                    .required_if(|_| async { (true, "lol") })
                    .sanitize(|_| async { false })
                    .on_failure(|_| async {})
                    .on_success(|_| async {}),
            )
            .field(
                "v3",
                VirtualField::validate(|_, _| Ok(true))
                    .alias("lol")
                    .re_validate(|_, _| Ok(true))
                    .required_if(|_| async { (true, "lol") })
                    .sanitize(|_| async { false })
                    // .ignore_if(|_| false)
                    .allow_update_if(|_| false)
                    .allow_init_if(|_| false)
                    // .ignore_init()
                    // .ignore_update()
                    .on_failure(|_| async {})
                    .on_failure(|_| async {})
                    .on_success(|_| async { println!("on success 1") })
                    .on_success(|_| async { println!("on success 2") }),
            )
    }
}
