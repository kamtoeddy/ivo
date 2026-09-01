// `IvoContext::raw_input()` must track the schema's original partial input --
// exactly what the caller passed to `create`/`update`, captured once and
// never mutated afterward -- distinct from `input()`, which evolves as the
// pipeline runs (e.g. a virtual field's `#[validate]` may rewrite its own
// value in `input()` before `#[re_validate]` runs). Previously `raw_input()`
// was just an alias for `input()`, so it always reported whatever `input()`
// currently held instead of the pristine original (see `TODO.md`).
//
// Each schema below rewrites a virtual field's value in `#[validate]`, then
// `#[re_validate]` -- which runs against the ctx rebuilt right after
// `#[validate]` -- asserts that `raw_input()` still reports the untouched
// original while `input()` reports the rewritten value, proving the two are
// genuinely distinct snapshots, not aliases of the same field.

use ivo::ivo_schema;

#[test]
fn should_expose_the_original_raw_input_distinct_from_current_input_during_create() {
    let (created, ..) = raw_input_create_schema::DataModel
        .create(
            raw_input_create_schema::PartialDataInput {
                virtual_field: Some("original".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.dependent, "REWRITTEN-original");
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod raw_input_create_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|v: String, _, _| Ok(Some(format!("REWRITTEN-{v}"))))]
        #[re_validate(|v: String, ctx, _| {
            assert_eq!(
                ctx.raw_input().virtual_field.as_deref(),
                Some("original"),
                "raw_input() must still show the value exactly as submitted"
            );
            assert_eq!(
                ctx.input().virtual_field.as_deref(),
                Some("REWRITTEN-original"),
                "input() must show the validator-rewritten value"
            );
            let _ = v;
            Ok(None)
        })]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(String::new())]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap())]
        pub dependent: String,
    }
}

#[test]
fn should_expose_the_original_raw_input_distinct_from_current_input_during_update() {
    let data = raw_input_update_schema::Data {
        dependent: "old".into(),
    };

    let (updated, ..) = raw_input_update_schema::DataModel
        .update(
            data,
            raw_input_update_schema::PartialDataInput {
                virtual_field: Some("original-update".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.dependent,
        Some("REWRITTEN-original-update".to_string())
    );
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod raw_input_update_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|v: String, _, _| Ok(Some(format!("REWRITTEN-{v}"))))]
        #[re_validate(|v: String, ctx, _| {
            assert_eq!(
                ctx.raw_input().virtual_field.as_deref(),
                Some("original-update"),
                "raw_input() must still show the value exactly as submitted"
            );
            assert_eq!(
                ctx.input().virtual_field.as_deref(),
                Some("REWRITTEN-original-update"),
                "input() must show the validator-rewritten value"
            );
            let _ = v;
            Ok(None)
        })]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(String::new())]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap())]
        pub dependent: String,
    }
}

// -----------------------------------------------------------------------------
// Lifecycle-hook (`on_success`/`on_failure`) trigger contexts previously
// built `ctx` entirely from the pristine original input, so `ctx.input()`
// inside those handlers reported the *original* value even after later
// pipeline phases (validate, re-validate, sanitize, post_validate) had
// rewritten it -- the same bug `raw_input()` had, just baked into the
// trigger-construction call sites instead of the accessor itself. `#[on_
// success]` runs only after the whole pipeline completes, so it's a direct
// way to prove `ctx.input()` there now reflects the final, fully-rewritten
// state while `ctx.raw_input()` still reflects the untouched original.
// -----------------------------------------------------------------------------

#[test]
fn should_expose_the_final_rewritten_input_not_the_original_in_on_success_triggers_during_create() {
    let (_, _ctx_options, handle_success) = raw_input_trigger_create_schema::DataModel
        .create(
            raw_input_trigger_create_schema::PartialDataInput {
                virtual_field: Some("original".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    handle_success();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod raw_input_trigger_create_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|v: String, _, _| Ok(Some(format!("REWRITTEN-{v}"))))]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(String::new())]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap())]
        pub dependent: String,
    }

    #[on_success(|ctx, _| {
        assert_eq!(
            ctx.raw_input().virtual_field.as_deref(),
            Some("original"),
            "raw_input() must still show the value exactly as submitted"
        );
        assert_eq!(
            ctx.input().virtual_field.as_deref(),
            Some("REWRITTEN-original"),
            "input() in an on_success trigger must reflect the pipeline's \
             final state, not the pristine original"
        );
    })]
    const _: () = ();
}

#[test]
fn should_expose_the_final_rewritten_input_not_the_original_in_on_success_triggers_during_update() {
    let data = raw_input_trigger_update_schema::Data {
        dependent: "old".into(),
    };

    let (_, _ctx_options, handle_success) = raw_input_trigger_update_schema::DataModel
        .update(
            data,
            raw_input_trigger_update_schema::PartialDataInput {
                virtual_field: Some("original-update".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    handle_success();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod raw_input_trigger_update_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|v: String, _, _| Ok(Some(format!("REWRITTEN-{v}"))))]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(String::new())]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap())]
        pub dependent: String,
    }

    #[on_success(|ctx, _| {
        assert_eq!(
            ctx.raw_input().virtual_field.as_deref(),
            Some("original-update"),
            "raw_input() must still show the value exactly as submitted"
        );
        assert_eq!(
            ctx.input().virtual_field.as_deref(),
            Some("REWRITTEN-original-update"),
            "input() in an on_success trigger must reflect the pipeline's \
             final state, not the pristine original"
        );
    })]
    const _: () = ();
}
