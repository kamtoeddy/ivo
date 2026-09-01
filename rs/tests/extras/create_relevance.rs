// `create`'s `post_validate` groups used to run unconditionally regardless of
// whether any of their fields were actually submitted, unlike `update` (see
// `update_relevance.rs`) and unlike `rs/`, whose `post_validate()` only runs
// a group if `fields.iter().any(|f| fields_collection.is_relevant_config_name(f))`
// -- built from presence in the *raw* input, before any default is applied.
// A lax field that's only ever defaulted (never submitted) doesn't count as
// relevant there, and a group covering only such fields should never run at
// creation. See `create_group_relevance_guard` in `crates/derive/src/lib.rs`
// and `TODO.md`.

use ivo::ivo_schema;

#[test]
fn should_not_run_post_validate_group_at_creation_when_none_of_its_fields_were_submitted() {
    let created = create_relevance_schema::DataInputModel
        .create(
            create_relevance_schema::PartialDataInput {
                a: None,
                b: None,
                trigger: Some("x".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data.a, 0);
    assert_eq!(created.data.b, 0);
    assert_eq!(created.data.trigger, "x");
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod create_relevance_schema {
    struct Fields {
        #[lax(0)]
        pub a: i32,

        #[lax(0)]
        pub b: i32,

        #[required]
        pub trigger: String,
    }

    #[post_validate(
        ["a", "b"],
        validate = |_, _| panic!(
            "post_validate group must not run at creation when none of its fields were submitted"
        ),
    )]
    const _: () = ();
}

// -----------------------------------------------------------------------------
// Positive case: the OR-across-fields relevance gating must still fire a
// group when *any* one of its fields was submitted, not just stay silent
// when none are (mirrors the equivalent check already made for `update`'s
// guard in `update_relevance.rs`).
// -----------------------------------------------------------------------------

#[test]
fn should_run_post_validate_group_at_creation_when_one_of_its_fields_was_submitted() {
    let created = create_relevance_positive_schema::DataInputModel
        .create(
            create_relevance_positive_schema::PartialDataInput {
                a: None,
                b: Some(7),
                trigger: Some("x".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    // `b` alone was submitted; the group must still run and its `validate`
    // handler unconditionally overwrites `a` to prove it fired.
    assert_eq!(created.data.a, 999);
    assert_eq!(created.data.b, 7);
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod create_relevance_positive_schema {
    struct Fields {
        #[lax(0)]
        pub a: i32,

        #[lax(0)]
        pub b: i32,

        #[required]
        pub trigger: String,
    }

    #[post_validate(
        ["a", "b"],
        validate = |_, _| {
            let mut updates = PartialDataInput::new();
            updates.set_a(999);
            Ok(Some(updates))
        },
    )]
    const _: () = ();
}

// -----------------------------------------------------------------------------
// Virtual fields reuse the same `__virtual_provided_*` flags `update` uses
// (see `create_group_relevance_guard`), not a re-derived check -- a virtual
// field only counts as relevant once actually provided and not ignored,
// same as everywhere else in the pipeline.
// -----------------------------------------------------------------------------

#[test]
fn should_not_run_post_validate_group_at_creation_when_its_only_virtual_field_was_not_provided() {
    let created = create_relevance_virtual_schema::DataModel
        .create(
            create_relevance_virtual_schema::PartialDataInput {
                a: None,
                v_trigger: None,
                trigger: Some("x".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data.a, 0);
    assert_eq!(created.data.trigger, "x");
}

#[test]
fn should_run_post_validate_group_at_creation_when_its_virtual_field_was_provided() {
    let created = create_relevance_virtual_schema::DataModel
        .create(
            create_relevance_virtual_schema::PartialDataInput {
                a: None,
                v_trigger: Some("anything".into()),
                trigger: Some("x".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data.a, 999);
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod create_relevance_virtual_schema {
    struct Fields {
        #[lax(0)]
        pub a: i32,

        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        pub v_trigger: String,

        #[required]
        pub trigger: String,

        // Every virtual field must be referenced by at least one dependent;
        // unrelated to the relevance gating under test.
        #[depends_on("v_trigger")]
        #[default(0)]
        #[resolve(|_, _| 0)]
        pub derived: i32,
    }

    #[post_validate(
        ["a", "v_trigger"],
        validate = |_, _| {
            let mut updates = PartialDataInput::new();
            updates.set_a(999);
            Ok(Some(updates))
        },
    )]
    const _: () = ();
}
