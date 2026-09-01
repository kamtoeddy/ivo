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
    const DEFAULT_DEPENDENT_VALUE: i32 = 1;
    const DEFAULT_LAX_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in default value resolver";

    let created = default_fn_schema::DataModel
        .create(
            default_fn_schema::PartialDataInput { lax: None },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        default_fn_schema::Data {
            dependent: DEFAULT_DEPENDENT_VALUE,
            lax: DEFAULT_LAX_VALUE
        }
    );

    assert_eq!(created.ctx_options.messages[0], MESSAGE);

    created.handle_success();
}

async_test_matrix!(
    "[on_success]: ctx_options updated in default value resolver",
    should_properly_update_ctx_options_in_default_resolver_and_provide_those_updates_in_on_success_handlers
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod default_fn_schema {
    use super::CtxOptions;

    struct Fields {
        #[depends_on("lax")]
        #[default(async |_, opts| {
            opts.write().await.add_message("ctx_options updated in default value resolver");
            1
        })]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        #[on_success(|_, opts| {
            if true {
                panic!("[on_success]: {}", opts.read_sync().messages[0])
            }
        })]
        pub dependent: i32,

        #[lax(1)]
        #[validate(|_, _, _| Ok(None))]
        pub lax: i32,
    }
}

// resolver at creation

async fn should_properly_update_ctx_options_in_value_resolver_and_provide_those_updates_in_on_success_handlers_at_creation(
) {
    const DEFAULT_DEPENDENT_VALUE: i32 = 1;
    const DEFAULT_LAX_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in value resolver";

    let created = resolver_create_schema::DataModel
        .create(
            resolver_create_schema::PartialDataInput {
                lax: Some(DEFAULT_LAX_VALUE + 1),
            },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        resolver_create_schema::Data {
            dependent: DEFAULT_DEPENDENT_VALUE + 1,
            lax: DEFAULT_LAX_VALUE + 1
        }
    );

    assert_eq!(created.ctx_options.messages[0], MESSAGE);

    created.handle_success();
}

async_test_matrix!(
    "[on_success]: ctx_options updated in value resolver",
    should_properly_update_ctx_options_in_value_resolver_and_provide_those_updates_in_on_success_handlers_at_creation
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod resolver_create_schema {
    use super::CtxOptions;

    struct Fields {
        #[depends_on("lax")]
        #[default(async |_, _| 1)]
        #[resolve(async |ctx, opts| {
            opts.write().await.add_message("ctx_options updated in value resolver");
            ctx.values().dependent + 1
        })]
        #[on_success(|_, opts| {
            if true {
                panic!("[on_success]: {}", opts.read_sync().messages[0])
            }
        })]
        pub dependent: i32,

        #[lax(1)]
        #[validate(|_, _, _| Ok(None))]
        pub lax: i32,
    }
}

// resolver during updates

async fn should_properly_update_ctx_options_in_value_resolver_and_provide_those_updates_in_on_success_handlers_during_updates(
) {
    const DEFAULT_DEPENDENT_VALUE: i32 = 1;
    const DEFAULT_LAX_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in value resolver";

    let data = resolver_update_schema::Data {
        dependent: DEFAULT_DEPENDENT_VALUE,
        lax: DEFAULT_LAX_VALUE,
    };

    let lax = Some(data.lax + 1);

    let updated = resolver_update_schema::DataModel
        .update(
            data.clone(),
            resolver_update_schema::PartialDataInput { lax },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        resolver_update_schema::PartialData {
            dependent: Some(data.dependent + 1),
            lax
        }
    );

    assert_eq!(updated.ctx_options.messages[0], MESSAGE);

    updated.handle_success();
}

async_test_matrix!(
    "[on_success]: ctx_options updated in value resolver",
    should_properly_update_ctx_options_in_value_resolver_and_provide_those_updates_in_on_success_handlers_during_updates
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod resolver_update_schema {
    use super::CtxOptions;

    struct Fields {
        #[depends_on("lax")]
        #[default(async |_, _| 1)]
        #[resolve(async |ctx, opts| {
            opts.write().await.add_message("ctx_options updated in value resolver");
            ctx.values().dependent + 1
        })]
        #[on_success(|_, opts| {
            if true {
                panic!("[on_success]: {}", opts.read_sync().messages[0])
            }
        })]
        pub dependent: i32,

        #[lax(1)]
        #[validate(|_, _, _| Ok(None))]
        pub lax: i32,
    }
}
