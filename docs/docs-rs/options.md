---
title: Schema Options
sidebar_position: 3
---

# Schema Options

Grouped, cross-field options attach to an anonymous `const _: () = ();` item directly inside the
schema module -- not chained onto the `#[ivo_schema(...)]` call itself. Use them when a rule or
side effect involves more than one field, or when you want to react to the entity as a whole.
Multiple option attributes may be stacked on one const, or spread across several.

## `ignore`

Skip processing for a group of lax or virtual fields together, based on a shared condition.
Requires at least two fields, and applies to both `create` and `update`.

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod ignore_group_schema {
    struct Fields {
        #[lax(String::new())]
        pub email: String,

        #[lax(String::new())]
        pub phone: String,
    }

    #[ignore(["email", "phone"], |ctx, _opts| {
        ctx.input().email.as_deref() == Some("skip")
    })]
    const _: () = ();
}

fn main() {
    let created = ignore_group_schema::DataInputModel
        .create(
            ignore_group_schema::PartialDataInput {
                email: Some("skip".into()),
                phone: Some("123".into()),
            },
            (),
        )
        .unwrap();

    println!("{:?}", created.data); // DataInput { email: "", phone: "" } -- both ignored, defaults used
}
```

See [`lax_with_ignore.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore.rs)
and field-level `#[ignore]` on virtual fields (Virtual Fields, in the Fields section of the sidebar).

## `ignore_update`

Same idea as `ignore`, but evaluated during updates only. `#[ignore_update([...], handler)]`
requires at least two fields; to ignore the _entire entity_ on update, omit the array and use the
bare `#[ignore_update(handler)]` entity-level form instead.

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod ignore_update_group_schema {
    struct Fields {
        #[lax(0)]
        pub a: i32,

        #[lax(0)]
        pub b: i32,
    }

    #[ignore_update(["a", "b"], |ctx, _opts| {
        ctx.input().a == Some(42)
    })]
    const _: () = ();
}

fn main() {
    let data = ignore_update_group_schema::DataInput { a: 42, b: 1 };

    // both fields ignored -> nothing actually changes -> "nothing to update"
    let err = ignore_update_group_schema::DataInputModel
        .update(
            data,
            ignore_update_group_schema::PartialDataInput {
                a: Some(42),
                b: Some(2),
            },
            (),
        )
        .unwrap_err();

    assert!(err.errors.is_none()); // `None` errors means "nothing to update", not a validation failure
}
```

See [`lax_with_ignore_update.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore_update.rs).

## `required`

Enforces that at least one of the listed lax/virtual fields is provided. The handler runs only
when _none_ of the listed fields were provided, and returns `Option<{InputName}Errors>` -- `Some`
merges per-field errors into the payload, `None` means the requirement doesn't apply. Requires at
least two fields. Commonly used for "provide email or phone" style rules.

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod required_group_schema {
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

        let reason = "provide either an email or a phone number";
        let mut errors = DataInputErrors::new();
        errors.set_email(reason, None);
        errors.set_phone_number(reason, None);
        Some(errors)
    })]
    const _: () = ();
}

fn main() {
    let err = required_group_schema::DataInputModel
        .create(
            required_group_schema::PartialDataInput {
                email: None,
                phone_number: None,
            },
            (),
        )
        .unwrap_err();

    println!("{:?}", err.errors); // both "email" and "phone_number" carry the same reason
}
```

`DataInputErrors` is generated automatically alongside `DataInput`/`PartialDataInput`. See the same
pattern in
[`main_demo/src/domain.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/domain.rs).

## `post_validate`

Cross-field validation that runs after every individual field's `re_validate`. Can also return
updated values for the group's own fields (`pre_validate` runs first and can feed updated values
into the main `validate`). Requires at least two fields, from lax, required or virtual fields.

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod post_validate_schema {
    struct Fields {
        #[lax(String::new())]
        pub password: String,

        #[lax(String::new())]
        pub confirm_password: String,
    }

    #[post_validate(["password", "confirm_password"], validate = |ctx, _opts| {
        let input = ctx.input();

        if input.password != input.confirm_password {
            let mut errors = DataInputErrors::new();
            errors.set_confirm_password("passwords do not match", None);
            return Err(errors);
        }

        Ok(None)
    })]
    const _: () = ();
}

fn main() {
    let err = post_validate_schema::DataInputModel
        .create(
            post_validate_schema::PartialDataInput {
                password: Some("a".into()),
                confirm_password: Some("b".into()),
            },
            (),
        )
        .unwrap_err();

    println!("{:?}", err.errors); // {"confirm_password": "passwords do not match"}
}
```

See the cross-field validation in
[`main_demo/src/domain.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/domain.rs).

## `on_success`

Register a handler that runs after a successful `create` or `update`, via the returned handle's
`handle_success()`. The bare, arrayless form fires on every success; `#[on_success([...], handler)]`
requires at least one field and fires when at least one of the listed fields is part of the
success payload.

