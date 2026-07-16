use std::future::ready;

use ivo::{IvoCtxOptions, IvoField, IvoInputStruct, IvoRwCtxOptions, IvoStruct, Model};

use crate::async_test_matrix;

// TODO:
// [x] default_fn
// [x] ignore
// [x] required
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

//  default_fn
async fn should_properly_update_ctx_options_in_default_resolver_and_provide_those_updates_in_on_success_handlers(
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
    const MESSAGE: &str = "ctx_options updated in default value resolver";

    let model = Model::<DataInput, Data, CtxOptions>::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default_fn(async |_, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        DEFAULT_VALUE
                    })
                    .validate(|_: i32, _, _| ready(Ok(None)))
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

    let (data, handle_success, ctx_options) = model
        .create(&PartialDataInput { lax: None }, CtxOptions::new())
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { lax: DEFAULT_VALUE });

    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_success().await;
}

async_test_matrix!(
    "[on_success]: ctx_options updated in default value resolver",
    should_properly_update_ctx_options_in_default_resolver_and_provide_those_updates_in_on_success_handlers
);

// ignore

async fn should_properly_update_ctx_options_in_ignore_resolver_and_provide_those_updates_in_on_success_handlers_at_creation(
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
    const MESSAGE: &str = "ctx_options updated in ignore resolver";

    let model = Model::<DataInput, Data, CtxOptions>::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_VALUE)
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .ignore(async |_, o: IvoRwCtxOptions<CtxOptions>| {
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

    let lax = DEFAULT_VALUE + 1;

    let (data, handle_success, ctx_options) = model
        .create(&PartialDataInput { lax: Some(lax) }, CtxOptions::new())
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { lax });

    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_success().await;
}

async_test_matrix!(
    "[on_success]: ctx_options updated in ignore resolver",
    should_properly_update_ctx_options_in_ignore_resolver_and_provide_those_updates_in_on_success_handlers_at_creation
);

async fn should_properly_update_ctx_options_in_ignore_resolver_and_provide_those_updates_in_on_success_handlers_during_updates(
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
    const MESSAGE: &str = "ctx_options updated in ignore resolver";

    let model = Model::<DataInput, Data, CtxOptions>::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_VALUE)
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .ignore(async |_, o: IvoRwCtxOptions<CtxOptions>| {
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

    let data = Data { lax: DEFAULT_VALUE };

    let lax = Some(data.lax + 1);

    let (data, handle_success, ctx_options) = model
        .update(&data, &PartialDataInput { lax }, CtxOptions::new())
        .await
        .ok()
        .unwrap();

    assert_eq!(data, PartialData { lax });
    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_success().await;
}

async_test_matrix!(
    "[on_success]: ctx_options updated in ignore resolver",
    should_properly_update_ctx_options_in_ignore_resolver_and_provide_those_updates_in_on_success_handlers_during_updates
);

// required

async fn should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_at_creation(
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
    const MESSAGE: &str = "ctx_options updated in ignore resolver";
    const REQUIRED_ERROR: &str = "lax is missing!";

    let model = Model::<DataInput, Data, CtxOptions>::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_VALUE)
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .required(async |_, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        Some(REQUIRED_ERROR.into())
                    }),
            )
        },
        |o| o,
    );

    let (err, _, ctx_options) = model
        .create(&PartialDataInput { lax: None }, CtxOptions::new())
        .await
        .err()
        .unwrap();

    assert_eq!(err.get("lax").unwrap().reason, REQUIRED_ERROR);
    assert_eq!(ctx_options.messages[0], MESSAGE);
}

async_test_matrix!(
    should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_at_creation
);

