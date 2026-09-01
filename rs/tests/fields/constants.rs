use ivo::ivo_schema;

#[test]
fn should_respect_sync_constants_with_static_values() {
    let constant = 1234;
    let lax = 400;

    let (created, ..) = sync_static_constant_schema::DataModel
        .create(
            sync_static_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        sync_static_constant_schema::Data { constant, lax }
    );

    let lax = 700;

    let (created, ..) = sync_static_constant_schema::DataModel
        .create(
            sync_static_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        sync_static_constant_schema::Data { constant, lax }
    );

    let lax = Some(200);

    let (updated, ..) = sync_static_constant_schema::DataModel
        .update(
            created.clone(),
            sync_static_constant_schema::PartialDataInput { lax },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        sync_static_constant_schema::PartialData {
            constant: None,
            lax
        }
    );
}

async fn should_respect_async_constants_with_static_values() {
    let constant = 1234;
    let lax = 400;

    let (created, ..) = async_static_constant_schema::DataModel
        .create(
            async_static_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        async_static_constant_schema::Data { constant, lax }
    );

    let lax = 700;

    let (created, ..) = async_static_constant_schema::DataModel
        .create(
            async_static_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        async_static_constant_schema::Data { constant, lax }
    );

    let lax = Some(200);

    let (updated, ..) = async_static_constant_schema::DataModel
        .update(
            created.clone(),
            async_static_constant_schema::PartialDataInput { lax },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        async_static_constant_schema::PartialData {
            constant: None,
            lax
        }
    );
}

async_test_matrix!(should_respect_async_constants_with_static_values);

#[test]
fn should_respect_sync_constants_with_computed_values() {
    let constant = 1234;
    let lax = 400;

    let (created, ..) = sync_dynamic_constant_schema::DataModel
        .create(
            sync_dynamic_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        sync_dynamic_constant_schema::Data { constant, lax }
    );

    let lax = 700;

    let (created, ..) = sync_dynamic_constant_schema::DataModel
        .create(
            sync_dynamic_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        sync_dynamic_constant_schema::Data { constant, lax }
    );

    let lax = Some(200);

    let (updated, ..) = sync_dynamic_constant_schema::DataModel
        .update(
            created.clone(),
            sync_dynamic_constant_schema::PartialDataInput { lax },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        sync_dynamic_constant_schema::PartialData {
            constant: None,
            lax
        }
    );
}

async fn should_respect_async_constants_with_computed_values() {
    let constant = 1234;
    let lax = 400;

    let (created, ..) = async_dynamic_constant_schema::DataModel
        .create(
            async_dynamic_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        async_dynamic_constant_schema::Data { constant, lax }
    );

    let lax = 700;

    let (created, ..) = async_dynamic_constant_schema::DataModel
        .create(
            async_dynamic_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        async_dynamic_constant_schema::Data { constant, lax }
    );

    let lax = Some(200);

    let (updated, ..) = async_dynamic_constant_schema::DataModel
        .update(
            created.clone(),
            async_dynamic_constant_schema::PartialDataInput { lax },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        async_dynamic_constant_schema::PartialData {
            constant: None,
            lax
        }
    );
}

async_test_matrix!(should_respect_async_constants_with_computed_values);

#[should_panic(expected = "[constant]: on_delete triggered with value: 1234")]
#[test]
fn should_trigger_sync_on_delete_handlers_with_static_values() {
    sync_static_on_delete_schema::DataModel.delete(
        &sync_static_on_delete_schema::Data {
            constant: 1234,
            lax: 400,
        },
        (),
    );
}

async fn should_trigger_async_on_delete_handlers_with_static_values() {
    async_static_on_delete_schema::DataModel
        .delete(
            &async_static_on_delete_schema::Data {
                constant: 1234,
                lax: 400,
            },
            (),
        )
        .await;
}

async_test_matrix!(
    "[constant]: on_delete triggered with value: 1234",
    should_trigger_async_on_delete_handlers_with_static_values
);

