// Compile-time validation tests for grouped `#[post_validate(...)]` are located
// in `compile_fail/post_validate.rs`.

use ivo::ivo_schema;

// -----------------------------------------------------------------------------
// Main `validate` does not run once `pre_validate` has already failed
// -----------------------------------------------------------------------------

#[test]
fn should_not_run_main_validate_once_pre_validate_has_failed() {
    let (errors, ..) = pre_validate_aborts_main_schema::DataInputModel
        .create(
            pre_validate_aborts_main_schema::PartialDataInput {
                field_a: Some("fail-pre".into()),
                field_b: Some("b".into()),
            },
            (),
        )
        .err().unwrap();

    assert_eq!(errors.get("field_a").unwrap().reason, "pre failed");
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod pre_validate_aborts_main_schema {
    struct Fields {
        #[required]
        pub field_a: String,

        #[required]
        pub field_b: String,
    }

    #[post_validate(
        ["field_a", "field_b"],
        pre_validate = |ctx, _| {
            if ctx.input().field_a.as_deref() == Some("fail-pre") {
                let mut errors = DataInputErrors::new();
                errors.set_field_a("pre failed", None);
                return Err(errors);
            }
            Ok(None)
        },
        validate = |_ctx, _opts| {
            panic!("main validate must not run once pre_validate has already failed");
        },
    )]
    const _: () = ();
}

// -----------------------------------------------------------------------------
// Multiple groups: each group's pre_validate/validate is batched against a
// snapshot from *before* the phase, not against a sibling group's updates
// (matches the reference implementation's two-phase, all-groups-at-once
// batching).
// -----------------------------------------------------------------------------

#[test]
fn should_not_let_one_group_see_another_groups_pre_validate_updates() {
    // group_a's pre_validate sets `shared` to "from-a"; group_b's pre_validate
    // reads `shared` and records what it saw. Since both groups' pre_validate
    // handlers are batched against the same pre-phase snapshot, group_b must
    // see the *original* value, not group_a's update.
    let (created, ..) = independent_post_validate_groups_schema::DataInputModel
        .create(
            independent_post_validate_groups_schema::PartialDataInput {
                shared: Some("original".into()),
                field_a: Some("a".into()),
                field_b: Some("b".into()),
                seen_by_b: Some(String::new()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.shared, "from-a");
    assert_eq!(created.seen_by_b, "original");
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod independent_post_validate_groups_schema {
    struct Fields {
        #[required]
        pub shared: String,

        #[required]
        pub field_a: String,

        #[required]
        pub field_b: String,

        #[required]
        pub seen_by_b: String,
    }

    #[post_validate(
        ["shared", "field_a"],
        validate = |ctx, _| {
            let mut updates = PartialDataInput::new();
            updates.set_shared("from-a".to_string());
            let _ = &ctx;
            Ok(Some(updates))
        },
    )]
    const _: () = ();

    #[post_validate(
        ["seen_by_b", "field_b"],
        validate = |ctx, _| {
            let mut updates = PartialDataInput::new();
            updates.set_seen_by_b(ctx.input().shared.clone().unwrap());
            Ok(Some(updates))
        },
    )]
    const _: () = ();
}

// -----------------------------------------------------------------------------
// Parallel resolution of independent groups' main `validate` handlers
// -----------------------------------------------------------------------------

#[tokio::test]
async fn should_run_independent_groups_main_validate_concurrently() {
    // Two separate `#[post_validate(...)]` groups' main validators must be
    // polled concurrently (not one `.await` at a time). `rendezvous()` only
    // returns once *both* have started.
    async_parallel_post_validate_groups_schema::STARTED
        .store(0, std::sync::atomic::Ordering::SeqCst);

    let (created, ..) = async_parallel_post_validate_groups_schema::DataInputModel
        .create(
            async_parallel_post_validate_groups_schema::DataInput {
                field_a: "a".into(),
                field_b: "b".into(),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(created.field_a, "a");
    assert_eq!(created.field_b, "b");
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod async_parallel_post_validate_groups_schema {
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub static STARTED: AtomicUsize = AtomicUsize::new(0);

    async fn rendezvous() {
        STARTED.fetch_add(1, Ordering::SeqCst);
        for _ in 0..10_000 {
            if STARTED.load(Ordering::SeqCst) >= 2 {
                return;
            }
            tokio::task::yield_now().await;
        }
        panic!("post_validate groups were not run concurrently");
    }

    struct Fields {
        #[required]
        pub field_a: String,

        #[required]
        pub field_b: String,
    }

    #[post_validate(
        ["field_a", "field_b"],
        validate = async |_ctx, _opts| {
            rendezvous().await;
            Ok(None)
        },
    )]
    const _: () = ();

    #[post_validate(
        ["field_b", "field_a"],
        validate = async |_ctx, _opts| {
            rendezvous().await;
            Ok(None)
        },
    )]
    const _: () = ();
}
