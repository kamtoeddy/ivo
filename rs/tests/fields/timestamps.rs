use std::time::Instant;

use ivo::{IvoField, IvoInputStruct, IvoStruct, IvoModel};

use crate::async_test_matrix;

async fn should_respect_created_at_timestamp_with_default_name() {
    type Timestamp = u128;

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        created_at: Timestamp,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let timer = Instant::now();

    let model: IvoModel<DataInput, Data, Option<()>, Timestamp> = IvoModel::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234)).timestamps(|t| {
                t.resolve(move || timer.elapsed().as_micros())
                    .created_at(None)
            })
        },
        |o| o,
    );

    let lax = 400;

    let (data, _, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data.lax, lax);
    assert!(data.created_at > 0);

    let lax_update = 200;

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax_update),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(updates.lax, Some(lax_update));
    assert_eq!(updates.created_at, None);
}

async_test_matrix!(should_respect_created_at_timestamp_with_default_name);

async fn should_respect_created_at_timestamp_with_custom_name() {
    type Timestamp = u128;

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        custom_created_at: Timestamp,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let timer = Instant::now();

    let model: IvoModel<DataInput, Data, Option<()>, Timestamp> = IvoModel::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234)).timestamps(|t| {
                t.resolve(move || timer.elapsed().as_micros())
                    .created_at(Some("custom_created_at"))
            })
        },
        |o| o,
    );

    let lax = 400;

    let (data, _, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data.lax, lax);
    assert!(data.custom_created_at > 0);

    let lax_update = 200;

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax_update),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(updates.lax, Some(lax_update));
    assert_eq!(updates.custom_created_at, None);
}

async_test_matrix!(should_respect_created_at_timestamp_with_custom_name);

async fn should_respect_updated_at_timestamp_with_default_name() {
    type Timestamp = u128;

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        updated_at: Timestamp,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let timer = Instant::now();

    let model: IvoModel<DataInput, Data, Option<()>, Timestamp> = IvoModel::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234)).timestamps(|t| {
                t.resolve(move || timer.elapsed().as_micros())
                    .updated_at(None)
            })
        },
        |o| o,
    );

    let lax = 400;

    let (data, _, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data.lax, lax);
    assert!(data.updated_at > 0);

    let lax_update = 200;

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax_update),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(updates.lax, Some(lax_update));
    assert!(updates.updated_at.is_some());
}

async_test_matrix!(should_respect_updated_at_timestamp_with_default_name);

async fn should_respect_updated_at_timestamp_with_custom_name() {
    type Timestamp = u128;

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        custom_updated_at: Timestamp,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }
    let timer = Instant::now();

    let model: IvoModel<DataInput, Data, Option<()>, Timestamp> = IvoModel::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234)).timestamps(|t| {
                t.resolve(move || timer.elapsed().as_micros())
                    .updated_at(Some("custom_updated_at"))
            })
        },
        |o| o,
    );

    let lax = 400;

    let (data, _, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data.lax, lax);
    assert!(data.custom_updated_at > 0);

    let lax_update = 200;

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax_update),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(updates.lax, Some(lax_update));
    assert!(updates.custom_updated_at.is_some());
}

async_test_matrix!(should_respect_updated_at_timestamp_with_custom_name);

async fn should_respect_optional_updated_at_timestamp_with_default_name() {
    type Timestamp = u128;

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        updated_at: Option<Timestamp>,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let timer = Instant::now();

    let model: IvoModel<DataInput, Data, Option<()>, Timestamp> = IvoModel::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234)).timestamps(|t| {
                t.resolve(move || timer.elapsed().as_micros())
                    .optional_updated_at(None)
            })
        },
        |o| o,
    );

    let lax = 400;

    let (data, _, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data.lax, lax);
    assert_eq!(data.updated_at, None);

    let lax_update = 200;

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax_update),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(updates.lax, Some(lax_update));
    assert!(updates.updated_at.is_some());

    let updated = data.clone_with_updates(&updates);

    assert_eq!(updated.lax, updates.lax.unwrap());
    assert_eq!(updated.updated_at, updates.updated_at.unwrap());
}

async_test_matrix!(should_respect_optional_updated_at_timestamp_with_default_name);

async fn should_respect_optional_updated_at_timestamp_with_custom_name() {
    type Timestamp = u128;

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        custom_updated_at: Option<Timestamp>,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let timer = Instant::now();

    let model: IvoModel<DataInput, Data, Option<()>, Timestamp> = IvoModel::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234)).timestamps(|t| {
                t.resolve(move || timer.elapsed().as_micros())
                    .optional_updated_at(Some("custom_updated_at"))
            })
        },
        |o| o,
    );

    let lax = 400;

    let (data, _, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data.lax, lax);
    assert_eq!(data.custom_updated_at, None);

    let lax_update = 200;

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax_update),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(updates.lax, Some(lax_update));
    assert!(updates.custom_updated_at.is_some());

    let updated = data.clone_with_updates(&updates);

    assert_eq!(updated.lax, updates.lax.unwrap());
    assert_eq!(
        updated.custom_updated_at,
        updates.custom_updated_at.unwrap()
    );
}

async_test_matrix!(should_respect_optional_updated_at_timestamp_with_custom_name);
