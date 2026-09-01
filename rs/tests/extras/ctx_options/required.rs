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

// ignore_update

async fn should_properly_update_ctx_options_in_ignore_update_resolver_and_provide_those_updates_in_on_success_handlers_during_updates(
) {
    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in ignore_update resolver";

    let data = ignore_update_schema::DataInput {
        required: DEFAULT_VALUE,
    };

    let required = Some(data.required + 1);

    let (updates, ctx_options, handle_success) = ignore_update_schema::DataInputModel
        .update(
            data.clone(),
            ignore_update_schema::PartialDataInput { required },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(updates, ignore_update_schema::PartialDataInput { required });
    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_success();
}

async_test_matrix!(
    "[on_success]: ctx_options updated in ignore_update resolver",
    should_properly_update_ctx_options_in_ignore_update_resolver_and_provide_those_updates_in_on_success_handlers_during_updates
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod ignore_update_schema {
    use super::CtxOptions;

    struct Fields {
        #[required]
        #[validate(|_: i32, _, _| Ok(None))]
        #[ignore_update(async |_, opts| {
            opts.write().await.add_message("ctx_options updated in ignore_update resolver");
            false
        })]
        #[on_success(|_, opts| {
            if true {
                panic!("[on_success]: {}", opts.read_sync().messages[0])
            }
        })]
        pub required: i32,
    }
}

// validate at creation

async fn should_properly_update_ctx_options_in_validators_and_provide_those_updates_in_on_failure_handlers_at_creation(
) {
    const MESSAGE: &str = "ctx_options updated in validator";
    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

    let (failed, ctx_options, handle_failure) = validate_create_schema::DataInputModel
        .create(
            validate_create_schema::PartialDataInput {
                required: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(failed.get("required").unwrap().reason, MIN_LENGTH_ERROR);
    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_failure();
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

    struct Fields {
        #[required]
        #[validate(async |v: String, _, opts| {
            opts.write().await.add_message("ctx_options updated in validator");

            let validated = v.trim();

            if validated.len() < 2 {
                return Err(("expected required to be at least 2 characters long".into(), None));
            }

            Ok(Some(validated.into()))
        })]
        #[on_failure(|_, opts| {
            if true {
                panic!("[on_failure]: {}", opts.read_sync().messages[0])
            }
        })]
        pub required: String,
    }
}

// validate during updates

async fn should_properly_update_ctx_options_in_validators_and_provide_those_updates_in_on_failure_handlers_during_updates(
) {
    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in validator";
    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

    let (errors, ctx_options, handle_failure) = validate_update_schema::DataInputModel
        .update(
            validate_update_schema::DataInput {
                required: DEFAULT_VALUE.into(),
            },
            validate_update_schema::PartialDataInput {
                required: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(
        errors.as_ref().unwrap().get("required").unwrap().reason,
        MIN_LENGTH_ERROR
    );
    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_failure();
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
        #[required]
        #[validate(async |v: String, _, opts| {
            opts.write().await.add_message("ctx_options updated in validator");

            let validated = v.trim();

            if validated.len() < 2 {
                return Err(("expected required to be at least 2 characters long".into(), None));
            }

            Ok(Some(validated.into()))
        })]
        #[on_failure(|_, opts| {
            if true {
                panic!("[on_failure]: {}", opts.read_sync().messages[0])
            }
        })]
        pub required: String,
    }
}

// re_validate at creation

async fn should_properly_update_ctx_options_in_re_validators_and_provide_those_updates_in_on_failure_handlers_at_creation(
) {
    const MESSAGE: &str = "ctx_options updated in re_validator";
    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

    let (failed, ctx_options, handle_failure) = re_validate_create_schema::DataInputModel
        .create(
            re_validate_create_schema::PartialDataInput {
                required: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(failed.get("required").unwrap().reason, MIN_LENGTH_ERROR);
    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_failure();
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

    struct Fields {
        #[required]
        #[validate(|_, _, _| Ok(None))]
        #[re_validate(async |v: String, _, opts| {
            opts.write().await.add_message("ctx_options updated in re_validator");

            let validated = v.trim();

            if validated.len() < 2 {
                return Err(("expected required to be at least 2 characters long".into(), None));
            }

            Ok(Some(validated.into()))
        })]
        #[on_failure(|_, opts| {
            if true {
                panic!("[on_failure]: {}", opts.read_sync().messages[0])
            }
        })]
        pub required: String,
    }
}

// re_validate during updates

async fn should_properly_update_ctx_options_in_re_validators_and_provide_those_updates_in_on_failure_handlers_during_updates(
) {
    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in re_validator";
    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

    let (errors, ctx_options, handle_failure) = re_validate_update_schema::DataInputModel
        .update(
            re_validate_update_schema::DataInput {
                required: DEFAULT_VALUE.into(),
            },
            re_validate_update_schema::PartialDataInput {
                required: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(
        errors.as_ref().unwrap().get("required").unwrap().reason,
        MIN_LENGTH_ERROR
    );
    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_failure();
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
        #[required]
        #[validate(|_, _, _| Ok(None))]
        #[re_validate(async |v: String, _, opts| {
            opts.write().await.add_message("ctx_options updated in re_validator");

            let validated = v.trim();

            if validated.len() < 2 {
                return Err(("expected required to be at least 2 characters long".into(), None));
            }

            Ok(Some(validated.into()))
        })]
        #[on_failure(|_, opts| {
            if true {
                panic!("[on_failure]: {}", opts.read_sync().messages[0])
            }
        })]
        pub required: String,
    }
}

// post_validate & on_success with no fields at creation

async fn should_properly_update_ctx_options_in_post_validators_and_provide_those_updates_in_grouped_on_success_handlers_with_no_fields_at_creation(
) {
    const MESSAGE: &str = "ctx_options updated in post_validator";

    let required = 2;

    let (created, ctx_options, handle_success) = post_validate_create_schema::DataInputModel
        .create(
            post_validate_create_schema::PartialDataInput {
                required: Some(required),
                required_1: Some(required),
            },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        post_validate_create_schema::DataInput {
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

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod post_validate_create_schema {
    use super::CtxOptions;

    struct Fields {
        #[required]
        #[validate(|_, _, _| Ok(None))]
        pub required: i32,

        #[required]
        #[validate(|_, _, _| Ok(None))]
        pub required_1: i32,
    }

    #[post_validate(["required", "required_1"], validate = async |_, opts| {
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
        required: DEFAULT_VALUE,
        required_1: DEFAULT_VALUE,
    };

    let required = Some(data.required + 1);

    let (updates, ctx_options, handle_success) = post_validate_update_schema::DataInputModel
        .update(
            data.clone(),
            post_validate_update_schema::PartialDataInput {
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
        post_validate_update_schema::PartialDataInput {
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

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod post_validate_update_schema {
    use super::CtxOptions;

    struct Fields {
        #[required]
        #[validate(|_, _, _| Ok(None))]
        pub required: i32,

        #[required]
        #[validate(|_, _, _| Ok(None))]
        pub required_1: i32,
    }

    #[post_validate(["required", "required_1"], validate = async |_, opts| {
        opts.write().await.add_message("ctx_options updated in post_validator");
        Ok(None)
    })]
    #[on_success(["required", "required_1"], async |_, opts| {
        panic!("[grouped_on_success]: {}", opts.read_sync().messages[0])
    })]
    const _: () = ();
}
