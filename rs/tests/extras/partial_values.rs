use ivo::ivo_schema;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

// -----------------------------------------------------------------------------
// `ctx.values()` is built up phase by phase at creation
// -----------------------------------------------------------------------------

static SNAPSHOTS: Mutex<Vec<(&'static str, snapshot_schema::PartialData)>> = Mutex::new(Vec::new());

fn record(phase: &'static str, values: &snapshot_schema::PartialData) {
    SNAPSHOTS.lock().unwrap().push((phase, values.clone()));
}

fn take_snapshots() -> Vec<(&'static str, snapshot_schema::PartialData)> {
    std::mem::take(&mut *SNAPSHOTS.lock().unwrap())
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug)))
)]
mod snapshot_schema {
    struct Fields {
        #[required]
        #[validate(|v: &String, ctx, _| {
            super::record("validate", ctx.values());
            Ok(Some(v.trim().to_string()))
        })]
        #[re_validate(|_, ctx, _| {
            super::record("re_validate", ctx.values());
            Ok(None)
        })]
        pub name: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("name")]
        #[default(|ctx, _| {
            super::record("dependent_default", ctx.values());
            0
        })]
        #[resolve(|ctx, _| {
            super::record("resolve", ctx.values());
            ctx.values().name.as_ref().unwrap().len()
        })]
        pub name_len: usize,

        #[constant(|ctx, _| {
            super::record("constant", ctx.values());
            7
        })]
        pub version: i32,

        #[created_at]
        pub created_at: u64,
    }

    #[timestamps(|| 1)]
    const _: () = ();

    #[on_success(|ctx, _| {
        super::record("on_success", ctx.values());
    })]
    const _: () = ();
}

#[test]
fn should_fill_values_phase_by_phase_at_creation_and_expose_every_value_during_updates() {
    use snapshot_schema::PartialData;

    let (created, _, handle_success) = snapshot_schema::DataModel
        .create(
            snapshot_schema::PartialDataInput {
                name: Some(" Ada ".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();
    handle_success();

    let validated = PartialData {
        name: Some("Ada".into()),
        lax: Some(10),
        ..PartialData::default()
    };
    let resolved = PartialData {
        name_len: Some(3),
        ..validated.clone()
    };
    let complete = PartialData {
        version: Some(7),
        created_at: Some(1),
        ..resolved.clone()
    };

    assert_eq!(
        take_snapshots(),
        vec![
            // provided raw value + missing lax default, nothing else yet
            (
                "validate",
                PartialData {
                    name: Some(" Ada ".into()),
                    ..validated.clone()
                }
            ),
            ("re_validate", validated.clone()),
            // dependent defaults are only computed once validation passed
            ("dependent_default", validated.clone()),
            (
                "resolve",
                PartialData {
                    name_len: Some(0),
                    ..validated.clone()
                }
            ),
            // constants see resolved dependents, but no timestamps yet
            ("constant", resolved),
            ("on_success", complete.clone()),
        ]
    );
    assert_eq!(created.name_len, 3);
    assert_eq!(created.version, 7);
    assert_eq!(created.created_at, 1);

    // During an update, every field is set from the start.
    let _ = snapshot_schema::DataModel.update(
        created,
        snapshot_schema::PartialDataInput {
            name: Some("Bob".into()),
            lax: None,
        },
        (),
    );
    let snapshots = take_snapshots();
    assert_eq!(snapshots[0], ("validate", complete));
}

// -----------------------------------------------------------------------------
// Dependent defaults, constants and timestamps are only computed once
// validation has passed
// -----------------------------------------------------------------------------

static DEPENDENT_DEFAULT_CALLS: AtomicUsize = AtomicUsize::new(0);
static CONSTANT_CALLS: AtomicUsize = AtomicUsize::new(0);
static TIMESTAMP_CALLS: AtomicUsize = AtomicUsize::new(0);

fn dependent_default() -> String {
    DEPENDENT_DEFAULT_CALLS.fetch_add(1, Ordering::SeqCst);
    String::new()
}

fn constant_value() -> i32 {
    CONSTANT_CALLS.fetch_add(1, Ordering::SeqCst);
    1
}

fn now() -> u64 {
    TIMESTAMP_CALLS.fetch_add(1, Ordering::SeqCst);
    1
}

fn calls() -> [usize; 3] {
    [
        DEPENDENT_DEFAULT_CALLS.load(Ordering::SeqCst),
        CONSTANT_CALLS.load(Ordering::SeqCst),
        TIMESTAMP_CALLS.load(Ordering::SeqCst),
    ]
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug)))
)]
mod lazy_schema {
    struct Fields {
        #[required]
        #[validate(|v: &String, _, _| {
            if v == "invalid" {
                return Err(("invalid".into(), None));
            }
            Ok(None)
        })]
        pub name: String,

        #[lax(String::new())]
        pub other: String,

        #[depends_on("name")]
        #[default(super::dependent_default)]
        #[resolve(|ctx, _| ctx.values().name.clone().unwrap())]
        pub dependent: String,

        #[constant(super::constant_value)]
        pub constant: i32,

        #[created_at]
        pub created_at: u64,
    }

    #[timestamps(super::now)]
    const _: () = ();

    #[post_validate(["name", "other"], |ctx, _| {
        if ctx.input().other.as_deref() == Some("post_fail") {
            let mut errors = DataInputErrors::new();
            errors.set_other("post failed", None);
            return Err(errors);
        }
        Ok(None)
    })]
    const _: () = ();
}

