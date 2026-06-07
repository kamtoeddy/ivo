use std::sync::LazyLock;

use ivo::{IvoField, IvoStruct, IvoSummary, Model, SchemaCore, validate_email};

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
    }
}

pub static USER_MODEL: LazyLock<Model<UserInput, User, UserCtxOptions>> =
    LazyLock::new(|| USER_SCHEMA.get_model());

pub static USER_SCHEMA: LazyLock<SchemaCore<UserInput, User, UserCtxOptions>> =
    LazyLock::new(|| {
        SchemaCore::new()
            .with_fields(|f| {
                f.set("id", IvoField::CONSTANT.computed(|_| 1234))
                    .set(
                        "email",
                        IvoField::REQUIRED
                            .validate(|email, _| validate_email(email).map_err(|e| (e, None))),
                    )
                    .set(
                        "role",
                        IvoField::LAX.default(UserRole::User).ignore_if(|_| true),
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
                            .depends_on(["username"])
                            .resolve(|_| Some(String::from("resolved value"))),
                    )
                    .set(
                        "slug_id",
                        IvoField::DEPENDENT
                            .default(SlugifiedString("".into()))
                            .depends_on(["username", "v_slug"])
                            .resolve(|s: MutUserSummary| {
                                if let Some(v_slug) = s.input().slug_id.clone() {
                                    return SlugifiedString(v_slug);
                                }

                                if let Some(slug) = s.get_options().slug_id.clone() {
                                    return slug;
                                }

                                SlugifiedString("something".into())
                            }),
                    )
                    .set(
                        "v_slug",
                        IvoField::VIRTUAL
                            .alias("slug_id")
                            .validate(|value: String, _| {
                                let value = slugify(value.trim()).0;

                                if value.len() < 2 {
                                    return Err((
                                        "slug ids must be at least 2 characters long".into(),
                                        None,
                                    ));
                                }

                                Ok(value)
                            }),
                    )
            })
            .with_options()
    });
