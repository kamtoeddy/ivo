---
title: Required Fields
---

# Required Fields

A required field is both an input and output field whose value must be provided at creation (e.g.
`username`). It's optional (and immutable unless explicitly allowed) on update.

- Declared with the bare `#[required]` field-type attribute.
- Must have a [validator](../validators.md) via `#[validate]`; may also have a `#[re_validate]`.
- May customize the missing-field error via `#[required_error(...)]` -- a static string or a
  `|raw_input, opts| -> String` closure.
- May leverage `#[ignore_update]` (resolver form only) and `#[readonly]` to prevent further
  updates.
- May have [`on_delete` and `on_success`](../life-cycles.md) event handlers, and
  [`on_failure`](../life-cycles.md#onfailure).

## Example

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod required_schema {
    struct Fields {
        #[required]
        #[validate(|v: String, _, _| {
            if v.len() < 3 {
                return Err(("username too short".into(), None));
            }
            Ok(None)
        })]
        pub username: String,
    }
}

fn main() {
    let err = required_schema::DataInputModel
        .create(required_schema::PartialDataInput { username: None }, ())
        .unwrap_err();
    println!("{:?}", err.errors.get("username").unwrap().reason); // "field is required"

    let created = required_schema::DataInputModel
        .create(
            required_schema::PartialDataInput {
                username: Some("jane".into()),
            },
            (),
        )
        .unwrap();
    println!("{:?}", created.data); // DataInput { username: "jane" }
}
```

## Custom required error

```rust
#[required]
#[required_error(|_raw_input, _opts| "\"username\" was not provided!".to_string())]
#[validate(|v: String, _, _| Ok(Some(v)))]
pub username: String,
```

## More examples

- [Required](https://github.com/kamtoeddy/ivo/blob/main/rs-next/examples/required.rs)
- [Custom required error](https://github.com/kamtoeddy/ivo/blob/main/rs-next/examples/required_error.rs)
- [Re-validators](https://github.com/kamtoeddy/ivo/blob/main/rs-next/examples/required_with_re_validate.rs)
- [Readonly](https://github.com/kamtoeddy/ivo/blob/main/rs-next/examples/required_readonly.rs)
- [Ignore update](https://github.com/kamtoeddy/ivo/blob/main/rs-next/examples/required_with_ignore_update.rs)

## Try it in the browser

`username` is required with no other constraints - leave the input empty to see the required
error, or provide a value to see it accepted.

<RustPlayground demo="required" />
