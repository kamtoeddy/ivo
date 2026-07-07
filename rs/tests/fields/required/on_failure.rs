use std::future::ready;

use ivo::{IvoContext, IvoField, IvoStruct, IvoUpdateError, Schema};

use crate::async_test_matrix;

async fn should_trigger_on_failure_handlers_at_creation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        required: String,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "required",
                IvoField::REQUIRED
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[required]: on_failure triggered with value: {}",
                                ctx.input().required.unwrap().as_str()
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
        required: Some("fail_validation".into()),
    };

    let r = model.create(&input, None).await;

    match r {
        Err((payload, _, handle_failure)) => {
            assert_eq!(
                payload.get("required").unwrap()[0].reason,
                "validation failed".to_string()
            );
            handle_failure().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[required]: on_failure triggered with value: fail_validation",
    should_trigger_on_failure_handlers_at_creation
);

async fn should_trigger_on_failure_handlers_during_updates() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        required: String,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "required",
                IvoField::REQUIRED
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[required]: on_failure triggered with value: {}",
                                ctx.input().required.unwrap().as_str()
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
        required: "some value".into(),
    };

    let input = PartialDataInput {
        required: Some("fail_validation".into()),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Err((IvoUpdateError::ValidationError(payload), _, handle_failure)) => {
            assert_eq!(
                payload.get("required").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[required]: on_failure triggered with value: fail_validation",
    should_trigger_on_failure_handlers_during_updates
);

async fn should_trigger_on_failure_handlers_during_updates_with_unchanged_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        required: String,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "required",
                IvoField::REQUIRED
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[required]: on_failure triggered with value: {}",
                                ctx.input().required.unwrap().as_str()
                            );
                        }

                        ready(())
                    })
                    .on_failure(|_, _| ready(())),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let required_value = "some_value".to_string();

    let data = Data {
        required: required_value.clone(),
    };

    let input = PartialDataInput {
        required: Some(required_value),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Err((e, _, handle_failure)) => {
            match e {
                IvoUpdateError::NothingToUpdate => {
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
    "[required]: on_failure triggered with value: some_value",
    should_trigger_on_failure_handlers_during_updates_with_unchanged_values
);

async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: String,
        required2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        required: String,
        required2: String,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "required",
                IvoField::REQUIRED
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_update(|_, _, _| ready(true))
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[required]: on_failure triggered with value: {}",
                                ctx.raw_input().required.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .set(
                "required2",
                IvoField::REQUIRED.validate(|v: String, _, _| {
                    if v == "fail_validation" {
                        return ready(Err(("validation failed".into(), None)));
                    }

                    ready(Ok(None))
                }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let data = Data {
        required: "required1".into(),
        required2: "required2".into(),
    };

    let input = PartialDataInput {
        required: Some("update to be ignored".into()),
        required2: Some("fail_validation".into()),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Err((IvoUpdateError::ValidationError(payload), _, handle_failure)) => {
            assert!(payload.get("required").is_none());

            assert_eq!(
                payload.get("required2").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[required]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored
);

async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_as_readonly(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: String,
        required2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        required: String,
        required2: String,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "required",
                IvoField::REQUIRED
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .readonly()
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[required]: on_failure triggered with value: {} as readonly",
                                ctx.raw_input().required.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .set(
                "required2",
                IvoField::REQUIRED.validate(|v: String, _, _| {
                    if v == "fail_validation" {
                        return ready(Err(("validation failed".into(), None)));
                    }

                    ready(Ok(None))
                }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let data = Data {
        required: "required1".into(),
        required2: "required2".into(),
    };

    let input = PartialDataInput {
        required: Some("update to be ignored".into()),
        required2: Some("fail_validation".into()),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Err((IvoUpdateError::ValidationError(payload), _, handle_failure)) => {
            assert!(payload.get("required").is_none());

            assert_eq!(
                payload.get("required2").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[required]: on_failure triggered with value: update to be ignored as readonly",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_as_readonly
);
