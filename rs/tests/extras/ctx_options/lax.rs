use ivo::ivo_schema;

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

// default_fn

async fn should_properly_update_ctx_options_in_default_resolver_and_provide_those_updates_in_on_success_handlers(
) {
    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in default value resolver";

    let created = default_fn_schema::DataInputModel
        .create(
            default_fn_schema::PartialDataInput { lax: None },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        default_fn_schema::DataInput { lax: DEFAULT_VALUE }
    );
    assert_eq!(created.ctx_options.read().await.messages[0], MESSAGE);

    created.handle_success();
}

async_test_matrix!(
    "[on_success]: ctx_options updated in default value resolver",
    should_properly_update_ctx_options_in_default_resolver_and_provide_those_updates_in_on_success_handlers
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod default_fn_schema {
    use super::CtxOptions;

    struct Fields {
        #[lax(async |_, opts| {
            opts.write().await.add_message("ctx_options updated in default value resolver");
            1
        })]
        #[validate(|_, _, _| Ok(None))]
        #[on_success(|_, opts| {
            if true {
                panic!("[on_success]: {}", opts.read_sync().messages[0])
            }
        })]
        pub lax: i32,
    }
}

// ignore at creation

async fn should_properly_update_ctx_options_in_ignore_resolver_and_provide_those_updates_in_on_success_handlers_at_creation(
) {
    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in ignore resolver";

    let lax = DEFAULT_VALUE + 1;

    let created = ignore_create_schema::DataInputModel
        .create(
            ignore_create_schema::PartialDataInput { lax: Some(lax) },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(created.data, ignore_create_schema::DataInput { lax });
    assert_eq!(created.ctx_options.read().await.messages[0], MESSAGE);

    created.handle_success();
}

async_test_matrix!(
    "[on_success]: ctx_options updated in ignore resolver",
    should_properly_update_ctx_options_in_ignore_resolver_and_provide_those_updates_in_on_success_handlers_at_creation
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod ignore_create_schema {
    use super::CtxOptions;

    struct Fields {
        #[lax(1)]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(async |_, opts| {
            opts.write().await.add_message("ctx_options updated in ignore resolver");
            false
        })]
        #[on_success(|_, opts| {
            if true {
                panic!("[on_success]: {}", opts.read_sync().messages[0])
            }
        })]
        pub lax: i32,
    }
}

// ignore during updates

async fn should_properly_update_ctx_options_in_ignore_resolver_and_provide_those_updates_in_on_success_handlers_during_updates(
) {
    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in ignore resolver";

    let data = ignore_update_schema::DataInput { lax: DEFAULT_VALUE };

    let lax = Some(data.lax + 1);

    let updated = ignore_update_schema::DataInputModel
        .update(
            data.clone(),
            ignore_update_schema::PartialDataInput { lax },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(updated.data, ignore_update_schema::PartialDataInput { lax });
    assert_eq!(updated.ctx_options.read().await.messages[0], MESSAGE);

    updated.handle_success();
}

async_test_matrix!(
    "[on_success]: ctx_options updated in ignore resolver",
    should_properly_update_ctx_options_in_ignore_resolver_and_provide_those_updates_in_on_success_handlers_during_updates
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod ignore_update_schema {
    use super::CtxOptions;

    struct Fields {
        #[lax(1)]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(async |_, opts| {
            opts.write().await.add_message("ctx_options updated in ignore resolver");
            false
        })]
        #[on_success(|_, opts| {
            if true {
                panic!("[on_success]: {}", opts.read_sync().messages[0])
            }
        })]
        pub lax: i32,
    }
}

// required at creation

async fn should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_at_creation(
) {
    use required_create_schema::{DataInputModel, MESSAGE, REQUIRED_ERROR};

    let failed = DataInputModel
        .create(
            required_create_schema::PartialDataInput { lax: None },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(failed.errors.get("lax").unwrap().reason, REQUIRED_ERROR);
    assert_eq!(failed.ctx_options.read().await.messages[0], MESSAGE);
}

async_test_matrix!(
    should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_at_creation
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod required_create_schema {
    use super::CtxOptions;

    const DEFAULT_VALUE: i32 = 1;
    pub const MESSAGE: &str = "ctx_options updated in ignore resolver";
    pub const REQUIRED_ERROR: &str = "lax is missing!";

    struct Fields {
        #[lax(DEFAULT_VALUE)]
        #[required(async |_, opts| {
            opts.write().await.add_message(MESSAGE.into());
            Some(REQUIRED_ERROR.into())
        })]
        pub lax: i32,
    }
}

// required during updates

