use ivo::{UpdateError, erase_value, parse_or_panic};
use std::{collections::HashMap, time::Instant};

mod domain;

use crate::{
    users::domain::{PartialUserInput, USER_MODEL, USER_SCHEMA, User, UserCtxOptions, UserRole},
    utils::{slugify::slugify, styled_text::Stylable},
};

pub async fn run_users_demo() {
    // let schema = DEMO::make_schema();
    println!("UserSchema props: {:?}\n", USER_SCHEMA.props);
    // let timer = Instant::now();
    let f = USER_SCHEMA.fields();
    // println!("Fields extracted in {:?}\n", timer.elapsed());

    // let timer = Instant::now();
    println!("User fields: {:?}\n", f);
    // println!("Fields printed in {:?}\n", timer.elapsed());

    let timer = Instant::now();

    let ctx_options = UserCtxOptions { slug_id: None };

    let r = USER_MODEL
        .create(
            &PartialUserInput {
                email: Some("1@1.com".to_string()),
                username: Some("john".to_string()),
                role: None,
                v_slug: None,
            },
            ctx_options.clone(),
        )
        .await;

    let _ = match r {
        Ok((data, handle_success)) => {
            println!("{:?}", data);
            handle_success
        }
        Err((payload, handle_failure)) => {
            println!("Failed to create: {:?}", payload);
            handle_failure
        }
    };

    println!(
        "{} {}\n",
        "\nCreate duration:".font_bold(),
        format!("{:?}", timer.elapsed()).colored_blue()
    );

    let timer = Instant::now();

    let (username, slug_id) = {
        let username = "John Doe";

        (username.to_owned(), slugify(username))
    };

    let user = User {
        // created_at: DateWithTz::default(),
        email: "1@1.com".into(),
        id: "id".into(),
        username,
        slug_id,
        role: UserRole::Admin,
        // username_updated_at: None,
        // updated_at: None,
    };

    println!("{:?}", user);

    let r = USER_MODEL
        .update(
            &user,
            &PartialUserInput {
                email: Some(user.email.clone()),
                // role: Some(user.role.clone()),
                role: Some(UserRole::User),
                username: Some(user.username.clone()),
                v_slug: None,
            },
            ctx_options,
        )
        .await;

    let _ = match r {
        Ok((data, handle_success)) => {
            println!("Updates: {:?}", data);
            handle_success
        }
        Err((error, handle_failure)) => {
            match error {
                UpdateError::NothingToUpdate => println!("Nothing to update"),
                UpdateError::ValidationError(payload) => {
                    println!("Failed to update: {:?}", payload)
                }
            };

            handle_failure
        }
    };

    println!(
        "{} {}\n",
        "\nUpdate duration:".font_bold(),
        format!("{:?}", timer.elapsed()).colored_blue()
    );

    let mut map = HashMap::new();
    map.insert("k", Some(erase_value(1)));

    // let l = map.get("k") ;
    if let Some(Some(v)) = map.get("k") {
        println!("k = {}", parse_or_panic::<i32>(&v))
    }
}
