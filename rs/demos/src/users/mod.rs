use ivo::{UpdateError, erase_value, parse_or_panic, types::WithUpdateDetails};
use std::{collections::HashMap, mem, time::Instant};

mod domain;

use crate::{
    users::domain::{PartialUserInput, USER_MODEL, User, UserCtxOptions, UserRole},
    utils::{format_bytes, styled_text::Stylable},
};

pub async fn run_users_demo() {
    let timer = Instant::now();

    let input = PartialUserInput {
        email: Some("1@1.com".into()),
        username: Some("john".into()),
        role: Some(UserRole::Moderator),
        slug_id: None,
    };

    let r = USER_MODEL.create(&input, UserCtxOptions::new()).await;

    println!("size:  {}", format_bytes(&mem::size_of_val(&r)));

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

        (username.into(), username.into())
    };

    let user = User {
        email: "1@1.com".into(),
        id: 1,
        username,
        username_updated_at: None,
        slug_id,
        role: UserRole::Admin,
    };

    println!("{:?}\n", user);

    let updates = PartialUserInput {
        // email: None,
        email: Some(user.email.clone()),
        // role: None,
        role: Some(UserRole::User),
        // username: Some("new_username".into()),
        username: None,
        slug_id: None,
    };

    let r = USER_MODEL
        .update(&user, &updates, UserCtxOptions::new())
        .await;

    match r {
        Ok((data, _handle_success)) => {
            println!("updates: {:?}\n", data);
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

    let mut map = HashMap::new();
    map.insert("k", Some(erase_value(1)));

    // let l = map.get("k") ;
    if let Some(Some(v)) = map.get("k") {
        println!("k = {}", parse_or_panic::<i32>(&v))
    }
}
