use chrono::{DateTime, Utc};
use ivo::ivo_schema;

type Timestamp = DateTime<Utc>;

#[async_std::main]
async fn main() {
    println!("\nTIMESTAMP FIELDS WITH CUSTOM NAMES\n");

    should_properly_create_and_update().await;

    println!("\nTIMESTAMP FIELDS WITH CUSTOM NAMES AND OPTIONAL UPDATED_AT\n");

    should_properly_create_and_update_with_optional_updated_at().await;
}

async fn should_properly_create_and_update() {
    let username = "john-doe".to_string();
    let datetime_before = Utc::now();

    let created = DataModel
        .create(
            PartialDataInput {
                username: Some(username.clone()),
            },
            (),
        )
        .unwrap();

    println!("\ncreated: {:#?}", created.data);

    assert_eq!(created.data.username, username);
    assert_eq!(
        created.data.custom_created_at,
        created.data.custom_updated_at
    );
    assert!(created.data.custom_created_at > datetime_before);
    assert!(created.data.custom_updated_at > datetime_before);

    let username = "jane-doe".to_string();

    let updated = DataModel
        .update(
            created.data.clone(),
            PartialDataInput {
                username: Some(username.clone()),
            },
            (),
        )
        .unwrap();

    println!("\nupdates: {:#?}", updated.data);

    assert_eq!(updated.data.username, Some(username));
    assert!(updated.data.custom_created_at.is_none());
    assert!(updated.data.custom_updated_at.is_some());
}

async fn should_properly_create_and_update_with_optional_updated_at() {
    let username = "john-doe".to_string();
    let datetime_before = Utc::now();

    let created = DataWithOptionalUpdatedAtModel
        .create(
            OptionalUpdatedAtPartialDataInput {
                username: Some(username.clone()),
            },
            (),
        )
        .unwrap();

    println!("\ncreated: {:#?}", created.data);

    assert_eq!(created.data.username, username);
    assert!(created.data.custom_created_at > datetime_before);
    assert!(created.data.custom_updated_at.is_none());

    let username = "jane-doe".to_string();

    let updated = DataWithOptionalUpdatedAtModel
        .update(
            created.data.clone(),
            OptionalUpdatedAtPartialDataInput {
                username: Some(username.clone()),
            },
            (),
        )
        .unwrap();

    println!("\nupdates: {:#?}", updated.data);

    assert_eq!(updated.data.username, Some(username));
    assert!(updated.data.custom_created_at.is_none());
    assert!(updated.data.custom_updated_at.unwrap().is_some());
}

pub use custom_name_schema::{Data, DataModel, PartialData, PartialDataInput};
pub use optional_updated_at_schema::{
    DataWithOptionalUpdatedAt, DataWithOptionalUpdatedAtModel,
    PartialDataInput as OptionalUpdatedAtPartialDataInput, PartialDataWithOptionalUpdatedAt,
};

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod custom_name_schema {
    use super::Timestamp;

    struct Fields {
        #[lax("default-value".into())]
        #[validate(|_, _, _| Ok(None))]
        pub username: String,

        #[created_at]
        pub custom_created_at: Timestamp,

        #[updated_at]
        pub custom_updated_at: Timestamp,
    }

    #[timestamps(chrono::Utc::now)]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(DataWithOptionalUpdatedAt, derive(Debug, Clone, PartialEq))
)]
mod optional_updated_at_schema {
    use super::Timestamp;

    struct Fields {
        #[lax("default-value".into())]
        #[validate(|_, _, _| Ok(None))]
        pub username: String,

        #[created_at]
        pub custom_created_at: Timestamp,

        #[optional_updated_at]
        pub custom_updated_at: Option<Timestamp>,
    }

    #[timestamps(chrono::Utc::now)]
    const _: () = ();
}
