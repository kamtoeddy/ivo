use std::sync::LazyLock;

// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::demo::slugify::{slugify, SlugifiedString};
use crate::fields::IvoField;
use crate::schema::Model;
use crate::IvoStruct;
use crate::{schema::SchemaCore, types::IvoSummary};

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
    pub slug_id: SlugifiedString,
    pub role: UserRole,
    // pub username_updated_at: Option<DateWithTz>,
    // pub updated_at: Option<DateWithTz>,
}

#[derive(Clone, Deserialize, Serialize, IvoStruct)]
pub struct UserInput {
    pub email: String,
    pub username: String,
    pub role: UserRole,
    pub v_slug: SlugifiedString,
}

type MutUserSummary = IvoSummary<UserInput, User, UserCtxOptions>;

#[derive(Clone)]
pub struct UserCtxOptions {
    pub slug_id: Option<SlugifiedString>,
}

impl UserCtxOptions {
    async fn find_user_by_slug_id(&self, _slug: &SlugifiedString) -> Option<User> {
        None
    }

    fn update_data(&mut self, slug: SlugifiedString) {
        self.slug_id = Some(slug.clone());
        println!("UserCtxOptions updated with slug id: \"{slug}\"")
    }
}

pub static USER_MODEL: LazyLock<Model<UserInput, User, UserCtxOptions>> =
    LazyLock::new(|| USER_SCHEMA.get_model());

pub static USER_SCHEMA: LazyLock<SchemaCore<UserInput, User, UserCtxOptions>> =
    LazyLock::new(|| {
        SchemaCore::new()
            .with_fields(|f| {
                f.set("id", IvoField::CONSTANT.value(1234))
                    .set(
                        "email",
                        IvoField::REQUIRED.validate(|_, _| Ok(String::from("Hello"))),
                    )
                    .set(
                        "username",
                        IvoField::REQUIRED
                            .validate(|v: String, _| {
                                const MIN_LEN: usize = 4;

                                if v.len() <= MIN_LEN {
                                    return Err((
                                        format!("Username must be atleast {MIN_LEN} long"),
                                        None,
                                    ));
                                }

                                Ok(v)
                            })
                            .re_validate_async(|uname: String, s: MutUserSummary| async move {
                                let mut ctx_options = s.get_options_mut();

                                let slug = slugify(&uname);

                                if ctx_options.find_user_by_slug_id(&slug).await.is_some() {
                                    return Err((
                                        format!("A user with a slug id: \"{slug}\" already exists"),
                                        None,
                                    ));
                                }

                                ctx_options.update_data(slug);

                                Ok(format!("{}-revalidated", uname.to_lowercase()))
                            }),
                    )
                    .set(
                        "username_last_updated_at",
                        IvoField::DEPENDENT
                            .default(Some(String::from("default value")))
                            .depends_on(vec!["username"])
                            .resolve(|_| Some(String::from("resolved value"))),
                    )
                    .set(
                        "slug",
                        IvoField::DEPENDENT
                            .default(SlugifiedString("".into()))
                            .depends_on(vec!["username", "v_slug"])
                            .resolve(|s: MutUserSummary| {
                                if let Some(v_slug) = s.input().v_slug.clone() {
                                    return v_slug;
                                }

                                if let Some(slug) = s.get_options().slug_id.clone() {
                                    return slug;
                                }

                                SlugifiedString("()".into())
                            }),
                    )
                    // general demo to make sure all fields work as expected
                    .set(
                        "c",
                        IvoField::CONSTANT
                            .value(String::from("String"))
                            .on_success(|_| async {})
                            .on_delete(|_, _| async {}),
                    )
                    .set(
                        "c1",
                        IvoField::CONSTANT
                            .value(Some(String::from("Option<String>")))
                            .on_success(|_| async {})
                            .on_delete(|_, _| async {}),
                    )
                    .set(
                        "c2",
                        IvoField::CONSTANT
                            .computed(|_| true)
                            .on_delete(|_, _| async {})
                            .on_success(|_| async { println!("on success 1") })
                            .on_success(|_| async { println!("on success 2") }),
                    )
                    .set(
                        "c3",
                        IvoField::CONSTANT
                            .computed_async(|_| async { false })
                            .on_delete(|_, _| async {})
                            .on_success(|_| async { println!("on success 1") })
                            .on_success(|_| async { println!("on success 2") }),
                    )
                    .set(
                        "enum",
                        IvoField::ENUM
                            .values(vec![true, false])
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
                        IvoField::DEPENDENT
                            .default(String::from("Hello"))
                            .depends_on(vec!["first_name", "last_name"])
                            .resolve(|_| resolve_full_name())
                            .on_delete(|_, _| async {})
                            .on_success(|_| async {}),
                    )
                    .set(
                        "d1",
                        IvoField::DEPENDENT
                            .default_fn(|_| true)
                            .depends_on(vec!["first_name", "last_name"])
                            .resolve_async(|_| async {
                                resolve_full_name();
                                false
                            })
                            .readonly()
                            .on_delete(|_, _| async {})
                            .on_success(|_| async {}),
                    )
                    .set(
                        "l",
                        IvoField::LAX
                            .default(false)
                            .validate(|_, _| Ok(true))
                            .readonly()
                            .on_delete(|_, _| async {})
                            .on_failure(|_| async {})
                            .on_success(|_| async {}),
                    )
                    .set(
                        "l1",
                        IvoField::LAX
                            .default_fn(|_| None)
                            .validate_async(|_, _| async { Ok(Some(1)) })
                            .re_validate(|_, _| Ok(Some(2)))
                            .readonly()
                            .on_delete(|_, _| async {})
                            .on_failure(|_| async {})
                            .on_success(|_| async {}),
                    )
                    .set(
                        "l2",
                        IvoField::LAX
                            .default(None)
                            .validate_async(|_, _| async { Ok(Some(true)) })
                            .re_validate_async(|v, _| async move { Ok(v) })
                            .readonly()
                            .on_delete(|_, _| async {})
                            .on_failure(|_| async {})
                            .on_success(|_| async {}),
                    )
                    .set(
                        "r",
                        IvoField::REQUIRED
                            .validate(|_, _| Err(("lol".into(), None)))
                            .re_validate(|_, _| Ok(true))
                            .readonly()
                            .on_failure(|_| async {})
                            .on_success(|_| async {})
                            .on_delete(|_, _| async {}),
                    )
                    .set(
                        "v",
                        IvoField::VIRTUAL
                            .alias("lol")
                            .validate(|_, _| Ok(true))
                            .re_validate_async(|_, _| async { Ok(true) })
                            .required_if(|_| async { (true, "lol".into()) })
                            .sanitize(|_| async { false })
                            .on_failure(|_| async {})
                            .on_success(|_| async {}),
                    )
                    .set(
                        "v1",
                        IvoField::VIRTUAL
                            .validate_async(|v, _| async move {
                                if v || !v {
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
                        IvoField::VIRTUAL
                            .validate(|_, _| Ok(true))
                            .re_validate(|_, _| Ok(true))
                            .alias("lol")
                            .required_if(|_| async { (true, "lol".into()) })
                            .sanitize(|_| async { false })
                            .on_failure(|_| async {})
                            .on_success(|_| async {}),
                    )
                    .set(
                        "v3",
                        IvoField::VIRTUAL
                            .validate(|_, _| Ok(true))
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
    });

fn resolve_full_name() -> String {
    String::from("full name")
}
