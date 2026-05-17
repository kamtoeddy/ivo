use ivo::{
    demo::{PartialUserInput, User, DEMO},
    schema::error::UpdateError,
};

#[tokio::main]
async fn main() {
    let user_model = DEMO::get_model();

    let r = user_model
        .create(&PartialUserInput {
            email: Some("1@1.com".to_string()),
            username: Some("john".to_string()),
        })
        .await;

    let _ = match r {
        Ok((data, handle_success)) => {
            println!("{:?}", data);
            handle_success
        }
        Err((payload, handle_failure)) => {
            println!("Error payload: {:?}", payload);
            handle_failure
        }
    };

    let r = user_model
        .update(
            &User {
                // created_at: DateWithTz::default(),
                email: "1@1.com".into(),
                // id: "id".into(),
                username: "john_doe".into(),
                // username_updated_at: None,
                // updated_at: None,
            },
            &PartialUserInput {
                email: Some("1@1.com".to_string()),
                username: Some("john".to_string()),
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
                    println!("Error payload: {:?}", payload)
                }
            };

            handle_failure
        }
    };
}
