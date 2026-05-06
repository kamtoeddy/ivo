use chrono::{DateTime, Utc};
use ivo::model::{CreateOutcome, HasPartial, Model, UpdateOutcome};
use partial_derive::MakePartial;
use serde::{Deserialize, Serialize};

type DateWithTz = DateTime<Utc>;

#[derive(Debug, Deserialize, Serialize, MakePartial)]
pub struct User {
    // created_at: DateWithTz,
    // id: String,
    email: String,
    username: String,
    // username_updated_at: Option<DateWithTz>,
    // updated_at: Option<DateWithTz>,
}

#[derive(Deserialize, Serialize, MakePartial)]
pub struct UserInput {
    pub email: String,
    pub username: String,
}

lazy_static::lazy_static! {
    static ref UserModel: Model<UserInput, User> = Model::<UserInput, User>::new();
}

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
