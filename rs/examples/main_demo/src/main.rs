use chrono::{Days, Utc};
use ivo::IvoStruct;
use std::{sync::LazyLock, time::Instant};

mod domain;
mod slugify;

use crate::domain::{PartialUserInput, User, UserCtxOptions, USER_MODEL};

#[async_std::main]
async fn main() {
    LazyLock::force(&USER_MODEL);

    // // required error (email or phone_number)
    // let input = PartialUserInput::new().with_username("user-10".into());

    // // validation error (email, slug_id, username)
    // let input = PartialUserInput::new()
    //     .with_email(Some("1.com".into()))
    //     .with_phone_number(Some("123 4567 8910".into()))
    //     .with_slug_id("s".into())
    //     .with_username("u".into());

    // // re_validation error "username taken"
    // let input = PartialUserInput::new()
    //     .with_email(Some("1@1.com".into()))
    //     .with_username("user-1".into());

    // // post-validation error "slug taken"
    // let input = PartialUserInput::new()
    //     .with_email(Some("1@1.com".into()))
    //     .with_username("user-10".into())
    //     .with_slug_id("user-1".into());

    // // crate success: 2/4 inputs (a)
    // let input = PartialUserInput::new()
    //     .with_email(Some("1@1.com".into()))
    //     .with_username("user-10".into());

    // // crate success: 2/4 inputs (b)
    // let input = PartialUserInput::new()
    //     .with_phone_number(Some("123 4567 8910".into()))
    //     .with_username("user-10".into());

    // // crate success: 3/4 inputs
    // let input = PartialUserInput::new()
    //     .with_email(Some("1@1.com".into()))
    //     .with_phone_number(Some("123 4567 8910".into()))
    //     .with_username("user-10".into());

    // // crate success: 4/4 inputs
    // let input = PartialUserInput::new()
    //     .with_email(Some("1@1.com".into()))
    //     .with_phone_number(Some("123 4567 8910".into()))
    //     .with_username("user-10".into())
    //     .with_slug_id("sloppy-slug-id".into());

    // let timer = Instant::now();

    // let r = USER_MODEL.create(&input, UserCtxOptions::new()).await;

    // println!("\nCreate duration: {:?}", timer.elapsed());

    // match r {
    //     Ok((data, handle_success, _)) => {
    //         println!("\n{:#?}\n", data);

    //         handle_success().await;
    //     }
    //     Err((payload, handle_failure, _)) => {
    //         println!("\nFailed to create: {:#?}", payload);

    //         handle_failure().await;
    //     }
    // };

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

    // // required error (email or phone_number)
    // let updates = PartialUserInput::new()
    //     .with_email(None)
    //     .with_phone_number(None);

    // // validation error (email, slug_id, username)
    // let updates = PartialUserInput::new()
    //     .with_email(Some("1.com".into()))
    //     .with_phone_number(Some("123 4567 8910".into()))
    //     .with_slug_id("s".into())
    //     .with_username("u".into());

    // // re_validation error "username taken"
    // let updates = PartialUserInput::new()
    //     .with_username("user-1".into());

    // // post-validation error "slug taken"
    // let updates = PartialUserInput::new()
    //     .with_slug_id("user-1".into());

    // // nothing to update: 1/4 inputs (a)
    // let updates = PartialUserInput::new().with_email(user.email.clone());

    // // nothing to update: 1/4 inputs (b)
    // let updates = PartialUserInput::new().with_phone_number(user.phone_number.clone());

    // // nothing to update: 1/4 inputs (c)
    // let updates = PartialUserInput::new().with_slug_id(user.slug_id.to_string().clone());

    // // nothing to update: 1/4 inputs (d)
    // let updates = PartialUserInput::new().with_username(user.username.clone());

    // // nothing to update: 2/4 inputs
    // let updates = PartialUserInput::new()
    //     .with_email(user.email.clone())
    //     .with_phone_number(user.phone_number.clone());

    // // nothing to update: 3/4 inputs
    // let updates = PartialUserInput::new()
    //     .with_email(user.email.clone())
    //     .with_phone_number(user.phone_number.clone())
    //     .with_username(user.username.clone());

    // // nothing to update: 4/4 inputs
    let updates = PartialUserInput::new()
        .with_email(user.email.clone())
        .with_phone_number(user.phone_number.clone())
        .with_username(user.username.clone())
        .with_slug_id(user.slug_id.to_string().clone());

    // // update success: 1/4 inputs (a)
    // let updates = PartialUserInput::new().with_email(Some("1@2.com".into()));

    // // update success: 1/4 inputs (b)
    // let updates = PartialUserInput::new().with_phone_number(Some("123 4567 8911".into()));

    // // update success: 1/4 inputs (c)
    // let updates = PartialUserInput::new().with_slug_id("newly-updated-slug-id: Lol".into());

    // // update success: 1/4 inputs (d)
    // let updates = PartialUserInput::new().with_username("new_username".into());

    // // update success: 3/4 inputs
    // let updates = PartialUserInput::new()
    // .with_email(Some("1@1.com".into()))
    // .with_phone_number(Some("123 4567 8910".into()))
    // .with_username("new_username".into());

    // // update success: 4/4 inputs
    // let updates = PartialUserInput::new()
    //     .with_email(Some("1@1.com".into()))
    //     .with_phone_number(Some("123 4567 8910".into()))
    //     .with_username("new_username".into())
    //     .with_slug_id("newly-updated-slug-id: Lol".into());

    let timer = Instant::now();

    let r = USER_MODEL
        .update(&user, &updates, UserCtxOptions::new())
        .await;

    println!("\nUpdate duration: {:?}", timer.elapsed());

    match r {
        Ok((data, handle_success, _)) => {
            println!("\nupdates: {:#?}", data);
            println!("\nold + updates: {:#?}\n", user.clone_with_updates(&data));

            handle_success().await;
        }
        Err((error, handle_failure, _)) => {
            match error {
                Some(payload) => {
                    println!("\nFailed to update: {:#?}\n", payload)
                }
                _ => println!("\nNothing to update\n"),
            };

            handle_failure().await;
        }
    };

    //     let timer = Instant::now();
    //     USER_MODEL.delete(&user, UserCtxOptions::new()).await;

    //     println!("\nDelete triggers: {:?}", timer.elapsed());
}
