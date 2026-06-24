#![cfg(test)]

use ivo::{IvoField, IvoStruct, Schema};
use std::{future::ready, panic};

use crate::async_test_matrix;

mod on_delete;
mod on_failure;
mod on_success;

// TODO:
// [ ] default
// [ ] default_fn
// [ ] allow_init_if
// [ ] allow_update_if
// [ ] ignore_if
// [ ] ignore_init
// [ ] readonly
// [ ] required_if
// [x] validate
// [ ] re_validate
// [ ] post_validate
// [x] on_delete
// [x] on_failure
// [x] on_success

async fn should_not_create_if_primary_validation_fails() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
    }

    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "lax",
                IvoField::LAX
                    .default("default_value".into())
                    .validate(|v: String, _, _| {
                        let validated = v.trim();

                        if validated.len() < 2 {
                            return ready(Err((MIN_LENGTH_ERROR.into(), None)));
                        }

                        ready(Ok(v))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let lax_values = [
        String::from(" "),
        String::from(" 1"),
        String::from("1"),
        String::from(" 1   "),
    ];

    for lax_value in lax_values {
        let r = model
            .create(
                &PartialDataInput {
                    lax: Some(lax_value),
                },
                None,
            )
            .await;

        match r {
            Err((p, _)) => {
                assert_eq!(p.get("lax").unwrap()[0].reason, MIN_LENGTH_ERROR);
            }
            _ => unreachable!(),
        }
    }
}

async_test_matrix!(should_not_create_if_primary_validation_fails);

async fn should_create_properly() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "lax",
                IvoField::LAX
                    .default("default_value".into())
                    .validate(|v: String, _, _| ready(Ok(v))),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let lax_value = String::from("value");

    let r = model
        .create(
            &PartialDataInput {
                lax: Some(lax_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Ok((d, _)) => {
            assert_eq!(d, Data { lax: lax_value })
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(should_create_properly);

async fn should_update_properly() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        lax: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1)))
                .set(
                    "lax",
                    IvoField::LAX
                        .default("default_value".into())
                        .validate(|v: String, _, _| ready(Ok(v))),
                )
        },
        |o| o,
    );

    let model = schema.get_model();

    let data = Data {
        id: 1,
        lax: String::from("value"),
    };

    let updated_value = String::from("updated value");

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(updated_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Ok((d, _)) => {
            assert_eq!(
                d,
                PartialData {
                    id: None,
                    lax: Some(updated_value.clone()),
                }
            )
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(should_update_properly);
