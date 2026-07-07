use std::future::ready;

use ivo::{IvoContext, IvoField, IvoStruct, Schema};

use crate::async_test_matrix;

async fn should_trigger_on_success_handlers_at_creation() {
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
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[required]: on_success triggered with value: {}",
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
        required2: "required2".into(),
        required: "required1".into(),
    };

    let input = PartialDataInput {
        required: Some(data.required.clone()),
        required2: Some(data.required2.clone()),
    };

    let r = model.create(&input, None).await;

    match r {
        Ok((created, _, handle_success)) => {
            assert_eq!(created, data);

            handle_success().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[required]: on_success triggered with value: required",
    should_trigger_on_success_handlers_at_creation
);

async fn should_trigger_on_success_handlers_during_updates_if_provided() {
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

    let required_value_value = "required_value_value".to_string();

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
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[required]: on_success triggered with value: {}",
                                ctx.values().required.unwrap().as_str()
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

    let required2 = "required2".to_string();

    let data = Data {
        required2: required2.clone(),
        required: required_value_value,
    };

    let updated_required_value = "updated_required_value".to_string();

    let input = PartialDataInput {
        required: Some(updated_required_value.clone()),
        required2: Some(required2),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Ok((updated, _, handle_success)) => {
            assert_eq!(
                updated,
                PartialData {
                    required2: None,
                    required: Some(updated_required_value),
                }
            );

            handle_success().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[required]: on_success triggered with value: updated_required_value",
    should_trigger_on_success_handlers_during_updates_if_provided
);

async fn should_not_trigger_on_success_handlers_during_updates_if_not_provided() {
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

    let required_value_value = "required_value_value".to_string();

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
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[required]: on_success triggered with value: {}",
                                ctx.values().required.unwrap().as_str()
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

    let required2 = "required2".to_string();

    let data = Data {
        required2: required2.clone(),
        required: required_value_value,
    };

    let updated_required2_value = "updated_required2_value".to_string();

    let input = PartialDataInput {
        required2: Some(updated_required2_value.clone()),
        required: None,
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Ok((updated, _, handle_success)) => {
            assert_eq!(
                updated,
                PartialData {
                    required2: Some(updated_required2_value),
                    required: None,
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
        required: String,
        required2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        required: String,
        required2: String,
    }

    let required_value_value = "required_value_value".to_string();

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
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[required]: on_success triggered with value: {}",
                                ctx.values().required.unwrap().as_str()
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

    let required2 = "required2".to_string();

    let data = Data {
        required2: required2.clone(),
        required: required_value_value,
    };

    let updated_required_value = "updated_required_value".to_string();
    let updated_required2_value = "updated_required2_value".to_string();

    let input = PartialDataInput {
        required2: Some(updated_required2_value.clone()),
        required: Some(updated_required_value),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Ok((updated, _, handle_success)) => {
            assert_eq!(
                updated,
                PartialData {
                    required2: Some(updated_required2_value),
                    required: None,
                }
            );

            handle_success().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(should_not_trigger_on_success_handlers_during_updates_if_provided_and_ignored);

async fn should_not_trigger_on_success_handlers_during_updates_if_provided_and_ignored_as_readonly()
{
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

    let required_value_value = "required_value_value".to_string();

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
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[required]: on_success triggered with value: {}",
                                ctx.values().required.unwrap().as_str()
                            );
                        }

                        ready(())
                    })
                    .on_success(async |_, _| ()),
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

    let required2 = "required2".to_string();

    let data = Data {
        required2: required2.clone(),
        required: required_value_value,
    };

    let updated_required_value = "updated_required_value".to_string();
    let updated_required2_value = "updated_required2_value".to_string();

    let input = PartialDataInput {
        required2: Some(updated_required2_value.clone()),
        required: Some(updated_required_value),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Ok((updated, _, handle_success)) => {
            assert_eq!(
                updated,
                PartialData {
                    required2: Some(updated_required2_value),
                    required: None,
                }
            );

            handle_success().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    should_not_trigger_on_success_handlers_during_updates_if_provided_and_ignored_as_readonly
);

async fn should_trigger_success_handlers_with_empty_fields_array_each_time_creation_is_successful()
{
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
        required_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        required: i32,
        required_1: i32,
    }

    let required_value = 1234;
    let required_1_value = 5678;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "required",
                IvoField::REQUIRED.validate(|_: i32, _, _| ready(Ok(None))),
            )
            .set(
                "required_1",
                IvoField::REQUIRED.validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.on_success([], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered at creation despite empty field array")
                    }

                    ready(())
                })
            })
        },
    );

    let model = schema.model();

    let (data, _, handle_success) = model
        .create(
            &PartialDataInput {
                required: Some(required_value),
                required_1: Some(required_1_value),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            required: required_value,
            required_1: required_1_value
        }
    );

    handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered at creation despite empty field array",
    should_trigger_success_handlers_with_empty_fields_array_each_time_creation_is_successful
);

async fn should_trigger_success_handlers_with_empty_fields_array_each_time_update_is_successful() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
        required_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        required: i32,
        required_1: i32,
    }

    let required_value = 1234;
    let required_1_value = 5678;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "required",
                IvoField::REQUIRED.validate(|_: i32, _, _| ready(Ok(None))),
            )
            .set(
                "required_1",
                IvoField::REQUIRED.validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.on_success([], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered during updates despite empty field array")
                    }

                    ready(())
                })
            })
        },
    );

    let model = schema.model();

    let data = Data {
        required: required_value,
        required_1: required_1_value,
    };

    let updated_required_1 = data.required_1 + 1;

    let (updates, _, handle_success) = model
        .update(
            &data,
            &PartialDataInput {
                required: None,
                required_1: Some(updated_required_1),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            required: None,
            required_1: Some(updated_required_1)
        }
    );

    handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered during updates despite empty field array",
    should_trigger_success_handlers_with_empty_fields_array_each_time_update_is_successful
);
