# Plan: `#[ignore]` option

This option does not change whether the schema uses single-struct or dual-struct mode; see [`struct_generation.md`](../struct_generation.md) for those rules.

## New syntax

```rust
#[options]
mod options {
    #[ignore(["secret"], |_ctx, _opts| async move { true })]
    const _: () = ();

    #[ignore(["hidden", "internal"], |_ctx, _opts| async move { false })]
    const _: () = ();
}
```

## Description

`#[ignore]` marks a group of fields as ignored when the provided resolver returns `true`. This corresponds to the current builder's `.ignore(fields, resolver)` method.

## Supported forms

| Syntax                                                   | Semantics                                           |
| -------------------------------------------------------- | --------------------------------------------------- |
| `#[ignore(["field"], \|ctx, opts\| async move { ... })]` | Async resolver decides whether to ignore the fields |
| `#[ignore(["field"], \|\| async move { ... })]`          | Async resolver with no context                      |
| `#[ignore(["field"], \|\| true)]`                        | Sync boolean (wrapped in async)                     |

## Mapping from current builder API

Current:

```rust
ivo::schema::<UserInput, User>()
    .ignore(["secret"], |_ctx, _opts| async move { true })
    .ignore(["hidden", "internal"], |_ctx, _opts| async move { false })
```

New:

```rust
#[options]
mod options {
    #[ignore(["secret"], |_ctx, _opts| async move { true })]
    const _: () = ();

    #[ignore(["hidden", "internal"], |_ctx, _opts| async move { false })]
    const _: () = ();
}
```

## Generated code sketch

```rust
struct UserSchemaIgnoreConfig {
    fields: &'static [&'static str],
    resolver: Box<dyn Fn(IvoContext<UserInput, User>, IvoRwCtxOptions<UserCtxOptions>) -> BoxFuture<'static, bool> + Send + Sync>,
}

impl UserSchemaModel {
    async fn apply_ignore_options(
        &self,
        raw_inputs: &mut UserInputPartial,
        ctx: IvoContext<UserInput, User>,
        options: IvoRwCtxOptions<UserCtxOptions>,
    ) {
        for config in &self.options.ignore {
            if (config.resolver)(ctx.clone(), options.clone()).await {
                for field in config.fields {
                    raw_inputs.unset(field);
                }
            }
        }
    }
}
```

## Field name validation

The macro checks at expansion time that every field in the array exists in `#[fields]`. Unknown field names produce a compile error.

## Implementation plan

1. Define `UserSchemaIgnoreConfig` (or generic equivalent).
2. Parse `#[ignore([...], closure)]` attributes in the options module.
3. Collect all ignore configs into a list.
4. Validate field names against the declared fields.
5. Generate `apply_ignore_options` and call it before per-field validation in create/update.

## Progress

- [ ] Define ignore config type
- [ ] Parse `#[ignore]` attributes
- [ ] Collect multiple ignore configs
- [ ] Validate field names at macro time
- [ ] Generate application logic
- [ ] Wire into create/update pipeline
- [ ] Write tests
