use std::future::ready;

use ivo::{lax_field, IvoContext, IvoInputStruct, IvoModel, IvoStruct};

use crate::async_test_matrix;

// ignore

async fn should_respect_the_ignore_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        other: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        other: String,
    }

    let default_lax_value = "default_lax_value";

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                lax_field("other")
                    .default(String::from("default_other_value"))
                    .validate(|_, _, _| ready(Ok(None))),
            )
            .field(
                lax_field("lax")
                    .default(default_lax_value.to_string())
                    .validate(|_, _, _| ready(Ok(None)))
                    .ignore(|ctx: IvoContext<DataInput, Data>, _| {
                        if ctx.is_update() {
                            if "ignore_lax_for_update" == ctx.previous_values().unwrap().other {
                                return ready(true);
                            }

                            return ready(false);
                        }

                        if Some("ignore_lax_for_init".into()) == ctx.input().other {
                            return ready(true);
                        }

                        ready(false)
                    }),
            )
        },
        |o| o,
    );

    let other_value = "ignore_lax_for_init".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some("value to be ignored".into()),
                other: Some(other_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax: default_lax_value.to_string(),
            other: other_value
        }
    );

    let updated_lax_value = "updated_lax_value".to_string();
    let other_value = "ignore_lax_for_update".to_string();

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(updated_lax_value.clone()),
                other: Some(other_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            lax: Some(updated_lax_value),
            other: Some(other_value)
        }
    );

    let data = data.clone_with_updates(&updates);

    let other_value = "some other update".to_string();

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some("some lax update".into()),
                other: Some(other_value.clone()),
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
            other: Some(other_value)
        }
    );
}

async_test_matrix!(should_respect_the_ignore_rule);

async fn should_respect_the_ignore_init_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        other: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        other: String,
    }

    let default_lax_value = "default_lax_value";

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                lax_field("other")
                    .default(String::from("default_other_value"))
                    .validate(|_, _, _| ready(Ok(None))),
            )
            .field(
                lax_field("lax")
                    .default(default_lax_value.to_string())
                    .validate(|_, _, _| ready(Ok(None)))
                    .ignore_init(),
            )
        },
        |o| o,
    );

    let other_value = "some other value".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some("value to be ignored".into()),
                other: Some(other_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax: default_lax_value.to_string(),
            other: other_value
        }
    );

    let updated_lax_value = "updated_lax_value".to_string();
    let other_value = "updated_other_value".to_string();

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(updated_lax_value.clone()),
                other: Some(other_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            lax: Some(updated_lax_value),
            other: Some(other_value)
        }
    );
}

async_test_matrix!(should_respect_the_ignore_init_rule);

async fn should_respect_the_ignore_update_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        other: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        other: String,
    }

    let default_lax_value = "default_lax_value";

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                lax_field("other")
                    .default(String::from("default_other_value"))
                    .validate(|_, _, _| ready(Ok(None))),
            )
            .field(
                lax_field("lax")
                    .default(default_lax_value.to_string())
                    .validate(|_, _, _| ready(Ok(None)))
                    .ignore_update(),
            )
        },
        |o| o,
    );

    let lax_value = "lax value".to_string();
    let other_value = "other value".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax_value.clone()),
                other: Some(other_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax: lax_value,
            other: other_value
        }
    );

    let updated_lax_value = "lax value to be ignored".to_string();
    let other_value = "updated other value".to_string();

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(updated_lax_value.clone()),
                other: Some(other_value.clone()),
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
            other: Some(other_value)
        }
    );
}

async_test_matrix!(should_respect_the_ignore_update_rule);

// grouped ignore

