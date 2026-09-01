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
    let (created, ..) = async_parallel_required_create_schema::DataInputModel
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
        created,
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

    let (updated, ..) = async_parallel_required_update_schema::DataInputModel
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
        updated,
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

// -----------------------------------------------------------------------------
// Grouped required handler's error payload: distinct errors per field, and a
// same-error-on-both-fields shortcut, in both `create` and `update` -- ported
// from `rs/tests/fields/lax/mod.rs::should_properly_handle_grouped_required_errors`,
// which `should_evaluate_field_level_and_grouped_required_resolvers_concurrently_*`
// above never actually exercised (those only cover concurrency, not the
// error-payload shape a real handler produces).
// -----------------------------------------------------------------------------

#[test]
fn should_properly_handle_grouped_required_errors() {
    const IGNORE_WITH_DIFFERENT_ERRORS: &str = "IGNORE_WITH_DIFFERENT_ERRORS";
    const IGNORE_WITH_SAME_ERROR: &str = "IGNORE_WITH_SAME_ERROR";
    const EXPECTED_LAX_OR_LAX_1: &str = "EXPECTED_LAX_OR_LAX_1";
    const LAX_IS_MISSING: &str = "LAX_IS_MISSING";
    const LAX_1_IS_MISSING: &str = "LAX_1_IS_MISSING";

    const DEFAULT_LAX_VALUE: &str = "default_lax_value";
    const DEFAULT_LAX_1_VALUE: &str = "default_lax_1_value";
    const DEFAULT_LAX_2_VALUE: &str = "default_lax_2_value";

    // create: same-error shortcut (lax_2 == IGNORE_WITH_SAME_ERROR)
    let (err, ..) = grouped_required_errors_schema::DataInputModel
        .create(
            grouped_required_errors_schema::PartialDataInput {
                lax: None,
                lax_1: None,
                lax_2: Some(IGNORE_WITH_SAME_ERROR.to_string()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.get("lax_2").is_none());
    assert_eq!(err.get("lax").unwrap().reason, EXPECTED_LAX_OR_LAX_1);
    assert_eq!(
        err.get("lax_1").unwrap().reason,
        EXPECTED_LAX_OR_LAX_1
    );

    // create: distinct-errors-per-field path (lax_2 == IGNORE_WITH_DIFFERENT_ERRORS)
    let (err, ..) = grouped_required_errors_schema::DataInputModel
        .create(
            grouped_required_errors_schema::PartialDataInput {
                lax: None,
                lax_1: None,
                lax_2: Some(IGNORE_WITH_DIFFERENT_ERRORS.to_string()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.get("lax_2").is_none());
    assert_eq!(err.get("lax").unwrap().reason, LAX_IS_MISSING);
    assert_eq!(err.get("lax_1").unwrap().reason, LAX_1_IS_MISSING);

    // updates

    let data = grouped_required_errors_schema::DataInput {
        lax: DEFAULT_LAX_VALUE.to_string(),
        lax_1: DEFAULT_LAX_1_VALUE.to_string(),
        lax_2: DEFAULT_LAX_2_VALUE.to_string(),
    };

    // update: same-error shortcut
    let (err, ..) = grouped_required_errors_schema::DataInputModel
        .update(
            data.clone(),
            grouped_required_errors_schema::PartialDataInput {
                lax: None,
                lax_1: None,
                lax_2: Some(IGNORE_WITH_SAME_ERROR.to_string()),
            },
            (),
        )
        .err()
        .unwrap();

    let payload = err.as_ref().unwrap();
    assert!(payload.get("lax_2").is_none());
    assert_eq!(payload.get("lax").unwrap().reason, EXPECTED_LAX_OR_LAX_1);
    assert_eq!(payload.get("lax_1").unwrap().reason, EXPECTED_LAX_OR_LAX_1);

    // update: distinct-errors-per-field path
    let (err, ..) = grouped_required_errors_schema::DataInputModel
        .update(
            data,
            grouped_required_errors_schema::PartialDataInput {
                lax: None,
                lax_1: None,
                lax_2: Some(IGNORE_WITH_DIFFERENT_ERRORS.to_string()),
            },
            (),
        )
        .err()
        .unwrap();

    let payload = err.as_ref().unwrap();
    assert!(payload.get("lax_2").is_none());
    assert_eq!(payload.get("lax").unwrap().reason, LAX_IS_MISSING);
    assert_eq!(payload.get("lax_1").unwrap().reason, LAX_1_IS_MISSING);
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod grouped_required_errors_schema {
    struct Fields {
        #[lax("default_lax_value".to_string())]
        pub lax: String,

        #[lax("default_lax_1_value".to_string())]
        pub lax_1: String,

        #[lax("default_lax_2_value".to_string())]
        pub lax_2: String,
    }

    #[required(["lax", "lax_1"], |ctx, _| {
        if let Some(lax_2) = ctx.input().lax_2.clone() {
            if lax_2 == "IGNORE_WITH_SAME_ERROR" {
                let mut errors = DataInputErrors::new();
                errors
                    .set_lax("EXPECTED_LAX_OR_LAX_1", None)
                    .set_lax_1("EXPECTED_LAX_OR_LAX_1", None);
                return Some(errors);
            }

            let mut errors = DataInputErrors::new();
            errors
                .set_lax("LAX_IS_MISSING", None)
                .set_lax_1("LAX_1_IS_MISSING", None);
            return Some(errors);
        }

        None
    })]
    const _: () = ();
}
