use chrono::{Days, Utc};
use ivo::{IvoStruct, IvoUpdateError};
use std::{sync::LazyLock, time::Instant};

mod domain;
mod slugify;

use crate::domain::{PartialUserInput, User, UserCtxOptions, UserRole, USER_MODEL};

#[async_std::main]
async fn main() {
    run_example().await;
}

async fn run_example() {
    LazyLock::force(&USER_MODEL);

    let input = PartialUserInput {
        // email: None,
        email: Some("1@1.com".into()),
        username: Some("user-10".into()),
        // role: None,
        role: Some(UserRole::Moderator),
        slug_id: None,
        // slug_id: Some("sloppy-slug-id".into()),
    };

    let timer = Instant::now();

    let r = USER_MODEL.create(&input, UserCtxOptions::new()).await;

    println!("\nCreate duration: {}", format!("{:?}", timer.elapsed()));

    match r {
        Ok((data, _, handle_success)) => {
            println!("\n{:?}\n", data);

            handle_success().await;
        }
        Err((payload, _, handle_failure)) => {
            println!("\nFailed to create: {:?}", payload);

            handle_failure().await;
        }
    };

    let (username, slug_id) = {
        let username = "John Doe";

        (username.into(), username.into())
    };

    let two_days_ago = Utc::now().checked_sub_days(Days::new(2)).unwrap();

    let user = User {
        created_at: two_days_ago,
        email: "1@1.com".into(),
        id: 1,
        username,
        username_last_updated_at: None,
        slug_id,
        role: UserRole::Admin,
        updated_at: two_days_ago,
    };

    println!("\n{:?}", user);

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

    let timer = Instant::now();

    let r = USER_MODEL
        .update(&user, &updates, UserCtxOptions::new())
        .await;

    println!("\nUpdate duration: {}", format!("{:?}", timer.elapsed()));

    let mut updated_user = None;

    match r {
        Ok((data, _, handle_success)) => {
            let merged_data = user.clone_with_updates(&data);

            println!("\nupdates: {:?}", data);
            println!("\nold + updates: {:?}\n", merged_data);

            updated_user = Some(merged_data);

            handle_success().await;
        }
        Err((error, _, handle_failure)) => {
            match error {
                IvoUpdateError::NothingToUpdate => println!("\nNothing to update"),
                IvoUpdateError::ValidationError(payload) => {
                    println!("\nFailed to update: {:?}", payload)
                }
            };

            handle_failure().await;
        }
    };

    let Some(user) = updated_user else {
        return;
    };

    let updates = PartialUserInput {
        // email: None,
        email: Some(user.email.clone()),
        // role: None,
        role: Some(user.role.clone()),
        // username: None,
        username: Some(user.username.clone()),
        slug_id: Some("newly-updated-slug-id: Lol".into()),
        // slug_id: None,
    };

    let timer = Instant::now();

    let r = USER_MODEL
        .update(&user, &updates, UserCtxOptions::new())
        .await;

    println!("\nUpdate duration: {}", format!("{:?}", timer.elapsed()));

    match r {
        Ok((data, _, handle_success)) => {
            println!("\nupdates: {:?}", data);
            println!("\nold + updates: {:?}\n", user.clone_with_updates(&data));

            handle_success().await;
        }
        Err((error, _, handle_failure)) => {
            match error {
                IvoUpdateError::NothingToUpdate => println!("\nNothing to update\n"),
                IvoUpdateError::ValidationError(payload) => {
                    println!("\nFailed to update: {:?}\n", payload)
                }
            };

            handle_failure().await;
        }
    };

    let timer = Instant::now();
    USER_MODEL.delete(user.clone(), UserCtxOptions::new()).await;

    println!("\nDelete triggers: {}", format!("{:?}", timer.elapsed()));
}
