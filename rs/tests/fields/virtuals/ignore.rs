use std::future::ready;

use ivo::{
    dependent_field, lax_field, virtual_field, IvoContext, IvoInputStruct, IvoModel, IvoStruct,
};

use crate::async_test_matrix;

// ignore

async fn should_respect_the_ignore_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent", ["virtual_field"])
                    .default(default_dependent_value)
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true)),
            )
        },
        |o| o,
    );

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax: default_lax_value
        }
    );

    let lax = default_lax_value + 10;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    let lax = Some(data.lax + 10);

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax
        }
    );

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await;

    match r {
        Err((err, _, _)) => assert!(err.is_none()),
        _ => unreachable!("expected nothing to update"),
    }
}

async_test_matrix!(should_respect_the_ignore_rule);

async fn should_respect_the_ignore_rule_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent", ["virtual_field"])
                    .default(default_dependent_value)
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true)),
            )
        },
        |o| o,
    );

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax: default_lax_value
        }
    );

    let lax = default_lax_value + 10;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    let lax = Some(data.lax + 10);

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax
        }
    );

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await;

    match r {
        Err((err, _, _)) => assert!(err.is_none()),
        _ => unreachable!("expected nothing to update"),
    }
}

async_test_matrix!(should_respect_the_ignore_rule_with_alias);

async fn should_respect_the_ignore_rule_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
        lax: i32,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent", ["virtual_field"])
                    .default(default_dependent_value)
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true)),
            )
        },
        |o| o,
    );

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax: default_lax_value
        }
    );

    let lax = default_lax_value + 10;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    let lax = Some(data.lax + 10);

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax
        }
    );

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await;

    match r {
        Err((err, _, _)) => assert!(err.is_none()),
        _ => unreachable!("expected nothing to update"),
    }
}

async_test_matrix!(should_respect_the_ignore_rule_with_alias_same_as_dependent);

async fn should_respect_the_ignore_init_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent", ["virtual_field"])
                    .default(default_dependent_value)
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_init(),
            )
        },
        |o| o,
    );

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax: default_lax_value
        }
    );

    let lax = default_lax_value + 10;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    let lax = Some(data.lax + 10);

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax
        }
    );

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax: None
        }
    );
}

async_test_matrix!(should_respect_the_ignore_init_rule);

async fn should_respect_the_ignore_init_rule_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent", ["virtual_field"])
                    .default(default_dependent_value)
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_init(),
            )
        },
        |o| o,
    );

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax: default_lax_value
        }
    );

    let lax = default_lax_value + 10;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    let lax = Some(data.lax + 10);

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax
        }
    );

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax: None
        }
    );
}

async_test_matrix!(should_respect_the_ignore_init_rule_with_alias);

async fn should_respect_the_ignore_init_rule_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        dependent: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent", ["virtual_field"])
                    .default(default_dependent_value)
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_init(),
            )
        },
        |o| o,
    );

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax: default_lax_value
        }
    );

    let lax = default_lax_value + 10;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    let lax = Some(data.lax + 10);

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax
        }
    );

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax: None
        }
    );
}

async_test_matrix!(should_respect_the_ignore_init_rule_with_alias_same_as_dependent);

async fn should_respect_the_ignore_update_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent", ["virtual_field"])
                    .default(default_dependent_value)
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_update(),
            )
        },
        |o| o,
    );

    let lax = default_lax_value + 10;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax
        }
    );

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value
        }
    );

    let lax = default_lax_value + 10;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax
        }
    );

    let lax = Some(data.lax + 10);

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax
        }
    );

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await;

    match r {
        Err((err, _, _)) => assert!(err.is_none()),
        _ => unreachable!("expected nothing to update"),
    }
}

async_test_matrix!(should_respect_the_ignore_update_rule);

async fn should_respect_the_ignore_update_rule_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent", ["virtual_field"])
                    .default(default_dependent_value)
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_update(),
            )
        },
        |o| o,
    );

    let lax = default_lax_value + 10;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax
        }
    );

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value
        }
    );

    let lax = default_lax_value + 10;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax
        }
    );

    let lax = Some(data.lax + 10);

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax
        }
    );

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await;

    match r {
        Err((err, _, _)) => assert!(err.is_none()),
        _ => unreachable!("expected nothing to update"),
    }
}

