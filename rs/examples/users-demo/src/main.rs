use ivo::{IvoStruct, IvoUpdateError};
use std::time::Instant;

mod domain;
mod slugify;

use crate::domain::{PartialUserInput, User, UserCtxOptions, UserRole, USER_MODEL};

#[async_std::main]
async fn main() {
    run_example().await;
}

async fn run_example() {
    let timer = Instant::now();

    let input = PartialUserInput {
        // email: None,
        email: Some("1@1.com".into()),
        username: Some("user-10".into()),
        // role: None,
        role: Some(UserRole::Moderator),
        // slug_id: None,
        slug_id: Some("sloppy-slug-id".into()),
    };

    // println!("runner2\n\n");
    let r = USER_MODEL.create(&input, UserCtxOptions::new()).await;

    match r {
        Ok((data, _, handle_success)) => {
            println!("{:?}\n", data);

            handle_success().await;
        }
        Err((payload, _, handle_failure)) => {
            println!("\nFailed to create: {:?}\n", payload);

            handle_failure().await;
        }
    };

    println!(
        "{} {}\n",
        "\nCreate duration:",
        format!("{:?}", timer.elapsed())
    );

    let timer = Instant::now();

    let (username, slug_id) = {
        let username = "John Doe";

        (username.into(), username.into())
    };

    let user = User {
        created_at: "2 days ago".into(),
        email: "1@1.com".into(),
        id: 1,
        username,
        username_last_updated_at: None,
        slug_id,
        role: UserRole::Admin,
        updated_on: "1 day ago".into(),
        // updated_on: None,
    };

    println!("{:?}\n", user);

    let updates = PartialUserInput {
        // email: None,
        email: Some(user.email.clone()),
        // role: None,
        role: Some(UserRole::Moderator),
        // username: None,
        username: Some("new_username".into()),
        slug_id: Some("updated-slug-id: Lol".into()),
        // slug_id: None,
    };

    let r = USER_MODEL
        .update(&user, &updates, UserCtxOptions::new())
        .await;

    match r {
        Ok((data, _, handle_success)) => {
            println!("updates: {:?}\n", data);
            println!("old + updates: {:?}\n", user.clone_with_updates(&data));

            handle_success().await;
        }
        Err((error, _, handle_failure)) => {
            match error {
                IvoUpdateError::NothingToUpdate => println!("Nothing to update\n"),
                IvoUpdateError::ValidationError(payload) => {
                    println!("Failed to update: {:?}\n", payload)
                }
            };

            handle_failure().await;
        }
    };

    println!(
        "{} {}\n",
        "\nUpdate duration:",
        format!("{:?}", timer.elapsed())
    );

    let timer = Instant::now();
    USER_MODEL.delete(user.clone(), UserCtxOptions::new()).await;

    println!(
        "{} {}\n",
        "\nDelete triggers:",
        format!("{:?}", timer.elapsed())
    );
}
