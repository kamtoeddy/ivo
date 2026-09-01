---
title: Timestamps
---

# Timestamps

Timestamp fields are output-only fields, automatically populated by a schema-level resolver when a
record is created or updated.

- `#[created_at]` -- set once, at creation.
- `#[updated_at]` -- set at creation and on every update.
- `#[optional_updated_at]` -- like `#[updated_at]`, but typed `Option<T>` and only ever set once an
  update actually happens; stays `None` until then.
- A schema may declare zero or one of each. Both use the same shared, **synchronous** resolver,
  declared once via `#[timestamps(|| ...)]` (or a bare function path) on an anonymous const item.
- Requires `output(...)` on the schema, since timestamps never appear on the input struct.

## Example: default field names

```rust
use chrono::{DateTime, Utc};
use ivo::ivo_schema;

type Timestamp = DateTime<Utc>;

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod timestamps_schema {
    use super::Timestamp;
    use chrono::Utc;

    struct Fields {
        #[lax("default_username".to_string())]
        pub username: String,

        #[created_at]
        pub created_at: Timestamp,

        #[updated_at]
        pub updated_at: Timestamp,
    }

    #[timestamps(|| Utc::now())]
    const _: () = ();
}

fn main() {
    let created = timestamps_schema::DataModel
        .create(timestamps_schema::PartialDataInput { username: None }, ())
        .unwrap();

    println!("{:#?}", created.data);
    // Data { username: "default_username", created_at: ..., updated_at: ... }
    // created_at == updated_at right after creation
}
```

## Example: custom field names, optional `updated_at`

Timestamp fields can be named anything -- the attribute, not the field name, is what matters. Use
`#[optional_updated_at]` when "never updated yet" should be a real, distinct `None` state rather
than defaulting to the creation timestamp:

```rust
struct Fields {
    #[lax("default_username".to_string())]
    pub username: String,

    #[created_at]
    pub inserted_at: Timestamp,

    #[optional_updated_at]
    pub modified_at: Option<Timestamp>,
}
```

`modified_at` is `None` on the freshly created record, and only becomes `Some(...)` after the
first `update`.

## More examples

- [Default names](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/timestamps_with_default_names.rs)
- [Custom names](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/timestamps_with_custom_names.rs)

## Try it in the browser

`username` is a lax field with a default. `created_at` and `updated_at` are populated automatically
from the timestamp resolver.

<RustPlayground demo="timestamps" />