async_test_matrix!(should_respect_the_ignore_update_rule_with_alias);

async fn should_respect_the_ignore_update_rule_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        dependent: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent", ["virtual_field"])
                    .default(default_dependent_value)
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_update(),
            )
        },
        |o| o,
    );

    let lax = default_lax_value + 10;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax
        }
    );

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value
        }
    );

    let lax = default_lax_value + 10;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax
        }
    );

    let lax = Some(data.lax + 10);

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax
        }
    );

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await;

    match r {
        Err((err, _, _)) => assert!(err.is_none()),
        _ => unreachable!("expected nothing to update"),
    }
}

async_test_matrix!(should_respect_the_ignore_update_rule_with_alias_same_as_dependent);

// grouped ignore

async fn should_properly_handle_grouped_ignore_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: String,
        lax_1: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
        virtual_field: String,
    }

    const IGNORE: &str = "IGNORE";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";
    let default_dependent_value = 1;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(lax_field("lax").default(default_lax_value.to_string()))
                .field(lax_field("lax_1").default(default_lax_1_value.to_string()))
                .field(
                    dependent_field("dependent", ["virtual_field"])
                        .default(default_dependent_value)
                        .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                            ready(ctx.values().dependent.unwrap() + 1)
                        }),
                )
                .field(virtual_field("virtual_field").validate(|_, _, _| ready(Ok(None::<String>))))
        },
        |o| {
            o.ignore(
                ["virtual_field", "lax"],
                |ctx: IvoContext<DataInput, Data>, _| ready(ctx.input().lax == Some(IGNORE.into())),
            )
        },
    );

    let lax = IGNORE.to_string();
    let lax_1 = "lax_1".to_string();
    let virtual_field = "virtual_field".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                virtual_field: Some(virtual_field.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax: default_lax_value.to_string(),
            lax_1,
        }
    );

    let lax = "some lax value".to_string();
    let lax_1 = "lax_1".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                virtual_field: Some(virtual_field.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax,
            lax_1,
        }
    );

    // updates

    let data = Data {
        dependent: default_dependent_value,
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
    };

    let lax = Some(IGNORE.to_string());
    let lax_1 = Some("lax_1".to_string());
    let virtual_field = Some("virtual_field".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: lax.clone(),
                lax_1: lax_1.clone(),
                virtual_field: virtual_field.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax: None,
            lax_1,
        }
    );

    let lax = Some("some lax value".to_string());
    let lax_1 = Some("lax_1".to_string());
    let virtual_field = Some("virtual_field".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: lax.clone(),
                lax_1: lax_1.clone(),
                virtual_field: virtual_field.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax,
            lax_1,
        }
    );
}

async_test_matrix!(should_properly_handle_grouped_ignore_rule);

async fn should_properly_handle_grouped_ignore_rule_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: String,
        lax_1: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
        virtual_alias: String,
    }

    const IGNORE: &str = "IGNORE";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";
    let default_dependent_value = 1;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(lax_field("lax").default(default_lax_value.to_string()))
                .field(lax_field("lax_1").default(default_lax_1_value.to_string()))
                .field(
                    dependent_field("dependent", ["virtual_field"])
                        .default(default_dependent_value)
                        .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                            ready(ctx.values().dependent.unwrap() + 1)
                        }),
                )
                .field(
                    virtual_field("virtual_field")
                        .alias("virtual_alias")
                        .validate(|_, _, _| ready(Ok(None::<String>))),
                )
        },
        |o| {
            o.ignore(
                ["virtual_field", "lax"],
                |ctx: IvoContext<DataInput, Data>, _| ready(ctx.input().lax == Some(IGNORE.into())),
            )
        },
    );

    let lax = IGNORE.to_string();
    let lax_1 = "lax_1".to_string();
    let virtual_field = "virtual_field".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                virtual_alias: Some(virtual_field.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax: default_lax_value.to_string(),
            lax_1,
        }
    );

    let lax = "some lax value".to_string();
    let lax_1 = "lax_1".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                virtual_alias: Some(virtual_field.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax,
            lax_1,
        }
    );

    // updates

    let data = Data {
        dependent: default_dependent_value,
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
    };

    let lax = Some(IGNORE.to_string());
    let lax_1 = Some("lax_1".to_string());
    let virtual_field = Some("virtual_field".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: lax.clone(),
                lax_1: lax_1.clone(),
                virtual_alias: virtual_field.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax: None,
            lax_1,
        }
    );

    let lax = Some("some lax value".to_string());
    let lax_1 = Some("lax_1".to_string());
    let virtual_field = Some("virtual_field".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: lax.clone(),
                lax_1: lax_1.clone(),
                virtual_alias: virtual_field.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax,
            lax_1,
        }
    );
}

