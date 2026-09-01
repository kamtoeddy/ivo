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
calling `handle_failure()` on the `IvoFailureHandle` returned from an unsuccessful `create` or
`update`.

```rust
let failed = DataInputModel.create(input, ()).unwrap_err();
println!("{:?}", failed.errors);
failed.handle_failure(); // runs any matching on_failure triggers; async if any handler is
```

## `onSuccess`

`#[on_success(|ctx, opts| { ... })]` -- registered on any individual field, or for
[a group of fields via the schema option](./options.md#on_success) (the bare, arrayless form fires
on every success regardless of which fields changed). Triggered by calling `handle_success()` on
the `IvoSuccessHandle` returned from a successful `create` or `update`.

```rust
let created = DataInputModel.create(input, ()).unwrap();
println!("{:?}", created.data);
created.handle_success(); // fires any on_success handler(s) whose fields actually changed
```

`handle_success`/`handle_failure` only exist on the returned handle at all when the schema
declares at least one matching `on_success`/`on_failure` handler *somewhere* -- calling one on a
schema with none is a compile error (the method isn't generated), not a silent no-op. Once it does
exist, it's still safe to call unconditionally: a grouped `on_success` whose fields didn't change
this call simply doesn't fire, without you having to check first.

## Custom context options

See [Getting Started - custom context options](./index.md#custom-context-options) for how to
thread extra data (dependency injection, caching, i18n, ...) through `create`/`update`/`delete`
operations and into these handlers.
