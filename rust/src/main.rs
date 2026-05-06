use ivo::{
    demo::{PartialUserInput, User, UserInput, UserModel},
    model::{CreateOutcome, UpdateOutcome},
};

fn main() {
    match UserModel.create(&UserInput {
        email: "dslfjlk".to_string(),
        username: "sdkjffk".to_string(),
    }) {
        CreateOutcome::Fail {
            error,
            handle_failure,
        } => {
            dbg!(error);
            handle_failure()
        }
        CreateOutcome::Success {
            data,
            handle_success,
        } => {
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
        UpdateOutcome::Fail {
            error,
            handle_failure,
        } => {
            dbg!(error);
            handle_failure()
        }
        UpdateOutcome::Success {
            data,
            handle_success,
        } => {
            println!("{:?}", data);
            handle_success()
        }
    };
}
