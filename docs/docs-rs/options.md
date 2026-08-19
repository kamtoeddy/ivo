---
title: Schema Options
sidebar_position: 3
---

# Schema Options

Schema-wide options are configured in the second closure of `IvoModel::new(..., |o| o...)`. Use them
when a rule or side effect involves more than one field, or when you want to react to the entity as
a whole.

## `on_success`

Register a handler that runs after a successful `create` or `update`. An empty fields array means
"trigger on every success"; otherwise it fires when at least one of the listed fields is part of
the success payload.

```rust
use std::{future::ready, sync::LazyLock};
use ivo::{lax_field, IvoInputStruct, IvoModel, IvoStruct};

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
struct DataInput {
    pub a: i32,
    pub b: i32,
}

#[derive(Clone, Debug, PartialEq, IvoStruct)]
struct Data {
    pub a: i32,
    pub b: i32,
}

static MODEL: LazyLock<IvoModel<DataInput, Data>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(lax_field("a").default(0))
                .field(lax_field("b").default(0))
        },
        |o| {
            o.on_success([], |b| {
                b.handle(|_, _| {
                    println!("entity created or updated");
                    ready(())
                })
            })
            .on_success(["a", "b"], |b| {
                b.handle(|_, _| {
                    println!("a and/or b changed");
                    ready(())
                })
            })
        },
    )
});
```

See the runnable [`option_on_success.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/option_on_success.rs)
example for more detail, including dependent and virtual fields.

## `on_delete`

Register one or more handlers that run when `model.delete(&entity, None).await` is invoked. These
are triggered for the whole entity, in addition to any per-field `on_delete` handlers.

```rust
use std::{future::ready, sync::LazyLock};
use ivo::{lax_field, IvoInputStruct, IvoModel, IvoStruct};

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
struct DataInput {
    pub a: i32,
}

#[derive(Clone, Debug, PartialEq, IvoStruct)]
struct Data {
    pub a: i32,
}

static MODEL: LazyLock<IvoModel<DataInput, Data>> = LazyLock::new(|| {
    IvoModel::new(
        |f| f.field(lax_field("a").default(0)),
        |o| {
            o.on_delete(|data, _| {
                println!("deleting entity with a = {}", data.a);
                ready(())
            })
        },
    )
});
```

See the `should_properly_trigger_on_delete_handlers` tests in
[`rs/tests/options/mod.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/options/mod.rs).

## `ignore`

A grouped ignore rule lets you skip processing of multiple lax or virtual fields at once based on a
shared condition. The resolver receives the input context and returns `true` to ignore the group.
Requires at least two fields.

```rust
use std::{future::ready, sync::LazyLock};
use ivo::{lax_field, IvoContext, IvoInputStruct, IvoModel, IvoStruct};

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
struct DataInput {
    pub email: String,
    pub phone: String,
}

#[derive(Clone, Debug, PartialEq, IvoStruct)]
struct Data {
    pub email: String,
    pub phone: String,
}

static MODEL: LazyLock<IvoModel<DataInput, Data>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(lax_field("email").default(String::new()))
                .field(lax_field("phone").default(String::new()))
        },
        |o| {
            o.ignore(["email", "phone"], |ctx: IvoContext<DataInput, Data>, _| {
                // ignore both when a special flag is present
                ready(ctx.input().email == "skip")
            })
        },
    )
});
```

See [`lax_with_ignore.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore.rs)
and the grouped ignore tests in
[`rs/tests/options/ignore.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/options/ignore.rs).

## `ignore_update`

Same idea as `ignore`, but only evaluated during updates. An empty fields array applies the rule to
the whole entity. With two or more fields, it applies to the group.

```rust
use std::{future::ready, sync::LazyLock};
use ivo::{lax_field, IvoInputStruct, IvoModel, IvoStruct};

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
struct DataInput {
    pub a: i32,
    pub b: i32,
}

#[derive(Clone, Debug, PartialEq, IvoStruct)]
struct Data {
    pub a: i32,
    pub b: i32,
}

static MODEL: LazyLock<IvoModel<DataInput, Data>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(lax_field("a").default(0))
                .field(lax_field("b").default(0))
        },
        |o| {
            o.ignore_update(["a", "b"], |input: PartialDataInput, _, _| {
                ready(input.a == Some(42))
            })
        },
    )
});
```

See [`lax_with_ignore_update.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/lax_with_ignore_update.rs)
and the grouped tests in
[`rs/tests/options/ignore_update.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/options/ignore_update.rs).

## `required`

A grouped required rule enforces that at least one of the listed lax or virtual fields is provided.
The resolver returns `Some(errors)` when the requirement is not met. It is commonly used for
"provide email or phone" style rules.

