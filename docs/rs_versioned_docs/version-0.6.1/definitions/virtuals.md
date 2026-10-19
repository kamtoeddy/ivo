---
title: Virtual Fields
---

# Virtual Fields

A virtual field is an input-only field whose value may or may not be provided at creation, used to
trigger a change in one or more [dependent fields](./dependents.md) that depend on it -- it's
validated/sanitized like a real field, but never stored on the output struct directly.

- Declared with `#[ivo_virtual]`, or `#[ivo_virtual("alias")]` to expose it under a different name
  on the generated input struct.
- Must be referenced by at least one `#[depends_on(...)]` field -- either by its declared name, or
  by its alias if it has one.
- Must have a [validator](../validators.md) via `#[validate]`; may also have a `#[re_validate]`.
- May have a `#[sanitize(|value, ctx, opts| -> T)]` mutator, run after
  validate/re-validate/post_validate succeed.
- May leverage `#[ignore]`, `#[ignore_init]` and `#[ignore_update]` provision rules.
- May have [`on_failure` and `on_success`](../life-cycles.md) event handlers.
- Requires `output(...)` on the schema, since it never appears on the output struct.

## Example: no alias

`computed` depends on the virtual field `trigger` and mirrors whatever value it was given:

```rust
use ivo::ivo_schema;

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod virtuals_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub trigger: String,

        #[depends_on("trigger")]
        #[default(String::new())]
        #[resolve(|ctx, _opts| ctx.input().trigger.clone().unwrap_or_default())]
        pub computed: String,
    }
}

fn main() {
    let (created, _ctx_options) = virtuals_schema::DataModel
        .create(
            virtuals_schema::PartialDataInput {
                trigger: Some("hello".into()),
            },
            (),
        )
        .unwrap();

    assert_eq!(created.computed, "hello");
    println!("{:?}", created); // Data { computed: "hello" }
}
```

## Example: aliased

`#[ivo_virtual("password_confirmation")]` exposes the field as `password_confirmation` on the
input struct, while the field itself is still named (and referenced by `#[depends_on(...)]`) as
`password_confirm`:

```rust
#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod virtuals_alias_schema {
    struct Fields {
        #[ivo_virtual("password_confirmation")]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub password_confirm: String,

        #[depends_on("password_confirm")]
        #[default(String::new())]
        #[resolve(|ctx, _opts| ctx.input().password_confirmation.clone().unwrap_or_default())]
        pub password: String,
    }
}
```

Note `ctx.input().password_confirmation` above -- the input struct's field is named after the
_alias_, not the virtual field's own declared name. `#[depends_on("password_confirm")]` still uses
the declared name (an alias may also depend on itself by its own name if it collides with an
existing dependent field name -- see the alias-collision example below).

## More examples

- [Validators & re-validators](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals.rs)
- [With alias name](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_alias_name.rs)
- [With alias name same as dependent](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_alias_name_same_as_dependent.rs)
- [Conditional required](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_required.rs)
- [Ignore](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_ignore.rs)
- [Ignore init](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_ignore_init.rs)
- [Ignore update](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_ignore_update.rs)

## Try it in the browser

`virtual_field` is a virtual input field. The output `dependent` uses its value when provided,
otherwise it falls back to a default. Leave `virtual_field` empty or remove it to see the default.

<RustPlayground demo="virtuals" />
