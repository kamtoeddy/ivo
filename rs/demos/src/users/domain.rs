use std::{collections::HashMap, future::ready, sync::LazyLock};

use ivo::{IvoField, IvoStruct, IvoSummary, IvoValues, Model, Schema, validate_email};

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
    // pub username_updated_at: Option<DateWithTz>,
    // pub updated_at: Option<DateWithTz>,
}

#[derive(Clone, Debug, PartialEq, Eq, IvoStruct)]
pub struct UserInput {
    pub email: String,
    pub username: String,
    pub role: UserRole,
    pub slug_id: String, // alias for v_slug
}

type Summary = IvoSummary<UserInput, User, UserCtxOptions>;

#[derive(Clone)]
pub struct UserCtxOptions {
    pub slug_id: Option<SlugifiedString>,
}

impl UserCtxOptions {
    fn find_user_by_slug_id(&self, _slug: &SlugifiedString) -> impl Future<Output = Option<User>> {
        ready(None)
    }

    fn update_slug_id(&mut self, slug_id: &SlugifiedString) {
        self.slug_id = Some(slug_id.clone());
    }
}

pub static USER_MODEL: LazyLock<Model<UserInput, User, UserCtxOptions>> =
    LazyLock::new(|| USER_SCHEMA.get_model());

pub static USER_SCHEMA: LazyLock<Schema<UserInput, User, UserCtxOptions>> = LazyLock::new(|| {
    Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_| ready(1234)))
                .set(
                    "email",
                    IvoField::REQUIRED
                        .required_error("\"email\" is required!")
                        .validate(|email, _| ready(validate_email(email).map_err(|e| (e, None)))),
                    // .required_error_fn(async |_| "Please provide an email address".into()),
                )
                .set(
                    "role",
                    IvoField::LAX
                        .default(UserRole::User)
                        .ignore_if(|_| ready(true)),
                )
                .set(
                    "username",
                    IvoField::REQUIRED
                        .required_error("Please provide a username")
                        .validate(|v: String, _| {
                            const MIN_LEN: usize = 4;

                            if v.len() <= MIN_LEN {
                                return ready(Err((
                                    format!("Username must be atleast {MIN_LEN} long"),
                                    None,
                                )));
                            }

                            ready(Ok(v))
                        }),
                )
                .set(
                    "username_last_updated_at",
                    IvoField::DEPENDENT
                        .default(Some(String::from("default value")))
                        .depends_on(["username"])
                        .resolve(|_| ready(Some(String::from("resolved value")))),
                )
                .set(
                    "slug_id",
                    IvoField::DEPENDENT
                        .default(SlugifiedString::from("value"))
                        .depends_on(["username", "v_slug"])
                        .resolve(|s: Summary| ready(s.get_options().slug_id.clone().unwrap())),
                )
                .set(
                    "v_slug",
                    IvoField::VIRTUAL
                        .alias("slug_id")
                        .validate(|value: String, s: Summary| {
                            let slug_id = slugify(&value);

                            let validated = slug_id.value();

                            if validated.len() < 2 {
                                return ready(Err((
                                    "slug ids must be at least 2 characters long".into(),
                                    None,
                                )));
                            }

                            s.get_options_mut().update_slug_id(&slug_id);

                            ready(Ok(validated))
                        }),
                )
        },
        |o| {
            o.post_validate(["username", "v_slug"], |b| {
                b.validate(async |s: Summary| {
                    let mut ctx_options = s.get_options_mut();
                    let input = s.input();

                    let slug_id = if input.slug_id.is_some() {
                        ctx_options.slug_id.clone().unwrap()
                    } else {
                        slugify(&input.username.clone().unwrap())
                    };

                    if ctx_options.find_user_by_slug_id(&slug_id).await.is_some() {
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

                        return Err(errors);
                    }

                    ctx_options.update_slug_id(&slug_id);

                    // let mut validated = IvoValues::new();

                    // validated
                    //     .set("slug_id".into(), slug_id.value())
                    //     .set("username".into(), "validated-username");

                    Ok(IvoValues::new())
                })
            })
            .on_success(["email"], |b| {
                b.handle(|_| {
                    println!("on success");
                    ready(())
                })
            })
            .on_success(["username", "v_slug"], |b| {
                b.handle(|_| {
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