```rust
use std::{future::ready, sync::LazyLock};
use ivo::{lax_field, IvoContext, IvoInputStruct, IvoModel, IvoStruct};

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
struct UserInput {
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Clone, Debug, PartialEq, IvoStruct)]
struct User {
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

static USER_MODEL: LazyLock<IvoModel<UserInput, User>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(lax_field("email").default(None::<String>))
                .field(lax_field("phone_number").default(None::<String>))
        },
        |o| {
            o.required(["email", "phone_number"], |ctx: IvoContext<UserInput, User>, _| {
                if ctx.is_update() || ctx.input().email.is_some() || ctx.input().phone_number.is_some() {
                    return ready(None);
                }

                let reason = "provide either an email or a phone number";

                ready(Some(
                    UserInputErrors::new()
                        .with_email(reason, None)
                        .with_phone_number(reason, None),
                ))
            })
        },
    )
});
```

`UserInputErrors` is generated by deriving `IvoInputStruct`. See the same pattern in
[`main_demo/src/domain.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/domain.rs)
and the grouped tests in
[`rs/tests/options/required.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/options/required.rs).

## `post_validate`

Cross-field validation runs after individual field validators. Use it to validate combinations of
lax, required or virtual fields. Requires at least two fields.

```rust
use std::{future::ready, sync::LazyLock};
use ivo::{lax_field, IvoContext, IvoInputStruct, IvoModel, IvoStruct};

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
struct UserInput {
    pub password: String,
    pub confirm_password: String,
}

#[derive(Clone, Debug, PartialEq, IvoStruct)]
struct User {
    pub password: String,
    pub confirm_password: String,
}

static USER_MODEL: LazyLock<IvoModel<UserInput, User>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(lax_field("password").default(String::new()))
                .field(lax_field("confirm_password").default(String::new()))
        },
        |o| {
            o.post_validate(["password", "confirm_password"], |b| {
                b.validate(|ctx: IvoContext<UserInput, User>, _| {
                    let input = ctx.input();

                    if input.password != input.confirm_password {
                        let mut errors = UserInputErrors::new();
                        errors.set_confirm_password("passwords do not match", None);
                        return ready(Err(errors));
                    }

                    ready(Ok(None))
                })
            })
        },
    )
});
```

See the cross-field validation in
[`main_demo/src/domain.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/domain.rs)
and the grouped tests in
[`rs/tests/options/post_validate.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/options/post_validate.rs).

## Custom context options

`IvoModel` accepts a `CtxOptions` type parameter for threading custom data through create, update
and delete operations (dependency injection, request context, caching, etc.).

```rust
use ivo::{IvoContext, IvoModel, IvoRwCtxOptions};

#[derive(Clone, Default)]
struct MyCtxOptions {
    db: Database,
}

type Ctx = IvoContext<Input, Output>;
type RwCtxOptions = IvoRwCtxOptions<MyCtxOptions>;

static MODEL: LazyLock<IvoModel<Input, Output, MyCtxOptions>> = LazyLock::new(|| {
    IvoModel::new(
        |f| { /* field definitions */ },
        |o| {
            o.post_validate(["a", "b"], |b| {
                b.validate(|ctx: Ctx, options: RwCtxOptions| async move {
                    let db = &options.read().await.db;
                    // use db ...
                    Ok(None)
                })
            })
        },
    )
});
```

See [`main_demo/src/domain.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/examples/main_demo/src/domain.rs)
for a complete working example.

## Custom error payloads with `IvoErrorSanitizer`

By default `ivo` returns errors as a `HashMap<String, FieldError<()>>`. You can change the shape of
the error payload by implementing `IvoErrorSanitizer`:

```rust
use std::{collections::HashMap, future::ready};
use ivo::{IvoErrorPayload, IvoErrorSanitizer};

#[derive(Clone)]
struct MyCtxOptions;

struct MyErrorSanitizer;

impl IvoErrorSanitizer<MyCtxOptions> for MyErrorSanitizer {
    type Metadata = Vec<String>;
    type Payload = HashMap<String, Vec<String>>;

    fn sanitize(payload: IvoErrorPayload<Self::Metadata>, _o: &MyCtxOptions) -> Self::Payload {
        payload
            .into_iter()
            .map(|(name, err)| {
                let mut errors = vec![err.reason];
                if let Some(meta) = err.metadata {
                    errors.extend(meta);
                }
                (name, errors)
            })
            .collect()
    }
}
```

The sanitizer becomes the fifth type parameter of `IvoModel`:

```rust
IvoModel<Input, Output, MyCtxOptions, Timestamp, MyErrorSanitizer>
```

See the full example in
[`rs/tests/extras/error_sanitizer.rs`](https://github.com/kamtoeddy/ivo/blob/main/rs/tests/extras/error_sanitizer.rs),
and the trait documentation on [docs.rs/crate/ivo](https://docs.rs/crate/ivo).

## API reference

For the exhaustive list of option builders and their signatures, see:

- **[docs.rs/crate/ivo](https://docs.rs/crate/ivo)** — hosted rustdoc for the published crate.
- **Local rustdoc** — run `cargo doc --no-deps --open` from the `rs/` directory to browse the same
  generated reference locally.
