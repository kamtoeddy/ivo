use std::future::ready;

use ivo::{IvoCtxOptions, IvoField, IvoInputStruct, IvoRwCtxOptions, IvoStruct, Schema};

use crate::async_test_matrix;

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

async fn should_properly_update_ctx_options_in_constant_value_resolver_and_provide_those_updates_in_on_success_handlers(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    const CONSTANT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in constant value resolver";

    let schema = Schema::<DataInput, Data, CtxOptions>::new(
        |f| {
            f.field(
                "id",
                IvoField::CONSTANT
                    .value_fn(async |_, o: IvoRwCtxOptions<CtxOptions>| {
                        let mut ctx_options = o.write().await;

                        ctx_options.add_message(MESSAGE);

                        CONSTANT_VALUE
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
                    .default(2)
                    .validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let value = 2;

    let (data, handle_success, ctx_options) = model
        .create(&PartialDataInput { lax: Some(value) }, CtxOptions::new())
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            id: CONSTANT_VALUE,
            lax: value
        }
    );

    assert_eq!(ctx_options.messages[0], MESSAGE);

    handle_success().await;
}

async_test_matrix!(
    "[on_success]: ctx_options updated in constant value resolver",
    should_properly_update_ctx_options_in_constant_value_resolver_and_provide_those_updates_in_on_success_handlers
);
