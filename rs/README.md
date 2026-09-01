<p align="center">
  <img src="https://raw.githubusercontent.com/kamtoeddy/ivo/main/docs/static/img/logo.png" alt="ivo logo" width="120" />
</p>

# Rust Implementation

This is the documentation of the Rust implementation of ivo.

Schemas are declared, not built imperatively: a single attribute macro, `#[ivo_schema(...)]`,
takes a module containing your field declarations and generates the input/output structs, their
partial/error counterparts, and a typed, schema-specific model with `create`/`update`/`delete`
methods.

# Installation

```bash
$ cargo add ivo
```

# Quickstart

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
        #[validate(|title, _, _| {
            let validated = title.trim();
            if validated.len() < 3 {
                return Err(("title must be at least 3 characters long".into(), None));
            }

            Ok(Some(title.trim().to_string()))
        })]
        pub title: String,

        #[lax(String::new())]
        pub body: String,
    }

    #[timestamps(Utc::now)]
    const _: () = ();
}

use post_schema::{PartialPostInput, PostModel};

fn main() {
    let (post, _ctx_options) = PostModel
        .create(
            PartialPostInput {
                title: Some("Hello, ivo!".into()),
                body: Some("My first post.".into()),
            },
            (), // ctx_options -- `()` when the schema declares none
        )
        .unwrap();

    println!("{:#?}", post); // -> Post { id, created_at, updated_at, title, body }

    let (updated, _ctx_options) = PostModel
        .update(
            post,
            PartialPostInput {
                title: None,
                body: Some("Edited.".into()),
            },
            (),
        )
        .unwrap();

    println!("{:#?}", updated); // -> PartialPost { body: Some("Edited."), .. rest None }
}
```

`create`/`update`/`delete` are `async` only if at least one handler they invoke is async --
otherwise the generated method is plain sync, with no runtime dependency forced on you.

`create`/`update` return `(data, ctx_options)` when the schema has no `on_success`/`on_failure`
handlers on that path, as above. If it does, a third element is appended -- a trigger you call to
run them: `(data, ctx_options, handle)`, where `handle` is `FnOnce()` if every captured handler is
sync, or `FnOnce() -> impl Future<Output = ()>` (call it, then `.await` the result) if any is
async. `Result::unwrap()`/`unwrap_err()` require `Debug` on the *other* arm, which a trigger
closure can't provide -- use `.ok().unwrap()` / `.err().unwrap()` instead (`Option::unwrap()` has
no such bound), matching every example under [`examples/`](./examples).

# Field types

Every field on `struct Fields { ... }` is declared with exactly one field-type attribute:

| Attribute                                    | Meaning                                                                                                                                    |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ | --- | ---------------- |
| `#[required]`                                | Must be provided at creation; optional (and immutable unless allowed) thereafter.                                                          |
| `#[lax(default_or_resolver)]`                | Optional; falls back to a static or resolved default when missing.                                                                         |
| `#[constant(value_or_resolver)]`             | Output-only; computed once at creation, never accepted from input.                                                                         |
| `#[depends_on("parent", ...)]`               | Output-only; recomputed via `#[resolve(...)]` whenever a listed parent field changes.                                                      |
| `#[ivo_virtual]` / `#[ivo_virtual("alias")]` | Input-only; validated/sanitized like a real field but never stored on the output directly -- typically feeds a `#[depends_on(...)]` field. |
| `#[created_at]` / `#[updated_at]`            | Output-only timestamp fields, populated by a schema-level `#[timestamps(                                                                   |     | ...)]` resolver. |

Each accepts its own set of behavior attributes -- `#[validate]`, `#[re_validate]`, `#[sanitize]`,
`#[ignore]` / `#[ignore_init]` / `#[ignore_update]`, `#[readonly]`, `#[required(...)]` (conditional,
distinct from the `#[required]` field type), `#[on_success]` / `#[on_failure]` / `#[on_delete]`,
and more. See [`GOAL.md`](./GOAL.md) for the full attribute matrix and every allowed combination,
or the runnable examples under [`examples/`](./examples) and [`tests/fields/`](./tests/fields).

# Schema options

Attached to an anonymous `const _: () = ();` item inside the schema module, not chained onto the
macro call:

```rust
// illustrative -- not a continuation of the `post_schema` example above
#[ivo_schema(input(ContactInput, derive(Debug, Clone, PartialEq)))]
mod contact_schema {
    struct Fields {
        #[lax(None)]
        pub email: Option<String>,

        #[lax(None)]
        pub phone_number: Option<String>,
    }

    #[required(["email", "phone_number"], |ctx, _opts| {
        if ctx.input().email.is_some() || ctx.input().phone_number.is_some() {
            return None;
        }
        let mut errors = ContactInputErrors::new();
        errors.set_email("either \"email\" or \"phone_number\" is required", None);
        Some(errors)
    })]
    const _: () = ();
}
```

- **`#[ignore([...], handler)]`** -- skip a group of `#[lax]`/`#[ivo_virtual]` fields together.
- **`#[ignore_update([...], handler)]`** -- same, but update only (or entity-wide with the bare,
  arrayless form).
- **`#[required([...], handler)]`** -- cross-field "at least one of these" requirement checks.
- **`#[post_validate([...], validate = ..., pre_validate = ...)]`** -- cross-field validation that
  can also return updated values for the group's own fields.
- **`#[on_success(...)]`** / **`#[on_delete(...)]`** -- grouped or entity-wide lifecycle triggers.
- **`#[timestamps(|| ...)]`** -- the shared, synchronous resolver for `#[created_at]`/`#[updated_at]`.

See [`GOAL.md` §19](./GOAL.md) for the full reference, including minimum field counts and which
field types each option accepts.

# Context options and error sanitizing

`ctx_options(YourType)` in the macro call threads a value of your own type (dependency injection,
caching, request-scoped data, ...) through every handler in a `create`/`update` call, wrapped in a
read/write lock so concurrent handlers can share and mutate it safely -- see
[`examples/main_demo`](./examples/main_demo) for a full demo. Pass `()` when a schema declares
none, as in the quickstart above.

`error_sanitizer(YourSanitizer)` lets you customize the shape of the error payload returned on
failure by implementing `IvoErrorSanitizer` -- see
[`tests/extras/error_sanitizer.rs`](./tests/extras/error_sanitizer.rs).

# Docs

[Read the docs](https://ivo.kamtoeddy.com/docs/rs)
