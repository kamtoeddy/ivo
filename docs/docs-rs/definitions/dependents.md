---
title: Dependent Fields
---

# Dependent Fields

A dependent field is a purely output field whose value changes whenever at least one field it
depends on is provided and accepted (e.g. `username_last_updated_at` should only update whenever
`username` changes).

- It must have either a default static value or a resolver for the default value.
- It must depend on at least one other field - [lax](./lax.md), [required](./required.md),
  [virtual](./virtuals.md), or another dependent field (no circular dependencies).
- It must have a resolver to generate new values whenever a parent field is provided and accepted.
- It may use [`readonly`](https://github.com/kamtoeddy/ivo#readonly) to stop accepting further
  updates once its value differs from its default.
- It may have [`on_delete` and `on_success`](../life-cycles.md) event handlers.

## Examples

- [Default values](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/dependent_defaults.rs)
- [Dependent on dependent](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/dependent_on_dependent.rs)
- [Readonly](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/dependent_readonly.rs)
