use std::collections::HashMap;

// use chrono::{DateTime, Utc};
use partial_derive::MakePartial;
use serde::{Deserialize, Serialize};

use crate::schema::{Model, SchemaCore};

// type DateWithTz = DateTime<Utc>;

#[derive(Debug, Deserialize, Serialize, MakePartial)]
pub struct User {
    // pub created_at: DateWithTz,
    // pub id: String,
    pub email: String,
    pub username: String,
    // pub username_updated_at: Option<DateWithTz>,
    // pub updated_at: Option<DateWithTz>,
}

#[derive(Deserialize, Serialize, MakePartial)]
pub struct UserInput {
    pub email: String,
    pub username: String,
}

pub struct DEMO;

impl DEMO {
    pub fn get_model() -> Model<UserInput, User> {
        Model::<UserInput, User>::new(SchemaCore::new(HashMap::new(), None))
    }
}
