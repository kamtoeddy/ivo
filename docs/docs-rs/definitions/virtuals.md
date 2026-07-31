---
title: Virtual Fields
---

# Virtual Fields

A virtual field is a purely input field whose value may or may not be provided at creation, used
to trigger a change in one or more fields that depend on it.

- It must have one or more [dependent fields](./dependents.md) depending on it.
- It must have a [validator](../validators.md).
- It may also have a re-validator.
- It may have an `alias` - a different field name on the input struct, used in place of the
  actual field name (only allowed if the corresponding output field is a dependent field that
  directly depends on this virtual field).
- It may have a sanitizer.
- It may leverage `ignore`, `ignore_init` and `ignore_update` provision rules.
- It may have [`on_failure` and `on_success`](../life-cycles.md) event handlers.

## Examples

- [Validators & re-validators](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals.rs)
- [With alias name](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_alias_name.rs)
- [With alias name same as dependent](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_alias_name_same_as_dependent.rs)
- [Required](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_required.rs)
- [Ignore](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_ignore.rs)
- [Ignore init](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_ignore_init.rs)
- [Ignore update](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_ignore_update.rs)
