// `update` must treat a field submitted with its *unchanged* value exactly
// as if it hadn't been submitted at all, matching `rs/`'s
// `filter_input_fields_allowed` (early: raw submitted value vs already-
// stored value) and `resolve_dependent_values` (which only resolves a
// dependent whose parent is in `relevant_fields_provided`). Previously
// rs-next only checked "was the field provided and not ignored/readonly" --
// never comparing against the stored value -- so an update that resubmitted
// unchanged values alongside a genuinely-changed one still ran the
// unchanged fields' `validate`/`re_validate`, ran every `post_validate`
// group covering only unchanged fields, and re-ran every dependent whose
// parent was merely *present* in the update rather than *changed*. In a
// schema with a dependent field feeding a virtual-backed slug (like
// `examples/main_demo`), this produced a "successful" update with silently
// mutated output instead of leaving the unchanged fields alone -- see
// `TODO.md`.
//
// `trigger` is the one field that's genuinely changed, so the update
// proceeds past the early "nothing to update" checkpoint -- this is what
// makes the test actually exercise `validate`/`re_validate`/`post_validate`/
// dependent-resolution's *own* relevance gating, not just the early
// checkpoint (which alone would already skip everything if every field were
// unchanged, masking a regression in any of the later gates). Every other
// handler panics if it's ever actually invoked, so this fails loudly if any
// of these phases regress back to running unconditionally for `name`/
// `other`/`touch_count`.

use ivo::ivo_schema;

#[test]
fn should_not_run_validate_re_validate_post_validate_or_dependent_resolution_for_unchanged_fields_even_when_the_update_is_otherwise_relevant(
) {
    let data = update_relevance_schema::Data {
        name: "same".into(),
        other: "same2".into(),
        touch_count: 0,
        trigger: "old".into(),
    };

    let (updated, ..) = update_relevance_schema::DataModel
        .update(
            data,
            update_relevance_schema::PartialDataInput {
                name: Some("same".into()),
                other: Some("same2".into()),
                trigger: Some("new".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        update_relevance_schema::PartialData {
            name: None,
            other: None,
            touch_count: None,
            trigger: Some("new".into()),
        }
    );
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod update_relevance_schema {
    struct Fields {
        #[required]
        #[validate(|_, _, _| panic!(
            "validate must not run for a field submitted with its unchanged value"
        ))]
        #[re_validate(|_, _, _| panic!(
            "re_validate must not run for a field submitted with its unchanged value"
        ))]
        pub name: String,

        #[required]
        #[validate(|_, _, _| panic!(
            "validate must not run for a field submitted with its unchanged value"
        ))]
        pub other: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub trigger: String,

        #[depends_on("name")]
        #[default(0)]
        #[resolve(|_, _| panic!(
            "dependent resolver must not run when its only parent is unchanged"
        ))]
        pub touch_count: i32,
    }

    #[post_validate(
        ["name", "other"],
        validate = |_, _| panic!(
            "post_validate group must not run when none of its fields are relevant"
        ),
    )]
    const _: () = ();
}

// -----------------------------------------------------------------------------
// Same bug, but with a *virtual* field as the one genuinely-relevant field
// that pushes the update past the early checkpoint (like `examples/main_demo`'s
// `slug_id`/`v_slug`). Virtual fields are always "relevant" once provided and
// not ignored -- their true "did it change" status can't be known until
// dependent resolution runs -- so they deliberately don't get the same
// unchanged-value exclusion as required/lax fields. Three variants cover
// alias handling specifically, since `update_group_relevance_guard` and the
// dependent-resolution `parent_guard` both have to resolve a virtual field's
// flag correctly regardless of whether it has no alias, an alias unrelated
// to any other field, or an alias that collides by name with an existing
// dependent field (the aliasing edge case audited earlier in `TODO.md`).
// -----------------------------------------------------------------------------

#[test]
fn should_correctly_gate_unrelated_fields_when_the_relevant_trigger_is_an_unaliased_virtual_field()
{
    let data = update_relevance_virtual_no_alias_schema::Data {
        name: "same".into(),
        other: "same2".into(),
        touch_count: 0,
        trigger_dependent: "old".into(),
    };

    let (updated, ..) = update_relevance_virtual_no_alias_schema::DataModel
        .update(
            data,
            update_relevance_virtual_no_alias_schema::PartialDataInput {
                name: Some("same".into()),
                other: Some("same2".into()),
                v_trigger: Some("new".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        update_relevance_virtual_no_alias_schema::PartialData {
            name: None,
            other: None,
            touch_count: None,
            trigger_dependent: Some("new".into()),
        }
    );
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod update_relevance_virtual_no_alias_schema {
    struct Fields {
        #[required]
        #[validate(|_, _, _| panic!(
            "validate must not run for a field submitted with its unchanged value"
        ))]
        #[re_validate(|_, _, _| panic!(
            "re_validate must not run for a field submitted with its unchanged value"
        ))]
        pub name: String,

        #[required]
        #[validate(|_, _, _| panic!(
            "validate must not run for a field submitted with its unchanged value"
        ))]
        pub other: String,

        #[ivo_virtual]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub v_trigger: String,

        #[depends_on("name")]
        #[default(0)]
        #[resolve(|_, _| panic!(
            "dependent resolver must not run when its only parent is unchanged"
        ))]
        pub touch_count: i32,

        #[depends_on("v_trigger")]
        #[default(String::new())]
        #[resolve(|ctx, _| ctx.input().v_trigger.clone().unwrap())]
        pub trigger_dependent: String,
    }

    #[post_validate(
        ["name", "other"],
        validate = |_, _| panic!(
            "post_validate group must not run when none of its fields are relevant"
        ),
    )]
    const _: () = ();

    #[post_validate(
        ["v_trigger", "other"],
        validate = |_, _| ::core::result::Result::Ok(::core::option::Option::None),
    )]
    const _: () = ();
}