async fn should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_during_updates(
) {
    use required_update_schema::{
        DataInput, DataInputModel, PartialDataInput, DEFAULT_VALUE, MESSAGE, REQUIRED_ERROR,
    };

    let failed = DataInputModel
        .update(
            DataInput {
                lax: DEFAULT_VALUE,
                lax_1: DEFAULT_VALUE,
            },
            PartialDataInput {
                lax: None,
                lax_1: Some(DEFAULT_VALUE + 1),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(
        failed.errors.as_ref().unwrap().get("lax").unwrap().reason,
        REQUIRED_ERROR
    );
    assert_eq!(failed.ctx_options.read().await.messages[0], MESSAGE);
}

async_test_matrix!(
    should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_during_updates
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod required_update_schema {
    use super::CtxOptions;

    pub const DEFAULT_VALUE: i32 = 1;
    pub const MESSAGE: &str = "ctx_options updated in ignore resolver";
    pub const REQUIRED_ERROR: &str = "lax is missing!";

    struct Fields {
        #[lax(DEFAULT_VALUE)]
        #[required(async |_, opts| {
            opts.write().await.add_message(MESSAGE);
            Some(REQUIRED_ERROR.into())
        })]
        pub lax: i32,

        #[lax(DEFAULT_VALUE)]
        pub lax_1: i32,
    }
}

// validate at creation

async fn should_properly_update_ctx_options_in_validators_and_provide_those_updates_in_on_failure_handlers_at_creation(
) {
    use validate_create_schema::{DataInputModel, PartialDataInput, MESSAGE, MIN_LENGTH_ERROR};

    let failed = DataInputModel
        .create(
            PartialDataInput {
                lax: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(failed.errors.get("lax").unwrap().reason, MIN_LENGTH_ERROR);
    assert_eq!(failed.ctx_options.read().await.messages[0], MESSAGE);

    failed.handle_failure();
}

async_test_matrix!(
    "[on_failure]: ctx_options updated in validator",
    should_properly_update_ctx_options_in_validators_and_provide_those_updates_in_on_failure_handlers_at_creation
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod validate_create_schema {
    use super::CtxOptions;

    const DEFAULT_VALUE: &str = "default_value";
    pub const MESSAGE: &str = "ctx_options updated in validator";
    pub const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    struct Fields {
        #[lax(DEFAULT_VALUE.into())]
        #[validate(async |v: String, _, opts| {
            opts.write().await.add_message(MESSAGE);

            let validated = v.trim();

            if validated.len() < 2 {
                return Err((MIN_LENGTH_ERROR.into(), None));
            }

            Ok(Some(validated.into()))
        })]
        #[on_failure(|_, opts| {
            if true {
                panic!("[on_failure]: {}", opts.read_sync().messages[0])
            }
        })]
        pub lax: String,
    }
}

// validate during updates

async fn should_properly_update_ctx_options_in_validators_and_provide_those_updates_in_on_failure_handlers_during_updates(
) {
    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in validator";
    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    let failed = validate_update_schema::DataInputModel
        .update(
            validate_update_schema::DataInput {
                lax: DEFAULT_VALUE.into(),
            },
            validate_update_schema::PartialDataInput {
                lax: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(
        failed.errors.as_ref().unwrap().get("lax").unwrap().reason,
        MIN_LENGTH_ERROR
    );
    assert_eq!(failed.ctx_options.read().await.messages[0], MESSAGE);

    failed.handle_failure();
}

async_test_matrix!(
    "[on_failure]: ctx_options updated in validator",
    should_properly_update_ctx_options_in_validators_and_provide_those_updates_in_on_failure_handlers_during_updates
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod validate_update_schema {
    use super::CtxOptions;

    struct Fields {
        #[lax("default_value".into())]
        #[validate(async |v: String, _, opts| {
            opts.write().await.add_message("ctx_options updated in validator");

            let validated = v.trim();

            if validated.len() < 2 {
                return Err(("expected lax to be at least 2 characters long".into(), None));
            }

            Ok(Some(validated.into()))
        })]
        #[on_failure(|_, opts| {
            if true {
                panic!("[on_failure]: {}", opts.read_sync().messages[0])
            }
        })]
        pub lax: String,
    }
}

// re_validate at creation

async fn should_properly_update_ctx_options_in_re_validators_and_provide_those_updates_in_on_failure_handlers_at_creation(
) {
    use re_validate_create_schema::{DataInputModel, PartialDataInput, MESSAGE, MIN_LENGTH_ERROR};

    let failed = DataInputModel
        .create(
            PartialDataInput {
                lax: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(failed.errors.get("lax").unwrap().reason, MIN_LENGTH_ERROR);
    assert_eq!(failed.ctx_options.read().await.messages[0], MESSAGE);

    failed.handle_failure();
}

async_test_matrix!(
    "[on_failure]: ctx_options updated in re_validator",
    should_properly_update_ctx_options_in_re_validators_and_provide_those_updates_in_on_failure_handlers_at_creation
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod re_validate_create_schema {
    use super::CtxOptions;

    const DEFAULT_VALUE: &str = "default_value";
    pub const MESSAGE: &str = "ctx_options updated in re_validator";
    pub const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    struct Fields {
        #[lax(DEFAULT_VALUE.into())]
        #[validate(|_, _, _| Ok(None))]
        #[re_validate(async |v: String, _, opts| {
            opts.write().await.add_message(MESSAGE);

            let validated = v.trim();

            if validated.len() < 2 {
                return Err((MIN_LENGTH_ERROR.into(), None));
            }

            Ok(Some(validated.into()))
        })]
        #[on_failure(|_, opts| {
            if true {
                panic!("[on_failure]: {}", opts.read_sync().messages[0])
            }
        })]
        pub lax: String,
    }
}

