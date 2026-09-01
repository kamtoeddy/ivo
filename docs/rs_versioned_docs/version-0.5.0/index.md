---
title: Getting Started
slug: /
---

# Getting Started

These docs cover `ivo` for Rust **v0.5.0**.

Schemas are declared, not built imperatively: a single attribute macro, `#[ivo_schema(...)]`,
takes a module containing your field declarations and generates the input/output structs, their
partial/error counterparts, and a typed, schema-specific model with `create`/`update`/`delete`
methods.

## Installation

```bash
cargo add ivo
```

## Quickstart

```rust
use chrono::{DateTime, Utc};
use ivo::ivo_schema;

type Timestamp = DateTime<Utc>;

#[ivo_schema(
    input(PostInput, derive(Debug, Clone, PartialEq)),
    output(Post, derive(Debug, Clone, PartialEq))
)]
mod post_schema {
    use super::Timestamp;
    use chrono::Utc;

    struct Fields {
        #[constant(1)]
        pub id: i32,

        #[created_at]
        pub created_at: Timestamp,

        #[updated_at]
        pub updated_at: Timestamp,

        #[required]
        #[validate(|title: String, _, _| {
            if title.trim().len() < 3 {
                return Err(("title must be at least 3 characters long".into(), None));
            }
            Ok(Some(title.trim().to_string()))
        })]
        pub title: String,

        #[lax(String::new())]
        pub body: String,
    }

    #[timestamps(|| Utc::now())]
    const _: () = ();
}

use post_schema::{PartialPostInput, PostModel};

fn main() {
    let created = PostModel
        .create(
            PartialPostInput {
                title: Some("Hello, ivo!".into()),
                body: Some("My first post.".into()),
            },
            (), // ctx_options -- `()` when the schema declares none
        )
        .unwrap();

    println!("{:#?}", created.data); // -> Post { id, created_at, updated_at, title, body }

    let updated = PostModel
        .update(
            created.data,
            PartialPostInput {
                title: None,
                body: Some("Edited.".into()),
            },
            (),
        )
        .unwrap();

    println!("{:#?}", updated.data); // -> PartialPost { body: Some("Edited."), .. rest None }
}
```

- `input(...)` names the generated input struct and is always required; `output(...)` names the
  output struct and is required only when the schema has input-only fields (`#[ivo_virtual]`) or
  output-only fields (`#[constant]`, `#[depends_on(...)]`, timestamps). A schema with only
  `#[required]`/`#[lax]` fields can omit `output(...)` entirely and use one struct for both.
- `derive(...)` adds derives to the generated struct; `derive_partial(...)` adds them to its
  partial counterpart (e.g. to derive `Serialize`/`Deserialize` for wire transport).
- The macro generates a `{OutputName}Model` unit value (or `{InputName}Model` for a single-struct
  schema) -- `post_schema::PostModel.create(...)` works directly, no `::new()` needed.
- `create`/`update`/`delete` are `async` only if at least one handler they invoke is async --
  otherwise the generated method (and any `handle_success`/`handle_failure` it returns) is plain
  sync, with no runtime dependency forced on you.

## Defining a schema

Fields on a schema fall into one of six categories - see each for the rules and a runnable
example:

- [Constant fields](./definitions/constants.md)
- [Dependent fields](./definitions/dependents.md)
- [Lax fields](./definitions/lax.md)
- [Required fields](./definitions/required.md)
- [Timestamps](./definitions/timestamps.md)
- [Virtual fields](./definitions/virtuals.md)

See [Validators](./validators.md) for how `#[validate]`/`#[re_validate]` work, and
[Life Cycles](./life-cycles.md) for `#[on_success]`/`#[on_failure]`/`#[on_delete]`.

## Schema options

Grouped, cross-field behavior attaches to an anonymous `const _: () = ();` item inside the schema
module, not chained onto the macro call -- see [Schema Options](./options.md) for `ignore`,
`ignore_update`, `required`, `post_validate`, `on_success`, `on_delete` and `timestamps`, each with
a runnable example.

## Custom context options

`ctx_options(YourType)` threads a value of your own type (dependency injection, caching,
request-scoped data, ...) through every handler in a `create`/`update` call. See
[Schema Options - Custom context options](./options.md#custom-context-options), or the full demo
in
[`examples/main_demo`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/domain.rs).

## Custom `ErrorSanitizer`

The default payload returned for unsuccessful operations has the signature:

```rust
type DefaultFieldErrorMetadata = ();

struct FieldError<Metadata: Clone = DefaultFieldErrorMetadata> {
    pub reason: String,
    pub metadata: Option<Metadata>,
}

type IvoErrorPayload<Metadata> = HashMap<String, FieldError<Metadata>>;
```

To customize this payload, provide an implementation of the `IvoErrorSanitizer` trait via
`error_sanitizer(YourSanitizer)` -- see
[Schema Options - Custom error payloads](./options.md#custom-error-payloads-with-ivoerrorsanitizer).

## API reference

The prose docs above cover the high-level concepts. For the exhaustive generated API reference
(types, functions, derive macros), see:

- **[docs.rs/crate/ivo](https://docs.rs/crate/ivo)** — hosted rustdoc for the published crate.
- **[crates.io/crates/ivo](https://crates.io/crates/ivo)** — crate registry page (versions, dependencies,
  README).
- **Local rustdoc** — run `cargo doc --no-deps --open` from the `rs/` directory to browse the
  same generated reference locally.
