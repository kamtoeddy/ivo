use std::future::ready;

use ivo::{IvoField, IvoStruct, IvoUpdateData, Schema, UpdateError};

use crate::async_test_matrix;

// ignore

async fn should_respect_the_ignore_update_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        required: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        required: i32,
    }

    const IGNORE_REQUIRED_FOR_UPDATE: &str = "ignore_required_for_update";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "required",
                IvoField::REQUIRED
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .ignore_update(|(_, values): IvoUpdateData<DataInput, Data>, _| {
                        if IGNORE_REQUIRED_FOR_UPDATE == values.lax {
                            return ready(true);
                        }

                        ready(false)
                    }),
            )
            .set(
                "lax",
                IvoField::LAX.default("default_lax_value".to_string()),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let lax = IGNORE_REQUIRED_FOR_UPDATE.to_string();
    let required = 1;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                required: Some(required),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data { lax, required },
        "should not evaluate the ignore_update rule of required fields at creation"
    );

    let required = required + 2;

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                required: Some(required),
            },
            None,
        )
        .await;

    match r {
        Err((UpdateError::NothingToUpdate, _, _)) => {}
        _ => unreachable!("expected nothig to update error"),
    }

    let data = Data {
        lax: "normal_lax_value".into(),
        ..data
    };

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                required: Some(required),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    lax: None,
                    required: Some(required)
                }
            );
        }
        _ => unreachable!("expected update to be successful"),
    }
}

async_test_matrix!(should_respect_the_ignore_update_rule);

async fn should_respect_the_readonly_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        required: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        required: i32,
    }

    const IGNORE_REQUIRED_FOR_UPDATE: &str = "ignore_required_for_update";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "required",
                IvoField::REQUIRED
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .readonly(),
            )
            .set(
                "lax",
                IvoField::LAX.default("default_lax_value".to_string()),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let lax = IGNORE_REQUIRED_FOR_UPDATE.to_string();
    let required = 1;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                required: Some(required),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data { lax, required },
        "should not evaluate the ignore_update rule of required fields at creation"
    );

    let required = required + 2;

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                required: Some(required),
            },
            None,
        )
        .await;

    match r {
        Err((UpdateError::NothingToUpdate, _, _)) => {}
        _ => unreachable!("expected nothig to update error"),
    }

    let data = Data {
        lax: "normal_lax_value".into(),
        ..data
    };

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                required: Some(required),
            },
            None,
        )
        .await;

    match r {
        Err((UpdateError::NothingToUpdate, _, _)) => {}
        _ => unreachable!("expected nothig to update error"),
    }
}

async_test_matrix!(should_respect_the_readonly_rule);
