use std::{
    array,
    collections::HashMap,
    future::{ready, Future},
    sync::LazyLock,
};

use chrono::{DateTime, Utc};
use ivo::{
    constant_field, dependent_field, lax_field, required_field, validate_email, virtual_field,
    FutureExt, IvoContext, IvoInputStruct, IvoModel, IvoRwCtxOptions, IvoShared, IvoStruct,
};

use crate::slugify::{slugify, SlugifiedString};

type Timestamp = DateTime<Utc>;

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct UserInput {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub username: String,
    pub slug_id: String, // alias for v_slug
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct User {
    pub created_at: Timestamp,
    pub id: i32,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub slug_id: SlugifiedString,
    pub username: String,
    pub username_last_updated_at: Option<Timestamp>,
    pub updated_at: Timestamp,
}

#[derive(Clone)]
pub struct UserCtxOptions {
    pub slug_id: Option<SlugifiedString>,
}

impl UserCtxOptions {
    pub fn new() -> Self {
        Self { slug_id: None }
    }

    fn find_user_by_username(
        &self,
        username: &str,
    ) -> impl Future<Output = Option<User>> + use<'_> {
        ready(USERS_BY_USERNAME.get(username).cloned())
    }

    fn find_user_by_slug_id(
        &self,
        slug_id: &SlugifiedString,
    ) -> impl Future<Output = Option<User>> + use<'_> {
        ready(USERS_BY_SLUG_ID.get(slug_id).cloned())
    }

    fn update_slug_id(&mut self, slug_id: &SlugifiedString) {
        self.slug_id = Some(slug_id.clone());
    }
}

type Ctx = IvoContext<UserInput, User>;
type RwCtxOptions = IvoRwCtxOptions<UserCtxOptions>;

