use std::{array, collections::HashMap, future::ready, sync::LazyLock, time::Instant};

use ivo::{
    IvoField, IvoStruct, IvoValues, Model, Schema, SharedCtxOptions, SharedData, SharedIvoContext,
    SharedRwCtxOptions, validate_email,
};

use crate::utils::{
    slugify::{SlugifiedString, slugify},
    styled_text::Stylable,
};

#[derive(Debug, PartialEq, Clone)]
pub enum UserRole {
    Admin,
    User,
    Moderator,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct User {
    // pub created_at: String,
    pub id: i32,
    pub email: String,
    pub username: String,
    pub slug_id: SlugifiedString,
    pub role: UserRole,
    pub username_last_updated_at: Option<String>,
    // pub updated_on: Option<String>,
}

#[derive(Clone, Debug, PartialEq, IvoStruct)]
pub struct UserInput {
    pub email: String,
    pub username: String,
    pub role: UserRole,
    pub slug_id: String, // alias for v_slug
}

#[derive(Clone)]
pub struct UserCtxOptions {
    pub slug_id: Option<SlugifiedString>,
    pub slug_id_resolver_run_count: i8,
    // pub locale: &'static str, // fr, en, de, etc
}

impl<'a> UserCtxOptions {
    pub fn new() -> Self {
        Self {
            slug_id: None,
            slug_id_resolver_run_count: 0,
        }
    }

    fn find_user_by_username(
        &self,
        username: &String,
    ) -> impl Future<Output = Option<User>> + use<'a> {
        ready(USERS_BY_USERNAME.get(username).cloned())
    }

    fn find_user_by_slug_id(
        &self,
        slug_id: &SlugifiedString,
    ) -> impl Future<Output = Option<User>> + use<'a> {
        ready(USERS_BY_SLUG_ID.get(slug_id).cloned())
    }

    fn update_slug_id(&mut self, slug_id: &SlugifiedString) {
        self.slug_id = Some(slug_id.clone());
    }
}

type Ctx = SharedIvoContext<UserInput, User>;
type CtxOptions = SharedCtxOptions<UserCtxOptions>;
type RwCtxOptions = SharedRwCtxOptions<UserCtxOptions>;

pub static USER_MODEL: LazyLock<Model<UserInput, User, UserCtxOptions>> =
    LazyLock::new(|| USER_SCHEMA.get_model());

pub static USER_SCHEMA: LazyLock<Schema<UserInput, User, UserCtxOptions>> = LazyLock::new(|| {
    let timer = Instant::now();

    println!("\nstart schema creation");
    let schema = Schema::new(
        |f| {
            f.set(
                "id",
                IvoField::CONSTANT
                    .computed(|_, _| ready(1234))
                    .on_success(|ctx: Ctx, _| {
                        println!("[id]: on success: {:?}", ctx.values().id);

                        ready(())
                    }),
            )
            .set(
                "email",
                IvoField::REQUIRED
                    .required_error("\"email\" was not provided!")
                    .validate(|email, _, _| ready(validate_email(email).map_err(|e| (e, None))))
                    .on_failure(|_, _| {
                        println!("[email]: on failure handled");

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
                        println!("[role]: on delete handled");

                        ready(())
                    })
                    .on_failure(|_, _| {
                        println!("[role]: on failure handled");

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
                    .re_validate(async |uname: String, _, o: RwCtxOptions| {
                        if o.read().await.find_user_by_username(&uname).await.is_some() {
                            return Err(("username: \"{uname}\" is already taken".into(), None));
                        }

                        Ok(format!("revalidated-'{}'", uname))
                    })
                    .allow_update_if(|ctx: Ctx, _| {
                        ready(is_username_or_slug_id_updatable(
                            ctx.values().username_last_updated_at.unwrap(),
                        ))
                    })
                    .on_delete(|_, _| {
                        println!("[username]: on delete 1 handled");

                        ready(())
                    })
                    .on_delete(|_, _| {
                        println!("[username]: on delete 2 handled");

                        ready(())
                    })
                    .on_failure(|_, _| {
                        println!("[username]: on failure handled");

                        ready(())
                    })
                    .on_success(|_, o: CtxOptions| {
                        println!("[username]: on success uname with slug_id: {:?}", o.slug_id);

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
                    .resolve(async |_, o: RwCtxOptions| {
                        let mut guard = o.write().await;

                        let slug_id = guard.slug_id.clone().unwrap();

                        guard.slug_id_resolver_run_count += 1;

                        slug_id
                    })
                    .on_success(|ctx: Ctx, o: CtxOptions| {
                        println!(
                            "[dependent_slug_id]: on success: {:?}",
                            ctx.values().slug_id
                        );
                        println!(
                            "[_________________]: on success with ctx_options.slug_id: {:?}",
                            o.slug_id,
                        );

                        assert_eq!(
                            o.slug_id_resolver_run_count, 1,
                            "this resolver should have run only once"
                        );

                        ready(())
                    })
                    .on_delete(|data: SharedData<User>, _| {
                        println!("[dependent_slug_id]: on delete: {:?}", data.slug_id);

                        ready(())
                    }),
            )
            .set(
                "v_slug",
                IvoField::VIRTUAL
                    .alias("slug_id")
                    .validate(|value: String, _, _| {
                        println!("[v_slug_as_slug_id]: validating: {}\n", value.clone());

                        let validated = value.trim();

                        if validated.len() < 2 {
                            return ready(Err((
                                "slug ids must be at least 2 characters long".into(),
                                None,
                            )));
                        }

                        ready(Ok(validated.into()))
                    })
                    .sanitize(|v, _, _| ready(format!("sanitized-'{v}'")))
                    .allow_update_if(|ctx: Ctx, _| {
                        ready(is_username_or_slug_id_updatable(
                            ctx.values().username_last_updated_at.unwrap(),
                        ))
                    })
                    .on_success(|ctx: Ctx, o: CtxOptions| {
                        println!(
                            "[v_slug_as_slug_id]: on success with ctx.input().slug_id: {:?}",
                            ctx.input().slug_id
                        );
                        println!(
                            "[_________________]: on success with ctx_options.slug_id: {:?}",
                            o.slug_id
                        );

                        ready(())
                    }),
            )
            // .created_at(|| "Date.now()", None)
            // .updated_at(|| "Date.now()", Some("updated_on"), true)
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

                    println!(
                        "post validating username & v_slug: [slug_string = {}] & [slug_id = {}]\n",
                        slug_string, slug_id
                    );

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

                    if input.slug_id.is_some() {
                        errors.insert("v_slug".into(), err);
                    } else if input.username.is_some() {
                        errors.insert("username".into(), err);
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
    );
    println!(
        "{} {}\n",
        "\nUser schema created:".font_bold(),
        format!("{:?}", timer.elapsed()).colored_red()
    );

    schema
});

fn is_username_or_slug_id_updatable(username_last_updated_at: Option<String>) -> bool {
    match username_last_updated_at {
        Some(v) => v.as_str() == "yesterday",
        _ => true,
    }
}

pub static USERS_LIST: LazyLock<[User; 3]> = LazyLock::new(|| {
    array::from_fn(|i| {
        let id = (i as i32) + 1;
        let username = format!("user-{id}");

        User {
            // created_at: "now".into(),
            email: format!("user-{id}@mail.com"),
            id,
            role: UserRole::Moderator,
            slug_id: SlugifiedString::from(username.as_str()),
            username,
            username_last_updated_at: None,
            // updated_on: None,
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
