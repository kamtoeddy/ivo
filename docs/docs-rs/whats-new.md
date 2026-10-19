---
title: What's new in v0.7.0
---

# What's new in v0.7.0

v0.7.0 changes how generated structs get their attributes, what handlers receive, and how errors
are stored, and it makes `create`/`update` much faster. Most of the breaking changes are caught at
compile time, and the macro's error messages say what to write instead.

## Breaking changes

### Struct and field attributes: `ivo_attrs(...)`

`derive(...)`/`derive_partial(...)` on the generated structs are replaced by `ivo_attrs(...)` and
`ivo_partial_attrs(...)`, which accept **any** attribute, not only derives:

```rust
// v0.6
#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq)),
    output(User, derive(Debug, Clone, PartialEq), derive_partial(Debug))
)]

// v0.7
#[ivo_schema(
    input(UserInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(
        User,
        ivo_attrs(derive(Debug, Clone, PartialEq, serde::Serialize)),
        ivo_partial_attrs(derive(serde::Serialize))
    )
)]
```

Field-level passthrough attributes were renamed the same way, and two new ones attach to the field
in every struct (or every partial struct) it's part of:

| v0.6                      | v0.7                                                      |
| ------------------------- | --------------------------------------------------------- |
| `#[input(...)]`           | `#[ivo_input_attrs(...)]`                                 |
| `#[output(...)]`          | `#[ivo_output_attrs(...)]`                                |
| `#[partial(...)]`         | `#[ivo_partial_attrs(...)]`                               |
| `#[input_partial(...)]`   | `#[ivo_input_partial_attrs(...)]`                         |
| `#[output_partial(...)]`  | `#[ivo_output_partial_attrs(...)]`                        |
| --                        | `#[ivo_attrs(...)]` (new: every non-partial struct)       |

```rust
#[required]
#[ivo_attrs(serde(rename = "userName"))]
#[validate(|v: &String, _, _| Ok(None))]
pub username: String,
```

See [Getting Started](./index.md#quickstart) for the full rules.

### `#[post_validate]`: no more `validate =`

The validator is passed directly, as the second argument:

```rust
// v0.6
#[post_validate(["password", "confirm_password"], validate = |ctx, _opts| { /* ... */ })]

// v0.7
#[post_validate(["password", "confirm_password"], |ctx, _opts| {
    let input = ctx.input();
    if input.password != input.confirm_password {
        let mut errors = UserInputErrors::new();
        errors.set_confirm_password("passwords do not match", None);
        return Err(errors);
    }
    Ok(None)
})]
const _: () = ();
```

`pre_validate = ...` is unchanged.

### Validators receive the value by reference

Validators and re-validators now get `&T` instead of `T`. Return `Ok(None)` to keep the value as it
is, or `Ok(Some(...))` with an owned value to replace it:

```rust
// v0.6
#[validate(|v: String, _, _| Ok(Some(v.trim().to_string())))]

// v0.7
#[validate(|v: &String, _, _| Ok(Some(v.trim().to_string())))]
```

### Handlers receive `&IvoContext`

Resolvers, validators, post-validators and `on_success`/`on_failure` handlers now receive the
context by reference. Closures with inferred parameters (`|ctx, _opts| ...`) need no change; clone
the context if you need to move it elsewhere:

```rust
#[on_success(|ctx, _opts| {
    let ctx = ctx.clone();
    std::thread::spawn(move || println!("created {:?}", ctx.values().id));
})]
const _: () = ();
```

Functions used as handlers must take `&IvoContext<...>` instead of `IvoContext<...>`.

### `ctx.values()` returns the partial output

`ctx.values()` now returns the schema's **partial** output struct, so every field is an `Option`.
During `create`, it fills in as the pipeline runs (a field not reached yet is `None`); during an
update and in `on_success` handlers, every field is `Some`:

```rust
// v0.6
#[resolve(|ctx, _opts| ctx.values().max_attempts * 2)]

// v0.7
#[resolve(|ctx, _opts| ctx.values().max_attempts.unwrap_or_default() * 2)]
```

See [what `ctx.values()` holds](./execution-pipeline.md#what-ctxvalues-holds) at each step.

### Error reasons are `Cow<'static, str>`

A field's error reason is now an `IvoErrorReason` (`Cow<'static, str>`): a string literal is used
as-is, without allocating, and a message built at runtime is owned. Validators build one with
`.into()`:

```rust
#[validate(|v: &String, _, _| {
    let name = v.trim();
    if name.len() < 3 {
        return Err(("too short".into(), None));
    }
    if name.len() > 20 {
        return Err((format!("at most 20 characters, got {}", name.len()).into(), None));
    }
    Ok(Some(name.to_string()))
})]
```

The error payload is now keyed by the schema's field names, as `&'static str`:

```rust
// v0.6: HashMap<String, FieldError<M>>, with `reason: String`
// v0.7: HashMap<&'static str, FieldError<M>>, with `reason: IvoErrorReason`
for (field, error) in &errors {
    let reason: &str = &error.reason;
    println!("{field}: {reason}");
}

let owned: String = errors.get("username").unwrap().reason.clone().into_owned();
```

`IvoErrorReason` is exported from the crate root. A custom error sanitizer receives this payload
(see [Options](./options.md)).

## New features

### Zero-argument functions as constants and defaults

`#[constant(...)]`, `#[lax(...)]` and a dependent field's `#[default(...)]` accept the path of a
zero-argument function. It's called to produce the value, so `|| make_value()` is no longer needed.
A path to a `const` or `static` is still used as the value itself:

```rust
fn new_id() -> u64 {
    42
}

fn default_role() -> String {
    "member".to_string()
}

const MAX_ATTEMPTS: u32 = 3;

// inside the schema module (with `use super::{default_role, new_id, MAX_ATTEMPTS};`)
struct Fields {
    #[constant(new_id)] // called: 42
    pub id: u64,

    #[lax(default_role)] // called: "member"
    pub role: String,

    #[lax(MAX_ATTEMPTS)] // used as the value: 3
    pub max_attempts: u32,
}
```

## Fixes

- `validate_email` (`validators` feature) only accepts values that are an email address as a
  whole, not values that merely contain one.
- `validate_credit_card` (`validators` feature) accepts valid card numbers whose check digit is
  `0`.

## Performance

`create` and `update` allocate and clone much less: contexts share the raw input and previous
values instead of copying them, handlers and validators borrow instead of taking ownership, error
reasons only allocate when built at runtime, and only the parts of the context that changed are
rebuilt between phases. Compared with the start of the v0.7 performance work, on an Apple M4:

| Benchmark                  | Before   | After     | Change |
| -------------------------- | -------- | --------- | ------ |
| minimal create             | 56.91 ns | 26.97 ns  | -53%   |
| user create                | 523.96 ns | 215.06 ns | -59%  |
| dependent chain length 10  | 1.46 µs  | 213.25 ns | -85%   |
| create 10 readonly lax fields | 1.61 µs | 644.49 ns | -60% |
| no-op update               | 205.81 ns | 55.66 ns | -73%   |
| single field update        | 576.87 ns | 199.61 ns | -65%  |

The full results are in
[`rs/PERF_RESULTS.md`](https://github.com/kamtoeddy/ivo/blob/main/rs/PERF_RESULTS.md).
