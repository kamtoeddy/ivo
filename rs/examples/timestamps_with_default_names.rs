use std::{future::ready, sync::LazyLock};

use chrono::{DateTime, Utc};
use ivo::{IvoField, IvoInputStruct, IvoStruct, Model, Schema};

#[async_std::main]
async fn main() {
    println!("\nTIMESTAMP FIELDS WITH DEFAULT NAMES\n");

    should_properly_create_and_update().await;

    println!("\nTIMESTAMP FIELDS WITH DEFAULT NAMES AND OPTIONAL UPDATED_AT\n");

    should_properly_create_and_update_with_optional_updated_at().await;
}

async fn should_properly_create_and_update() {
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
    assert_eq!(data.created_at, data.updated_at);
    assert!(data.created_at > datetime_before);
    assert!(data.updated_at > datetime_before);

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
    assert!(updates.created_at.is_none());
    assert!(updates.updated_at.is_some());
}

async fn should_properly_create_and_update_with_optional_updated_at() {
    let username = "john-doe".to_string();
    let datetime_before = Utc::now();

    let (data, _, _) = DATA_MODEL_WITH_OPTIONAL_UPDATED_AT
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
    assert!(data.created_at > datetime_before);
    assert!(data.updated_at.is_none());

    let username = "jane-doe".to_string();

    let (updates, _, _) = DATA_MODEL_WITH_OPTIONAL_UPDATED_AT
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
    assert!(updates.created_at.is_none());
    assert!(updates.updated_at.unwrap().is_some());
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    username: String,
}

type Timestamp = DateTime<Utc>;

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    created_at: Timestamp,
    username: String,
    updated_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct DataWithOptionalUpdatedAt {
    created_at: Timestamp,
    username: String,
    updated_at: Option<Timestamp>,
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
                .timestamps(|t| t.resolve(Utc::now).created_at(None).updated_at(None))
            },
            |o| o,
        )
    });

pub static DATA_MODEL_WITH_OPTIONAL_UPDATED_AT: LazyLock<
    Model<DataInput, DataWithOptionalUpdatedAt, Option<()>, Timestamp>,
> = LazyLock::new(|| DATA_SCHEMA_WITH_OPTIONAL_UPDATED_AT.model());

pub static DATA_SCHEMA_WITH_OPTIONAL_UPDATED_AT: LazyLock<
    Schema<DataInput, DataWithOptionalUpdatedAt, Option<()>, Timestamp>,
> = LazyLock::new(|| {
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
                    .created_at(None)
                    .optional_updated_at(None)
            })
        },
        |o| o,
    )
});
