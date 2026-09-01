---
title: Dependent Fields
---

# Dependent Fields

A dependent field is an output-only field whose value is recomputed whenever at least one of its
declared parent fields is provided (e.g. `username_last_updated_at` should only update whenever
`username` changes).

- Declared with `#[depends_on("parent", ...)]` -- at least one parent, each a string literal
  naming another field on the schema ([lax](./lax.md), [required](./required.md),
  [virtual](./virtuals.md), or another dependent field; no circular dependencies).
- Requires a resolver via `#[resolve(|ctx, opts| -> T)]`, run whenever a parent changes.
- Requires a default via `#[default(value_or_resolver)]`, used until a parent first changes (and
  as the fallback if none of the parents were ever provided).
- May use `#[readonly]` to stop accepting further changes once its value differs from its default.
- May have [`on_delete` and `on_success`](../life-cycles.md) event handlers.
- Requires `output(...)` on the schema, since it never appears on the input struct.

## Example

`computed` depends on `value` (a lax field defaulting to `0`) and resolves to `value + 1`:

```rust
use ivo::ivo_schema;

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod dependents_schema {
    struct Fields {
        #[lax(0)]
        pub value: i32,

        #[depends_on("value")]
        #[default(1)]
        #[resolve(|ctx, _opts| ctx.values().value + 1)]
        pub computed: i32,
    }
}

fn main() {
    // `value` defaults to 0 -- but a lax field's default still counts as "provided" for its
    // own resolution, so `computed` still resolves once: 0 + 1 = 1.
    let created = dependents_schema::DataModel
        .create(dependents_schema::PartialDataInput { value: None }, ())
        .unwrap();
    println!("{:?}", created.data); // Data { value: 0, computed: 1 }

    let created = dependents_schema::DataModel
        .create(
            dependents_schema::PartialDataInput { value: Some(5) },
            (),
        )
        .unwrap();
    println!("{:?}", created.data); // Data { value: 5, computed: 6 }

    let updated = dependents_schema::DataModel
        .update(
            created.data,
            dependents_schema::PartialDataInput { value: Some(10) },
            (),
        )
        .unwrap();
    println!("{:?}", updated.data); // PartialData { value: Some(10), computed: Some(11) }
}
```

`ctx.values()` inside the resolver gives access to every field resolved so far in the same
`create`/`update` call, including sibling dependents earlier in the dependency graph -- see
[Execution Pipeline](../execution-pipeline.md) for the exact phase ordering.

## More examples

- [Default values](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/dependent_defaults.rs)
- [Dependent on dependent](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/dependent_on_dependent.rs)
- [Readonly](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/dependent_readonly.rs)

## Try it in the browser

`value` is a lax field with a default of `0`. `computed` is a dependent field that equals
`value + 1` (with its own fallback default of `1`).

<RustPlayground demo="dependents" />
