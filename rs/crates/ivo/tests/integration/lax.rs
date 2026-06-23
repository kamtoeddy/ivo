#![cfg(test)]

use ivo::{IvoField, IvoStruct, Schema, SharedData};
use std::future::ready;

use crate::test_matrix;

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

test_matrix!(should_not_create_if_primary_validation_fails, async {
    should_not_create_if_primary_validation_fails().await
});

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

test_matrix!(should_create_properly, async {
    should_create_properly().await
});

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

test_matrix!(should_update_properly, async {
    should_update_properly().await
});

// LAX: ON_DELETE
async fn should_trigger_on_delete_handlers() {
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
                    .validate(|v: String, _, _| ready(Ok(v)))
                    .on_delete(|data: SharedData<Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_delete triggered with value: {}",
                                data.lax.as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    model
        .delete(
            Data {
                lax: String::from("lax_string_value"),
            },
            None,
        )
        .await;
}

test_matrix!(
    should_trigger_on_delete_handlers,
    "[lax]: on_delete triggered with value: lax_string_value",
    async { should_trigger_on_delete_handlers().await }
);
