use chrono::{Days, Utc};
use std::time::Instant;

mod domain;
mod slugify;

use crate::domain::{PartialUserInput, User, UserCtxOptions, UserModel};

#[async_std::main]
async fn main() {
    let input = PartialUserInput::new()
        // .with_email(Some("1@1.com".into()))
        // .with_phone_number(Some("123 4567 8910".into()))
        // .with_slug_id("sloppy-slug-id".into())
        .with_username("user-10".into());

    let timer = Instant::now();

    let r = UserModel.create(input, UserCtxOptions::new()).await;

    println!("\nCreate duration: {:?}", timer.elapsed());

    match r {
        Ok(handle) => {
            println!("\n{:#?}\n", handle.data);

            handle.handle_success().await;
        }
        Err(handle) => {
            println!("\nFailed to create: {:#?}", handle.errors);

            handle.handle_failure().await;
        }
    };

    let (username, slug_id) = {
        let username = "John Doe";

        (username.into(), username.into())
    };

    let two_days_ago = Utc::now().checked_sub_days(Days::new(2)).unwrap();

    let user = User {
        id: 1,
        created_at: two_days_ago,
        updated_at: two_days_ago,
        email: Some("1@1.com".into()),
        phone_number: Some("123 4567 8910".into()),
        username,
        username_last_updated_at: None,
        slug_id,
    };

    println!("\n{:#?}", user);

    let updates = PartialUserInput::new()
        .with_email(user.email.clone())
        .with_phone_number(Some("123 4567 8910".into()))
        .with_slug_id("updated-slug-id: Lol".into())
        .with_username("new_username".into());

    let timer = Instant::now();

    let r = UserModel
        .update(user.clone(), updates, UserCtxOptions::new())
        .await;

    println!("\nUpdate duration: {:?}", timer.elapsed());

    let mut updated_user = None;

    match r {
        Ok(handle) => {
            let merged_data = user.clone_with_updates(&handle.data);

            println!("\nupdates: {:#?}", handle.data);
            println!("\nold + updates: {:#?}\n", merged_data);

            updated_user = Some(merged_data);

            handle.handle_success().await;
        }
        Err(handle) => {
            match handle.errors.as_ref() {
                Some(payload) => println!("\nFailed to update: {:#?}", payload),
                None => println!("\nNothing to update"),
            };

            handle.handle_failure().await;
        }
    };

    let Some(user) = updated_user else {
        return;
    };

    let updates = PartialUserInput::new()
        .with_email(user.email.clone())
        .with_phone_number(user.phone_number.clone())
        .with_slug_id("newly-updated-slug-id: Lol".into())
        .with_username(user.username.clone());

    let timer = Instant::now();

    let r = UserModel
        .update(user.clone(), updates, UserCtxOptions::new())
        .await;

    println!("\nUpdate duration: {:?}", timer.elapsed());

    match r {
        Ok(handle) => {
            println!("\nupdates: {:#?}", handle.data);
            println!(
                "\nold + updates: {:#?}\n",
                user.clone_with_updates(&handle.data)
            );

            handle.handle_success().await;
        }
        Err(handle) => {
            match handle.errors.as_ref() {
                Some(payload) => println!("\nFailed to update: {:#?}\n", payload),
                None => println!("\nNothing to update\n"),
            };

            handle.handle_failure().await;
        }
    };

    let timer = Instant::now();
    UserModel.delete(&user, UserCtxOptions::new());

    println!("\nDelete triggers: {:?}", timer.elapsed());
}
