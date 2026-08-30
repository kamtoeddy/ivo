use ivo::ivo_schema;

#[test]
fn should_allow_required_fields_with_and_without_validators() {
    let _ = required_without_validator_schema::DataInputModel;
    let _ = required_with_validator_schema::DataInputModel;
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod required_without_validator_schema {
    struct Fields {
        #[required]
        pub name: String,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod required_with_validator_schema {
    struct Fields {
        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub name: String,
    }
}

// -----------------------------------------------------------------------------
// Parallel re-validation of independent fields
// -----------------------------------------------------------------------------

#[tokio::test]
async fn should_re_validate_independent_fields_concurrently() {
    // Two required fields' re-validators must be polled concurrently (not one
    // `.await` at a time). `rendezvous()` only returns once *both* have
    // started.
    let created = async_parallel_re_validate_schema::DataInputModel
        .create(
            async_parallel_re_validate_schema::PartialDataInput {
                field_a: Some("a".into()),
                field_b: Some("b".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_parallel_re_validate_schema::DataInput {
            field_a: "revalidated-a".into(),
            field_b: "revalidated-b".into(),
        }
    );
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod async_parallel_re_validate_schema {
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
        panic!("re_validate handlers were not run concurrently");
    }

    struct Fields {
        #[required]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        #[re_validate(async |v: String, _, _| {
            rendezvous().await;
            Ok(Some(format!("revalidated-{v}")))
        })]
        pub field_a: String,

        #[required]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        #[re_validate(async |v: String, _, _| {
            rendezvous().await;
            Ok(Some(format!("revalidated-{v}")))
        })]
        pub field_b: String,
    }
}

// -----------------------------------------------------------------------------
// Parallel validation of independent fields during update
// -----------------------------------------------------------------------------

#[tokio::test]
async fn should_validate_independent_updated_fields_concurrently() {
    // Two required fields' primary validators must be polled concurrently
    // during `update` (not one `.await` at a time). `rendezvous()` only
    // returns once *both* have started.
    let existing = async_parallel_update_validate_schema::DataInput {
        field_a: "a".into(),
        field_b: "b".into(),
    };

    async_parallel_update_validate_schema::STARTED.store(0, std::sync::atomic::Ordering::SeqCst);

    let updated = async_parallel_update_validate_schema::DataInputModel
        .update(
            existing,
            async_parallel_update_validate_schema::PartialDataInput {
                field_a: Some("aa".into()),
                field_b: Some("bb".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_parallel_update_validate_schema::PartialDataInput {
            field_a: Some("validated-aa".into()),
            field_b: Some("validated-bb".into()),
        }
    );
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod async_parallel_update_validate_schema {
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
        panic!("update validate handlers were not run concurrently");
    }

    struct Fields {
        #[required]
        #[validate(async |v: String, _, _| {
            rendezvous().await;
            Ok(Some(format!("validated-{v}")))
        })]
        pub field_a: String,

        #[required]
        #[validate(async |v: String, _, _| {
            rendezvous().await;
            Ok(Some(format!("validated-{v}")))
        })]
        pub field_b: String,
    }
}