async_test_matrix!(should_properly_handle_grouped_ignore_rule_with_alias);

async fn should_properly_handle_grouped_ignore_rule_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: String,
        lax_1: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
        dependent: String,
    }

    const IGNORE: &str = "IGNORE";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";
    let default_dependent_value = 1;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(lax_field("lax").default(default_lax_value.to_string()))
                .field(lax_field("lax_1").default(default_lax_1_value.to_string()))
                .field(
                    dependent_field("dependent", ["virtual_field"])
                        .default(default_dependent_value)
                        .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                            ready(ctx.values().dependent.unwrap() + 1)
                        }),
                )
                .field(
                    virtual_field("virtual_field")
                        .alias("dependent")
                        .validate(|_, _, _| ready(Ok(None::<String>))),
                )
        },
        |o| {
            o.ignore(
                ["virtual_field", "lax"],
                |ctx: IvoContext<DataInput, Data>, _| ready(ctx.input().lax == Some(IGNORE.into())),
            )
        },
    );

    let lax = IGNORE.to_string();
    let lax_1 = "lax_1".to_string();
    let virtual_field = "virtual_field".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                dependent: Some(virtual_field.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax: default_lax_value.to_string(),
            lax_1,
        }
    );

    let lax = "some lax value".to_string();
    let lax_1 = "lax_1".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                dependent: Some(virtual_field.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax,
            lax_1,
        }
    );

    // updates

    let data = Data {
        dependent: default_dependent_value,
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
    };

    let lax = Some(IGNORE.to_string());
    let lax_1 = Some("lax_1".to_string());
    let virtual_field = Some("virtual_field".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: lax.clone(),
                lax_1: lax_1.clone(),
                dependent: virtual_field.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax: None,
            lax_1,
        }
    );

    let lax = Some("some lax value".to_string());
    let lax_1 = Some("lax_1".to_string());
    let virtual_field = Some("virtual_field".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: lax.clone(),
                lax_1: lax_1.clone(),
                dependent: virtual_field.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax,
            lax_1,
        }
    );
}

async_test_matrix!(should_properly_handle_grouped_ignore_rule_with_alias_same_as_dependent);

// grouped ignore_update

async fn should_properly_handle_grouped_ignore_update_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: String,
        lax_1: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
        virtual_field: String,
    }

    const DEFAULT_DEPENDENT_VALUE: i32 = 1;
    const IGNORE: &str = "IGNORE";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent", ["virtual_field"])
                    .default(DEFAULT_DEPENDENT_VALUE)
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value.to_string()))
            .field(lax_field("lax_1").default(default_lax_1_value.to_string()))
            .field(virtual_field("virtual_field").validate(|_, _, _| ready(Ok(None::<String>))))
        },
        |o| {
            o.ignore_update(
                ["lax", "virtual_field"],
                |raw_input: PartialDataInput, _, _| ready(raw_input.lax == Some(IGNORE.into())),
            )
        },
    );

    let lax = IGNORE.to_string();
    let lax_1 = "lax_1".to_string();
    let virtual_field = "some value".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                virtual_field: Some(virtual_field.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: DEFAULT_DEPENDENT_VALUE + 1,
            lax,
            lax_1
        }
    );

    let lax = "some lax value".to_string();
    let lax_1 = "lax_1".to_string();
    let virtual_field = "some value".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                virtual_field: Some(virtual_field.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: DEFAULT_DEPENDENT_VALUE + 1,
            lax,
            lax_1,
        }
    );

    // updates

    let data = Data {
        dependent: DEFAULT_DEPENDENT_VALUE,
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
    };

    let lax = Some(IGNORE.to_string());
    let lax_1 = Some("lax_1".to_string());
    let virtual_field = Some("updated value".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                lax_1: lax_1.clone(),
                virtual_field,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax: None,
            lax_1,
        }
    );

    let lax = Some("some lax value".to_string());
    let lax_1 = Some("lax_1".to_string());
    let virtual_field = Some("updated value".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: lax.clone(),
                lax_1: lax_1.clone(),
                virtual_field: virtual_field.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax,
            lax_1,
        }
    );
}

