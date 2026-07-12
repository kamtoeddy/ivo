use std::future::ready;

use ivo::{
    IvoContext, IvoCtxOptions, IvoField, IvoInputStruct, IvoRwCtxOptions, IvoStruct, Schema,
};

use crate::async_test_matrix;

// [x] default_fn
// [x] resolver
// [x] on_success

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
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    const DEFAULT_DEPENDENT_VALUE: i32 = 1;
    const DEFAULT_LAX_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in default value resolver";

    let schema = Schema::<DataInput, Data, CtxOptions>::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default_fn(async |_, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        DEFAULT_DEPENDENT_VALUE
                    })
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    })
                    .on_success(|_, o: IvoCtxOptions<CtxOptions>| {
                        if true {
                            panic!("[on_success]: {}", o.messages[0])
                        }

                        ready(())
                    }),
            )
            .field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_LAX_VALUE)
                    .validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let (data, handle_success, ctx_options) = model
        .create(&PartialDataInput { lax: None }, CtxOptions::new())
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: DEFAULT_DEPENDENT_VALUE,
            lax: DEFAULT_LAX_VALUE
        }
    );

    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_success().await;
}

async_test_matrix!(
    "[on_success]: ctx_options updated in default value resolver",
    should_properly_update_ctx_options_in_default_resolver_and_provide_those_updates_in_on_success_handlers
);

// resolver

async fn should_properly_update_ctx_options_in_value_resolver_and_provide_those_updates_in_on_success_handlers_at_creation(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    const DEFAULT_DEPENDENT_VALUE: i32 = 1;
    const DEFAULT_LAX_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in value resolver";

    let schema = Schema::<DataInput, Data, CtxOptions>::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default_fn(async |_, _| DEFAULT_DEPENDENT_VALUE)
                    .depends_on(["lax"])
                    .resolve(
                        async |ctx: IvoContext<DataInput, Data>, o: IvoRwCtxOptions<CtxOptions>| {
                            let mut ctx_options = o.write().await;

                            ctx_options.add_message(MESSAGE);

                            ctx.values().dependent.unwrap() + 1
                        },
                    )
                    .on_success(|_, o: IvoCtxOptions<CtxOptions>| {
                        if true {
                            panic!("[on_success]: {}", o.messages[0])
                        }

                        ready(())
                    }),
            )
            .field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_LAX_VALUE)
                    .validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let value = DEFAULT_LAX_VALUE + 1;

    let (data, handle_success, ctx_options) = model
        .create(&PartialDataInput { lax: Some(value) }, CtxOptions::new())
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: DEFAULT_DEPENDENT_VALUE + 1,
            lax: value
        }
    );

    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_success().await;
}

async_test_matrix!(
    "[on_success]: ctx_options updated in value resolver",
    should_properly_update_ctx_options_in_value_resolver_and_provide_those_updates_in_on_success_handlers_at_creation
);

async fn should_properly_update_ctx_options_in_value_resolver_and_provide_those_updates_in_on_success_handlers_during_updates(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    const DEFAULT_DEPENDENT_VALUE: i32 = 1;
    const DEFAULT_LAX_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in value resolver";

    let schema = Schema::<DataInput, Data, CtxOptions>::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default_fn(async |_, _| DEFAULT_DEPENDENT_VALUE)
                    .depends_on(["lax"])
                    .resolve(
                        async |ctx: IvoContext<DataInput, Data>, o: IvoRwCtxOptions<CtxOptions>| {
                            let mut ctx_options = o.write().await;

                            ctx_options.add_message(MESSAGE);

                            ctx.values().dependent.unwrap() + 1
                        },
                    )
                    .on_success(|_, o: IvoCtxOptions<CtxOptions>| {
                        if true {
                            panic!("[on_success]: {}", o.messages[0])
                        }

                        ready(())
                    }),
            )
            .field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_LAX_VALUE)
                    .validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let data = Data {
        dependent: DEFAULT_DEPENDENT_VALUE,
        lax: DEFAULT_LAX_VALUE,
    };

    let lax = Some(data.lax + 1);

    let (updates, handle_success, ctx_options) = model
        .update(&data, &PartialDataInput { lax }, CtxOptions::new())
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

    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_success().await;
}

async_test_matrix!(
    "[on_success]: ctx_options updated in value resolver",
    should_properly_update_ctx_options_in_value_resolver_and_provide_those_updates_in_on_success_handlers_during_updates
);