#[test]
fn should_not_compute_dependent_defaults_constants_or_timestamps_when_creation_fails() {
    let create = |name: Option<&str>, other: Option<&str>| {
        lazy_schema::DataModel.create(
            lazy_schema::PartialDataInput {
                name: name.map(Into::into),
                other: other.map(Into::into),
            },
            (),
        )
    };

    // missing required field, failed validator, failed post-validator
    assert!(create(None, None).is_err());
    assert!(create(Some("invalid"), None).is_err());
    assert!(create(Some("ok"), Some("post_fail")).is_err());
    assert_eq!(calls(), [0, 0, 0]);

    let (created, ..) = create(Some("ok"), None).ok().unwrap();
    assert_eq!(calls(), [1, 1, 1]);
    assert_eq!(created.dependent, "ok");
}

// -----------------------------------------------------------------------------
// Field types don't need `Default`
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Money(u32);

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug)))
)]
mod no_default_schema {
    use super::Money;

    struct Fields {
        #[required]
        pub price: Money,

        #[lax(Money(5))]
        pub fee: Money,

        #[depends_on("price")]
        #[default(Money(0))]
        #[resolve(|ctx, _| {
            Money(ctx.values().price.as_ref().unwrap().0 + ctx.values().fee.as_ref().unwrap().0)
        })]
        pub total: Money,

        #[constant(Money(1))]
        pub version: Money,
    }
}

#[test]
fn should_support_field_types_without_a_default_implementation() {
    let (created, ..) = no_default_schema::DataModel
        .create(
            no_default_schema::PartialDataInput {
                price: Some(Money(10)),
                fee: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.fee, Money(5));
    assert_eq!(created.total, Money(15));
    assert_eq!(created.version, Money(1));

    let (updates, ..) = no_default_schema::DataModel
        .update(
            created,
            no_default_schema::PartialDataInput {
                price: Some(Money(20)),
                fee: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(updates.price, Some(Money(20)));
    assert_eq!(updates.total, Some(Money(25)));
}

// -----------------------------------------------------------------------------
// Partial -> full conversion
// -----------------------------------------------------------------------------

#[test]
fn should_convert_a_complete_partial_into_the_full_struct() {
    let full: no_default_schema::Data = no_default_schema::PartialData {
        price: Some(Money(1)),
        fee: Some(Money(2)),
        total: Some(Money(3)),
        version: Some(Money(4)),
    }
    .into();

    assert_eq!(full.price, Money(1));
    assert_eq!(full.version, Money(4));
}

#[test]
#[should_panic(
    expected = "ivo: field `version` was not set before converting `PartialData` into `Data`"
)]
fn should_panic_when_converting_an_incomplete_partial_into_the_full_struct() {
    let _: no_default_schema::Data = no_default_schema::PartialData {
        price: Some(Money(1)),
        fee: Some(Money(2)),
        total: Some(Money(3)),
        version: None,
    }
    .into();
}

// -----------------------------------------------------------------------------
// Handlers receive `ctx` by reference; a hook keeps it by cloning it
// -----------------------------------------------------------------------------

static SPAWNED_HOOK_NAME: Mutex<Option<String>> = Mutex::new(None);

#[ivo_schema(input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))))]
mod borrowed_ctx_schema {
    struct Fields {
        #[required]
        #[on_success(|ctx, _| {
            // `ctx` is borrowed; clone it to move it into a 'static thread
            let ctx = ctx.clone();
            std::thread::spawn(move || {
                *super::SPAWNED_HOOK_NAME.lock().unwrap() = ctx.values().name.clone();
            })
            .join()
            .unwrap();
        })]
        pub name: String,
    }
}

#[test]
fn should_let_hooks_keep_ctx_beyond_the_handler_by_cloning_it() {
    let (_, _, handle_success) = borrowed_ctx_schema::DataInputModel
        .create(
            borrowed_ctx_schema::PartialDataInput {
                name: Some("Ada".into()),
            },
            (),
        )
        .ok()
        .unwrap();
    handle_success();

    assert_eq!(*SPAWNED_HOOK_NAME.lock().unwrap(), Some("Ada".to_string()));
}
