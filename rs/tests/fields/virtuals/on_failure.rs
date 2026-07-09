use std::future::ready;

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoStruct, IvoUpdateError, Schema};

use crate::async_test_matrix;

async fn should_trigger_on_failure_handlers_at_creation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        virtual_field: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.input().virtual_field.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some("fail_validation".into()),
            },
            None,
        )
        .await;

    match r {
        Err((payload, _, handle_failure)) => {
            assert_eq!(
                payload.get("virtual_field").unwrap()[0].reason,
                "validation failed".to_string()
            );
            handle_failure().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: fail_validation",
    should_trigger_on_failure_handlers_at_creation
);

async fn should_trigger_on_failure_handlers_at_creation_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        virtual_alias: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.input().virtual_alias.unwrap().as_str()
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

    let r = model
        .create(
            &PartialDataInput {
                virtual_alias: Some("fail_validation".into()),
            },
            None,
        )
        .await;

    match r {
        Err((payload, _, handle_failure)) => {
            assert_eq!(
                payload.get("virtual_alias").unwrap()[0].reason,
                "validation failed".to_string()
            );
            handle_failure().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: fail_validation",
    should_trigger_on_failure_handlers_at_creation_with_alias
);

async fn should_trigger_on_failure_handlers_at_creation_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.input().dependent.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let r = model
        .create(
            &PartialDataInput {
                dependent: Some("fail_validation".into()),
            },
            None,
        )
        .await;

    match r {
        Err((payload, _, handle_failure)) => {
            assert_eq!(
                payload.get("dependent").unwrap()[0].reason,
                "validation failed".to_string()
            );
            handle_failure().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: fail_validation",
    should_trigger_on_failure_handlers_at_creation_with_alias_same_as_dependent
);

async fn should_trigger_on_failure_handlers_during_updates() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        virtual_field: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.input().virtual_field.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let r = model
        .update(
            &Data {
                dependent: default_dependent_value,
            },
            &PartialDataInput {
                virtual_field: Some("fail_validation".into()),
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(payload), _, handle_failure)) => {
            assert_eq!(
                payload.get("virtual_field").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: fail_validation",
    should_trigger_on_failure_handlers_during_updates
);

async fn should_trigger_on_failure_handlers_during_updates_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        virtual_alias: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.input().virtual_alias.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let r = model
        .update(
            &Data {
                dependent: default_dependent_value,
            },
            &PartialDataInput {
                virtual_alias: Some("fail_validation".into()),
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(payload), _, handle_failure)) => {
            assert_eq!(
                payload.get("virtual_alias").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: fail_validation",
    should_trigger_on_failure_handlers_during_updates_with_alias
);

async fn should_trigger_on_failure_handlers_during_updates_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.input().dependent.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let r = model
        .update(
            &Data {
                dependent: default_dependent_value,
            },
            &PartialDataInput {
                dependent: Some("fail_validation".into()),
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(payload), _, handle_failure)) => {
            assert_eq!(
                payload.get("dependent").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: fail_validation",
    should_trigger_on_failure_handlers_during_updates_with_alias_same_as_dependent
);

async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        virtual_field: String,
        virtual_field2: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field", "virtual_field2"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true))
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.raw_input().virtual_field.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "virtual_field2",
                IvoField::VIRTUAL.validate(|v: String, _, _| {
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

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some("update to be ignored".into()),
                virtual_field2: Some("fail_validation".into()),
            },
            None,
        )
        .await;

    match r {
        Err((payload, _, handle_failure)) => {
            assert!(payload.get("virtual_field").is_none());

            assert_eq!(
                payload.get("virtual_field2").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation
);

async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        virtual_alias: String,
        virtual_field2: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field", "virtual_field2"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true))
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.raw_input().virtual_alias.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "virtual_field2",
                IvoField::VIRTUAL.validate(|v: String, _, _| {
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

    let r = model
        .create(
            &PartialDataInput {
                virtual_alias: Some("update to be ignored".into()),
                virtual_field2: Some("fail_validation".into()),
            },
            None,
        )
        .await;

    match r {
        Err((payload, _, handle_failure)) => {
            assert!(payload.get("virtual_alias").is_none());

            assert_eq!(
                payload.get("virtual_field2").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation_with_alias
);

async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
        virtual_field2: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field", "virtual_field2"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true))
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.raw_input().dependent.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "virtual_field2",
                IvoField::VIRTUAL.validate(|v: String, _, _| {
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

    let r = model
        .create(
            &PartialDataInput {
                dependent: Some("update to be ignored".into()),
                virtual_field2: Some("fail_validation".into()),
            },
            None,
        )
        .await;

    match r {
        Err((payload, _, handle_failure)) => {
            assert!(payload.get("dependent").is_none());

            assert_eq!(
                payload.get("virtual_field2").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation_with_alias_same_as_dependent
);

async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        virtual_field: String,
        virtual_field2: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field", "virtual_field2"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true))
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.raw_input().virtual_field.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "virtual_field2",
                IvoField::VIRTUAL.validate(|v: String, _, _| {
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

    let r = model
        .update(
            &Data {
                dependent: default_dependent_value,
            },
            &PartialDataInput {
                virtual_field: Some("update to be ignored".into()),
                virtual_field2: Some("fail_validation".into()),
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(payload), _, handle_failure)) => {
            assert!(payload.get("virtual_field").is_none());

            assert_eq!(
                payload.get("virtual_field2").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates
);

async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        virtual_alias: String,
        virtual_field2: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field", "virtual_field2"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true))
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.raw_input().virtual_alias.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "virtual_field2",
                IvoField::VIRTUAL.validate(|v: String, _, _| {
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

    let r = model
        .update(
            &Data {
                dependent: default_dependent_value,
            },
            &PartialDataInput {
                virtual_alias: Some("update to be ignored".into()),
                virtual_field2: Some("fail_validation".into()),
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(payload), _, handle_failure)) => {
            assert!(payload.get("virtual_alias").is_none());

            assert_eq!(
                payload.get("virtual_field2").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates_with_alias
);

async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
        virtual_field2: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field", "virtual_field2"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true))
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.raw_input().dependent.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "virtual_field2",
                IvoField::VIRTUAL.validate(|v: String, _, _| {
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

    let r = model
        .update(
            &Data {
                dependent: default_dependent_value,
            },
            &PartialDataInput {
                dependent: Some("update to be ignored".into()),
                virtual_field2: Some("fail_validation".into()),
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(payload), _, handle_failure)) => {
            assert!(payload.get("dependent").is_none());

            assert_eq!(
                payload.get("virtual_field2").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates_with_alias_same_as_dependent
);

async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_init_fn(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        virtual_field: String,
        virtual_field2: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field", "virtual_field2"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_init()
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.raw_input().virtual_field.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "virtual_field2",
                IvoField::VIRTUAL.validate(|v: String, _, _| {
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

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some("update to be ignored".into()),
                virtual_field2: Some("fail_validation".into()),
            },
            None,
        )
        .await;

    match r {
        Err((payload, _, handle_failure)) => {
            assert!(payload.get("virtual_field").is_none());

            assert_eq!(
                payload.get("virtual_field2").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_init_fn
);

async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_init_fn_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        virtual_alias: String,
        virtual_field2: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field", "virtual_field2"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_init()
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.raw_input().virtual_alias.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "virtual_field2",
                IvoField::VIRTUAL.validate(|v: String, _, _| {
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

    let r = model
        .create(
            &PartialDataInput {
                virtual_alias: Some("update to be ignored".into()),
                virtual_field2: Some("fail_validation".into()),
            },
            None,
        )
        .await;

    match r {
        Err((payload, _, handle_failure)) => {
            assert!(payload.get("virtual_alias").is_none());

            assert_eq!(
                payload.get("virtual_field2").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_init_fn_with_alias
);

async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_init_fn_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
        virtual_field2: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field", "virtual_field2"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_init()
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.raw_input().dependent.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "virtual_field2",
                IvoField::VIRTUAL.validate(|v: String, _, _| {
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

    let r = model
        .create(
            &PartialDataInput {
                dependent: Some("update to be ignored".into()),
                virtual_field2: Some("fail_validation".into()),
            },
            None,
        )
        .await;

    match r {
        Err((payload, _, handle_failure)) => {
            assert!(payload.get("dependent").is_none());

            assert_eq!(
                payload.get("virtual_field2").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_init_fn_with_alias_same_as_dependent
);

async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_update_fn(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        virtual_field: String,
        virtual_field2: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field", "virtual_field2"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_update()
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.raw_input().virtual_field.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "virtual_field2",
                IvoField::VIRTUAL.validate(|v: String, _, _| {
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

    let input = PartialDataInput {
        virtual_field: Some("update to be ignored".into()),
        virtual_field2: Some("fail_validation".into()),
    };

    let r = model
        .update(
            &Data {
                dependent: default_dependent_value,
            },
            &input,
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(payload), _, handle_failure)) => {
            assert!(payload.get("virtual_field").is_none());

            assert_eq!(
                payload.get("virtual_field2").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_update_fn
);

async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_update_fn_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        virtual_alias: String,
        virtual_field2: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field", "virtual_field2"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_update()
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.raw_input().virtual_alias.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "virtual_field2",
                IvoField::VIRTUAL.validate(|v: String, _, _| {
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

    let input = PartialDataInput {
        virtual_alias: Some("update to be ignored".into()),
        virtual_field2: Some("fail_validation".into()),
    };

    let r = model
        .update(
            &Data {
                dependent: default_dependent_value,
            },
            &input,
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(payload), _, handle_failure)) => {
            assert!(payload.get("virtual_alias").is_none());

            assert_eq!(
                payload.get("virtual_field2").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_update_fn_with_alias
);

async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_update_fn_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
        virtual_field2: String,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field", "virtual_field2"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_update()
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_failure triggered with value: {}",
                                ctx.raw_input().dependent.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
            .field(
                "virtual_field2",
                IvoField::VIRTUAL.validate(|v: String, _, _| {
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

    let input = PartialDataInput {
        dependent: Some("update to be ignored".into()),
        virtual_field2: Some("fail_validation".into()),
    };

    let r = model
        .update(
            &Data {
                dependent: default_dependent_value,
            },
            &input,
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(payload), _, handle_failure)) => {
            assert!(payload.get("dependent").is_none());

            assert_eq!(
                payload.get("virtual_field2").unwrap()[0].reason,
                "validation failed".to_string()
            );

            handle_failure().await;
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_update_fn_with_alias_same_as_dependent
);