#[test]
fn should_correctly_gate_unrelated_fields_when_the_relevant_trigger_is_a_virtual_field_with_an_unrelated_alias(
) {
    let data = update_relevance_virtual_alias_schema::Data {
        name: "same".into(),
        other: "same2".into(),
        touch_count: 0,
        trigger_dependent: "old".into(),
    };

    let (updated, ..) = update_relevance_virtual_alias_schema::DataModel
        .update(
            data,
            update_relevance_virtual_alias_schema::PartialDataInput {
                name: Some("same".into()),
                other: Some("same2".into()),
                trigger: Some("new".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        update_relevance_virtual_alias_schema::PartialData {
            name: None,
            other: None,
            touch_count: None,
            trigger_dependent: Some("new".into()),
        }
    );
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod update_relevance_virtual_alias_schema {
    struct Fields {
        #[required]
        #[validate(|_, _, _| panic!(
            "validate must not run for a field submitted with its unchanged value"
        ))]
        #[re_validate(|_, _, _| panic!(
            "re_validate must not run for a field submitted with its unchanged value"
        ))]
        pub name: String,

        #[required]
        #[validate(|_, _, _| panic!(
            "validate must not run for a field submitted with its unchanged value"
        ))]
        pub other: String,

        #[ivo_virtual("trigger")]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub v_trigger: String,

        #[depends_on("name")]
        #[default(0)]
        #[resolve(|_, _| panic!(
            "dependent resolver must not run when its only parent is unchanged"
        ))]
        pub touch_count: i32,

        #[depends_on("v_trigger")]
        #[default(String::new())]
        #[resolve(|ctx, _| ctx.input().trigger.clone().unwrap())]
        pub trigger_dependent: String,
    }

    #[post_validate(
        ["name", "other"],
        validate = |_, _| panic!(
            "post_validate group must not run when none of its fields are relevant"
        ),
    )]
    const _: () = ();

    #[post_validate(
        ["v_trigger", "other"],
        validate = |_, _| ::core::result::Result::Ok(::core::option::Option::None),
    )]
    const _: () = ();
}

#[test]
fn should_correctly_gate_unrelated_fields_when_the_relevant_trigger_is_a_virtual_field_whose_alias_collides_with_a_dependent_field_name(
) {
    let data = update_relevance_virtual_alias_same_as_dependent_schema::Data {
        name: "same".into(),
        other: "same2".into(),
        touch_count: "old".into(),
    };

    let (updated, ..) = update_relevance_virtual_alias_same_as_dependent_schema::DataModel
        .update(
            data,
            update_relevance_virtual_alias_same_as_dependent_schema::PartialDataInput {
                name: Some("same".into()),
                other: Some("same2".into()),
                touch_count: Some("new".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        update_relevance_virtual_alias_same_as_dependent_schema::PartialData {
            name: None,
            other: None,
            touch_count: Some("new".into()),
        }
    );
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod update_relevance_virtual_alias_same_as_dependent_schema {
    struct Fields {
        #[required]
        #[validate(|_, _, _| panic!(
            "validate must not run for a field submitted with its unchanged value"
        ))]
        #[re_validate(|_, _, _| panic!(
            "re_validate must not run for a field submitted with its unchanged value"
        ))]
        pub name: String,

        #[required]
        #[validate(|_, _, _| panic!(
            "validate must not run for a field submitted with its unchanged value"
        ))]
        pub other: String,

        // Alias is deliberately the same string as this schema's own
        // dependent field's Rust name below -- the input-facing name and
        // the output-facing dependent field name collide by string, which
        // must not confuse `update_group_relevance_guard` or the dependent
        // resolver's own `parent_guard` into looking up the wrong flag.
        #[ivo_virtual("touch_count")]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub v_trigger: String,

        #[depends_on("v_trigger")]
        #[default(String::new())]
        #[resolve(|ctx, _| ctx.input().touch_count.clone().unwrap())]
        pub touch_count: String,
    }

    #[post_validate(
        ["name", "other"],
        validate = |_, _| panic!(
            "post_validate group must not run when none of its fields are relevant"
        ),
    )]
    const _: () = ();

    #[post_validate(
        ["v_trigger", "other"],
        validate = |_, _| ::core::result::Result::Ok(::core::option::Option::None),
    )]
    const _: () = ();
}
