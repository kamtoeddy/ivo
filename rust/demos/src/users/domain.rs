use std::sync::LazyLock;

use ivo::{IvoField, IvoStruct, IvoSummary, Model, Schema, validate_email};

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

    fn update_slug_id(&mut self, slug_id: &SlugifiedString) {
        self.slug_id = Some(slug_id.clone());
    }
}

pub static USER_MODEL: LazyLock<Model<UserInput, User, UserCtxOptions>> =
    LazyLock::new(|| USER_SCHEMA.get_model());

pub static USER_SCHEMA: LazyLock<Schema<UserInput, User, UserCtxOptions>> = LazyLock::new(|| {
    Schema::new(
        |f| {
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
                IvoField::REQUIRED.validate(|v: String, _| {
                    const MIN_LEN: usize = 4;

                    if v.len() <= MIN_LEN {
                        return Err((
                            format!("Username must be atleast {MIN_LEN} long"),
                            None,
                        ));
                    }

                    Ok(v)
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
                        s.get_options()
                            .slug_id
                            .clone()
                            .expect("\"slug_id\" should have been generated in post validator by this point")
                    }),
            )
            .set(
                "v_slug",
                IvoField::VIRTUAL.alias("slug_id").validate(
                    |value: String, s: MutUserSummary| {
                        let slug = slugify(value.trim());

                        let value = slug.0.clone();

                        if value.len() < 2 {
                            return Err((
                                "slug ids must be at least 2 characters long".into(),
                                None,
                            ));
                        }

                        s.get_options_mut().update_slug_id(&slug);

                        Ok(value)
                    },
                ),
            )
        },
        |o| {
            o.post_validate(["username", "v_slug"], |pv| {
                pv.validate(|s: MutUserSummary| async move {
                    let mut ctx_options = s.get_options_mut();
                    let input = s.input();

                    let slug_string = if let Some(slug_id) = input.slug_id.clone() {
                        slug_id
                    } else {
                        input.username.clone().unwrap()
                    };

                    let slug_id = slugify(&slug_string);

                    if ctx_options.find_user_by_slug_id(&slug_id).await.is_some() {
                        let err = (
                            format!("A user with a slug id: \"{slug_id}\" already exists"),
                            None,
                        );

                        let mut errors = Vec::with_capacity(2);

                        if input.username.is_some() {
                            errors.push(("username", err.clone()));
                        }

                        if input.slug_id.is_some() {
                            errors.push(("v_slug", err.clone()));
                        }

                        return Err(errors);
                    }

                    ctx_options.update_slug_id(&slug_id);

                    Ok(Vec::with_capacity(0))
                })
            })
        },
    )
});
