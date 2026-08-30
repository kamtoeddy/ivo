// Compile-time validation tests for grouped `#[ignore(...)]` are located in
// `compile_fail/ignore.rs`. The old runtime panic tests are now macro errors.

use ivo::ivo_schema;

// -----------------------------------------------------------------------------
// Parallel resolution of independent ignore resolvers
// -----------------------------------------------------------------------------
//
// Each test below uses its own dedicated schema (and static counter) rather
// than sharing one across create/update variants: cargo runs tests
// concurrently by default, and two tests racing on the same counter could
// spuriously satisfy each other's rendezvous, masking a real regression.

#[tokio::test]
async fn should_evaluate_field_level_and_grouped_ignore_resolvers_concurrently_on_create() {
    // A field-level `#[ignore(...)]` and a grouped `#[ignore([...], ...)]` are
    // batched into a single "one go" phase (matching `rs/`'s
    // `filter_input_fields_allowed`), regardless of field type or which kind
    // of ignore option they came from. `rendezvous()` only returns once
    // *both* have started.
    let created = async_parallel_ignore_create_schema::DataInputModel
        .create(
            async_parallel_ignore_create_schema::DataInput {
                field_a: 10,
                field_b: 20,
                field_c: 30,
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_parallel_ignore_create_schema::DataInput {
            field_a: 10,
            field_b: 20,
            field_c: 30,
        }
    );
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod async_parallel_ignore_create_schema {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static STARTED: AtomicUsize = AtomicUsize::new(0);

    async fn rendezvous() -> bool {
        STARTED.fetch_add(1, Ordering::SeqCst);
        for _ in 0..10_000 {
            if STARTED.load(Ordering::SeqCst) >= 2 {
                return false;
            }
            tokio::task::yield_now().await;
        }
        panic!("ignore resolvers were not evaluated concurrently on create");
    }

    struct Fields {
        #[lax(1)]
        #[ignore(async |_ctx, _opts| { rendezvous().await })]
        pub field_a: i32,

        #[lax(2)]
        pub field_b: i32,

        #[lax(3)]
        pub field_c: i32,
    }

    #[ignore(["field_b", "field_c"], async |_ctx, _opts| { rendezvous().await })]
    const _: () = ();
}

#[tokio::test]
async fn should_evaluate_field_level_and_grouped_ignore_resolvers_concurrently_on_update() {
    let existing = async_parallel_ignore_update_schema::DataInput {
        field_a: 1,
        field_b: 2,
        field_c: 3,
    };

    let updated = async_parallel_ignore_update_schema::DataInputModel
        .update(
            existing,
            async_parallel_ignore_update_schema::PartialDataInput {
                field_a: Some(10),
                field_b: Some(20),
                field_c: Some(30),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_parallel_ignore_update_schema::PartialDataInput {
            field_a: Some(10),
            field_b: Some(20),
            field_c: Some(30),
        }
    );
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod async_parallel_ignore_update_schema {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static STARTED: AtomicUsize = AtomicUsize::new(0);

    async fn rendezvous() -> bool {
        STARTED.fetch_add(1, Ordering::SeqCst);
        for _ in 0..10_000 {
            if STARTED.load(Ordering::SeqCst) >= 2 {
                return false;
            }
            tokio::task::yield_now().await;
        }
        panic!("ignore resolvers were not evaluated concurrently on update");
    }

    struct Fields {
        #[lax(1)]
        #[ignore(async |_ctx, _opts| { rendezvous().await })]
        pub field_a: i32,

        #[lax(2)]
        pub field_b: i32,

        #[lax(3)]
        pub field_c: i32,
    }

    #[ignore(["field_b", "field_c"], async |_ctx, _opts| { rendezvous().await })]
    const _: () = ();
}
