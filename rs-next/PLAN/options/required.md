# Plan: `#[required]` option

This option does not change whether the schema uses single-struct or dual-struct mode; see [`struct_generation.md`](../struct_generation.md) for those rules.

## New syntax

```rust
#[options]
mod options {
    #[required(["name", "email"], |_ctx, _opts| async move {
        // Return PartialErrors if the group requirement is not met
        None
    })]
    const _: () = ();

    #[required(["phone", "email"], |_ctx, _opts| async move {
        // At least one of phone or email must be provided
        None
    })]
    const NAME_EMAIL_REQUIRED: () = ();
}
```

## Description

`#[required]` expresses a cross-field required constraint. The resolver returns `Option<I::PartialErrors>` when the constraint is violated. This corresponds to the current builder's `.required(fields, resolver)` method.

## Supported forms

| Syntax                                                 | Semantics                                |
| ------------------------------------------------------ | ---------------------------------------- |
| `#[required([...], \|ctx, opts\| async move { ... })]` | Async resolver returning optional errors |
| `#[required([...], \|\| async move { ... })]`          | Async resolver with no context           |
| `#[required([...], \|\| None)]`                        | Sync resolver (wrapped in async)         |

## Mapping from current builder API

Current:

```rust
ivo::schema::<UserInput, User>()
    .required(["name", "email"], |_ctx, _opts| async move { ... })
```

New:

```rust
#[options]
mod options {
    #[required(["name", "email"], |_ctx, _opts| async move { ... })]
    const _: () = ();
}
```

## Generated code sketch

```rust
struct UserSchemaRequiredConfig {
    fields: &'static [&'static str],
    resolver: Box<dyn Fn(IvoContext<UserInput, User>, IvoRwCtxOptions<UserCtxOptions>) -> BoxFuture<'static, Option<UserInputPartialErrors>> + Send + Sync>,
}

impl UserSchemaModel {
    async fn apply_required(
        &self,
        ctx: IvoContext<UserInput, User>,
        options: IvoRwCtxOptions<UserCtxOptions>,
        errors: &mut ErrorTool,
    ) {
        for config in &self.options.required {
            if let Some(partial_errors) = (config.resolver)(ctx.clone(), options.clone()).await {
                for (field_name, error) in partial_errors.entries() {
                    if config.fields.contains(&field_name.as_str()) {
                        errors.set(&field_name, error);
                    }
                }
            }
        }
    }
}
```

## Notes

- The resolver returns the same `PartialErrors` type used by the sanitizer/validator pipeline.
- Field names in the array are validated at macro expansion time.
- The optional named const (`NAME_EMAIL_REQUIRED`) can be used for debug identification.
- This schema-level `#[required]` is distinct from the field-level `#[required]` attribute. The macro distinguishes them by context: field-level inside `#[fields]`, schema-level inside `#[options]`.

## Implementation plan

1. Define `UserSchemaRequiredConfig`.
2. Parse `#[required([...], closure)]` attributes.
3. Collect all required configs into a list.
4. Validate field names against declared fields.
5. Generate `apply_required` and call it after individual field validation in create/update.

## Progress

- [ ] Define required config type
- [ ] Parse `#[required]` attributes
- [ ] Collect multiple configs
- [ ] Validate field names at macro time
- [ ] Generate application logic
- [ ] Wire into create/update pipeline
- [ ] Write tests