#[should_panic(expected = "[constant]: on_delete triggered with value: 1234")]
#[test]
fn should_trigger_sync_on_delete_handlers_with_computed_values() {
    sync_dynamic_on_delete_schema::DataModel.delete(
        &sync_dynamic_on_delete_schema::Data {
            constant: 1234,
            lax: 400,
        },
        (),
    );
}

async fn should_trigger_async_on_delete_handlers_with_computed_values() {
    async_dynamic_on_delete_schema::DataModel
        .delete(
            &async_dynamic_on_delete_schema::Data {
                constant: 1234,
                lax: 400,
            },
            (),
        )
        .await;
}

async_test_matrix!(
    "[constant]: on_delete triggered with value: 1234",
    should_trigger_async_on_delete_handlers_with_computed_values
);

#[should_panic(expected = "[constant]: on_success triggered with value: 1234")]
#[test]
fn should_trigger_sync_on_success_handlers_with_static_values() {
    let constant = 1234;
    let lax = 400;

    let (created, _ctx_options, handle_success) = sync_static_on_success_schema::DataModel
        .create(
            sync_static_on_success_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        sync_static_on_success_schema::Data { constant, lax }
    );

    handle_success();
}

async fn should_trigger_async_on_success_handlers_with_static_values() {
    let constant = 1234;
    let lax = 400;

    let (created, _ctx_options, handle_success) = async_static_on_success_schema::DataModel
        .create(
            async_static_on_success_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        async_static_on_success_schema::Data { constant, lax }
    );

    handle_success().await;
}

async_test_matrix!(
    "[constant]: on_success triggered with value: 1234",
    should_trigger_async_on_success_handlers_with_static_values
);

#[should_panic(expected = "[constant]: on_success triggered with value: 1234")]
#[test]
fn should_trigger_sync_on_success_handlers_with_computed_values() {
    let constant = 1234;
    let lax = 400;

    let (created, _ctx_options, handle_success) = sync_dynamic_on_success_schema::DataModel
        .create(
            sync_dynamic_on_success_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        sync_dynamic_on_success_schema::Data { constant, lax }
    );

    handle_success();
}

async fn should_trigger_async_on_success_handlers_with_computed_values() {
    let constant = 1234;
    let lax = 400;

    let (created, _ctx_options, handle_success) = async_dynamic_on_success_schema::DataModel
        .create(
            async_dynamic_on_success_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        async_dynamic_on_success_schema::Data { constant, lax }
    );

    handle_success().await;
}

