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

// required at creation

async fn should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_at_creation(
) {
    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in ignore resolver";
    const REQUIRED_ERROR: &str = "virtual_field is missing!";

    let failed = required_create_schema::DataModel
        .create(
            required_create_schema::PartialDataInput {
                virtual_field: None,
                virtual_field_1: Some(1),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(
        failed.errors.get("virtual_field").unwrap().reason,
        REQUIRED_ERROR
    );
    assert_eq!(failed.ctx_options.read().await.messages[0], MESSAGE);
}

async_test_matrix!(
    should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_at_creation
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod required_create_schema {
    use super::CtxOptions;

    struct Fields {
        #[depends_on(virtual_field, virtual_field_1)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_field.unwrap_or(0) + 1)]
        pub dependent: i32,

        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[required(async |_, opts| {
            opts.write().await.add_message("ctx_options updated in ignore resolver");
            Some("virtual_field is missing!".into())
        })]
        pub virtual_field: i32,

        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field_1: i32,
    }
}

// required during updates

async fn should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_during_updates(
) {
    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in ignore resolver";
    const REQUIRED_ERROR: &str = "virtual_field is missing!";

    let failed = required_update_schema::DataModel
        .update(
            required_update_schema::Data {
                dependent: DEFAULT_VALUE,
            },
            required_update_schema::PartialDataInput {
                virtual_field: None,
                virtual_field_1: Some(1),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(
        failed
            .errors
            .as_ref()
            .unwrap()
            .get("virtual_field")
            .unwrap()
            .reason,
        REQUIRED_ERROR
    );
    assert_eq!(failed.ctx_options.read().await.messages[0], MESSAGE);
}

async_test_matrix!(
    should_properly_update_ctx_options_in_required_resolver_and_provide_those_updates_in_on_failure_handlers_during_updates
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod required_update_schema {
    use super::CtxOptions;

    struct Fields {
        #[depends_on(virtual_field, virtual_field_1)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_field.unwrap_or(0) + 1)]
        pub dependent: i32,

        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[required(async |_, opts| {
            opts.write().await.add_message("ctx_options updated in ignore resolver");
            Some("virtual_field is missing!".into())
        })]
        pub virtual_field: i32,

        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field_1: i32,
    }
}

// ignore_update during updates

async fn should_properly_update_ctx_options_in_ignore_update_resolver_and_provide_those_updates_in_on_success_handlers_during_updates(
) {
    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in ignore_update resolver";

    let data = ignore_update_schema::Data {
        dependent: DEFAULT_VALUE,
    };

    let value = Some(data.dependent + 1);

    let updated = ignore_update_schema::DataModel
        .update(
            data.clone(),
            ignore_update_schema::PartialDataInput {
                virtual_field: value,
            },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        ignore_update_schema::PartialData {
            dependent: value.map(|v| v + 1)
        }
    );
    assert_eq!(updated.ctx_options.read().await.messages[0], MESSAGE);

    updated.handle_success();
}

async_test_matrix!(
    "[on_success]: ctx_options updated in ignore_update resolver",
    should_properly_update_ctx_options_in_ignore_update_resolver_and_provide_those_updates_in_on_success_handlers_during_updates
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod ignore_update_schema {
    use super::CtxOptions;

    struct Fields {
        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_field.unwrap() + 1)]
        pub dependent: i32,

        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(async |_, opts| {
            opts.write().await.add_message("ctx_options updated in ignore_update resolver");
            false
        })]
        #[on_success(|_, opts| {
            if true {
                panic!("[on_success]: {}", opts.read_sync().messages[0])
            }
        })]
        pub virtual_field: i32,
    }
}

// validate at creation

async fn should_properly_update_ctx_options_in_validators_and_provide_those_updates_in_on_failure_handlers_at_creation(
) {
    const DEFAULT_VALUE: &str = "default_value";
    const MESSAGE: &str = "ctx_options updated in validator";
    const MIN_LENGTH_ERROR: &str = "expected virtual_field to be at least 2 characters long";

    let failed = validate_create_schema::DataModel
        .create(
            validate_create_schema::PartialDataInput {
                virtual_field: Some(String::from(" ")),
            },
            CtxOptions::new(),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(
        failed.errors.get("virtual_field").unwrap().reason,
        MIN_LENGTH_ERROR
    );
    assert_eq!(failed.ctx_options.read().await.messages[0], MESSAGE);

    failed.handle_failure();
}

async_test_matrix!(
    "[on_failure]: ctx_options updated in validator",
    should_properly_update_ctx_options_in_validators_and_provide_those_updates_in_on_failure_handlers_at_creation
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod validate_create_schema {
    use super::CtxOptions;

    struct Fields {
        #[depends_on(virtual_field)]
        #[default("default_value".into())]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap())]
        pub dependent: String,

        #[ivo_virtual]
        #[validate(async |v: String, _, opts| {
            opts.write().await.add_message("ctx_options updated in validator");

            let validated = v.trim();

            if validated.len() < 2 {
                return Err(("expected virtual_field to be at least 2 characters long".into(), None));
            }

            Ok(Some(validated.into()))
        })]
        #[on_failure(|_, opts| {
            if true {
                panic!("[on_failure]: {}", opts.read_sync().messages[0])
            }
        })]
        pub virtual_field: String,
    }
}

