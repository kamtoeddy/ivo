---
title: Validators
---

# Validators

A validator assesses (and may transform) a field's value. [Lax](./definitions/lax.md),
[required](./definitions/required.md) and [virtual](./definitions/virtuals.md) fields can each have
up to two: a primary `#[validate]` and a secondary `#[re_validate]`, which only runs once the
primary validator has already succeeded.

Both share the same signature: `|value, ctx, opts| -> Result<Option<T>, (String, Option<Metadata>)>`.

- `Ok(None)` accepts the value as-is.
- `Ok(Some(new_value))` replaces it.
- `Err((reason, metadata))` rejects it -- `metadata` is `None` unless you're using a
  [custom `IvoErrorSanitizer`](./options.md#custom-error-payloads-with-ivoerrorsanitizer).

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod re_validate_schema {
    struct Fields {
        #[required]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        #[re_validate(|v: String, _, _| Ok(Some(format!("revalidated-{v}"))))]
        pub username: String,
    }
}

fn main() {
    let (created, _ctx_options) = re_validate_schema::DataInputModel
        .create(
            re_validate_schema::PartialDataInput {
                username: Some("jane".into()),
            },
            (),
        )
        .unwrap();

    println!("{:?}", created); // DataInput { username: "revalidated-jane" }
}
```

`#[re_validate]` requires `#[validate]` to be present on the same field -- it's a compile error
otherwise. See it used for real (checking a username isn't already taken, via `ctx_options`) in
[`examples/main_demo`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/domain.rs).

- Validators & re-validators: see
  [lax fields](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_validators.rs),
  [required fields](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_with_re_validate.rs)
  and [virtual fields](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals.rs)
- Custom required errors: see [Required Fields](./definitions/required.md#custom-required-error)

## Built-in validation helpers

Enable the optional `validators` feature (`ivo = { version = "*", features = ["validators"] }`,
crate `ivo-validators`) for a small set of built-in validators:

- `validate_email(value: &str) -> Result<String, String>`
- `validate_credit_card(value: &str) -> Result<String, String>`

```rust
#[lax(None)]
#[validate(|v: Option<String>, _, _| {
    let Some(email) = v else { return Ok(None) };
    match validate_email(&email) {
        Ok(validated) => Ok(Some(Some(validated))),
        Err(e) => Err((e, None)),
    }
})]
pub email: Option<String>,
```

See the
[crate source](https://github.com/kamtoeddy/ivo/blob/main/rs/crates/validators/src/lib.rs)
for implementation details, and
[`main_demo`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/domain.rs)
for a schema that uses them.
