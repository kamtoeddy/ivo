---
title: Lax Fields
---

# Lax Fields

A lax field is both an input and output field whose value may or may not be provided at creation
(e.g. `email`, `phone_number`).

- It must have either a default static value or a resolver for the default value.
- It may have a [validator](../validators.md).
- It may also have a re-validator.
- It may leverage `ignore`, `ignore_init` and `ignore_update` provision rules.
- It may leverage `readonly` if the default value is static.
- It may have [`on_delete` and `on_success`](../life-cycles.md) event handlers, and
  [`on_failure`](../life-cycles.md#onfailure) if it has a validator.

## Examples

- [Default values](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_defaults.rs)
- [Validators & re-validators](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_validators.rs)
- [Readonly](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_readonly.rs)
- [Required](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_required.rs)
- [Ignore](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore.rs)
- [Ignore init](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore_init.rs)
- [Ignore update](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore_update.rs)

## Try it in the browser

`username` has a static default and no validator - leave the input empty to see the default kick
in.

<RustPlayground demo="lax_defaults" />