// sanitize at creation

async fn should_properly_update_ctx_options_in_sanitizers_and_provide_those_updates_on_success_handlers_at_creation(
) {
    let _default_dependent_value = "default_dependent_value";
    const MESSAGE: &str = "ctx_options updated in sanitizer";

    fn sanitize(value: &str) -> String {
        format!("sanitized-{value}")
    }

    let virtual_value = "virtual_value".to_string();

    let created = sanitize_create_schema::DataModel
        .create(
            sanitize_create_schema::PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
            },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sanitize_create_schema::Data {
            dependent: sanitize(&virtual_value)
        }
    );
    assert_ne!(
        created.data,
        sanitize_create_schema::Data {
            dependent: virtual_value
        }
    );
    assert_eq!(created.ctx_options.read().await.messages[0], MESSAGE);

    created.handle_success();
}

async_test_matrix!(
    "[on_success]: ctx_options updated in sanitizer",
    should_properly_update_ctx_options_in_sanitizers_and_provide_those_updates_on_success_handlers_at_creation
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod sanitize_create_schema {
    use super::CtxOptions;

    struct Fields {
        #[depends_on(virtual_field)]
        #[default("default_dependent_value".into())]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap())]
        pub dependent: String,

        #[ivo_virtual]
        #[validate(|_: String, _, _| Ok(None))]
        #[sanitize(async |value: String, _, opts| {
            opts.write().await.add_message("ctx_options updated in sanitizer");
            format!("sanitized-{value}")
        })]
        #[on_success(|_, opts| {
            if true {
                panic!("[on_success]: {}", opts.read_sync().messages[0])
            }
        })]
        pub virtual_field: String,
    }
}

// post_validate & on_success with no fields at creation

async fn should_properly_update_ctx_options_in_post_validators_and_provide_those_updates_in_grouped_on_success_handlers_with_no_fields_at_creation(
) {
    const DEFAULT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in post_validator";

    let value = DEFAULT_VALUE + 1;

    let created = post_validate_create_schema::DataModel
        .create(
            post_validate_create_schema::PartialDataInput {
                virtual_field: Some(value),
                virtual_field_1: Some(value),
            },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        post_validate_create_schema::Data { dependent: value }
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
    output(Data, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod post_validate_create_schema {
    use super::CtxOptions;

    struct Fields {
        #[depends_on(virtual_field, virtual_field_1)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_field.unwrap())]
        pub dependent: i32,

        #[ivo_virtual]
        #[validate(|_: i32, _, _| Ok(None))]
        pub virtual_field: i32,

        #[ivo_virtual]
        #[validate(|_: i32, _, _| Ok(None))]
        pub virtual_field_1: i32,
    }

    #[post_validate(["virtual_field", "virtual_field_1"], validate = async |_, opts| {
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

    let data = post_validate_update_schema::Data {
        dependent: DEFAULT_VALUE,
    };

    let value = Some(data.dependent + 1);

    let updated = post_validate_update_schema::DataModel
        .update(
            data.clone(),
            post_validate_update_schema::PartialDataInput {
                virtual_field: value,
                virtual_field_1: None,
            },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        post_validate_update_schema::PartialData { dependent: value }
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
    output(Data, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod post_validate_update_schema {
    use super::CtxOptions;

    struct Fields {
        #[depends_on(virtual_field, virtual_field_1)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_field.unwrap())]
        pub dependent: i32,

        #[ivo_virtual]
        #[validate(|_: i32, _, _| Ok(None))]
        pub virtual_field: i32,

        #[ivo_virtual]
        #[validate(|_: i32, _, _| Ok(None))]
        pub virtual_field_1: i32,
    }

    #[post_validate(["virtual_field", "virtual_field_1"], validate = async |_, opts| {
        opts.write().await.add_message("ctx_options updated in post_validator");
        Ok(None)
    })]
    #[on_success(["virtual_field", "virtual_field_1"], async |_, opts| {
        panic!("[grouped_on_success]: {}", opts.read_sync().messages[0])
    })]
    const _: () = ();
}
