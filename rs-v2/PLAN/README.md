# Ivo Rust v2 Implementation Plan

This directory contains the implementation plans for the macro-generated schema API introduced in `rs-v2`.

## Overview

`rs-v2` replaces the runtime, builder-based schema construction of `rs` with a compile-time, macro-generated schema model. The goals are:

1. **Eliminate `Box<dyn CloneableAny>` from the hot path** by moving values as concrete types.
2. **Generate schema-specific validation/resolution code** instead of runtime string dispatch.
3. **Generate input/output structs from the schema** so users do not have to keep two struct definitions in sync.
4. **Provide a declarative, attribute-driven schema syntax** that is easier to read and write.
5. **Preserve all existing semantics** of the current builder API.

## User preferences

See [`/rs-v2/PREFERENCES.md`](../PREFERENCES.md) for the complete list of API design preferences.

## Plan structure

### Cross-cutting concerns

- [`struct_generation.md`](struct_generation.md) — how `#[ivo_schema]` generates input/output structs from the schema.

### Field definitions

Each field type has its own plan under `PLAN/fields/`:

- [`required.md`](fields/required.md) — `#[required]` fields with validators, re-validators, required errors, and lifecycle handlers.
- [`lax.md`](fields/lax.md) — `#[lax]` fields with defaults, ignore flags, and lifecycle handlers.
- [`constant.md`](fields/constant.md) — `#[constant]` output-only fields with static or resolved values.
- [`dependent.md`](fields/dependent.md) — `#[dependent]` fields with dependency graph resolution and fallback defaults.
- [`virtual.md`](fields/virtual.md) — `#[virtual]` input-only fields with aliases, sanitizers, and validators.

### Schema options

Each grouped option has its own plan under `PLAN/options/`:

- [`ignore.md`](options/ignore.md) — `#[ignore]` grouped ignore rules.
- [`ignore_update.md`](options/ignore_update.md) — `#[ignore_update]` update-time ignore rules.
- [`required.md`](options/required.md) — `#[required]` cross-field required constraints.
- [`post_validate.md`](options/post_validate.md) — `#[post_validate]` pre/post validation groups.
- [`on_success.md`](options/on_success.md) — `#[on_success]` grouped success handlers.
- [`on_delete.md`](options/on_delete.md) — `#[on_delete]` delete handlers.
- [`timestamps.md`](options/timestamps.md) — `#[timestamps]` timestamp configuration.

## Common syntax conventions

### Schema module shape

```rust
#[ivo_schema(input = User)]
mod user_schema {
    #[fields]
    mod fields {
        #[required]
        #[validate(|name, _ctx, _opts| async move { Ok(Some(name)) })]
        name: String,

        #[lax("user")]
        role: String,
    }

    #[options]
    mod options {
        #[ignore(["secret"], |_ctx, _opts| async move { true })]
        const _: () = ();

        #[required(["name"], |_ctx, _opts| async move { None })]
        const _: () = ();
    }
}
```

Because this schema contains only `#[required]` and `#[lax]` fields and no timestamps, the macro uses a single struct for both input and output: it generates `User`, derives both `IvoInputStruct` and `IvoStruct` on it, and produces `PartialUser`, `UserErrors`, and `UserSchemaModel`.

If the schema contains `#[constant]`, `#[dependent]`, `#[virtual]`, or `#[timestamps]`, both `input = ...` and `output = ...` are required, and the macro generates two separate structs. See [`struct_generation.md`](struct_generation.md) for details.

### Default/value forms

Both `#[default(...)]` (for lax and dependent fields) and `#[value(...)]` (for constants) accept:

| Form               | Example                                        |
| ------------------ | ---------------------------------------------- |
| Static             | `#[default("user")]`                           |
| Sync no-context    | `#[default(\|\| default_timezone())]`          |
| Async no-context   | `#[default(async \|\| fetch_locale().await)]`  |
| Async with context | `#[default(\|ctx, opts\| async move { ... })]` |

### Repeated options

Options that can be called multiple times in the builder API are expressed by multiple attributes inside `#[options]`:

```rust
#[options]
mod options {
    #[ignore(["secret"], |_ctx, _opts| async move { true })]
    const _: () = ();

    #[ignore(["hidden"], |_ctx, _opts| async move { true })]
    const _: () = ();
}
```

## Implementation status legend

Each plan file ends with a progress checklist using the following states:

- `[ ]` Not started
- `[~]` In progress
- `[x]` Done

## Next steps

1. Finalize the proc-macro crate structure (extend `ivo-derive` or create a new `ivo-schema-derive` crate).
2. Implement the schema parser and token-tree transformations.
3. Generate typed field configs for one field type (start with `#[required]`).
4. Generate the schema model skeleton with `create`/`update`/`delete` methods.
5. Add one grouped option at a time, validating field names at macro expansion time.
6. Port the existing `rs` test suite to the new API to ensure parity.
