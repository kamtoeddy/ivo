---
title: Dependent Fields
---

# Dependent Fields

A dependent field is a purely output field whose value changes whenever at least one field it
depends on is provided and accepted (e.g. `username_last_updated_at` should only update whenever
`username` changes).

- It must have either a default static value or a resolver for the default value.
- It must depend on at least one other field - [lax](./lax.md), [required](./required.md),
  [virtual](./virtuals.md), or another dependent field (no circular dependencies). The parent
  fields are supplied as the second argument to `dependent_field`.
- It must have a resolver to generate new values whenever a parent field is provided and accepted.
- It may use [`readonly`](https://github.com/kamtoeddy/ivo#readonly) to stop accepting further
  updates once its value differs from its default.
- It may have [`on_delete` and `on_success`](../life-cycles.md) event handlers.

## Example

`value` is a lax field with a default of `0`. `computed` is a dependent field that equals
`value + 1` (with its own fallback default of `1`). The dependency on `value` is declared as the
second argument to `dependent_field`:

```rust
use std::{future::ready, sync::LazyLock};
use ivo::{dependent_field, lax_field, IvoContext, IvoInputStruct, IvoModel, IvoStruct};

type Ctx = IvoContext<DataInput, Data>;

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
struct DataInput {
    pub value: i32,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
struct Data {
    pub value: i32,
    pub computed: i32,
}

static MODEL: LazyLock<IvoModel<DataInput, Data>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(lax_field("value").default(0)).field(
                dependent_field("computed", ["value"])
                    .default(1)
                    .resolve(|ctx: Ctx, _| ready(ctx.values().value.unwrap_or(0) + 1)),
            )
        },
        |o| o,
    )
});
```

## Examples

- [Default values](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/dependent_defaults.rs)
- [Dependent on dependent](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/dependent_on_dependent.rs)
- [Readonly](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/dependent_readonly.rs)

## Try it in the browser

`value` is a lax field with a default of `0`. `computed` is a dependent field that equals `value + 1`
(with its own fallback default of `1`).

<RustPlayground demo="dependents" />
