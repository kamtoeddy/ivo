---
title: Validators
---

# Validators

A validator is a function that assesses the validity of a field's value (one validator per
field). Fields can have up to two validators (primary and re-validator) - see the
[root README](https://github.com/kamtoeddy/ivo#resolvers) for the full definitions of
`validator`, `re-validator`, `post-validator` and `required resolver`, which apply the same way in
Rust as in TypeScript.

- Validators & re-validators: see
  [lax fields](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_validators.rs),
  [required fields](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_with_re_validate.rs)
  and [virtual fields](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals.rs)
- Custom required errors: see
  [this example](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_error.rs)

## Built-in validation helpers

Enable the optional `validators` feature (`ivo = { version = "*", features = ["validators"] }`,
crate `ivo-validators`) for a small set of built-in validators:

- `validate_email(value: &str) -> Result<String, String>`
- `validate_credit_card(value: &str) -> Result<String, String>`

See the
[crate source](https://github.com/kamtoeddy/ivo/blob/main/rs/crates/validators/src/lib.rs) for
implementation details, and [`main_demo`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/main.rs)
for a schema that uses them.
