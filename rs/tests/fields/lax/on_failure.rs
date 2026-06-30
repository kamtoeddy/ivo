use std::future::ready;

use ivo::{IvoField, IvoStruct, Schema, SharedIvoContext, UpdateError};

use crate::async_test_matrix;

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

                        ready(Ok(Some(v)))
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

    let model = schema.model();

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

                        ready(Ok(Some(v)))
                    })
                    .ignore_init()
                    .on_failure(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_failure triggered with value: {}",
                                ctx.raw_input().lax.unwrap().as_str()
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

                        ready(Ok(Some(v)))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

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

                        ready(Ok(Some(v)))
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

    let model = schema.model();

    let data = Data {
        lax: "some value".into(),
    };

    let input = PartialDataInput {
        lax: Some("fail_validation".into()),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Err((UpdateError::ValidationError(payload), handle_failure)) => {
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

                        ready(Ok(Some(v)))
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

    let model = schema.model();

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

                        ready(Ok(Some(v)))
                    })
                    .ignore_update()
                    .on_failure(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_failure triggered with value: {}",
                                ctx.raw_input().lax.unwrap().as_str()
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

                        ready(Ok(Some(v)))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

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
        Err((UpdateError::ValidationError(payload), handle_failure)) => {
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
    "[lax]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored
);