async fn should_properly_handle_grouped_ignore_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        lax_1: String,
        lax_2: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
        lax_2: String,
    }

    const IGNORE: &str = "IGNORE";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";
    let default_lax_2_value = "default_lax_2_value";

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(lax_field("lax").default(default_lax_value.to_string()))
                .field(lax_field("lax_1").default(default_lax_1_value.to_string()))
                .field(lax_field("lax_2").default(default_lax_2_value.to_string()))
        },
        |o| {
            o.ignore(["lax", "lax_1"], |ctx: IvoContext<DataInput, Data>, _| {
                ready(ctx.input().lax == Some(IGNORE.into()))
            })
        },
    );

    let lax = IGNORE.to_string();
    let lax_1 = "lax_1".to_string();
    let lax_2 = "lax_2".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                lax_2: Some(lax_2.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax: default_lax_value.to_string(),
            lax_1: default_lax_1_value.to_string(),
            lax_2
        }
    );

    let lax = "some lax value".to_string();
    let lax_1 = "lax_1".to_string();
    let lax_2 = "lax_2".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                lax_2: Some(lax_2.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { lax, lax_1, lax_2 });

    // updates

    let data = Data {
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        lax_2: default_lax_2_value.to_string(),
    };

    let lax = Some(IGNORE.to_string());
    let lax_1 = Some("lax_1".to_string());
    let lax_2 = Some("lax_2".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: lax.clone(),
                lax_1: lax_1.clone(),
                lax_2: lax_2.clone(),
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
            lax_1: None,
            lax_2
        }
    );

    let lax = Some("some lax value".to_string());
    let lax_1 = Some("lax_1".to_string());
    let lax_2 = Some("lax_2".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: lax.clone(),
                lax_1: lax_1.clone(),
                lax_2: lax_2.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(updates, PartialData { lax, lax_1, lax_2 });
}

async_test_matrix!(should_properly_handle_grouped_ignore_rule);

// grouped ignore_update

async fn should_properly_handle_grouped_ignore_update_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        lax_1: String,
        lax_2: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
        lax_2: String,
    }

    const IGNORE: &str = "IGNORE";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";
    let default_lax_2_value = "default_lax_2_value";

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(lax_field("lax").default(default_lax_value.to_string()))
                .field(lax_field("lax_1").default(default_lax_1_value.to_string()))
                .field(lax_field("lax_2").default(default_lax_2_value.to_string()))
        },
        |o| {
            o.ignore_update(["lax", "lax_1"], |raw_input: PartialDataInput, _, _| {
                ready(raw_input.lax == Some(IGNORE.into()))
            })
        },
    );

    let lax = IGNORE.to_string();
    let lax_1 = "lax_1".to_string();
    let lax_2 = "lax_2".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                lax_2: Some(lax_2.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { lax, lax_1, lax_2 });

    let lax = "some lax value".to_string();
    let lax_1 = "lax_1".to_string();
    let lax_2 = "lax_2".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                lax_2: Some(lax_2.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { lax, lax_1, lax_2 });

    // updates

    let data = Data {
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        lax_2: default_lax_2_value.to_string(),
    };

    let lax = Some(IGNORE.to_string());
    let lax_1 = Some("lax_1".to_string());
    let lax_2 = Some("lax_2".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: lax.clone(),
                lax_1: lax_1.clone(),
                lax_2: lax_2.clone(),
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
            lax_1: None,
            lax_2
        }
    );

    let lax = Some("some lax value".to_string());
    let lax_1 = Some("lax_1".to_string());
    let lax_2 = Some("lax_2".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: lax.clone(),
                lax_1: lax_1.clone(),
                lax_2: lax_2.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(updates, PartialData { lax, lax_1, lax_2 });
}

async_test_matrix!(should_properly_handle_grouped_ignore_update_rule);

// readonly

async fn should_ignore_updates_on_readonly_fields_if_values_are_different_from_default_after_creation(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let default_value = 1;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| f.field(lax_field("lax").default(default_value).readonly()),
        |o| o,
    );

    let lax_value = 40;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax_value),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { lax: lax_value });

    let updated_value = 2;

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(updated_value),
            },
            None,
        )
        .await;

    match r {
        Err((err, _, _)) => assert!(err.is_none()),
        _ => unreachable!(),
    }
}

async_test_matrix!(
    should_ignore_updates_on_readonly_fields_if_values_are_different_from_default_after_creation
);

async fn should_ignore_updates_on_readonly_fields_if_values_are_different_from_default_after_updates(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    const DEFAULT_VALUE: i32 = 1;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| f.field(lax_field("lax").default(DEFAULT_VALUE).readonly()),
        |o| o,
    );

    let (data, _, _) = model
        .create(&PartialDataInput { lax: None }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { lax: DEFAULT_VALUE });

    let updated_value = 2;

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(updated_value),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            lax: Some(updated_value)
        }
    );

    let data = data.clone_with_updates(&updates);

    assert_eq!(data, Data { lax: updated_value });

    let updated_value = 3;

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(updated_value),
            },
            None,
        )
        .await;

    match r {
        Err((err, _, _)) => assert!(err.is_none()),
        _ => unreachable!(),
    }
}

async_test_matrix!(
    should_ignore_updates_on_readonly_fields_if_values_are_different_from_default_after_updates
);
