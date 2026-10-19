---
title: Life Cycles
---

# Life Cycles

`ivo` lets you react to changes on a domain entity or its individual fields. The concepts below
are shared across both implementations - see the
[root README](https://github.com/kamtoeddy/ivo#lifecycle-events) for the full language-agnostic
definitions. This page covers how to wire them up in Rust.

## `onDelete`

`#[on_delete(|data, opts| { ... })]` -- triggered directly by calling a schema's generated `delete`
method. Subscribe per output field, or entity-wide via the
[`on_delete` schema option](./options.md#on_delete). `delete` is only generated when the schema
declares at least one `on_delete` handler (field-level or schema-level), and is `async` only if
any of them are.

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod delete_schema {
    struct Fields {
        #[required]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        #[on_delete(|data, _opts| {
            println!("[username]: on_delete: {}", data.username);
        })]
        pub username: String,
    }
}

fn main() {
    let data = delete_schema::DataInput {
        username: "jane".into(),
    };

    delete_schema::DataInputModel.delete(&data, ());
}
```

## `onFailure`

`#[on_failure(|ctx, opts| { ... })]` -- registered on a field that has a validator, triggered by
calling the handle returned as the third element of the `Err` tuple from an unsuccessful `create`
or `update`.

## `onSuccess`

`#[on_success(|ctx, opts| { ... })]` -- registered on any individual field, or for
[a group of fields via the schema option](./options.md#on_success) (the bare, arrayless form fires
on every success regardless of which fields changed). Triggered by calling the handle returned as
the third element of the `Ok` tuple from a successful `create` or `update`.

## Triggering handlers

`create`/`update` return `(data, ctx_options)` when the schema has no matching `on_success`/
`on_failure` handler anywhere, and `(data, ctx_options, handle)` when it does -- calling `handle`
runs every matching trigger for that call. `handle` is a plain `FnOnce()` if every captured handler
is sync, or `FnOnce() -> impl Future<Output = ()>` (call it, then `.await` the result) if any is
async -- resolved once per schema at compile time, not behind a runtime check.

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod notify_schema {
    struct Fields {
        #[required]
        #[validate(|v: String, _, _| {
            if v.is_empty() {
                return Err(("username must not be empty".into(), None));
            }
            Ok(Some(v))
        })]
        #[on_success(|ctx, _| {
            println!("[username]: on_success: {}", ctx.values().username);
        })]
        #[on_failure(|ctx, _| {
            println!("[username]: on_failure: {:?}", ctx.input().username);
        })]
        pub username: String,
    }
}

fn main() {
    let (created, _ctx_options, handle_success) = notify_schema::DataInputModel
        .create(notify_schema::DataInput { username: "jane".into() }, ())
        .ok()
        .unwrap();
    println!("{:?}", created);
    handle_success(); // runs the matching on_success trigger

    let (errors, _ctx_options, handle_failure) = notify_schema::DataInputModel
        .create(notify_schema::DataInput { username: "".into() }, ())
        .err()
        .unwrap();
    println!("{:?}", errors);
    handle_failure(); // runs the matching on_failure trigger
}
```

`Result::unwrap()`/`unwrap_err()` require `Debug` on the *other* arm of the `Result`, which the
trigger closure can't provide -- use `.ok().unwrap()` / `.err().unwrap()` instead
(`Option::unwrap()` has no such bound). When a schema has neither `on_success` nor `on_failure`
handlers, the tuple has no trigger element and plain `.unwrap()`/`.unwrap_err()` work fine, as in
the [Getting Started](./index.md#quickstart) example.

## Custom context options

See [Getting Started - custom context options](./index.md#custom-context-options) for how to
thread extra data (dependency injection, caching, i18n, ...) through `create`/`update`/`delete`
operations and into these handlers.
