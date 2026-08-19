---
title: Getting Started
slug: /
---

# Getting Started

`ivo` for Rust expects you to define your data model with structs that implement `IvoInputStruct`
(required for input structs) and `IvoStruct`. This is done via their respective derive macros.

## Installation

```bash
cargo add ivo
```

## Defining structs

```rs
use chrono::{DateTime, Utc};
use ivo::{IvoInputStruct, IvoStruct};

#[derive(Clone, PartialEq, IvoInputStruct)]
struct UserInput {
    email: Option<String>,
    phone_number: Option<String>,
    username: String,
}

type Timestamp = DateTime<Utc>;

#[derive(Clone, PartialEq, IvoStruct)]
struct User {
    id: String,
    created_at: Timestamp,
    email: Option<String>,
    phone_number: Option<String>,
    updated_at: Option<Timestamp>,
    username: String,
    username_last_updated_at: Option<Timestamp>,
}
```

### `IvoStruct`

Deriving `IvoStruct` on `User` generates a `PartialUser` struct, plus helper methods:

```rs
impl IvoStruct for User {
    fn append_updates(&mut self, updates: &Self::Partial);
    fn clone_with_updates(&self, updates: &Self::Partial) -> Self;
}

impl From<User> for PartialUser {
    fn from(value: User) -> PartialUser;
}
```

`PartialUser` gets a constructor, `set_*`/`with_*` builder methods and `unset_*` methods per
field, plus `into_option()` and `is_empty()`:

```rs
struct PartialUser {
    id: Option<String>,
    created_at: Option<Timestamp>,
    email: Option<String>,
    phone_number: Option<Option<String>>,
    updated_at: Option<Option<Timestamp>>,
    username: Option<String>,
    username_last_updated_at: Option<Option<Timestamp>>,
}
```

The `#[ivo(...)]` attribute customizes generated partial structs and their fields, e.g. to derive
`Serialize`/`Deserialize` or forward `#[serde(...)]` attributes onto generated fields - see the
[Rust README](https://github.com/kamtoeddy/ivo/blob/main/rs/README.md#ivostruct) for the full
example.

### `IvoInputStruct`

Deriving `IvoInputStruct` on `UserInput` automatically implements `IvoStruct` and additionally
generates a `UserInputErrors` struct, used to return errors from
[post-validators](https://github.com/kamtoeddy/ivo#post-validator) and grouped required
resolvers.

## Defining a schema

Fields on a schema fall into one of six categories - see each for rules and a runnable example:

- [Constant fields](./definitions/constants.md)
- [Dependent fields](./definitions/dependents.md)
- [Lax fields](./definitions/lax.md)
- [Required fields](./definitions/required.md)
- [Timestamps](./definitions/timestamps.md)
- [Virtual fields](./definitions/virtuals.md)

## Schema options

- **Ignore (grouped)**: with
  [lax fields](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/lax/ignore.rs) or
  [virtual fields](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/virtuals/ignore.rs)
- **Ignore update (grouped)**: for the
  [entire entity](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/opions/mod.rs), with
  [lax fields](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/lax/ignore.rs) or
  [required fields](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/required/ignore.rs)
- **Required (grouped)**: with
  [lax fields](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/lax/mod.rs) or
  [virtual fields](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/virtuals/mod.rs)
- **Post-validate**: with
  [lax fields](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/lax/mod.rs),
  [required fields](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/required/mod.rs) or
  [virtual fields](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/fields/virtuals/mod.rs)
- **On success / on delete**: see [Life cycles](./life-cycles.md)

## Custom context options

Context options let you thread extra data (dependency injection, caching, i18n, ...) through an
operation. See the
[demo](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/domain.rs).

## Custom `ErrorSanitizer`

The default payload returned for unsuccessful operations has the signature:

```rs
type DefaultFieldErrorMetadata = ();

struct FieldError<Metadata: Clone = DefaultFieldErrorMetadata> {
    pub reason: String,
    pub metadata: Option<Metadata>,
}

type IvoErrorPayload<Metadata: Clone> = HashMap<String, FieldError<Metadata>>;
```

To customize this payload, provide an implementation of the `IvoErrorSanitizer` trait - see
[this example](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/extras/error_sanitizer.rs).

## API reference

The prose docs above cover the high-level concepts. For the exhaustive generated API reference
(types, functions, derive macros), see:

- **[docs.rs/crate/ivo](https://docs.rs/crate/ivo)** — hosted rustdoc for published crates.
  (Not yet available because `ivo` has not been published to crates.io.)
- **Local rustdoc** — run `cargo doc --no-deps --open` from the `rs/` directory to browse the same
  generated reference locally.
