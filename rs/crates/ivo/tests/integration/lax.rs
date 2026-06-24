#![cfg(test)]

use ivo::{IvoField, IvoStruct, Schema, SharedData, SharedIvoContext, UpdateError};
use std::{future::ready, panic};

use crate::async_test_matrix;

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

async_test_matrix!(
    "[lax]: on_delete triggered with value: lax_string_value",
    should_trigger_on_delete_handlers
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

async_test_matrix!(
    "[lax]: on_failure triggered with value: fail_validation",
    should_trigger_on_failure_handlers_at_creation
);

async fn should_trigger_on_failure_handlers_at_creation_even_if_provided_and_ignored() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        lax2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        lax2: String,
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
                    .ignore_init()
                    .on_failure(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_failure triggered with value: {}",
                                ctx.input_values().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .set(
                "lax2",
                IvoField::LAX
                    .default("default_value".into())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let input = PartialDataInput {
        lax: Some("to be ignored".into()),
        lax2: Some("fail_validation".into()),
    };

    let r = model.create(&input, None).await;

    match r {
        Err((payload, handle_failure)) => {
            assert!(payload.get("lax").is_none());

            assert_eq!(
                payload.get("lax2").unwrap()[0].reason,
                "validation failed".to_string()
            );
            handle_failure().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[lax]: on_failure triggered with value: to be ignored",
    should_trigger_on_failure_handlers_at_creation_even_if_provided_and_ignored
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

async_test_matrix!(
    "[lax]: on_failure triggered with value: fail_validation",
    should_trigger_on_failure_handlers_during_updates
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

async_test_matrix!(
    "[lax]: on_failure triggered with value: some_value",
    should_trigger_on_failure_handlers_during_updates_with_unchanged_values
);

async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        lax2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        lax2: String,
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
                    .allow_update_if(|_, _| ready(false))
                    .on_failure(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_failure triggered with value: {}",
                                ctx.input_values().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .set(
                "lax2",
                IvoField::LAX
                    .default("default_value".into())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let data = Data {
        lax: "lax1".into(),
        lax2: "lax2".into(),
    };

    let input = PartialDataInput {
        lax: Some("update to be ignored".into()),
        lax2: Some("fail_validation".into()),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Err((e, handle_failure)) => {
            match e {
                UpdateError::ValidationError(payload) => {
                    assert!(payload.get("lax").is_none());

                    assert_eq!(
                        payload.get("lax2").unwrap()[0].reason,
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

async_test_matrix!(
    "[lax]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored
);

// ON_SUCCESS
async fn should_trigger_on_success_handlers_at_creation_if_provided() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        lax2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        lax2: String,
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
                    .allow_update_if(|_, _| ready(false))
                    .on_success(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_success triggered with value: {}",
                                ctx.input_values().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .set(
                "lax2",
                IvoField::LAX
                    .default("default_value".into())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let data = Data {
        lax2: "lax2".into(),
        lax: "lax1".into(),
    };

    let input = PartialDataInput {
        lax: Some(data.lax.clone()),
        lax2: Some(data.lax2.clone()),
    };

    let r = model.create(&input, None).await;

    match r {
        Ok((created, handle_success)) => {
            assert_eq!(created, data);

            handle_success().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[lax]: on_success triggered with value: lax",
    should_trigger_on_success_handlers_at_creation_if_provided
);

async fn should_trigger_on_success_handlers_at_creation_even_if_not_provided() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        lax2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        lax2: String,
    }

    let default_lax_value = "default_lax_value".to_string();

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.clone())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    })
                    .allow_update_if(|_, _| ready(false))
                    .on_success(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_success triggered with value: {}",
                                ctx.values().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .set(
                "lax2",
                IvoField::LAX
                    .default("default_lax2_value".into())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let lax2 = "lax2".to_string();

    let input = PartialDataInput {
        lax: None,
        lax2: Some(lax2.clone()),
    };

    let r = model.create(&input, None).await;

    match r {
        Ok((created, handle_success)) => {
            assert_eq!(
                created,
                Data {
                    lax2,
                    lax: default_lax_value,
                }
            );

            handle_success().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[lax]: on_success triggered with value: default_lax_value",
    should_trigger_on_success_handlers_at_creation_even_if_not_provided
);

async fn should_trigger_on_success_handlers_at_creation_even_if_provided_and_ignored() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        lax2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        lax2: String,
    }

    let default_lax_value = "default_lax_value".to_string();

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.clone())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    })
                    .ignore_init()
                    .on_success(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_success triggered with value: {}",
                                ctx.values().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .set(
                "lax2",
                IvoField::LAX
                    .default("default_lax2_value".into())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let lax_value = "lax_value".to_string();
    let lax2_value = "lax2_value".to_string();

    let input = PartialDataInput {
        lax2: Some(lax2_value.clone()),
        lax: Some(lax_value),
    };

    let r = model.create(&input, None).await;

    match r {
        Ok((created, handle_success)) => {
            assert_eq!(
                created,
                Data {
                    lax2: lax2_value,
                    lax: default_lax_value,
                }
            );

            handle_success().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[lax]: on_success triggered with value: default_lax_value",
    should_trigger_on_success_handlers_at_creation_even_if_provided_and_ignored
);

async fn should_trigger_on_success_handlers_during_updates_if_provided() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        lax2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        lax2: String,
    }

    let default_lax_value = "default_lax_value".to_string();

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.clone())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    })
                    .on_success(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_success triggered with value: {}",
                                ctx.values().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .set(
                "lax2",
                IvoField::LAX
                    .default("default_lax2_value".into())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let lax2 = "lax2".to_string();

    let data = Data {
        lax2: lax2.clone(),
        lax: default_lax_value,
    };

    let updated_lax_value = "updated_lax_value".to_string();

    let input = PartialDataInput {
        lax: Some(updated_lax_value.clone()),
        lax2: Some(lax2),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Ok((updated, handle_success)) => {
            assert_eq!(
                updated,
                PartialData {
                    lax2: None,
                    lax: Some(updated_lax_value),
                }
            );

            handle_success().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[lax]: on_success triggered with value: updated_lax_value",
    should_trigger_on_success_handlers_during_updates_if_provided
);

async fn should_not_trigger_on_success_handlers_during_updates_if_not_provided() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        lax2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        lax2: String,
    }

    let default_lax_value = "default_lax_value".to_string();

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.clone())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    })
                    .on_success(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_success triggered with value: {}",
                                ctx.values().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .set(
                "lax2",
                IvoField::LAX
                    .default("default_lax2_value".into())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let lax2 = "lax2".to_string();

    let data = Data {
        lax2: lax2.clone(),
        lax: default_lax_value,
    };

    let updated_lax2_value = "updated_lax2_value".to_string();

    let input = PartialDataInput {
        lax2: Some(updated_lax2_value.clone()),
        lax: None,
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Ok((updated, handle_success)) => {
            assert_eq!(
                updated,
                PartialData {
                    lax2: Some(updated_lax2_value),
                    lax: None,
                }
            );

            handle_success().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(should_not_trigger_on_success_handlers_during_updates_if_not_provided);

async fn should_not_trigger_on_success_handlers_during_updates_if_provided_and_ignored() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        lax2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        lax2: String,
    }

    let default_lax_value = "default_lax_value".to_string();

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.clone())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    })
                    .allow_update_if(|_, _| ready(false))
                    .on_success(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_success triggered with value: {}",
                                ctx.values().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .set(
                "lax2",
                IvoField::LAX
                    .default("default_lax2_value".into())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(v))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let lax2 = "lax2".to_string();

    let data = Data {
        lax2: lax2.clone(),
        lax: default_lax_value,
    };

    let updated_lax_value = "updated_lax_value".to_string();
    let updated_lax2_value = "updated_lax2_value".to_string();

    let input = PartialDataInput {
        lax2: Some(updated_lax2_value.clone()),
        lax: Some(updated_lax_value),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Ok((updated, handle_success)) => {
            assert_eq!(
                updated,
                PartialData {
                    lax2: Some(updated_lax2_value),
                    lax: None,
                }
            );

            handle_success().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(should_not_trigger_on_success_handlers_during_updates_if_provided_and_ignored);
