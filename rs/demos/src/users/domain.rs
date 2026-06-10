use std::{collections::HashMap, future::ready, sync::LazyLock};

use ivo::{FutureExt, IvoContext, IvoField, IvoStruct, IvoValues, Model, Schema, validate_email};

use crate::utils::slugify::{SlugifiedString, slugify};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UserRole {
    Admin,
    User,
    Moderator,
}

#[derive(Debug, Clone, PartialEq, Eq, IvoStruct)]
pub struct User {
    // pub created_at: DateWithTz,
    pub id: i32,
    pub email: String,
    pub username: String,
    pub slug_id: SlugifiedString,
    pub role: UserRole,
    pub username_updated_at: Option<String>,
    // pub updated_at: Option<DateWithTz>,
}

#[derive(Clone, Debug, PartialEq, Eq, IvoStruct)]
pub struct UserInput {
    pub email: String,
    pub username: String,
    pub role: UserRole,
    pub slug_id: String, // alias for v_slug
}

#[derive(Clone)]
pub struct UserCtxOptions {
    pub slug_id: Option<SlugifiedString>,
    // pub locale: &'static str, // fr, en, de, etc
}

impl<'a> UserCtxOptions {
    fn find_user_by_slug_id(
        &self,
        _slug_id: &SlugifiedString,
    ) -> impl Future<Output = Option<User>> + use<'a> {
        ready(None)
    }

    fn update_slug_id(&self, _slug_id: &SlugifiedString) {
        // self.slug_id = Some(slug_id.clone());
    }
}

type Ctx = IvoContext<UserInput, User>;

pub static USER_MODEL: LazyLock<Model<UserInput, User, UserCtxOptions>> =
    LazyLock::new(|| USER_SCHEMA.get_model());

pub static USER_SCHEMA: LazyLock<Schema<UserInput, User, UserCtxOptions>> = LazyLock::new(|| {
    Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set(
                    "email",
                    IvoField::REQUIRED
                        .required_error("\"email\" is required!")
                        .validate(|email, _, _| {
                            ready(validate_email(email).map_err(|e| (e, None)))
                        }), // .required_error_fn(|_, _| ready("Please provide an email address".into())),
                )
                .set(
                    "role",
                    IvoField::LAX
                        .default(UserRole::User)
                        .ignore_if(|_, _| ready(true)),
                )
                .set(
                    "username",
                    IvoField::REQUIRED
                        .required_error("Please provide a username")
                        .validate(|v: String, _, _| {
                            const MIN_LEN: usize = 4;

                            if v.len() <= MIN_LEN {
                                return ready(Err((
                                    format!("Username must be atleast {MIN_LEN} long"),
                                    None,
                                )));
                            }

                            ready(Ok(v))
                        })
                        .allow_update_if(|ctx: Ctx, _| {
                            ready(is_username_or_slug_id_updatable(
                                ctx.values().username_updated_at.unwrap(),
                            ))
                        }),
                )
                .set(
                    "username_last_updated_at",
                    IvoField::DEPENDENT
                        .default(None)
                        .depends_on(["username"])
                        .resolve(|_, _| ready(Some(String::from("now")))),
                )
                .set(
                    "slug_id",
                    IvoField::DEPENDENT
                        .default(SlugifiedString::from(""))
                        .depends_on(["username", "v_slug"])
                        .resolve(|_, o: UserCtxOptions| ready(o.slug_id.clone().unwrap())),
                )
                .set(
                    "v_slug",
                    IvoField::VIRTUAL
                        .alias("slug_id")
                        .validate(|value: String, _, o: UserCtxOptions| {
                            let slug_id = slugify(&value);

                            let validated = slug_id.value();

                            if validated.len() < 2 {
                                return ready(Err((
                                    "slug ids must be at least 2 characters long".into(),
                                    None,
                                )));
                            }

                            o.update_slug_id(&slug_id);

                            ready(Ok(validated))
                        })
                        .allow_update_if(|ctx: Ctx, _| {
                            ready(is_username_or_slug_id_updatable(
                                ctx.values().username_updated_at.unwrap(),
                            ))
                        }),
                )
                .created_at(|| "Date.now()", None)
                .updated_at(|| "Date.now()", Some("updated_on"), true)
        },
        |o| {
            o.post_validate(["username", "v_slug"], |b| {
                b.validate(|ctx: Ctx, o: UserCtxOptions| {
                    let input = ctx.input();

                    let slug_id = if input.slug_id.is_some() {
                        o.slug_id.clone().unwrap()
                    } else {
                        slugify(&input.username.clone().unwrap())
                    };

                    o.find_user_by_slug_id(&slug_id).map(move |user| {
                        if user.is_none() {
                            o.update_slug_id(&slug_id);

                            // let mut validated = IvoValues::new();

                            // validated
                            //     .set("slug_id".into(), slug_id.value())
                            //     .set("username".into(), "validated-username");

                            return Ok(IvoValues::new());
                        }

                        let err = (
                            format!("A user with a slug id: \"{slug_id}\" already exists"),
                            None,
                        );

                        let mut errors = HashMap::new();

                        if input.username.is_some() {
                            errors.insert("username".into(), err.clone());
                        }

                        if input.slug_id.is_some() {
                            errors.insert("v_slug".into(), err);
                        }

                        Err(errors)
                    })
                })
            })
            .on_success(["email"], |b| {
                b.handle(|_, _| {
                    println!("on success");
                    ready(())
                })
            })
            .on_success(["username", "v_slug"], |b| {
                b.handle(|_, _| {
                    println!("on success");

                    ready(())
                })
            })
            .on_delete(|_, _| {
                println!("on delete");

                ready(())
            })
            .on_delete(|_, _| {
                println!("on delete");

                ready(())
            })
        },
    )
});

fn is_username_or_slug_id_updatable(username_updated_at: Option<String>) -> bool {
    match username_updated_at {
        Some(v) => v.as_str() == "yesterday",
        _ => true,
    }
}
