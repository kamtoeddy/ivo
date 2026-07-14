use std::future::ready;

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoStruct, Schema};

use crate::async_test_matrix;

async fn should_trigger_on_success_handlers_at_creation_if_provided() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        lax_1: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
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
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_success triggered with value: {}",
                                ctx.raw_input().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "lax_1",
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
        lax_1: "lax_1".into(),
        lax: "lax1".into(),
    };

    let input = PartialDataInput {
        lax: Some(data.lax.clone()),
        lax_1: Some(data.lax_1.clone()),
    };

    let r = model.create(&input, None).await;

    match r {
        Ok((created, handle_success, _)) => {
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
        lax_1: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
    }

    let default_lax_value = "default_lax_value".to_string();

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.clone())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(Some(v)))
                    })
                    .ignore_update()
                    .on_success(async |_, _| ())
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_success triggered with value: {}",
                                ctx.values().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "lax_1",
                IvoField::LAX
                    .default("default_lax_1_value".into())
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

    let lax_1 = "lax_1".to_string();

    let input = PartialDataInput {
        lax: None,
        lax_1: Some(lax_1.clone()),
    };

    let r = model.create(&input, None).await;

    match r {
        Ok((created, handle_success, _)) => {
            assert_eq!(
                created,
                Data {
                    lax: default_lax_value,
                    lax_1,
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
        lax_1: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
    }

    let default_lax_value = "default_lax_value".to_string();

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.clone())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(Some(v)))
                    })
                    .ignore_init()
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_success triggered with value: {}",
                                ctx.values().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "lax_1",
                IvoField::LAX
                    .default("default_lax_1_value".into())
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

    let lax_value = "lax_value".to_string();
    let lax_1_value = "lax_1_value".to_string();

    let input = PartialDataInput {
        lax: Some(lax_value),
        lax_1: Some(lax_1_value.clone()),
    };

    let r = model.create(&input, None).await;

    match r {
        Ok((created, handle_success, _)) => {
            assert_eq!(
                created,
                Data {
                    lax: default_lax_value,
                    lax_1: lax_1_value,
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
        lax_1: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
    }

    let default_lax_value = "default_lax_value".to_string();

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.clone())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(Some(v)))
                    })
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_success triggered with value: {}",
                                ctx.values().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "lax_1",
                IvoField::LAX
                    .default("default_lax_1_value".into())
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

    let lax_1 = "lax_1".to_string();

    let data = Data {
        lax: default_lax_value,
        lax_1: lax_1.clone(),
    };

    let updated_lax_value = "updated_lax_value".to_string();

    let input = PartialDataInput {
        lax: Some(updated_lax_value.clone()),
        lax_1: Some(lax_1),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Ok((updated, handle_success, _)) => {
            assert_eq!(
                updated,
                PartialData {
                    lax: Some(updated_lax_value),
                    lax_1: None,
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
        lax_1: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
    }

    let default_lax_value = "default_lax_value".to_string();

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.clone())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(Some(v)))
                    })
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_success triggered with value: {}",
                                ctx.values().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "lax_1",
                IvoField::LAX
                    .default("default_lax_1_value".into())
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

    let lax_1 = "lax_1".to_string();

    let data = Data {
        lax: default_lax_value,
        lax_1: lax_1.clone(),
    };

    let updated_lax_1_value = "updated_lax_1_value".to_string();

    let input = PartialDataInput {
        lax: None,
        lax_1: Some(updated_lax_1_value.clone()),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Ok((updated, handle_success, _)) => {
            assert_eq!(
                updated,
                PartialData {
                    lax_1: Some(updated_lax_1_value),
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
        lax_1: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
    }

    let default_lax_value = "default_lax_value".to_string();

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.clone())
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(Some(v)))
                    })
                    .ignore_update()
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_success triggered with value: {}",
                                ctx.values().lax.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "lax_1",
                IvoField::LAX
                    .default("default_lax_1_value".into())
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

    let lax_1 = "lax_1".to_string();

    let data = Data {
        lax: default_lax_value,
        lax_1: lax_1.clone(),
    };

    let updated_lax_value = "updated_lax_value".to_string();
    let updated_lax_1_value = "updated_lax_1_value".to_string();

    let input = PartialDataInput {
        lax: Some(updated_lax_value),
        lax_1: Some(updated_lax_1_value.clone()),
    };

    let r = model.update(&data, &input, None).await;

    match r {
        Ok((updated, handle_success, _)) => {
            assert_eq!(
                updated,
                PartialData {
                    lax: None,
                    lax_1: Some(updated_lax_1_value),
                }
            );

            handle_success().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(should_not_trigger_on_success_handlers_during_updates_if_provided_and_ignored);

async fn should_trigger_success_handlers_with_empty_fields_array_each_time_creation_is_successful()
{
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let default_lax = 1234;
    let default_lax_1 = 5678;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field("lax", IvoField::LAX.default(default_lax))
                .field("lax_1", IvoField::LAX.default(default_lax_1))
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

    let (data, handle_success, _) = model
        .create(&PartialDataInput::new(), None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax: default_lax,
            lax_1: default_lax_1
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
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let default_lax = 1234;
    let default_lax_1 = 5678;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field("lax", IvoField::LAX.default(default_lax))
                .field("lax_1", IvoField::LAX.default(default_lax_1))
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
        lax: default_lax,
        lax_1: default_lax_1,
    };

    let updated_lax_1 = data.lax_1 + 1;

    let (updates, handle_success, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                lax_1: Some(updated_lax_1),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            lax: None,
            lax_1: Some(updated_lax_1)
        }
    );

    handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered during updates despite empty field array",
    should_trigger_success_handlers_with_empty_fields_array_each_time_update_is_successful
);