async fn should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_during_updates(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in ignore resolver";
    const REQUIRED_ERROR: &str = "lax is missing!";

    let model = Model::<DataInput, Data, CtxOptions>::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_VALUE)
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .required(async |_, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        Some(REQUIRED_ERROR.into())
                    }),
            )
            .field(
                "lax_1",
                IvoField::LAX
                    .default(DEFAULT_VALUE)
                    .validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let (err, _, ctx_options) = model
        .update(
            &Data {
                lax: DEFAULT_VALUE,
                lax_1: DEFAULT_VALUE,
            },
            &PartialDataInput {
                lax: None,
                lax_1: Some(DEFAULT_VALUE + 1),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    match err {
        Some(payload) => {
            assert_eq!(payload.get("lax").unwrap().reason, REQUIRED_ERROR);
        }
        _ => unreachable!("expected a validation error"),
    }

    assert_eq!(ctx_options.messages[0], MESSAGE);
}

async_test_matrix!(
    should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_during_updates
);

// validate

async fn should_properly_update_ctx_options_in_validators_and_provide_those_updates_in_on_failure_handlers_at_creation(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
    }

    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in validator";
    const REQUIRED_ERROR: &str = "lax is missing!";

    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    let model: Model<DataInput, Data, CtxOptions> = Model::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_VALUE.into())
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
                lax: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(err.get("lax").unwrap().reason, MIN_LENGTH_ERROR);
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
        lax: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
    }

    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in validator";
    const REQUIRED_ERROR: &str = "lax is missing!";

    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    let model: Model<DataInput, Data, CtxOptions> = Model::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_VALUE.into())
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
                lax: DEFAULT_VALUE.into(),
            },
            &PartialDataInput {
                lax: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    match err {
        Some(payload) => {
            assert_eq!(payload.get("lax").unwrap().reason, MIN_LENGTH_ERROR);
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
        lax: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
    }

    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in re_validator";
    const REQUIRED_ERROR: &str = "lax is missing!";

    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    let model: Model<DataInput, Data, CtxOptions> = Model::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_VALUE.into())
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
                lax: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(err.get("lax").unwrap().reason, MIN_LENGTH_ERROR);
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
        lax: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
    }

    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in re_validator";
    const REQUIRED_ERROR: &str = "lax is missing!";

    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    let model: Model<DataInput, Data, CtxOptions> = Model::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_VALUE.into())
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
                lax: DEFAULT_VALUE.into(),
            },
            &PartialDataInput {
                lax: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    match err {
        Some(payload) => {
            assert_eq!(payload.get("lax").unwrap().reason, MIN_LENGTH_ERROR);
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
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in post_validator";
    const REQUIRED_ERROR: &str = "lax is missing!";

    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    let model: Model<DataInput, Data, CtxOptions> = Model::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_VALUE)
                    .validate(|_, _, _| ready(Ok(None))),
            )
            .field(
                "lax_1",
                IvoField::LAX
                    .default(DEFAULT_VALUE)
                    .validate(|_, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.post_validate(["lax", "lax_1"], |v| {
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

    let lax = DEFAULT_VALUE + 1;

    let (data, handle_success, ctx_options) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                lax_1: None,
            },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax,
            lax_1: DEFAULT_VALUE
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
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in post_validator";
    const REQUIRED_ERROR: &str = "lax is missing!";

    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    let model: Model<DataInput, Data, CtxOptions> = Model::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_VALUE)
                    .validate(|_, _, _| ready(Ok(None))),
            )
            .field(
                "lax_1",
                IvoField::LAX
                    .default(DEFAULT_VALUE)
                    .validate(|_, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.post_validate(["lax", "lax_1"], |v| {
                v.validate(async |_, o: IvoRwCtxOptions<CtxOptions>| {
                    let mut ctx_options = o.write().await;

                    ctx_options.add_message(MESSAGE);

                    Ok(None)
                })
            })
            .on_success(["lax", "lax_1"], |s| {
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
        lax: DEFAULT_VALUE,
        lax_1: DEFAULT_VALUE,
    };

    let lax = Some(data.lax + 1);

    let (updates, handle_success, ctx_options) = model
        .update(
            &data,
            &PartialDataInput { lax, lax_1: None },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(updates, PartialData { lax, lax_1: None });
    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_success().await;
}

async_test_matrix!(
    "[grouped_on_success]: ctx_options updated in post_validator",
    should_properly_update_ctx_options_in_post_validators_and_provide_those_updates_in_grouped_on_success_handlers_during_updates
);
