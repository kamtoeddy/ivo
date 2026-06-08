use std::sync::LazyLock;

use ivo::{IvoField, IvoStruct, IvoSummary, Model, Schema};

use crate::utils::slugify::SlugifiedString;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UserRole {
    Admin,
    User,
    Moderator,
}

#[derive(Debug, Clone, PartialEq, Eq, IvoStruct)]
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

#[derive(Clone, Debug, PartialEq, Eq, IvoStruct)]
pub struct UserInput {
    pub email: String,
    pub username: String,
    pub role: UserRole,
    pub v_slug: SlugifiedString,
}

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

pub static USER_SCHEMA: LazyLock<Schema<UserInput, User, UserCtxOptions>> = LazyLock::new(|| {
    Schema::new(
        |f| {
            f.set(
                "c",
                IvoField::CONSTANT
                    .value(String::from("String"))
                    .on_success(async |_| {})
                    .on_delete(async |_, _| {}),
            )
            .set(
                "c1",
                IvoField::CONSTANT
                    .value(Some(String::from("Option<String>")))
                    .on_success(async |_| {})
                    .on_delete(async |_, _| {}),
            )
            .set(
                "c2",
                IvoField::CONSTANT
                    .computed(async |_| true)
                    .on_delete(async |_, _| {})
                    .on_success(async |_| println!("on success 1"))
                    .on_success(async |_| println!("on success 2")),
            )
            .set(
                "enum",
                IvoField::ENUM
                    .values([true, false])
                    .error_fn(|_| ("".into(), None))
                    // .error("invalid option provided")
                    .default_fn(async |_| true)
                    .readonly()
                    .on_delete(async |_, _| {})
                    .on_failure(async |_| {})
                    .on_success(async |_| {}),
            )
            .set(
                "d",
                IvoField::DEPENDENT
                    .default(String::from("Hello"))
                    .depends_on(["first_name", "last_name"])
                    .resolve(async |_| resolve_full_name())
                    .on_delete(|_, _| async {})
                    .on_success(|_| async {}),
            )
            .set(
                "d1",
                IvoField::DEPENDENT
                    .default_fn(async |_| true)
                    .depends_on(["first_name", "last_name"])
                    .resolve(async |_| {
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
                    .validate(async |_, _| Ok(true))
                    .readonly()
                    .on_delete(async |_, _| {})
                    .on_failure(async |_| {})
                    .on_success(async |_| {}),
            )
            .set(
                "l1",
                IvoField::LAX
                    .default_fn(async |_| None)
                    .validate(|_, _| async move { Ok(Some(1)) })
                    .re_validate(async |_, _| Ok(Some(2)))
                    .readonly()
                    .on_delete(async |_, _| {})
                    .on_failure(async |_| {})
                    .on_success(async |_| {}),
            )
            .set(
                "l2",
                IvoField::LAX
                    .default(None)
                    .validate(|_, _| async { Ok(Some(true)) })
                    .re_validate(|v, _| async move { Ok(v) })
                    .readonly()
                    .on_delete(|_, _| async {})
                    .on_failure(|_| async {})
                    .on_success(|_| async {}),
            )
            .set(
                "r",
                IvoField::REQUIRED
                    .validate(async |_, _| Err(("lol".into(), None)))
                    .re_validate(async |_, _| Ok(true))
                    .readonly()
                    .on_failure(|_| async {})
                    .on_success(|_| async {})
                    .on_delete(|_, _| async {}),
            )
            .set(
                "v",
                IvoField::VIRTUAL
                    .alias("lol")
                    .validate(async |_, _| Ok(true))
                    .re_validate(async |_, _| Ok(true))
                    .required_if(async |_| (true, "lol".into()))
                    .sanitize(async |_| false)
                    .on_failure(async |_| {})
                    .on_success(async |_| {}),
            )
            .set(
                "v1",
                IvoField::VIRTUAL
                    .validate(async |v, _| {
                        if v || !v {
                            Ok(v)
                        } else {
                            Err(("Invalid boolean".into(), None))
                        }
                    })
                    .re_validate(async |_, _| Ok(true))
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
                    .validate(async |_, _| Ok(true))
                    .re_validate(async |_, _| Ok(true))
                    .alias("lol")
                    .required_if(|_| async { (true, "lol".into()) })
                    .sanitize(|_| async { false })
                    .on_failure(|_| async {})
                    .on_success(|_| async {}),
            )
            .set(
                "v3",
                IvoField::VIRTUAL
                    .validate(async |_, _| Ok(true))
                    .alias("v3")
                    .re_validate(async |_, _| Ok(true))
                    .required_if(async |_| (true, "lol".into()))
                    .sanitize(async |_| false)
                    // .ignore_if(|_| false)
                    .allow_update_if(async |_| false)
                    .allow_init_if(async |_| false)
                    // .ignore_init()
                    // .ignore_update()
                    .on_failure(async |_| {})
                    .on_failure(async |_| {})
                    .on_success(async |_| println!("on success 1"))
                    .on_success(async |_| println!("on success 2")),
            )
        },
        |o| o,
    )
});

fn resolve_full_name() -> String {
    String::from("full name")
}
