use std::{array, collections::HashMap, sync::LazyLock};

use chrono::{DateTime, Utc};
use ivo::ivo_schema;

use crate::slugify::SlugifiedString;

type Timestamp = DateTime<Utc>;

#[derive(Clone)]
pub struct UserCtxOptions {
    pub slug_id: Option<SlugifiedString>,
}

impl UserCtxOptions {
    pub fn new() -> Self {
        Self { slug_id: None }
    }

    pub async fn find_user_by_username(&self, username: &str) -> Option<User> {
        USERS_BY_USERNAME.get(username).cloned()
    }

    pub async fn find_user_by_slug_id(&self, slug_id: &SlugifiedString) -> Option<User> {
        USERS_BY_SLUG_ID.get(slug_id).cloned()
    }

    pub fn update_slug_id(&mut self, slug_id: &SlugifiedString) {
        self.slug_id = Some(slug_id.clone());
    }
}

#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq)),
    output(User, derive(Debug, Clone, PartialEq)),
    ctx_options(UserCtxOptions)
)]
mod user_schema {
    use super::{SlugifiedString, Timestamp, UserCtxOptions};
    use crate::slugify::slugify;
    use chrono::Utc;
    use ivo::validate_email;

    struct Fields {
        #[constant(1234)]
        pub id: i32,

        #[created_at]
        pub created_at: Timestamp,

        #[updated_at]
        pub updated_at: Timestamp,

        #[lax(None)]
        #[validate(|v, _, _| {
            if let Some(email) = v {
                match validate_email(&email) {
                    Ok(validated) => Ok(Some(Some(validated))),
                    Err(e) => Err((e, None)),
                }
            } else {
                Ok(None)
            }
        })]
        pub email: Option<String>,

        #[lax(None)]
        #[validate(|_, _, _| Ok(None))]
        pub phone_number: Option<String>,

        #[required]
        #[required_error(|_, _| "\"username\" was not provided!".to_string())]
        #[validate(|v, _, _| {
            const MIN_LEN: usize = 4;
            if v.len() < MIN_LEN {
                return Err((
                    format!("\"username\" must be at least {MIN_LEN} characters long"),
                    None,
                ));
            }
            Ok(None)
        })]
        #[re_validate(async |uname, _, o| {
            if o.read().await.find_user_by_username(&uname).await.is_some() {
                return Err(("username: \"{uname}\" is already taken".into(), None));
            }
            Ok(Some(format!("revalidated-'{uname}'")))
        })]
        #[on_delete(|_, _| {})]
        #[on_delete(|_, _| {})]
        pub username: String,

        #[depends_on("username")]
        #[default(None)]
        #[resolve(|ctx, _| {
            if ctx.is_update() {
                Some(Utc::now())
            } else {
                None
            }
        })]
        pub username_last_updated_at: Option<Timestamp>,

        #[depends_on("username", "v_slug")]
        #[default(SlugifiedString::from(""))]
        #[resolve(async |_, o| { o.read().await.slug_id.clone().unwrap() })]
        #[on_delete(|data, _| {
            let _ = data.slug_id.clone();
        })]
        pub slug_id: SlugifiedString,

        #[ivo_virtual("slug_id")]
        #[validate(|value, _, _| {
            let validated = value.trim();
            if validated.len() < 2 {
                return Err((
                    "slug ids must be at least 2 characters long".into(),
                    None,
                ));
            }
            Ok(Some(validated.into()))
        })]
        #[sanitize(|v, _, _| format!("sanitized-'{v}'"))]
        pub v_slug: String,
    }

    #[timestamps(Utc::now)]
    const _: () = ();

    #[required(["email", "phone_number"], |ctx, _| {
        if ctx.is_update() {
            return None;
        }
        let error = "provide either an \"email\" or a \"phone number\" to proceed";

        Some(UserInputErrors::new()
            .with_email(error, None)
            .with_phone_number(error, None)
        )
    })]
    const _: () = ();

    #[post_validate(["email", "phone_number"], validate =  |ctx, _| {
        if !ctx.is_update() {
            return Ok(None);
        }

        let input = ctx.input();

        let is_valid =
            input.email.as_ref().is_some_and(|e| e.is_some()) ||
            input.phone_number.as_ref().is_some_and(|p| p.is_some());

        if is_valid {
            return Ok(None);
        }

        let error = "provide either an \"email\" or a \"phone number\" to proceed";

        Err(UserInputErrors::new()
            .with_email(error, None)
            .with_phone_number(error, None)
        )
    })]
    const _: () = ();

    #[ignore_update(["username", "v_slug"], async |ctx, _| {
        match ctx.values().username_last_updated_at {
            Some(dt) => (Utc::now() - dt).num_days() < 30,
            _ => false,
        }
    })]
    const _: () = ();

    #[post_validate(["username", "v_slug"], validate = async |ctx, o| {
        let input = ctx.input();
        let input_slug_id = input.slug_id.clone();

        let slug_string = input_slug_id
            .clone()
            .unwrap_or_else(|| input.username.clone().unwrap());

        let slug_id = slugify(&slug_string);

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
    })]
    const _: () = ();

    #[on_success(["email"], async |_, _| {})]
    const _: () = ();

    #[on_success(["username", "v_slug"], async |_, _| {})]
    const _: () = ();

    #[on_delete(|_, _| {})]
    const _: () = ();

    #[on_delete(|_, _| {})]
    const _: () = ();
}

pub use user_schema::{PartialUserInput, User, UserModel};

pub static USERS_LIST: LazyLock<[User; 3]> = LazyLock::new(|| {
    array::from_fn(|i| {
        let id = (i as i32) + 1;
        let username = format!("user-{id}");

        let now = Utc::now();

        User {
            id,
            created_at: now,
            updated_at: now,
            email: Some(format!("user-{id}@mail.com")),
            phone_number: None,
            username: username.clone(),
            username_last_updated_at: None,
            slug_id: SlugifiedString::from(username.as_str()),
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
