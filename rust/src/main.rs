use std::time::Instant;

use ivo::{
    demo::{PartialUserInput, User, UserRole, DEMO},
    schema::error::UpdateError,
};

#[tokio::main]
async fn main() {
    let user_schema = DEMO::get_schema();
    let user_model = user_schema.get_model();

    println!("UserSchema props: {:?}\n", user_schema.props);

    let timer = Instant::now();

    let r = user_model
        .create(&PartialUserInput {
            email: Some("1@1.com".to_string()),
            username: Some("john".to_string()),
            role: None,
            is_admin: None,
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

    let r = user_model
        .update(
            &User {
                // created_at: DateWithTz::default(),
                email: "1@1.com".into(),
                // id: "id".into(),
                username: "john_doe".into(),
                role: UserRole::User,
                // username_updated_at: None,
                // updated_at: None,
            },
            &PartialUserInput {
                email: Some("1@1.com".to_string()),
                username: Some("john".to_string()),
                role: Some(UserRole::Admin),
                is_admin: Some(None),
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
