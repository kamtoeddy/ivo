---
title: Timestamps
---

# Timestamps

Timestamp fields are output-only fields that are automatically populated by the schema when a record
is created or updated.

- A schema can declare a `created_at` field (set once, on creation).
- A schema can declare an `updated_at` field (set on creation and on every update).
- `updated_at` can be optional, in which case it is only updated when the field already has a value.

## Examples

- [Default names](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/timestamps_with_default_names.rs)
- [Custom names](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/timestamps_with_custom_names.rs)

## Try it in the browser

`username` is a lax field with a default. `created_at` and `updated_at` are populated automatically
from the timestamp resolver.

<RustPlayground demo="timestamps" />
