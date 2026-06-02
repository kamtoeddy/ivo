use std::time::Instant;

use crate::{
    demo::{
        example::{PartialUserInput, User, UserRole, DEMO},
        slugify::slugify,
    },
    schema::error::UpdateError,
};

pub async fn run_example() {
    let user_schema = DEMO::get_schema();
    let user_model = user_schema.get_model();

    println!("UserSchema props: {:?}\n", user_schema.props);

    let timer = Instant::now();

    let r = user_model
        .create(&PartialUserInput {
            email: Some("1@1.com".to_string()),
            username: Some("john".to_string()),
            role: None,
        })
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

    println!("Create duration {:?}\n", timer.elapsed());

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
        role: UserRole::User,
        // username_updated_at: None,
        // updated_at: None,
    };

    println!("{:?}", user);

    let r = user_model
        .update(
            &user,
            &PartialUserInput {
                email: Some("1@1.com".to_string()),
                username: Some("john".to_string()),
                role: Some(UserRole::Admin),
            },
        )
        .await;

    let _ = match r {
        Ok((data, handle_success)) => {
            println!("{:?}", data);
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

    println!("Update duration {:?}", timer.elapsed());
}
