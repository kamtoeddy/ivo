use ivo::{UpdateError, erase_value, parse_or_panic, types::IvoStructMethods};
use std::{collections::HashMap, mem, sync::LazyLock, time::Instant};

mod domain;

use crate::{
    users::domain::{PartialUserInput, USER_MODEL, User, UserCtxOptions, UserRole},
    utils::{format_bytes, styled_text::Stylable},
};

pub async fn run_users_demo() {
    // println!("runner\n\n");
    LazyLock::force(&USER_MODEL);

    let timer = Instant::now();

    let input = PartialUserInput {
        // email: None,
        email: Some("1@1.com".into()),
        username: Some("user-10".into()),
        role: None,
        // role: Some(UserRole::Moderator),
        // slug_id: None,
        slug_id: Some("sloppy-slug-id".into()),
    };

    // println!("runner2\n\n");
    let r = USER_MODEL.create(&input, UserCtxOptions::new()).await;

    println!("size:  {}", format_bytes(&mem::size_of_val(&r)));

    match r {
        Ok((data, handle_success)) => {
            println!("{:?}\n", data);

            handle_success().await;
        }
        Err((payload, handle_failure)) => {
            println!("\nFailed to create: {:?}\n", payload);

            handle_failure().await;
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
        Ok((data, handle_success)) => {
            println!("updates: {:?}\n", data);
            println!(
                "old + updates: {:?}\n",
                user.ivo_internal_clone_with_ref(&data)
            );

            handle_success().await;
        }
        Err((error, handle_failure)) => {
            match error {
                UpdateError::NothingToUpdate => println!("Nothing to update\n"),
                UpdateError::ValidationError(payload) => {
                    println!("Failed to update: {:?}\n", payload)
                }
            };

            handle_failure().await;
        }
    };

    println!(
        "{} {}\n",
        "\nUpdate duration:".font_bold(),
        format!("{:?}", timer.elapsed()).colored_blue()
    );

    let timer = Instant::now();
    USER_MODEL.delete(user.clone(), UserCtxOptions::new()).await;

    println!(
        "{} {}\n",
        "\nDelete triggers:".font_bold(),
        format!("{:?}", timer.elapsed()).colored_blue()
    );

    let mut map = HashMap::new();
    map.insert("k", Some(erase_value(1)));

    // let l = map.get("k") ;
    if let Some(Some(v)) = map.get("k") {
        println!("k = {}", parse_or_panic::<i32>(v, Some("k")))
    }
}
