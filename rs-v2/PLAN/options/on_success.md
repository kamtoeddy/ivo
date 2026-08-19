# Plan: `#[on_success]` option

This option does not change whether the schema uses single-struct or dual-struct mode; see [`struct_generation.md`](../struct_generation.md) for those rules.

## New syntax

```rust
#[options]
mod options {
    #[on_success(
        fields = ["name"],
        handle = [
            |_ctx, _opts| async move { ... },
            |_ctx, _opts| async move { ... },
        ],
    )]
    const _: () = ();

    #[on_success(
        fields = ["email"],
        handle = |_ctx, _opts| async move { ... },
    )]
    const _: () = ();
}
```

## Description

`#[on_success]` registers handlers that run after the specified fields validate successfully. This corresponds to the current builder's `.on_success(fields, |b| b.handle(...))` method.

## Supported forms

| Syntax                                                   | Semantics                 |
| -------------------------------------------------------- | ------------------------- |
| `#[on_success(fields = [...], handle = closure)]`        | Single success handler    |
| `#[on_success(fields = [...], handle = [closure, ...])]` | Multiple success handlers |

Handlers have signature `|ctx, opts| async move { () }`.

## Mapping from current builder API

Current:

```rust
ivo::schema::<UserInput, User>()
    .on_success(["name"], |b| b
        .handle(|_ctx, _opts| async move { ... })
        .handle(|_ctx, _opts| async move { ... })
    )
```

New:

```rust
#[options]
mod options {
    #[on_success(
        fields = ["name"],
        handle = [
            |_ctx, _opts| async move { ... },
            |_ctx, _opts| async move { ... },
        ],
    )]
    const _: () = ();
}
```

## Generated code sketch

```rust
struct UserSchemaOnSuccessConfig {
    fields: &'static [&'static str],
    handlers: Vec<SuccessHandler<UserInput, User, UserCtxOptions>>,
}

impl UserSchemaModel {
    async fn run_on_success_handlers(
        &self,
        ctx: IvoContext<UserInput, User>,
        options: IvoRwCtxOptions<UserCtxOptions>,
        succeeded_fields: &HashSet<&str>,
    ) {
        for config in &self.options.on_success {
            let all_succeeded = config.fields.iter().all(|f| succeeded_fields.contains(f));
            if all_succeeded {
                for handler in &config.handlers {
                    handler(ctx.clone(), options.clone()).await;
                }
            }
        }
    }
}
```

## Notes

- Success handlers run only if all fields in the group validated without errors.
- The generated create/update pipeline tracks which fields succeeded.
- Field-level `#[on_success(...)]` attributes also exist; they run when that single field succeeds.
- Field names are validated at macro expansion time.

## Implementation plan

1. Define `UserSchemaOnSuccessConfig`.
2. Parse `#[on_success(fields = [...], handle = ...)]` attributes.
3. Support single handler and array of handlers.
4. Collect all on-success configs into a list.
5. Validate field names.
6. Track per-field success state during validation.
7. Generate `run_on_success_handlers` and call it after validation succeeds.

## Progress

- [ ] Define on-success config type
- [ ] Parse `#[on_success]` attributes
- [ ] Support single and multiple handlers
- [ ] Collect multiple configs
- [ ] Validate field names at macro time
- [ ] Track field success state
- [ ] Generate handler execution logic
- [ ] Wire into create/update pipeline
- [ ] Write tests
