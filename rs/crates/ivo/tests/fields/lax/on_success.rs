use std::future::ready;

use ivo::{IvoField, IvoStruct, Schema, SharedIvoContext};

use crate::async_test_matrix;

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
                    .ignore_update()
                    .on_success(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_success triggered with value: {}",
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
                    .ignore_update()
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
                    .ignore_update()
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
