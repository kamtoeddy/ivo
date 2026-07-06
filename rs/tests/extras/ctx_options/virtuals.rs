use std::future::ready;

use ivo::{
    IvoContext, IvoField, IvoRwCtxOptions, IvoSharedCtxOptions, IvoStruct, IvoUpdateError, Schema,
};

use crate::async_test_matrix;

// TODO:
// [x] ignore
// [x] required
// [x] validate
// [x] re_validate
// [x] sanitizer
// [x] post_validator
// [x] on_failure
// [x] on_success
// [x] o.on_success
// [x] o.post_validate

#[derive(Clone)]
struct CtxOptions {
    messages: Vec<String>,
}

impl CtxOptions {
    fn new() -> Self {
        Self { messages: vec![] }
    }

    fn add_message(&mut self, msg: &str) {
        self.messages.push(msg.to_owned());
    }
}

// required

async fn should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_at_creation(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: i32,
        virtual_field_1: i32,
    }

    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in ignore resolver";
    const REQUIRED_ERROR: &str = "virtual_field is missing!";

    let schema = Schema::<DataInput, Data, CtxOptions>::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(DEFAULT_VALUE)
                    .depends_on(["virtual_field", "virtual_field_1"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap() + 1)
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .required(async |_, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        Some(REQUIRED_ERROR.into())
                    }),
            )
            .set(
                "virtual_field_1",
                IvoField::VIRTUAL.validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let (err, ctx_options, _) = model
        .create(
            &PartialDataInput {
                virtual_field: None,
                virtual_field_1: Some(1),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(err.get("virtual_field").unwrap()[0].reason, REQUIRED_ERROR);
    assert_eq!(ctx_options.messages[0], MESSAGE);
}

async_test_matrix!(
    should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_at_creation
);

async fn should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_during_updates(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: i32,
        virtual_field_1: i32,
    }

    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in ignore resolver";
    const REQUIRED_ERROR: &str = "virtual_field is missing!";

    let schema = Schema::<DataInput, Data, CtxOptions>::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(DEFAULT_VALUE)
                    .depends_on(["virtual_field", "virtual_field_1"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap() + 1)
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .required(async |_, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        Some(REQUIRED_ERROR.into())
                    }),
            )
            .set(
                "virtual_field_1",
                IvoField::VIRTUAL.validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let (err, ctx_options, _) = model
        .update(
            &Data {
                dependent: DEFAULT_VALUE,
            },
            &PartialDataInput {
                virtual_field: None,
                virtual_field_1: Some(1),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    match err {
        IvoUpdateError::ValidationError(payload) => {
            assert_eq!(
                payload.get("virtual_field").unwrap()[0].reason,
                REQUIRED_ERROR
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    assert_eq!(ctx_options.messages[0], MESSAGE);
}

async_test_matrix!(
    should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_during_updates
);

// ignore_update

async fn should_properly_update_ctx_options_in_ignore_update_resolver_and_provide_those_updates_in_on_success_handlers_during_updates(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: i32,
    }

    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in ignore_update resolver";

    let schema = Schema::<DataInput, Data, CtxOptions>::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(DEFAULT_VALUE)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap() + 1)
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .ignore(async |_, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        false
                    })
                    .on_success(|_, o: IvoSharedCtxOptions<CtxOptions>| {
                        if true {
                            panic!("[on_success]: {}", o.messages[0])
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let data = Data {
        dependent: DEFAULT_VALUE,
    };

    let value = Some(data.dependent + 1);

    let (data, ctx_options, handle_success) = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: value,
            },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        PartialData {
            dependent: value.map(|v| v + 1)
        }
    );
    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_success().await;
}

async_test_matrix!(
    "[on_success]: ctx_options updated in ignore_update resolver",
    should_properly_update_ctx_options_in_ignore_update_resolver_and_provide_those_updates_in_on_success_handlers_during_updates
);

// validate

async fn should_properly_update_ctx_options_in_validators_and_provide_those_updates_in_on_failure_handlers_at_creation(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in validator";
    const VIRTUAL_FIELD_ERROR: &str = "virtual_field is missing!";

    const MIN_LENGTH_ERROR: &str = "expected virtual_field to be at least 2 characters long";

    let schema: Schema<DataInput, Data, CtxOptions> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(DEFAULT_VALUE.into())
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(async |v: String, _, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        let validated = v.trim();

                        if validated.len() < 2 {
                            return Err((MIN_LENGTH_ERROR.into(), None));
                        }

                        Ok(Some(validated.into()))
                    })
                    .on_failure(|_, o: IvoSharedCtxOptions<CtxOptions>| {
                        if true {
                            panic!("[on_failure]: {}", o.messages[0])
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let (err, ctx_options, handle_failure) = model
        .create(
            &PartialDataInput {
                virtual_field: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(
        err.get("virtual_field").unwrap()[0].reason,
        MIN_LENGTH_ERROR
    );
    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_failure().await;
}

async_test_matrix!(
    "[on_failure]: ctx_options updated in validator",
    should_properly_update_ctx_options_in_validators_and_provide_those_updates_in_on_failure_handlers_at_creation
);

async fn should_properly_update_ctx_options_in_validators_and_provide_those_updates_in_on_failure_handlers_during_updates(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in validator";
    const VIRTUAL_FIELD_ERROR: &str = "virtual_field is missing!";

    const MIN_LENGTH_ERROR: &str = "expected virtual_field to be at least 2 characters long";

    let schema: Schema<DataInput, Data, CtxOptions> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(DEFAULT_VALUE.into())
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(async |v: String, _, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        let validated = v.trim();

                        if validated.len() < 2 {
                            return Err((MIN_LENGTH_ERROR.into(), None));
                        }

                        Ok(Some(validated.into()))
                    })
                    .on_failure(|_, o: IvoSharedCtxOptions<CtxOptions>| {
                        if true {
                            panic!("[on_failure]: {}", o.messages[0])
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let (err, ctx_options, handle_failure) = model
        .update(
            &Data {
                dependent: DEFAULT_VALUE.into(),
            },
            &PartialDataInput {
                virtual_field: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    match err {
        IvoUpdateError::ValidationError(payload) => {
            assert_eq!(
                payload.get("virtual_field").unwrap()[0].reason,
                MIN_LENGTH_ERROR
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_failure().await;
}

async_test_matrix!(
    "[on_failure]: ctx_options updated in validator",
    should_properly_update_ctx_options_in_validators_and_provide_those_updates_in_on_failure_handlers_during_updates
);

// re_validate

async fn should_properly_update_ctx_options_in_re_validators_and_provide_those_updates_in_on_failure_handlers_at_creation(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in re_validator";
    const VIRTUAL_FIELD_ERROR: &str = "virtual_field is missing!";

    const MIN_LENGTH_ERROR: &str = "expected virtual_field to be at least 2 characters long";

    let schema: Schema<DataInput, Data, CtxOptions> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(DEFAULT_VALUE.into())
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|_, _, _| ready(Ok(None)))
                    .re_validate(async |v: String, _, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        let validated = v.trim();

                        if validated.len() < 2 {
                            return Err((MIN_LENGTH_ERROR.into(), None));
                        }

                        Ok(Some(validated.into()))
                    })
                    .on_failure(|_, o: IvoSharedCtxOptions<CtxOptions>| {
                        if true {
                            panic!("[on_failure]: {}", o.messages[0])
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let (err, ctx_options, handle_failure) = model
        .create(
            &PartialDataInput {
                virtual_field: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(
        err.get("virtual_field").unwrap()[0].reason,
        MIN_LENGTH_ERROR
    );
    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_failure().await;
}

async_test_matrix!(
    "[on_failure]: ctx_options updated in re_validator",
    should_properly_update_ctx_options_in_re_validators_and_provide_those_updates_in_on_failure_handlers_at_creation
);

async fn should_properly_update_ctx_options_in_re_validators_and_provide_those_updates_in_on_failure_handlers_during_updates(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in re_validator";
    const VIRTUAL_FIELD_ERROR: &str = "virtual_field is missing!";

    const MIN_LENGTH_ERROR: &str = "expected virtual_field to be at least 2 characters long";

    let schema: Schema<DataInput, Data, CtxOptions> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(DEFAULT_VALUE.into())
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|_, _, _| ready(Ok(None)))
                    .re_validate(async |v: String, _, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        let validated = v.trim();

                        if validated.len() < 2 {
                            return Err((MIN_LENGTH_ERROR.into(), None));
                        }

                        Ok(Some(validated.into()))
                    })
                    .on_failure(|_, o: IvoSharedCtxOptions<CtxOptions>| {
                        if true {
                            panic!("[on_failure]: {}", o.messages[0])
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let (err, ctx_options, handle_failure) = model
        .update(
            &Data {
                dependent: DEFAULT_VALUE.into(),
            },
            &PartialDataInput {
                virtual_field: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    match err {
        IvoUpdateError::ValidationError(payload) => {
            assert_eq!(
                payload.get("virtual_field").unwrap()[0].reason,
                MIN_LENGTH_ERROR
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_failure().await;
}

async_test_matrix!(
    "[on_failure]: ctx_options updated in re_validator",
    should_properly_update_ctx_options_in_re_validators_and_provide_those_updates_in_on_failure_handlers_during_updates
);

// sanitize

async fn should_properly_update_ctx_options_in_sanitizers_and_provide_those_updates_on_success_handlers_at_creation(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    let default_dependent_value = "default_dependent_value";
    const MESSAGE: &str = "ctx_options updated in sanitizer";

    fn sanitize(value: &str) -> String {
        format!("sanitized-{value}")
    }

    let schema: Schema<DataInput, Data, CtxOptions> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value.into())
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|_: String, _, _| ready(Ok(None)))
                    .sanitize(async |value: String, _, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        sanitize(&value)
                    })
                    .on_success(|_, o: IvoSharedCtxOptions<CtxOptions>| {
                        if true {
                            panic!("[on_success]: {}", o.messages[0])
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let virtual_value = "virtual_value".to_string();

    let (data, ctx_options, handle_success) = model
        .create(
            &PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
            },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: sanitize(&virtual_value)
        },
    );

    assert_ne!(
        data,
        Data {
            dependent: virtual_value
        }
    );

    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_success().await;
}

async_test_matrix!(
    "[on_success]: ctx_options updated in sanitizer",
    should_properly_update_ctx_options_in_sanitizers_and_provide_those_updates_on_success_handlers_at_creation
);

async fn should_properly_update_ctx_options_in_sanitizers_and_provide_those_updates_on_success_handlers_during_updates(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    let default_dependent_value = "default_dependent_value";
    const MESSAGE: &str = "ctx_options updated in sanitizer";

    fn sanitize(value: &str) -> String {
        format!("sanitized-{value}")
    }

    let schema: Schema<DataInput, Data, CtxOptions> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value.into())
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|_: String, _, _| ready(Ok(None)))
                    .sanitize(async |value: String, _, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        sanitize(&value)
                    })
                    .on_success(|_, o: IvoSharedCtxOptions<CtxOptions>| {
                        if true {
                            panic!("[on_success]: {}", o.messages[0])
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let data = Data {
        dependent: default_dependent_value.into(),
    };

    let updated_virtual_value = "updated_virtual_value".to_string();

    let (updates, ctx_options, handle_success) = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: Some(updated_virtual_value.clone()),
            },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(sanitize(&updated_virtual_value))
        },
    );

    assert_ne!(
        updates,
        PartialData {
            dependent: Some(updated_virtual_value)
        },
    );

    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_success().await;
}

async_test_matrix!(
    "[on_success]: ctx_options updated in sanitizer",
    should_properly_update_ctx_options_in_sanitizers_and_provide_those_updates_on_success_handlers_during_updates
);

// o.post_validate & o.on_success

async fn should_properly_update_ctx_options_in_post_validators_and_provide_those_updates_in_grouped_on_success_handlers_with_no_fields_at_creation(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: i32,
        virtual_field_1: i32,
    }

    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in post_validator";
    const VIRTUAL_FIELD_ERROR: &str = "virtual_field is missing!";

    const MIN_LENGTH_ERROR: &str = "expected virtual_field to be at least 2 characters long";

    let schema: Schema<DataInput, Data, CtxOptions> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(DEFAULT_VALUE)
                    .depends_on(["virtual_field", "virtual_field_1"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL.validate(|_: i32, _, _| ready(Ok(None))),
            )
            .set(
                "virtual_field_1",
                IvoField::VIRTUAL.validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.post_validate(["virtual_field", "virtual_field_1"], |v| {
                v.validate(async |_, o: IvoRwCtxOptions<CtxOptions>| {
                    let mut ctx_options = o.write().await;

                    ctx_options.add_message(MESSAGE);

                    Ok(None)
                })
            })
            .on_success([], |s| {
                s.handle(|_, o: IvoSharedCtxOptions<CtxOptions>| {
                    if true {
                        panic!("[grouped_on_success]: {}", o.messages[0])
                    }

                    ready(())
                })
            })
        },
    );

    let model = schema.model();

    let value = DEFAULT_VALUE + 1;

    let (data, ctx_options, handle_success) = model
        .create(
            &PartialDataInput {
                virtual_field: Some(value),
                virtual_field_1: Some(value),
            },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { dependent: value });
    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_success().await;
}

async_test_matrix!(
    "[grouped_on_success]: ctx_options updated in post_validator",
    should_properly_update_ctx_options_in_post_validators_and_provide_those_updates_in_grouped_on_success_handlers_with_no_fields_at_creation
);

async fn should_properly_update_ctx_options_in_post_validators_and_provide_those_updates_in_grouped_on_success_handlers_during_updates(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: i32,
        virtual_field_1: i32,
    }

    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in post_validator";
    const VIRTUAL_FIELD_ERROR: &str = "virtual_field is missing!";

    const MIN_LENGTH_ERROR: &str = "expected virtual_field to be at least 2 characters long";

    let schema: Schema<DataInput, Data, CtxOptions> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(DEFAULT_VALUE)
                    .depends_on(["virtual_field", "virtual_field_1"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL.validate(|_: i32, _, _| ready(Ok(None))),
            )
            .set(
                "virtual_field_1",
                IvoField::VIRTUAL.validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.post_validate(["virtual_field", "virtual_field_1"], |v| {
                v.validate(async |_, o: IvoRwCtxOptions<CtxOptions>| {
                    let mut ctx_options = o.write().await;

                    ctx_options.add_message(MESSAGE);

                    Ok(None)
                })
            })
            .on_success(["virtual_field", "virtual_field_1"], |s| {
                s.handle(|_, o: IvoSharedCtxOptions<CtxOptions>| {
                    if true {
                        panic!("[grouped_on_success]: {}", o.messages[0])
                    }

                    ready(())
                })
            })
        },
    );

    let model = schema.model();

    let data = Data {
        dependent: DEFAULT_VALUE,
    };

    let value = Some(data.dependent + 1);

    let (updates, ctx_options, handle_success) = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: value,
                virtual_field_1: None,
            },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(updates, PartialData { dependent: value });
    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_success().await;
}

async_test_matrix!(
    "[grouped_on_success]: ctx_options updated in post_validator",
    should_properly_update_ctx_options_in_post_validators_and_provide_those_updates_in_grouped_on_success_handlers_during_updates
);
