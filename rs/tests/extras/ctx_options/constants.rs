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

async fn should_properly_update_ctx_options_in_constant_value_resolver_and_provide_those_updates_in_on_success_handlers(
) {
    const CONSTANT_VALUE: i32 = 1;
    const MESSAGE: &str = "ctx_options updated in constant value resolver";

    let created = data_schema::DataModel
        .create(
            data_schema::PartialDataInput { lax: Some(2) },
            CtxOptions::new(),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        data_schema::Data {
            id: CONSTANT_VALUE,
            lax: 2
        }
    );

    assert_eq!(created.ctx_options.read().await.messages[0], MESSAGE);

    created.handle_success();
}

async_test_matrix!(
    "[on_success]: ctx_options updated in constant value resolver",
    should_properly_update_ctx_options_in_constant_value_resolver_and_provide_those_updates_in_on_success_handlers
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod data_schema {
    use super::CtxOptions;

    struct Fields {
        #[constant(async |_, opts| {
            opts.write().await.add_message("ctx_options updated in constant value resolver");
            1
        })]
        #[on_success(|_, opts| {
            if true {
                panic!("[on_success]: {}", opts.read_sync().messages[0])
            }
        })]
        pub id: i32,

        #[lax(2)]
        pub lax: i32,
    }
}
