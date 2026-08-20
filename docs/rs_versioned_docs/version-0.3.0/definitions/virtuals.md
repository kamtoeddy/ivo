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

## Example

`virtual_field` is a virtual input field. The output `dependent` uses its value when provided,
otherwise it falls back to a default. The dependent field declares its dependency with
`.depends_on(...)`:

```rust
use std::{future::ready, sync::LazyLock};
use ivo::{dependent_field, virtual_field, IvoContext, IvoInputStruct, IvoModel, IvoStruct};

const DEFAULT_DEPENDENT: &str = "DEFAULT_DEPENDENT_VALUE";

type Ctx = IvoContext<DataInput, Data>;

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
struct DataInput {
    pub virtual_field: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
struct Data {
    pub dependent: String,
}

static MODEL: LazyLock<IvoModel<DataInput, Data>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(virtual_field("virtual_field").validate(|v: String, _, _| ready(Ok(Some(v)))))
                .field(
                    dependent_field("dependent")
                        .default(DEFAULT_DEPENDENT.to_string())
                        .depends_on(["virtual_field"])
                        .resolve(|ctx: Ctx, _| {
                            ready(
                                ctx.input()
                                    .virtual_field
                                    .clone()
                                    .unwrap_or_else(|| DEFAULT_DEPENDENT.to_string()),
                            )
                        }),
                )
        },
        |o| o,
    )
});
```

## Examples

- [Validators & re-validators](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals.rs)
- [With alias name](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_alias_name.rs)
- [With alias name same as dependent](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_alias_name_same_as_dependent.rs)
- [Required](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_required.rs)
- [Ignore](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_ignore.rs)
- [Ignore init](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_ignore_init.rs)
- [Ignore update](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/virtuals_with_ignore_update.rs)
