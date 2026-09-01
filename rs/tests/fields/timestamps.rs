use ivo::ivo_schema;

//
// NOTE ON PORTING
//
// The new `#[ivo_schema(...)]` macro supports timestamps via:
//   - `#[created_at]` / `#[updated_at]` / `#[optional_updated_at]` on a field
//   - `#[timestamps(|| ...)]` or `#[timestamps(path::to_now)]` const _: () = ();` to provide the resolver
//
// `updated_at` is re-resolved on every successful update; `optional_updated_at`
// is `None` on create and `Some(value)` on update.
//
// Async timestamp resolvers are not supported by the current macro, so only
// sync variants are ported.
//

#[test]
fn should_respect_created_at_timestamp_with_default_name() {
    let lax = 400;

    let (created, ..) = sync_created_at_default_name_schema::DataModel
        .create(
            sync_created_at_default_name_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.lax, lax);
    assert!(created.created_at > 0);

    let lax_update = 200;

    let (updated, ..) = sync_created_at_default_name_schema::DataModel
        .update(
            created.clone(),
            sync_created_at_default_name_schema::PartialDataInput {
                lax: Some(lax_update),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(updated.lax, Some(lax_update));
    assert_eq!(updated.created_at, None);
}

#[test]
fn should_respect_created_at_timestamp_with_custom_name() {
    let lax = 400;

    let (created, ..) = sync_created_at_custom_name_schema::DataModel
        .create(
            sync_created_at_custom_name_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.lax, lax);
    assert!(created.custom_created_at > 0);

    let lax_update = 200;

    let (updated, ..) = sync_created_at_custom_name_schema::DataModel
        .update(
            created.clone(),
            sync_created_at_custom_name_schema::PartialDataInput {
                lax: Some(lax_update),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(updated.lax, Some(lax_update));
    assert_eq!(updated.custom_created_at, None);
}

#[test]
fn should_respect_updated_at_timestamp_with_default_name() {
    let lax = 400;

    let (created, ..) = sync_updated_at_default_name_schema::DataModel
        .create(
            sync_updated_at_default_name_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.lax, lax);
    assert!(created.updated_at > 0);

    let lax_update = 200;

    let (updated, ..) = sync_updated_at_default_name_schema::DataModel
        .update(
            created.clone(),
            sync_updated_at_default_name_schema::PartialDataInput {
                lax: Some(lax_update),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(updated.lax, Some(lax_update));
    assert!(updated.updated_at.is_some());
    assert!(updated.updated_at.unwrap() >= created.updated_at);
}

#[test]
fn should_respect_updated_at_timestamp_with_custom_name() {
    let lax = 400;

    let (created, ..) = sync_updated_at_custom_name_schema::DataModel
        .create(
            sync_updated_at_custom_name_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.lax, lax);
    assert!(created.custom_updated_at > 0);

    let lax_update = 200;

    let (updated, ..) = sync_updated_at_custom_name_schema::DataModel
        .update(
            created.clone(),
            sync_updated_at_custom_name_schema::PartialDataInput {
                lax: Some(lax_update),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(updated.lax, Some(lax_update));
    assert!(updated.custom_updated_at.is_some());
    assert!(updated.custom_updated_at.unwrap() >= created.custom_updated_at);
}

#[test]
fn should_respect_optional_updated_at_timestamp_with_default_name() {
    let lax = 400;

    let (created, ..) = sync_optional_updated_at_default_name_schema::DataModel
        .create(
            sync_optional_updated_at_default_name_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.lax, lax);
    assert_eq!(created.updated_at, None);

    let lax_update = 200;

    let (updated, ..) = sync_optional_updated_at_default_name_schema::DataModel
        .update(
            created.clone(),
            sync_optional_updated_at_default_name_schema::PartialDataInput {
                lax: Some(lax_update),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(updated.lax, Some(lax_update));
    assert!(updated.updated_at.is_some());
}

#[test]
fn should_respect_optional_updated_at_timestamp_with_custom_name() {
    let lax = 400;

    let (created, ..) = sync_optional_updated_at_custom_name_schema::DataModel
        .create(
            sync_optional_updated_at_custom_name_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.lax, lax);
    assert_eq!(created.custom_updated_at, None);

    let lax_update = 200;

    let (updated, ..) = sync_optional_updated_at_custom_name_schema::DataModel
        .update(
            created.clone(),
            sync_optional_updated_at_custom_name_schema::PartialDataInput {
                lax: Some(lax_update),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(updated.lax, Some(lax_update));
    assert!(updated.custom_updated_at.is_some());
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_created_at_default_name_schema {
    type Timestamp = u128;

    fn now() -> Timestamp {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros()
    }

    struct Fields {
        #[lax(20)]
        pub lax: i32,

        #[created_at]
        pub created_at: Timestamp,
    }

    #[timestamps(now)]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_created_at_custom_name_schema {
    type Timestamp = u128;

    fn now() -> Timestamp {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros()
    }

    struct Fields {
        #[lax(20)]
        pub lax: i32,

        #[created_at]
        pub custom_created_at: Timestamp,
    }

    #[timestamps(|| now())]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_updated_at_default_name_schema {
    type Timestamp = u128;

    fn now() -> Timestamp {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros()
    }

    struct Fields {
        #[lax(20)]
        pub lax: i32,

        #[updated_at]
        pub updated_at: Timestamp,
    }

    #[timestamps(|| now())]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_updated_at_custom_name_schema {
    type Timestamp = u128;

    fn now() -> Timestamp {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros()
    }

    struct Fields {
        #[lax(20)]
        pub lax: i32,

        #[updated_at]
        pub custom_updated_at: Timestamp,
    }

    #[timestamps(|| now())]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_optional_updated_at_default_name_schema {
    type Timestamp = u128;

    fn now() -> Timestamp {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros()
    }

    struct Fields {
        #[lax(20)]
        pub lax: i32,

        #[optional_updated_at]
        pub updated_at: Option<Timestamp>,
    }

    #[timestamps(|| now())]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_optional_updated_at_custom_name_schema {
    type Timestamp = u128;

    fn now() -> Timestamp {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros()
    }

    struct Fields {
        #[lax(20)]
        pub lax: i32,

        #[optional_updated_at]
        pub custom_updated_at: Option<Timestamp>,
    }

    #[timestamps(|| now())]
    const _: () = ();
}

// -----------------------------------------------------------------------------
// Resolver call count
// -----------------------------------------------------------------------------

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_created_and_updated_at_call_count_schema {
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    fn now() -> u128 {
        CALL_COUNT.fetch_add(1, Ordering::SeqCst);
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros()
    }

    struct Fields {
        #[lax(20)]
        pub lax: i32,

        #[created_at]
        pub created_at: u128,

        #[updated_at]
        pub updated_at: u128,
    }

    #[timestamps(|| now())]
    const _: () = ();
}

#[test]
fn should_call_the_timestamp_resolver_at_most_once_per_create_call() {
    use std::sync::atomic::Ordering;

    let before = sync_created_and_updated_at_call_count_schema::CALL_COUNT.load(Ordering::SeqCst);

    let (created, ..) = sync_created_and_updated_at_call_count_schema::DataModel
        .create(
            sync_created_and_updated_at_call_count_schema::PartialDataInput { lax: Some(1) },
            (),
        )
        .ok()
        .unwrap();

    let after = sync_created_and_updated_at_call_count_schema::CALL_COUNT.load(Ordering::SeqCst);

    assert_eq!(
        after - before,
        1,
        "the timestamp resolver must be invoked exactly once per create call, even with both \
         #[created_at] and #[updated_at] declared"
    );
    assert_eq!(created.created_at, created.updated_at);
}
