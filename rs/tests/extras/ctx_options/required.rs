use std::future::ready;

use ivo::{required_field, IvoCtxOptions, IvoInputStruct, IvoModel, IvoRwCtxOptions, IvoStruct};

use crate::async_test_matrix;

// TODO:
// [x] ignore_update
// [x] validate
// [x] re_validate
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

// ignore_update

async fn should_properly_update_ctx_options_in_ignore_update_resolver_and_provide_those_updates_in_on_success_handlers_during_updates(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: i32,
    }

    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in ignore_update resolver";

    let model = IvoModel::<DataInput, Data, CtxOptions>::new(
        |f| {
            f.field(
                required_field("required")
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .ignore_update(async |_, _, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        false
                    })
                    .on_success(|_, o: IvoCtxOptions<CtxOptions>| {
                        if true {
                            panic!("[on_success]: {}", o.messages[0])
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let data = Data {
        required: DEFAULT_VALUE,
    };

    let required = Some(data.required + 1);

    let (data, handle_success, ctx_options) = model
        .update(&data, &PartialDataInput { required }, CtxOptions::new())
        .await
        .ok()
        .unwrap();

    assert_eq!(data, PartialData { required });
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
        required: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: String,
    }

    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in validator";
    const REQUIRED_ERROR: &str = "required is missing!";

    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

    let model: IvoModel<DataInput, Data, CtxOptions> = IvoModel::new(
        |f| {
            f.field(
                required_field("required")
                    .validate(async |v: String, _, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        let validated = v.trim();

                        if validated.len() < 2 {
                            return Err((MIN_LENGTH_ERROR.into(), None));
                        }

                        Ok(Some(validated.into()))
                    })
                    .on_failure(|_, o: IvoCtxOptions<CtxOptions>| {
                        if true {
                            panic!("[on_failure]: {}", o.messages[0])
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let (err, handle_failure, ctx_options) = model
        .create(
            &PartialDataInput {
                required: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(err.get("required").unwrap().reason, MIN_LENGTH_ERROR);
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
        required: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: String,
    }

    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in validator";
    const REQUIRED_ERROR: &str = "required is missing!";

    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

    let model: IvoModel<DataInput, Data, CtxOptions> = IvoModel::new(
        |f| {
            f.field(
                required_field("required")
                    .validate(async |v: String, _, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        let validated = v.trim();

                        if validated.len() < 2 {
                            return Err((MIN_LENGTH_ERROR.into(), None));
                        }

                        Ok(Some(validated.into()))
                    })
                    .on_failure(|_, o: IvoCtxOptions<CtxOptions>| {
                        if true {
                            panic!("[on_failure]: {}", o.messages[0])
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let (err, handle_failure, ctx_options) = model
        .update(
            &Data {
                required: DEFAULT_VALUE.into(),
            },
            &PartialDataInput {
                required: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    match err {
        Some(payload) => {
            assert_eq!(payload.get("required").unwrap().reason, MIN_LENGTH_ERROR);
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
        required: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: String,
    }

    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in re_validator";
    const REQUIRED_ERROR: &str = "required is missing!";

    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

    let model: IvoModel<DataInput, Data, CtxOptions> = IvoModel::new(
        |f| {
            f.field(
                required_field("required")
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
                    .on_failure(|_, o: IvoCtxOptions<CtxOptions>| {
                        if true {
                            panic!("[on_failure]: {}", o.messages[0])
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let (err, handle_failure, ctx_options) = model
        .create(
            &PartialDataInput {
                required: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(err.get("required").unwrap().reason, MIN_LENGTH_ERROR);
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
        required: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: String,
    }

    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in re_validator";
    const REQUIRED_ERROR: &str = "required is missing!";

    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

    let model: IvoModel<DataInput, Data, CtxOptions> = IvoModel::new(
        |f| {
            f.field(
                required_field("required")
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
                    .on_failure(|_, o: IvoCtxOptions<CtxOptions>| {
                        if true {
                            panic!("[on_failure]: {}", o.messages[0])
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let (err, handle_failure, ctx_options) = model
        .update(
            &Data {
                required: DEFAULT_VALUE.into(),
            },
            &PartialDataInput {
                required: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    match err {
        Some(payload) => {
            assert_eq!(payload.get("required").unwrap().reason, MIN_LENGTH_ERROR);
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

// o.post_validate & o.on_success

async fn should_properly_update_ctx_options_in_post_validators_and_provide_those_updates_in_grouped_on_success_handlers_with_no_fields_at_creation(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
        required_1: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: i32,
        required_1: i32,
    }

    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in post_validator";
    const REQUIRED_ERROR: &str = "required is missing!";

    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

    let model: IvoModel<DataInput, Data, CtxOptions> = IvoModel::new(
        |f| {
            f.field(required_field("required").validate(|_: i32, _, _| ready(Ok(None))))
                .field(required_field("required_1").validate(|_: i32, _, _| ready(Ok(None))))
        },
        |o| {
            o.post_validate(["required", "required_1"], |v| {
                v.validate(async |_, o: IvoRwCtxOptions<CtxOptions>| {
                    let mut ctx_options = o.write().await;

                    ctx_options.add_message(MESSAGE);

                    Ok(None)
                })
            })
            .on_success([], |s| {
                s.handle(|_, o: IvoCtxOptions<CtxOptions>| {
                    if true {
                        panic!("[grouped_on_success]: {}", o.messages[0])
                    }

                    ready(())
                })
            })
        },
    );

    let required = DEFAULT_VALUE + 1;

    let (data, handle_success, ctx_options) = model
        .create(
            &PartialDataInput {
                required: Some(required),
                required_1: Some(required),
            },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            required,
            required_1: required
        }
    );
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
        required: i32,
        required_1: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: i32,
        required_1: i32,
    }

    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in post_validator";
    const REQUIRED_ERROR: &str = "required is missing!";

    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

    let model: IvoModel<DataInput, Data, CtxOptions> = IvoModel::new(
        |f| {
            f.field(required_field("required").validate(|_: i32, _, _| ready(Ok(None))))
                .field(required_field("required_1").validate(|_: i32, _, _| ready(Ok(None))))
        },
        |o| {
            o.post_validate(["required", "required_1"], |v| {
                v.validate(async |_, o: IvoRwCtxOptions<CtxOptions>| {
                    let mut ctx_options = o.write().await;

                    ctx_options.add_message(MESSAGE);

                    Ok(None)
                })
            })
            .on_success(["required", "required_1"], |s| {
                s.handle(|_, o: IvoCtxOptions<CtxOptions>| {
                    if true {
                        panic!("[grouped_on_success]: {}", o.messages[0])
                    }

                    ready(())
                })
            })
        },
    );

    let data = Data {
        required: DEFAULT_VALUE,
        required_1: DEFAULT_VALUE,
    };

    let required = Some(data.required + 1);

    let (updates, handle_success, ctx_options) = model
        .update(
            &data,
            &PartialDataInput {
                required,
                required_1: None,
            },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            required,
            required_1: None
        }
    );
    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_success().await;
}

async_test_matrix!(
    "[grouped_on_success]: ctx_options updated in post_validator",
    should_properly_update_ctx_options_in_post_validators_and_provide_those_updates_in_grouped_on_success_handlers_during_updates
);
