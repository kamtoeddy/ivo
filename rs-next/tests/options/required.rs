// Compile-time validation tests for grouped `#[required(...)]` are located
// in `compile_fail/required.rs`.

use ivo::ivo_schema;

// -----------------------------------------------------------------------------
// Parallel resolution of independent conditional-required resolvers
// -----------------------------------------------------------------------------
//
// Each test below uses its own dedicated schema (and static counter) rather
// than sharing one across create/update variants: cargo runs tests
// concurrently by default, and two tests racing on the same counter could
// spuriously satisfy each other's rendezvous, masking a real regression.

#[tokio::test]
async fn should_evaluate_field_level_and_grouped_required_resolvers_concurrently_on_create() {
    // A field-level `#[required(...)]` and a grouped `#[required([...], ...)]`
    // are batched into a single "one go" phase (matching `rs/`'s
    // `evaluate_missing_required_fields`), regardless of which kind of
    // required option they came from. `rendezvous()` only returns once
    // *both* have started.
    let created = async_parallel_required_create_schema::DataInputModel
        .create(
            async_parallel_required_create_schema::PartialDataInput {
                field_a: None,
                field_b: None,
                field_c: None,
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_parallel_required_create_schema::DataInput {
            field_a: None,
            field_b: None,
            field_c: None,
        }
    );
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod async_parallel_required_create_schema {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static STARTED: AtomicUsize = AtomicUsize::new(0);

    async fn rendezvous() -> Option<String> {
        STARTED.fetch_add(1, Ordering::SeqCst);
        for _ in 0..10_000 {
            if STARTED.load(Ordering::SeqCst) >= 2 {
                return None;
            }
            tokio::task::yield_now().await;
        }
        panic!("required resolvers were not evaluated concurrently on create");
    }

    struct Fields {
        #[lax(None)]
        #[required(async |_ctx, _opts| { rendezvous().await })]
        pub field_a: Option<String>,

        #[lax(None)]
        pub field_b: Option<String>,

        #[lax(None)]
        pub field_c: Option<String>,
    }

    #[required(["field_b", "field_c"], async |_ctx, _opts| {
        rendezvous().await;
        None::<DataInputErrors>
    })]
    const _: () = ();
}

#[tokio::test]
async fn should_evaluate_field_level_and_grouped_required_resolvers_concurrently_on_update() {
    let existing = async_parallel_required_update_schema::DataInput {
        field_a: None,
        field_b: None,
        field_c: None,
    };

    let updated = async_parallel_required_update_schema::DataInputModel
        .update(
            existing,
            async_parallel_required_update_schema::PartialDataInput {
                field_a: Some(Some("a".to_string())),
                field_b: None,
                field_c: None,
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_parallel_required_update_schema::PartialDataInput {
            field_a: Some(Some("a".to_string())),
            field_b: None,
            field_c: None,
        }
    );
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod async_parallel_required_update_schema {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static STARTED: AtomicUsize = AtomicUsize::new(0);

    async fn rendezvous() -> Option<String> {
        STARTED.fetch_add(1, Ordering::SeqCst);
        for _ in 0..10_000 {
            if STARTED.load(Ordering::SeqCst) >= 2 {
                return None;
            }
            tokio::task::yield_now().await;
        }
        panic!("required resolvers were not evaluated concurrently on update");
    }

    struct Fields {
        #[lax(None)]
        #[required(async |_ctx, _opts| { rendezvous().await })]
        pub field_a: Option<String>,

        #[lax(None)]
        pub field_b: Option<String>,

        #[lax(None)]
        pub field_c: Option<String>,
    }

    #[required(["field_b", "field_c"], async |_ctx, _opts| {
        rendezvous().await;
        None::<DataInputErrors>
    })]
    const _: () = ();
}