pub static USER_MODEL: LazyLock<IvoModel<UserInput, User, UserCtxOptions, Timestamp>> =
    LazyLock::new(|| {
        IvoModel::new(
            |f| {
                f.field(constant_field("id").value_fn(|_, _| ready(1234)))
                    .field(
                        lax_field("email")
                            .default(None)
                            .validate(|v: Option<String>, _, _| {
                                if let Some(email) = v {
                                    let r = validate_email(&email)
                                        .map(|v| Some(Some(v)))
                                        .map_err(|e| (e, None));

                                    return ready(r);
                                }

                                ready(Ok(None))
                            }),
                    )
                    .field(
                        lax_field("phone_number")
                            .default(None::<String>)
                            .validate(|_, _, _| ready(Ok(None))),
                    )
                    .field(
                        required_field("username")
                            // .required_error("\"username\" was not provided!")
                            .required_error_fn(|_, _| {
                                ready("\"username\" was not provided!".into())
                            })
                            .validate(|v: String, _, _| {
                                const MIN_LEN: usize = 4;

                                if v.len() < MIN_LEN {
                                    return ready(Err((
                                        format!(
                                        "\"username\" must be at least {MIN_LEN} characters long"
                                    ),
                                        None,
                                    )));
                                }

                                ready(Ok(None))
                            })
                            .re_validate(async |uname: String, _, o: RwCtxOptions| {
                                if o.read().await.find_user_by_username(&uname).await.is_some() {
                                    return Err((
                                        "username: \"{uname}\" is already taken".into(),
                                        None,
                                    ));
                                }

                                Ok(Some(format!("revalidated-'{}'", uname)))
                            })
                            .on_delete(|_, _| {
                                println!("[username]: on delete 1 handled");

                                ready(())
                            })
                            .on_delete(|_, _| {
                                println!("[username]: on delete 2 handled");

                                ready(())
                            }),
                    )
                    .field(
                        dependent_field("username_last_updated_at", ["username"])
                            .default(None)
                            .resolve(|ctx: Ctx, _| {
                                let value = if ctx.is_update() {
                                    Some(Utc::now())
                                } else {
                                    None
                                };

                                ready(value)
                            }),
                    )
                    .field(
                        dependent_field("slug_id", ["username", "v_slug"])
                            .default(SlugifiedString::from(""))
                            .resolve(|_, o: RwCtxOptions| {
                                o.read().map(|g| g.slug_id.clone().unwrap())
                            })
                            .on_delete(|data: IvoShared<User>, _| {
                                println!("[dependent_slug_id]: on delete: {:?}", data.slug_id);

                                ready(())
                            }),
                    )
                    .field(
                        virtual_field("v_slug")
                            .alias("slug_id")
                            .validate(|value: String, _, _| {
                                let validated = value.trim();

                                if validated.len() < 2 {
                                    return ready(Err((
                                        "slug ids must be at least 2 characters long".into(),
                                        None,
                                    )));
                                }

                                ready(Ok(Some(validated.into())))
                            })
                            .sanitize(|v, _, _| ready(format!("sanitized-'{v}'"))),
                    )
                    .timestamps(|t| {
                        t.resolve(Utc::now)
                            .created_at(None)
                            .updated_at(Some("updated_at"))
                    })
            },
            |o| {
                o
                    .ignore_update(["username", "v_slug"], |_, user: User, _| {
                        ready(match user.username_last_updated_at {
                            Some(dt) => (Utc::now() - dt).num_days() < 30,
                            _ => false,
                        })
                    })
                    .required(["email", "phone_number"], |ctx: Ctx, _| {
                    if ctx.is_update() {
                        return ready(None);
                    }

                    let error = "provide either an \"email\" or a \"phone number\" to proceed";

                    ready(Some(
                        UserInputErrors::new()
                            .with_email(error, None)
                            .with_phone_number(error, None),
                    ))
                })
                    .post_validate(["email", "phone_number"], |b| {
                        b.validate(async |ctx: Ctx, _| {
                            if !ctx.is_update() {
                                return Ok(None);
                            }

                            let input = ctx.input();

                            let is_valid = input.email.as_ref().map_or(false, |e| e.is_some())
                                || input.phone_number.as_ref().map_or(false, |p| p.is_some());

                            if is_valid {
                                return Ok(None);
                            }

                            let error = "provide either an \"email\" or a \"phone number\" to proceed";

                            Err(UserInputErrors::new()
                                .with_email(error, None)
                                .with_phone_number(error, None))
                        })
                    })
                    .post_validate(["username", "v_slug"], |b| {
                        b.validate(async |ctx: Ctx, o: RwCtxOptions| {
                            let input = ctx.input();
                            let input_slug_id = ctx.input().slug_id.clone();

                            let slug_string = input_slug_id
                                .clone()
                                .unwrap_or_else(|| input.username.clone().unwrap());

                            let slug_id = slugify(&slug_string);

                            println!(
                                "\npost validating username & v_slug: [slug_string = {}] & [slug_id = {}]",
                                slug_string, slug_id
                            );

                            let mut options = o.write().await;

                            if options.find_user_by_slug_id(&slug_id).await.is_none() {
                                options.update_slug_id(&slug_id);

                                return Ok(None);
                            }

                            drop(options);

                            let (reason, metadata) = (
                                &format!("A user with a slug id: \"{slug_id}\" already exists"),
                                None,
                            );

                            let mut errors = UserInputErrors::new();

                            if input_slug_id.is_some() {
                                errors.set_slug_id(reason, metadata);
                            } else if input.username.is_some() {
                                errors.set_username(reason, metadata);
                            }

                            Err(errors)
                        })
                    })
                    .on_success(["email"], |b| {
                        b.handle(|_, _| {
                            println!("[options.on_success]: [email]");
                            ready(())
                        })
                    })
                    .on_success(["username", "v_slug"], |b| {
                        b.handle(|_, _| {
                            println!("[options.on_success]: [username, v_slug]");

                            ready(())
                        })
                    })
                    .on_delete(|_, _| {
                        println!("[options.on_delete]: fn 1");

                        ready(())
                    })
                    .on_delete(|_, _| {
                        println!("[options.on_delete]: fn 2");

                        ready(())
                    })
            },
        )
    });

pub static USERS_LIST: LazyLock<[User; 3]> = LazyLock::new(|| {
    array::from_fn(|i| {
        let id = (i as i32) + 1;
        let username = format!("user-{id}");

        let now = Utc::now();

        User {
            created_at: now,
            email: Some(format!("user-{id}@mail.com")),
            id,
            phone_number: None,
            slug_id: SlugifiedString::from(username.as_str()),
            username,
            username_last_updated_at: None,
            updated_at: now,
        }
    })
});

pub static USERS_BY_SLUG_ID: LazyLock<HashMap<SlugifiedString, User>> = LazyLock::new(|| {
    let collection: [(SlugifiedString, User); 3] = array::from_fn(|i| {
        let u = USERS_LIST[i].clone();
        (u.slug_id.clone(), u)
    });

    HashMap::from(collection)
});

pub static USERS_BY_USERNAME: LazyLock<HashMap<String, User>> = LazyLock::new(|| {
    let collection: [(String, User); 3] = array::from_fn(|i| {
        let u = USERS_LIST[i].clone();
        (u.username.clone(), u)
    });

    HashMap::from(collection)
});