async_test_matrix!(should_properly_handle_grouped_ignore_update_rule);

async fn should_properly_handle_grouped_ignore_update_rule_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: String,
        lax_1: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
        virtual_alias: String,
    }

    const DEFAULT_DEPENDENT_VALUE: i32 = 1;
    const IGNORE: &str = "IGNORE";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent", ["virtual_field"])
                    .default(DEFAULT_DEPENDENT_VALUE)
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value.to_string()))
            .field(lax_field("lax_1").default(default_lax_1_value.to_string()))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|_, _, _| ready(Ok(None::<String>))),
            )
        },
        |o| {
            o.ignore_update(
                ["lax", "virtual_field"],
                |raw_input: PartialDataInput, _, _| ready(raw_input.lax == Some(IGNORE.into())),
            )
        },
    );

    let lax = IGNORE.to_string();
    let lax_1 = "lax_1".to_string();
    let virtual_alias = "some value".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                virtual_alias: Some(virtual_alias.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: DEFAULT_DEPENDENT_VALUE + 1,
            lax,
            lax_1
        }
    );

    let lax = "some lax value".to_string();
    let lax_1 = "lax_1".to_string();
    let virtual_alias = "some value".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                virtual_alias: Some(virtual_alias.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: DEFAULT_DEPENDENT_VALUE + 1,
            lax,
            lax_1,
        }
    );

    // updates

    let data = Data {
        dependent: DEFAULT_DEPENDENT_VALUE,
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
    };

    let lax = Some(IGNORE.to_string());
    let lax_1 = Some("lax_1".to_string());
    let virtual_alias = Some("updated value".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                lax_1: lax_1.clone(),
                virtual_alias,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax: None,
            lax_1,
        }
    );

    let lax = Some("some lax value".to_string());
    let lax_1 = Some("lax_1".to_string());
    let virtual_alias = Some("updated value".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: lax.clone(),
                lax_1: lax_1.clone(),
                virtual_alias: virtual_alias.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax,
            lax_1,
        }
    );
}

async_test_matrix!(should_properly_handle_grouped_ignore_update_rule_with_alias);

async fn should_properly_handle_grouped_ignore_update_rule_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: String,
        lax_1: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
        lax: String,
        lax_1: String,
    }

    const DEFAULT_DEPENDENT_VALUE: i32 = 1;
    const IGNORE: &str = "IGNORE";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent", ["virtual_field"])
                    .default(DEFAULT_DEPENDENT_VALUE)
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value.to_string()))
            .field(lax_field("lax_1").default(default_lax_1_value.to_string()))
            .field(
                virtual_field("virtual_field")
                    .alias("dependent")
                    .validate(|_, _, _| ready(Ok(None::<String>))),
            )
        },
        |o| {
            o.ignore_update(
                ["lax", "virtual_field"],
                |raw_input: PartialDataInput, _, _| ready(raw_input.lax == Some(IGNORE.into())),
            )
        },
    );

    let dependent = "some value".to_string();
    let lax = IGNORE.to_string();
    let lax_1 = "lax_1".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                dependent: Some(dependent.clone()),
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: DEFAULT_DEPENDENT_VALUE + 1,
            lax,
            lax_1
        }
    );

    let dependent = "some value".to_string();
    let lax = "some lax value".to_string();
    let lax_1 = "lax_1".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                dependent: Some(dependent.clone()),
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: DEFAULT_DEPENDENT_VALUE + 1,
            lax,
            lax_1,
        }
    );

    // updates

    let data = Data {
        dependent: DEFAULT_DEPENDENT_VALUE,
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
    };

    let dependent = Some("updated value".to_string());
    let lax = Some(IGNORE.to_string());
    let lax_1 = Some("lax_1".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                dependent,
                lax,
                lax_1: lax_1.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax: None,
            lax_1,
        }
    );

    let dependent = Some("updated value".to_string());
    let lax = Some("some lax value".to_string());
    let lax_1 = Some("lax_1".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                dependent: dependent.clone(),
                lax: lax.clone(),
                lax_1: lax_1.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax,
            lax_1,
        }
    );
}

async_test_matrix!(should_properly_handle_grouped_ignore_update_rule_with_alias_same_as_dependent);
