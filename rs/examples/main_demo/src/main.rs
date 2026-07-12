use chrono::{Days, Utc};
use ivo::IvoStruct;
use std::{sync::LazyLock, time::Instant};

mod domain;
mod slugify;

use crate::domain::{PartialUserInput, User, UserCtxOptions, USER_MODEL};

#[async_std::main]
async fn main() {
    LazyLock::force(&USER_MODEL);

    let input = PartialUserInput {
        email: None,
        // email: Some(Some("1@1.com".into())),
        username: Some("user-10".into()),
        phone_number: None,
        // phone_number: Some(Some("123 4567 8910".into())),
        slug_id: None,
        // slug_id: Some("sloppy-slug-id".into()),
    };

    let timer = Instant::now();

    let r = USER_MODEL.create(&input, UserCtxOptions::new()).await;

    println!("\nCreate duration: {:?}", timer.elapsed());

    match r {
        Ok((data, _, handle_success)) => {
            println!("\n{:#?}\n", data);

            handle_success().await;
        }
        Err((payload, _, handle_failure)) => {
            println!("\nFailed to create: {:#?}", payload);

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
        email: Some("1@1.com".into()),
        id: 1,
        username,
        username_last_updated_at: None,
        slug_id,
        phone_number: Some("123 4567 8910".into()),
        updated_at: two_days_ago,
    };

    println!("\n{:#?}", user);

    let updates = PartialUserInput {
        // email: None,
        email: Some(user.email.clone()),
        // phone_number: None,
        phone_number: Some(Some("123 4567 8910".into())),
        // username: None,
        username: Some("new_username".into()),
        slug_id: Some("updated-slug-id: Lol".into()),
        // slug_id: None,
    };

    let timer = Instant::now();

    let r = USER_MODEL
        .update(&user, &updates, UserCtxOptions::new())
        .await;

    println!("\nUpdate duration: {:?}", timer.elapsed());

    let mut updated_user = None;

    match r {
        Ok((data, _, handle_success)) => {
            let merged_data = user.clone_with_updates(&data);

            println!("\nupdates: {:#?}", data);
            println!("\nold + updates: {:#?}\n", merged_data);

            updated_user = Some(merged_data);

            handle_success().await;
        }
        Err((error, _, handle_failure)) => {
            match error {
                Some(payload) => {
                    println!("\nFailed to update: {:#?}", payload)
                }
                _ => println!("\nNothing to update"),
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
        // phone_number: None,
        phone_number: Some(user.phone_number.clone()),
        // username: None,
        username: Some(user.username.clone()),
        slug_id: Some("newly-updated-slug-id: Lol".into()),
        // slug_id: None,
    };

    let timer = Instant::now();

    let r = USER_MODEL
        .update(&user, &updates, UserCtxOptions::new())
        .await;

    println!("\nUpdate duration: {:?}", timer.elapsed());

    match r {
        Ok((data, _, handle_success)) => {
            println!("\nupdates: {:#?}", data);
            println!("\nold + updates: {:#?}\n", user.clone_with_updates(&data));

            handle_success().await;
        }
        Err((error, _, handle_failure)) => {
            match error {
                Some(payload) => {
                    println!("\nFailed to update: {:#?}\n", payload)
                }
                _ => println!("\nNothing to update\n"),
            };

            handle_failure().await;
        }
    };

    let timer = Instant::now();
    USER_MODEL.delete(&user, UserCtxOptions::new()).await;

    println!("\nDelete triggers: {:?}", timer.elapsed());
}