async_test_matrix!(
    "[constant]: on_success triggered with value: 1234",
    should_trigger_async_on_success_handlers_with_computed_values
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_static_constant_schema {
    struct Fields {
        #[constant(1234)]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_static_constant_schema {
    struct Fields {
        #[constant(async |_, _| 1234)]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_dynamic_constant_schema {
    struct Fields {
        #[constant(|_, _| 1234)]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_dynamic_constant_schema {
    struct Fields {
        #[constant(async |_, _| 1234)]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_static_on_delete_schema {
    struct Fields {
        #[constant(1234)]
        #[on_delete(|data, _| {
            panic!("[constant]: on_delete triggered with value: {}", data.constant);
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_static_on_delete_schema {
    struct Fields {
        #[constant(async |_, _| 1234)]
        #[on_delete(async |data, _| {
            panic!("[constant]: on_delete triggered with value: {}", data.constant);
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_dynamic_on_delete_schema {
    struct Fields {
        #[constant(|_, _| 1234)]
        #[on_delete(|data, _| {
            panic!("[constant]: on_delete triggered with value: {}", data.constant);
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_dynamic_on_delete_schema {
    struct Fields {
        #[constant(async |_, _| 1234)]
        #[on_delete(async |data, _| {
            panic!("[constant]: on_delete triggered with value: {}", data.constant);
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_static_on_success_schema {
    struct Fields {
        #[constant(1234)]
        #[on_success(|ctx, _| {
            panic!(
                "[constant]: on_success triggered with value: {}",
                ctx.values().constant
            );
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_static_on_success_schema {
    struct Fields {
        #[constant(async |_, _| 1234)]
        #[on_success(async |ctx, _| {
            panic!(
                "[constant]: on_success triggered with value: {}",
                ctx.values().constant
            );
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_dynamic_on_success_schema {
    struct Fields {
        #[constant(|_, _| 1234)]
        #[on_success(|ctx, _| {
            panic!(
                "[constant]: on_success triggered with value: {}",
                ctx.values().constant
            );
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_dynamic_on_success_schema {
    struct Fields {
        #[constant(async |_, _| 1234)]
        #[on_success(async |ctx, _| {
            panic!(
                "[constant]: on_success triggered with value: {}",
                ctx.values().constant
            );
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

// -----------------------------------------------------------------------------
// Ordering: constants are attached after dependents resolve
// -----------------------------------------------------------------------------

#[test]
fn should_attach_constants_after_dependents_have_resolved() {
    // Per GOAL.md §17, constants (step 9) are attached after dependents
    // resolve (step 8), so a constant's resolver may read an already-resolved
    // dependent's value via `ctx.values()`.
    let (created, ..) = constant_reads_dependent_schema::DataModel
        .create(
            constant_reads_dependent_schema::PartialDataInput { name: Some("abc".into()) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.dependent, "abc");
    assert_eq!(created.constant, "constant-saw-abc".to_string());
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod constant_reads_dependent_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[depends_on("name")]
        #[default(String::new())]
        #[resolve(|ctx, _| ctx.input().name.clone().unwrap())]
        pub dependent: String,

        #[constant(|ctx, _| format!("constant-saw-{}", ctx.values().dependent))]
        pub constant: String,
    }
}

// -----------------------------------------------------------------------------
// Parallel resolution of independent constants
// -----------------------------------------------------------------------------

#[tokio::test]
async fn should_resolve_independent_constants_concurrently() {
    // Two constants' resolvers must be polled concurrently (not one `.await`
    // at a time). `rendezvous()` only returns once *both* have started.
    let (created, ..) = async_parallel_constants_schema::DataModel
        .create(async_parallel_constants_schema::PartialDataInput { lax: Some(1) }, ())
        .await
        .ok()
        .unwrap();

    assert_eq!(created.constant_a, 1234);
    assert_eq!(created.constant_b, 5678);
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_parallel_constants_schema {
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
        panic!("constant resolvers were not run concurrently");
    }

    struct Fields {
        #[lax(20)]
        pub lax: i32,

        #[constant(async |_, _| {
            rendezvous().await;
            1234
        })]
        pub constant_a: u32,

        #[constant(async |_, _| {
            rendezvous().await;
            5678
        })]
        pub constant_b: u32,
    }
}

// -----------------------------------------------------------------------------
// Ordering: constants are attached before timestamps
// -----------------------------------------------------------------------------

#[test]
fn should_attach_constants_before_timestamps() {
    // Per GOAL.md §17, constants (step 9) are attached before timestamps
    // (step 10), so a constant's resolver observes the timestamp field still
    // at its default (unset) value.
    let (created, ..) = constant_runs_before_timestamps_schema::DataModel
        .create(
            constant_runs_before_timestamps_schema::PartialDataInput { lax: Some(1) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.constant, "constant-saw-created_at=0");
    assert_eq!(created.created_at, 1234);
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod constant_runs_before_timestamps_schema {
    struct Fields {
        #[lax(20)]
        pub lax: i32,

        #[constant(|ctx, _| format!("constant-saw-created_at={}", ctx.values().created_at))]
        pub constant: String,

        #[created_at]
        pub created_at: u128,
    }

    #[timestamps(|| 1234)]
    const _: () = ();
}
