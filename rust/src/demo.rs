// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::IvoStruct;
use crate::{
    schema::{
        properties::{
            constants::ConstantField, dependents::DependentField, enumerated::EnumeratedField,
            lax::LaxField, required::RequiredField, virtuals::VirtualField,
        },
        SchemaCore,
    },
    types::IvoSummary,
};

fn slugify(w: &str) -> String {
    format!("slugified: {}", w)
}

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
    pub id: String,
    pub email: String,
    pub username: String,
    pub slug_id: String,
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

type MutUserSummary = IvoSummary<UserInput, User, UserCtxOptions>;

// type CtxOptions = HashMap<String, Value>;
// type CtxOptions = Option<String>;
#[derive(Clone)]
pub struct UserCtxOptions {
    pub slug_id: String,
}

impl UserCtxOptions {
    async fn find_by_username(&self, _username: &str) -> Option<User> {
        None
    }

    fn update_data(&mut self, d: String) {
        self.slug_id = d
    }
}

pub struct DEMO;

impl DEMO {
    pub fn get_schema() -> SchemaCore<UserInput, User, UserCtxOptions> {
        let resolver = || String::from("full name");

        SchemaCore::new()
            .with_fields(|f| {
                f.set("id", ConstantField::value(1234))
                    .set(
                        "email",
                        RequiredField::validate(|_, _| Ok(String::from("Hello"))),
                    )
                    .set(
                        "username",
                        RequiredField::validate(|v: String, _| {
                            const MIN_LEN: usize = 4;

                            if v.len() <= MIN_LEN {
                                return Err((
                                    format!("Username must be atleast {MIN_LEN} long"),
                                    None,
                                ));
                            }

                            return Ok(String::from(v));
                        })
                        .re_validate_async(
                            |uname: String, s: MutUserSummary| async move {
                                let mut ctx_options = s.options;

                                if ctx_options.find_by_username(&uname).await.is_some() {
                                    return Err((
                                        format!("Username \"{uname}\" already taken"),
                                        None,
                                    ));
                                }

                                ctx_options.update_data(slugify(&uname));

                                Ok(uname)
                            },
                        ),
                    )
                    .set(
                        "username_last_updated_at",
                        DependentField::default(Some("default value"))
                            .depends_on(vec!["username"])
                            .resolve(|_| Some("resolved value")),
                    )
                    // general demo to make sure all fields work as expected
                    .set(
                        "c",
                        ConstantField::value(String::from("String"))
                            .on_success(|_| async {})
                            .on_delete(|_, _| async {}),
                    )
                    .set(
                        "c1",
                        ConstantField::value(Some(String::from("Option<String>")))
                            .on_success(|_| async {})
                            .on_delete(|_, _| async {}),
                    )
                    .set(
                        "c2",
                        ConstantField::computed(|_| true)
                            .on_delete(|_, _| async {})
                            .on_success(|_| async { println!("on success 1") })
                            .on_success(|_| async { println!("on success 2") }),
                    )
                    .set(
                        "c3",
                        ConstantField::computed_async(|_| async { false })
                            .on_delete(|_, _| async {})
                            .on_success(|_| async { println!("on success 1") })
                            .on_success(|_| async { println!("on success 2") }),
                    )
                    .set(
                        "enum",
                        EnumeratedField::values(vec![true, false])
                            .error_fn(|_| "".into())
                            // .error("invalid option provided")
                            .default_fn(|_| true)
                            .readonly()
                            .on_delete(|_, _| async {})
                            .on_failure(|_| async {})
                            .on_success(|_| async {}),
                    )
                    .set(
                        "d",
                        DependentField::default(String::from("Hello"))
                            .depends_on(vec!["first_name", "last_name"])
                            .resolve(move |_| resolver())
                            .on_delete(|_, _| async {})
                            .on_success(|_| async {}),
                    )
                    .set(
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
                    .set(
                        "l",
                        LaxField::default(false)
                            .validate(|_, _| Ok(true))
                            .readonly()
                            .on_delete(|_, _| async {})
                            .on_failure(|_| async {})
                            .on_success(|_| async {}),
                    )
                    .set(
                        "l1",
                        LaxField::default_fn(|_| None)
                            .validate_async(|_, _| async { Ok(Some(1)) })
                            .re_validate(|_, _| Ok(Some(2)))
                            .readonly()
                            .on_delete(|_, _| async {})
                            .on_failure(|_| async {})
                            .on_success(|_| async {}),
                    )
                    .set(
                        "l2",
                        LaxField::default(None)
                            .validate_async(|_, _| async { Ok(Some(true)) })
                            .re_validate_async(|v, _| async move { Ok(v) })
                            .readonly()
                            .on_delete(|_, _| async {})
                            .on_failure(|_| async {})
                            .on_success(|_| async {}),
                    )
                    .set(
                        "r",
                        RequiredField::validate(|_, _| Err(("lol".into(), None)))
                            .re_validate(|_, _| Ok(true))
                            .readonly()
                            .on_failure(|_| async {})
                            .on_success(|_| async {})
                            .on_delete(|_, _| async {}),
                    )
                    .set(
                        "v",
                        VirtualField::alias("lol")
                            .validate(|_, _| Ok(true))
                            .re_validate_async(|_, _| async { Ok(true) })
                            .required_if(|_| async { (true, "lol".into()) })
                            .sanitize(|_| async { false })
                            .on_failure(|_| async {})
                            .on_success(|_| async {}),
                    )
                    .set(
                        "v1",
                        VirtualField::validate_async(|v, _| async move {
                            if v == true || v == false {
                                Ok(v)
                            } else {
                                Err(("Invalid boolean".into(), None))
                            }
                        })
                        .re_validate(|_, _| Ok(true))
                        .alias("lol")
                        .required_if(|_| async {
                            (true, "this field is required in this scenario".into())
                        })
                        .sanitize(|_| async { false })
                        .on_failure(|_| async {})
                        .on_success(|_| async {}),
                    )
                    .set(
                        "v2",
                        VirtualField::validate(|_, _| Ok(true))
                            .re_validate(|_, _| Ok(true))
                            .alias("lol")
                            .required_if(|_| async { (true, "lol".into()) })
                            .sanitize(|_| async { false })
                            .on_failure(|_| async {})
                            .on_success(|_| async {}),
                    )
                    .set(
                        "v3",
                        VirtualField::validate(|_, _| Ok(true))
                            .alias("v3")
                            .re_validate(|_, _| Ok(true))
                            .required_if(|_| async { (true, "lol".into()) })
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
            })
            .with_options()
    }
}
