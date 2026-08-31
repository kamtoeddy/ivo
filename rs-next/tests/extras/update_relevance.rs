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
fn should_not_run_validate_re_validate_post_validate_or_dependent_resolution_for_unchanged_fields_even_when_the_update_is_otherwise_relevant()
{
    let data = update_relevance_schema::Data {
        name: "same".into(),
        other: "same2".into(),
        touch_count: 0,
        trigger: "old".into(),
    };

    let updated = update_relevance_schema::DataModel
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
        updated.data,
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
