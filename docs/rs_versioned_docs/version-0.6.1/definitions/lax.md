---
title: Lax Fields
---

# Lax Fields

A lax field is both an input and output field whose value may or may not be provided at creation
(e.g. `email`, `phone_number`).

- Declared with `#[lax(default_or_resolver)]` -- a static default, or a `|ctx, opts| -> T`
  resolver, used whenever the field is missing.
- May have a [validator and re-validator](../validators.md) via `#[validate]`/`#[re_validate]`.
- May leverage `#[ignore]`, `#[ignore_init]` and `#[ignore_update]` to skip processing under a
  condition.
- May use `#[readonly]` to reject updates, if the default is static.
- May have [`on_delete` and `on_success`](../life-cycles.md) event handlers, and
  [`on_failure`](../life-cycles.md#onfailure) if it has a validator.

## Example

`bio` falls back to a static default when missing, and is validated when provided:

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_schema {
    struct Fields {
        #[lax("default_bio".to_string())]
        #[validate(|v: String, _, _| {
            if v.len() > 100 {
                return Err(("bio too long".into(), None));
            }
            Ok(None)
        })]
        pub bio: String,
    }
}

fn main() {
    let (created, _ctx_options) = lax_schema::DataInputModel
        .create(lax_schema::PartialDataInput { bio: None }, ())
        .unwrap();

    assert_eq!(created.bio, "default_bio");
    println!("{:?}", created); // DataInput { bio: "default_bio" }
}
```

## More examples

- [Default values](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_defaults.rs)
- [Validators & re-validators](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_validators.rs)
- [Readonly](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_readonly.rs)
- [Conditional required](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_required.rs)
- [Ignore](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore.rs)
- [Ignore init](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore_init.rs)
- [Ignore update](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore_update.rs)

## Try it in the browser

`username` has a static default and no validator - leave the input empty to see the default kick
in.

<RustPlayground demo="lax_defaults" />
