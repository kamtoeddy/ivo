---
title: Constant Fields
---

# Constant Fields

A constant is an output-only field whose value is computed once, at creation, and never accepted
from input or changed by an update (e.g. `id`).

- Declared with `#[constant(value_or_resolver)]` -- a static value, or a `|ctx, opts| -> T`
  resolver (sync or async, with access to context and options like any other handler).
- Requires `output(...)` on the schema, since it never appears on the input struct.
- May have [`on_delete` and `on_success`](../life-cycles.md) event handlers.

## Example

`id` is a static constant. `label` is computed once via a zero-argument resolver closure:

```rust
use ivo::ivo_schema;

#[ivo_schema(
    input(ItemInput, derive(Debug, Clone, PartialEq)),
    output(Item, derive(Debug, Clone, PartialEq))
)]
mod item_schema {
    struct Fields {
        #[constant(1234)]
        pub id: i32,

        #[constant(|| "generated".to_string())]
        pub label: String,

        #[required]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub name: String,
    }
}

fn main() {
    let created = item_schema::ItemModel
        .create(
            item_schema::PartialItemInput {
                name: Some("widget".into()),
            },
            (),
        )
        .unwrap();

    assert_eq!(created.data.id, 1234);
    assert_eq!(created.data.label, "generated");

    println!("{:#?}", created.data);
    // Item { id: 1234, label: "generated", name: "widget" }
}
```

An update to `item_schema::PartialItemInput` has no `id`/`label` fields at all -- there's nothing
to submit for a constant, and no way to change it after creation.

## Try it in the browser

`id` is a constant (always `1234`); `username` is lax with a default. Edit the input and run it.

<RustPlayground demo="constants" />
