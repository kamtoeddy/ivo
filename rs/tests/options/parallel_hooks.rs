// Proves that independent lifecycle hooks of the same kind (multiple
// `#[on_success]`, `#[on_failure]`, or `#[on_delete]` handlers) are polled
// concurrently rather than one `.await` at a time, the same way independent
// field handlers are. Each `rendezvous()` only returns once *both* hooks of
// that kind have started, which can only happen if they're in flight
// together.
use ivo::ivo_schema;

macro_rules! rendezvous_fn {
    ($name:ident, $counter:expr, $label:literal) => {
        async fn $name() {
            $counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            for _ in 0..10_000 {
                if $counter.load(std::sync::atomic::Ordering::SeqCst) >= 2 {
                    return;
                }
                tokio::task::yield_now().await;
            }
            panic!(concat!($label, " hooks were not run concurrently"));
        }
    };
}

#[tokio::test]
async fn should_run_independent_on_success_hooks_concurrently_on_create() {
    on_success_create_schema::STARTED.store(0, std::sync::atomic::Ordering::SeqCst);

    let (created, _ctx_options, handle_success) = on_success_create_schema::DataInputModel
        .create(
            on_success_create_schema::DataInput {
                field_a: "a".into(),
                field_b: "b".into(),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.field_a, "a");
    handle_success().await;
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod on_success_create_schema {
    use std::sync::atomic::AtomicUsize;

    pub static STARTED: AtomicUsize = AtomicUsize::new(0);

    rendezvous_fn!(rendezvous, STARTED, "on_success");

    struct Fields {
        #[required]
        #[on_success(async |_ctx, _opts| { rendezvous().await; })]
        pub field_a: String,

        #[required]
        #[on_success(async |_ctx, _opts| { rendezvous().await; })]
        pub field_b: String,
    }
}

#[tokio::test]
async fn should_run_independent_on_failure_hooks_concurrently_on_create() {
    on_failure_create_schema::STARTED.store(0, std::sync::atomic::Ordering::SeqCst);

    let (.., handle_failure) = on_failure_create_schema::DataInputModel
        .create(
            on_failure_create_schema::PartialDataInput {
                field_a: None,
                field_b: None,
            },
            (),
        )
        .err()
        .unwrap();

    handle_failure().await;
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod on_failure_create_schema {
    use std::sync::atomic::AtomicUsize;

    pub static STARTED: AtomicUsize = AtomicUsize::new(0);

    rendezvous_fn!(rendezvous, STARTED, "on_failure");

    struct Fields {
        #[required]
        #[on_failure(async |_ctx, _opts| { rendezvous().await; })]
        pub field_a: String,

        #[required]
        #[on_failure(async |_ctx, _opts| { rendezvous().await; })]
        pub field_b: String,
    }
}

#[tokio::test]
async fn should_run_independent_on_success_hooks_concurrently_on_update() {
    on_success_update_schema::STARTED.store(0, std::sync::atomic::Ordering::SeqCst);

    let existing = on_success_update_schema::DataInput {
        field_a: "a".into(),
        field_b: "b".into(),
    };

    let (.., handle_success) = on_success_update_schema::DataInputModel
        .update(
            existing,
            on_success_update_schema::PartialDataInput {
                field_a: Some("aa".into()),
                field_b: Some("bb".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    handle_success().await;
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod on_success_update_schema {
    use std::sync::atomic::AtomicUsize;

    pub static STARTED: AtomicUsize = AtomicUsize::new(0);

    rendezvous_fn!(rendezvous, STARTED, "on_success (update)");

    struct Fields {
        #[required]
        #[on_success(async |_ctx, _opts| { rendezvous().await; })]
        pub field_a: String,

        #[required]
        #[on_success(async |_ctx, _opts| { rendezvous().await; })]
        pub field_b: String,
    }
}

#[tokio::test]
async fn should_run_independent_on_delete_hooks_concurrently() {
    on_delete_schema::STARTED.store(0, std::sync::atomic::Ordering::SeqCst);

    let data = on_delete_schema::DataInput {
        field_a: "a".into(),
        field_b: "b".into(),
    };

    on_delete_schema::DataInputModel.delete(&data, ()).await;
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod on_delete_schema {
    use std::sync::atomic::AtomicUsize;

    pub static STARTED: AtomicUsize = AtomicUsize::new(0);

    rendezvous_fn!(rendezvous, STARTED, "on_delete");

    struct Fields {
        #[required]
        #[on_delete(async |_data, _opts| { rendezvous().await; })]
        pub field_a: String,

        #[required]
        #[on_delete(async |_data, _opts| { rendezvous().await; })]
        pub field_b: String,
    }
}