```rust
use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod hooks_schema {
    struct Fields {
        #[lax(0)]
        pub a: i32,

        #[lax(0)]
        pub b: i32,
    }

    #[on_success(|_ctx, _opts| {
        println!("[on_success]: entity created or updated");
    })]
    const _: () = ();

    #[on_success(["a", "b"], |_ctx, _opts| {
        println!("[on_success]: a and/or b changed");
    })]
    const _: () = ();
}

fn main() {
    let created = hooks_schema::DataInputModel
        .create(
            hooks_schema::PartialDataInput {
                a: Some(1),
                b: None,
            },
            (),
        )
        .unwrap();

    created.handle_success(); // prints both lines above
}
```

See the runnable
[`option_on_success.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/option_on_success.rs)
example for more detail, including dependent and virtual fields.

## `on_delete`

Register one or more handlers that run when a schema's generated `delete` method is invoked, in
addition to any per-field `#[on_delete]` handlers -- see the Life Cycles page (onDelete) in the
sidebar.

```rust
#[on_delete(|data, _opts| {
    println!("deleting entity with a = {}", data.a);
})]
const _: () = ();
```

## `timestamps`

The shared, **synchronous** resolver for `#[created_at]`/`#[updated_at]`/`#[optional_updated_at]`
fields -- see the Timestamps page in the Fields section of the sidebar for the full picture.

```rust
#[timestamps(|| chrono::Utc::now())]
const _: () = ();
```

Accepts either a zero-arg closure or a bare function path (`#[timestamps(chrono::Utc::now)]`).

## Custom context options

`ctx_options(YourType)` in the macro call threads a value of your own type (dependency injection,
caching, request-scoped data, ...) through every handler in a `create`/`update` call, wrapped in a
read/write lock so concurrent handlers can share and mutate it safely. Async handlers use
`opts.read().await`/`opts.write().await`; sync handlers use `opts.read_sync()`/`opts.write_sync()`.

```rust
use ivo::ivo_schema;

#[derive(Clone, Default)]
pub struct AppCtxOptions {
    pub calls: u32,
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    ctx_options(AppCtxOptions)
)]
mod ctx_options_schema {
    use super::AppCtxOptions;

    struct Fields {
        #[required]
        #[validate(|v: String, _, opts| {
            opts.write_sync().calls += 1;
            Ok(Some(v))
        })]
        pub name: String,
    }
}

fn main() {
    let created = ctx_options_schema::DataInputModel
        .create(
            ctx_options_schema::PartialDataInput {
                name: Some("jane".into()),
            },
            AppCtxOptions::default(),
        )
        .unwrap();

    println!(
        "name={:?} calls={}",
        created.data.name,
        created.ctx_options.read_sync().calls
    );
}
```

Pass `()` when a schema declares no `ctx_options(...)`, as in every other example on this page.
See [`main_demo/src/domain.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/domain.rs)
for a complete, realistic example (dependency lookups, uniqueness checks, and mutation across
several handlers in the same call).

## Custom error payloads with `IvoErrorSanitizer`

By default `ivo` returns errors as `HashMap<String, FieldError<()>>`. Change the shape of the
error payload by implementing `IvoErrorSanitizer` and passing it via `error_sanitizer(...)`:

```rust
use std::collections::HashMap;
use ivo::{ivo_schema, IvoErrorPayload, IvoErrorSanitizer};

struct MyErrorSanitizer;

impl IvoErrorSanitizer<()> for MyErrorSanitizer {
    type Metadata = Vec<String>;
    type Payload = HashMap<String, Vec<String>>;

    fn sanitize(payload: IvoErrorPayload<Self::Metadata>, _opts: &()) -> Self::Payload {
        payload
            .into_iter()
            .map(|(name, err)| {
                let mut messages = vec![err.reason];
                if let Some(meta) = err.metadata {
                    messages.extend(meta);
                }
                (name, messages)
            })
            .collect()
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    error_sanitizer(MyErrorSanitizer)
)]
mod sanitized_schema {
    use super::MyErrorSanitizer;

    struct Fields {
        #[required]
        #[validate(|v: String, _, _| {
            if v.len() < 3 {
                return Err(("too short".into(), Some(vec!["min length is 3".into()])));
            }
            Ok(None)
        })]
        pub username: String,
    }
}

fn main() {
    let err = sanitized_schema::DataInputModel
        .create(
            sanitized_schema::PartialDataInput {
                username: Some("ab".into()),
            },
            (),
        )
        .unwrap_err();

    println!("{:?}", err.errors); // {"username": ["too short", "min length is 3"]}
}
```

See the full example, including a custom `ctx_options` type, in
[`tests/extras/error_sanitizer.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/extras/error_sanitizer.rs).

## API reference

For the exhaustive list of grouped-option signatures and constraints, see:

- **[docs.rs/crate/ivo](https://docs.rs/crate/ivo)** — hosted rustdoc for the published crate.
- **Local rustdoc** — run `cargo doc --no-deps --open` from the `rs/` directory to browse the
  same generated reference locally.
