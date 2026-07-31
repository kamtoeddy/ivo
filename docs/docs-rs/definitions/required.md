---
title: Required Fields
---

# Required Fields

A required field is both an input and output field whose value must be provided at creation
(e.g. `username`).

- It must have a [validator](../validators.md).
- It may also have a re-validator.
- It may leverage `ignore_update` and `readonly` to prevent further updates.
- It may have [`on_delete` and `on_success`](../life-cycles.md) event handlers, and
  [`on_failure`](../life-cycles.md#onfailure).

## Examples

- [Required](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required.rs)
- [Custom required error](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_error.rs)
- [Re-validators](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_with_re_validate.rs)
- [Readonly](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_readonly.rs)
- [Ignore update](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/required_with_ignore_update.rs)

## Try it in the browser

`username` is required with no other constraints - leave the input empty to see the required
error, or provide a value to see it accepted.

<RustPlayground demo="required" />
