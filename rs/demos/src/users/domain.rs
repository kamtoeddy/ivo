use std::{collections::HashMap, future::ready, sync::LazyLock};

use ivo::{
    FutureExt, IvoField, IvoStruct, IvoValues, Model, Schema, SharedIvoContext, SharedRwCtxOptions,
    validate_email,
};

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
    pub username_last_updated_at: Option<String>,
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
    pub fn new() -> Self {
        Self { slug_id: None }
    }

    fn find_user_by_slug_id(
        &self,
        _slug_id: &SlugifiedString,
    ) -> impl Future<Output = Option<User>> + use<'a> {
        ready(None)
    }

    fn update_slug_id(&mut self, slug_id: &SlugifiedString) {
        self.slug_id = Some(slug_id.clone());
    }
}

type Ctx = SharedIvoContext<UserInput, User>;
// type CtxOptions = SharedCtxOptions<UserCtxOptions>;
type RwCtxOptions = SharedRwCtxOptions<UserCtxOptions>;

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
                        .validate(|email, _, _| ready(validate_email(email).map_err(|e| (e, None))))
                        .on_failure(|_, _| {
                            println!("email: on failure handled");

                            ready(())
                        }),
                    // .required_error_fn(|_, _| ready("Please provide an email address".into())),
                )
                .set(
                    "role",
                    IvoField::LAX
                        .default(UserRole::User)
                        .validate(|v, _, _| ready(Ok(v)))
                        .ignore_if(|_, _| ready(true))
                        .on_delete(|_, _| {
                            println!("role: on delete handled");

                            ready(())
                        })
                        .on_failure(|_, _| {
                            println!("role: on failure handled");

                            ready(())
                        }),
                )
                .set(
                    "username",
                    IvoField::REQUIRED
                        .required_error("Please provide a username")
                        .validate(|v: String, _, _| {
                            const MIN_LEN: usize = 4;

                            if v.len() < MIN_LEN {
                                return ready(Err((
                                    format!("\"username\" must be at least {MIN_LEN} characters long"),
                                    None,
                                )));
                            }

                            ready(Ok(v))
                        })
                        .re_validate(|v: String, _, _| {
                            const MIN_LEN: usize = 5;

                            if v.len() < MIN_LEN {
                                return ready(Err((
                                    format!("re-validation requires \"username\" to be at least {MIN_LEN} characters long"),
                                    None,
                                )));
                            }

                            ready(Ok(format!("revalidated-'{}'",v)))
                        })
                        .allow_update_if(|ctx: Ctx, _| {
                            ready(is_username_or_slug_id_updatable(
                                ctx.values().username_last_updated_at.unwrap(),
                            ))
                        })
                        .on_delete(|_, _| {
                            println!("username: on delete 1 handled");

                            ready(())
                        })
                        .on_delete(|_, _| {
                            println!("username: on delete 2 handled");

                            ready(())
                        })
                        .on_failure(|_, _| {
                            println!("username: on failure handled");

                            ready(())
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
                        .resolve(|_, o: RwCtxOptions| o.read().map(|g| g.slug_id.clone().unwrap())),
                )
                .set(
                    "v_slug",
                    IvoField::VIRTUAL
                        .alias("slug_id")
                        .validate(|value: String, _, _| {
                            println!("validating v_slug as slug_id with: {}\n",value.clone());

                            let validated = value.trim();

                            if validated.len() < 2 {
                                return ready(Err((
                                    "slug ids must be at least 2 characters long".into(),
                                    None,
                                )));
                            }

                            ready(Ok(validated.into()))
                        })
                        .allow_update_if(|ctx: Ctx, _| {
                            ready(is_username_or_slug_id_updatable(
                                ctx.values().username_last_updated_at.unwrap(),
                            ))
                        }),
                )
                .created_at(|| "Date.now()", None)
                .updated_at(|| "Date.now()", Some("updated_on"), true)
        },
        |o| {
            o.post_validate(["username", "v_slug"], |b| {
                b.validate(async |ctx: Ctx, o: RwCtxOptions| {
                    let input = ctx.input();

                    let slug_string = match &input.slug_id {
                        Some(v) => v.clone(),
                        _ => input.username.as_ref().unwrap().clone(),
                    };

                    let slug_id = slugify(&slug_string);

                    println!("post validating username & v_slug: [slug_string = {slug_string}] & [slug_id = {slug_id}]\n");

                    let mut options = o.write().await;

                    if options.find_user_by_slug_id(&slug_id).await.is_none() {
                        options.update_slug_id(&slug_id);

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
                println!("on delete fn 1");

                ready(())
            })
            .on_delete(|_, _| {
                println!("on delete fn 2");

                ready(())
            })
        },
    )
});

fn is_username_or_slug_id_updatable(username_last_updated_at: Option<String>) -> bool {
    match username_last_updated_at {
        Some(v) => v.as_str() == "yesterday",
        _ => true,
    }
}
