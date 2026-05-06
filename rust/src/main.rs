use ivo::demo::{PartialUserInput, User, UserInput, UserModel};

fn main() {
    match UserModel.create(&UserInput {
        email: "dslfjlk".to_string(),
        username: "sdkjffk".to_string(),
    }) {
        Err((payload, handle_failure)) => {
            println!("Error payload: {:?}", payload);
            handle_failure()
        }
        Ok((data, handle_success)) => {
            println!("{:?}", data);
            handle_success()
        }
    };

    match UserModel.update(
        &User {
            // created_at: DateWithTz::default(),
            email: "".into(),
            // id: "id".into(),
            username: "u".into(),
            // username_updated_at: None,
            // updated_at: None,
        },
        &PartialUserInput {
            email: Some("dslfjlk".to_string()),
            username: Some("sdkjffk".to_string()),
        },
    ) {
        Err((error, handle_failure)) => {
            match error {
                ivo::error::UpdateError::NothingToUpdate => println!("Nothing to update"),
                ivo::error::UpdateError::ValidationError(payload) => {
                    println!("Error payload: {:?}", payload)
                }
            };

            handle_failure()
        }
        Ok((data, handle_success)) => {
            println!("{:?}", data);
            handle_success()
        }
    };
}
