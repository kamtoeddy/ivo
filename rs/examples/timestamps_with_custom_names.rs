use std::{future::ready, sync::LazyLock};

use chrono::{DateTime, Utc};
use ivo::{IvoField, IvoInputStruct, IvoStruct, Model, Schema};

#[async_std::main]
async fn main() {
    let username = "john-doe".to_string();
    let datetime_before = Utc::now();

    let (data, _, _) = DATA_MODEL
        .create(
            &PartialDataInput {
                username: Some(username.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", data);

    assert_eq!(data.username, username);
    assert_eq!(data.custom_created_at, data.custom_updated_at);
    assert!(data.custom_created_at > datetime_before);
    assert!(data.custom_updated_at > datetime_before);

    let username = "jane-doe".to_string();

    let (updates, _, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                username: Some(username.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    println!("\nupdates: {:#?}", updates);

    assert_eq!(updates.username, Some(username));
    assert!(updates.custom_created_at.is_none());
    assert!(updates.custom_updated_at.is_some());
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    username: String,
}

type Timestamp = DateTime<Utc>;

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    custom_created_at: Timestamp,
    username: String,
    custom_updated_at: Timestamp,
}

pub static DATA_MODEL: LazyLock<Model<DataInput, Data, Option<()>, Timestamp>> =
    LazyLock::new(|| DATA_SCHEMA.model());

pub static DATA_SCHEMA: LazyLock<Schema<DataInput, Data, Option<()>, Timestamp>> =
    LazyLock::new(|| {
        Schema::new(
            |f| {
                f.field(
                    "username",
                    IvoField::LAX
                        .default("default-value".into())
                        .validate(|_, _, _| ready(Ok(None::<String>))),
                )
                .timestamps(|t| {
                    t.resolve(Utc::now)
                        .created_at(Some("custom_created_at"))
                        .updated_at(Some("custom_updated_at"))
                })
            },
            |o| o,
        )
    });
