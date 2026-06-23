#![cfg(test)]

use ivo::{IvoField, IvoStruct, Schema, SharedData, SharedIvoContext, UpdateError};
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

// LAX: ON_FAILURE
async fn should_trigger_on_failure_handlers_at_creation() {
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
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    })
                    .on_failure(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_failure triggered with value: {}",
                                ctx.input().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let input = PartialDataInput {
        lax: Some("fail_validation".into()),
    };

    let r = model.create(&input, None).await;

    match r {
        Err((payload, handle_failure)) => {
            assert_eq!(
                payload.get("lax").unwrap()[0].reason,
                "validation failed".to_string()
            );
            handle_failure().await;
        }
        _ => unreachable!(),
    }
}

test_matrix!(
    should_trigger_on_failure_handlers_at_creation,
    "[lax]: on_failure triggered with value: fail_validation",
    async { should_trigger_on_failure_handlers_at_creation().await }
);

async fn should_trigger_on_failure_handlers_during_updates() {
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
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    })
                    .on_failure(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_failure triggered with value: {}",
                                ctx.input().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let data = Data {
        lax: "some value".into(),
    };

    let input = PartialDataInput {
        lax: Some("fail_validation".into()),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Err((e, handle_failure)) => {
            match e {
                UpdateError::ValidationError(payload) => {
                    assert_eq!(
                        payload.get("lax").unwrap()[0].reason,
                        "validation failed".to_string()
                    );
                }
                _ => unreachable!(),
            }

            handle_failure().await;
        }
        _ => unreachable!(),
    }
}

test_matrix!(
    should_trigger_on_failure_handlers_during_updates,
    "[lax]: on_failure triggered with value: fail_validation",
    async { should_trigger_on_failure_handlers_during_updates().await }
);

async fn should_trigger_on_failure_handlers_during_updates_with_unchanged_values() {
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
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    })
                    .on_failure(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_failure triggered with value: {}",
                                ctx.input().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let lax_value = "some_value".to_string();

    let data = Data {
        lax: lax_value.clone(),
    };

    let input = PartialDataInput {
        lax: Some(lax_value),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Err((e, handle_failure)) => {
            match e {
                UpdateError::NothingToUpdate => {
                    assert!(!false)
                }
                _ => unreachable!(),
            }

            handle_failure().await;
        }
        _ => unreachable!(),
    }
}

test_matrix!(
    should_trigger_on_failure_handlers_during_updates_with_unchanged_values,
    "[lax]: on_failure triggered with value: some_value",
    async { should_trigger_on_failure_handlers_during_updates_with_unchanged_values().await }
);
