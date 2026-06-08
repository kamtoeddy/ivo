use ivo::{UpdateError, traits::WithUpdateDetails};
use std::time::Instant;

mod domain;

use crate::{
    users::domain::{PartialUserInput, USER_MODEL, User, UserCtxOptions, UserRole},
    utils::{slugify::slugify, styled_text::Stylable},
};

pub async fn run_users_demo() {
    let timer = Instant::now();

    let ctx_options = UserCtxOptions { slug_id: None };

    let input = PartialUserInput {
        email: Some("1@1.com".to_string()),
        username: Some("john".to_string()),
        role: Some(UserRole::Moderator),
        slug_id: None,
    };

    let r = USER_MODEL.create(&input, ctx_options.clone()).await;

    match r {
        Ok((data, _handle_success)) => {
            println!("{:?}", data);
        }
        Err((payload, _handle_failure)) => {
            println!("Failed to create: {:?}", payload);
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
        email: "1@1.com".into(),
        id: 1,
        username,
        slug_id,
        role: UserRole::Admin,
    };

    println!("{:?}", user);

    let updates = PartialUserInput {
        email: Some(user.email.clone()),
        role: None,
        // role: Some(UserRole::User),
        username: Some(user.username.clone()),
        slug_id: None,
    };

    let r = USER_MODEL.update(&user, &updates, ctx_options).await;

    match r {
        Ok((data, _handle_success)) => {
            println!("updates: {:?}", data);
            println!("old + updates: {:?}", user.ivo_internal_clone_with(&data));
        }
        Err((error, _handle_failure)) => {
            match error {
                UpdateError::NothingToUpdate => println!("Nothing to update"),
                UpdateError::ValidationError(payload) => {
                    println!("Failed to update: {:?}", payload)
                }
            };
        }
    };

    println!(
        "{} {}\n",
        "\nUpdate duration:".font_bold(),
        format!("{:?}", timer.elapsed()).colored_blue()
    );
}
