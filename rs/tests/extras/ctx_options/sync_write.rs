use ivo::ivo_schema;

// `IvoRwCtxOptions::write_sync()` -- every other ctx_options test in this
// directory mutates options via the async `opts.write().await` path (and
// reads them back via `opts.read_sync()` in a sync lifecycle hook, which
// *is* covered elsewhere). `write_sync()` itself -- the blocking write path
// meant for fully-synchronous handlers -- was untested. `rs/` has no sync
// ctx_options accessors at all, so there's no equivalent to port; this is
// rs-next-only API. See TODO.md.

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

#[test]
#[should_panic(expected = "ctx_options updated synchronously in validator")]
fn should_properly_write_ctx_options_synchronously_in_a_sync_validator_and_read_them_back_synchronously_in_an_on_success_handler(
) {
    let (_, _ctx_options, handle_success) = sync_write_schema::DataInputModel
        .create(
            sync_write_schema::PartialDataInput { required: Some(1) },
            CtxOptions::new(),
        )
        .ok()
        .unwrap();

    handle_success();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(CtxOptions)
)]
mod sync_write_schema {
    use super::CtxOptions;

    struct Fields {
        #[required]
        #[validate(|v: i32, _, opts| {
            opts.write_sync()
                .add_message("ctx_options updated synchronously in validator");
            Ok(Some(v))
        })]
        #[on_success(|_, opts| {
            panic!("{}", opts.read_sync().messages[0])
        })]
        pub required: i32,
    }
}