// re_validate during updates

async fn should_properly_update_ctx_options_in_re_validators_and_provide_those_updates_in_on_failure_handlers_during_updates(
) {
    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in re_validator";
    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    let failed = re_validate_update_schema::DataInputModel
        .update(
            re_validate_update_schema::DataInput {
                lax: DEFAULT_VALUE.into(),
            },
            re_validate_update_schema::PartialDataInput {
                lax: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(
        failed.errors.as_ref().unwrap().get("lax").unwrap().reason,
        MIN_LENGTH_ERROR
    );
    assert_eq!(failed.ctx_options.read().await.messages[0], MESSAGE);

    failed.handle_failure();
}

async_test_matrix!(
    "[on_failure]: ctx_options updated in re_validator",
    should_properly_update_ctx_options_in_re_validators_and_provide_those_updates_in_on_failure_handlers_during_updates
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod re_validate_update_schema {
    use super::CtxOptions;

    struct Fields {
        #[lax("default_value".into())]
        #[validate(|_, _, _| Ok(None))]
        #[re_validate(async |v: String, _, opts| {
            opts.write().await.add_message("ctx_options updated in re_validator");

            let validated = v.trim();

            if validated.len() < 2 {
                return Err(("expected lax to be at least 2 characters long".into(), None));
            }

            Ok(Some(validated.into()))
        })]
        #[on_failure(|_, opts| {
            if true {
                panic!("[on_failure]: {}", opts.read_sync().messages[0])
            }
        })]
        pub lax: String,
    }
}

// post_validate & on_success with no fields at creation

async fn should_properly_update_ctx_options_in_post_validators_and_provide_those_updates_in_grouped_on_success_handlers_with_no_fields_at_creation(
) {
    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in post_validator";

    let lax = DEFAULT_VALUE + 1;

    let created = post_validate_create_schema::DataInputModel
        .create(
            post_validate_create_schema::PartialDataInput {
                lax: Some(lax),
                lax_1: None,
            },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        post_validate_create_schema::DataInput {
            lax,
            lax_1: DEFAULT_VALUE
        }
    );
    assert_eq!(created.ctx_options.read().await.messages[0], MESSAGE);

    created.handle_success().await;
}

async_test_matrix!(
    "[grouped_on_success]: ctx_options updated in post_validator",
    should_properly_update_ctx_options_in_post_validators_and_provide_those_updates_in_grouped_on_success_handlers_with_no_fields_at_creation
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod post_validate_create_schema {
    use super::CtxOptions;

    struct Fields {
        #[lax(1)]
        #[validate(|_, _, _| Ok(None))]
        pub lax: i32,

        #[lax(1)]
        #[validate(|_, _, _| Ok(None))]
        pub lax_1: i32,
    }

    #[post_validate(["lax", "lax_1"], validate = async |_, opts| {
        opts.write().await.add_message("ctx_options updated in post_validator");
        Ok(None)
    })]
    #[on_success(async |_, opts| {
        panic!("[grouped_on_success]: {}", opts.read_sync().messages[0])
    })]
    const _: () = ();
}

// post_validate & on_success with fields during updates

async fn should_properly_update_ctx_options_in_post_validators_and_provide_those_updates_in_grouped_on_success_handlers_during_updates(
) {
    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in post_validator";

    let data = post_validate_update_schema::DataInput {
        lax: DEFAULT_VALUE,
        lax_1: DEFAULT_VALUE,
    };

    let lax = Some(data.lax + 1);

    let updated = post_validate_update_schema::DataInputModel
        .update(
            data.clone(),
            post_validate_update_schema::PartialDataInput { lax, lax_1: None },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        post_validate_update_schema::PartialDataInput { lax, lax_1: None }
    );
    assert_eq!(updated.ctx_options.read().await.messages[0], MESSAGE);

    updated.handle_success().await;
}

async_test_matrix!(
    "[grouped_on_success]: ctx_options updated in post_validator",
    should_properly_update_ctx_options_in_post_validators_and_provide_those_updates_in_grouped_on_success_handlers_during_updates
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod post_validate_update_schema {
    use super::CtxOptions;

    struct Fields {
        #[lax(1)]
        #[validate(|_, _, _| Ok(None))]
        pub lax: i32,

        #[lax(1)]
        #[validate(|_, _, _| Ok(None))]
        pub lax_1: i32,
    }

    #[post_validate(["lax", "lax_1"], validate = async |_, opts| {
        opts.write().await.add_message("ctx_options updated in post_validator");
        Ok(None)
    })]
    #[on_success(["lax", "lax_1"], async |_, opts| {
        panic!("[grouped_on_success]: {}", opts.read_sync().messages[0])
    })]
    const _: () = ();
}
