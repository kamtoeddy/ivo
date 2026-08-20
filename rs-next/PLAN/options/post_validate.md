# Plan: `#[post_validate]` option

This option does not change whether the schema uses single-struct or dual-struct mode; see [`struct_generation.md`](../struct_generation.md) for those rules.

## New syntax

```rust
#[options]
mod options {
    #[post_validate(
        fields = ["name", "email"],
        pre_validate = |_ctx, _opts| async move {
            // Run before field validators
            None
        },
        validate = [
            |_ctx, _opts| async move { None },
            |_ctx, _opts| async move { None },
        ],
    )]
    const _: () = ();

    #[post_validate(
        fields = ["age"],
        validate = |_ctx, _opts| async move { None },
    )]
    const _: () = ();
}
```

## Description

`#[post_validate]` groups one or more post-validators that run after all field-level validators. It can optionally include a `pre_validate` step that runs before field validators. This corresponds to the current builder's `.post_validate(fields, |b| b.pre_validate(...).validate(...))` method.

## Supported forms

| Syntax                                                                     | Semantics                   |
| -------------------------------------------------------------------------- | --------------------------- |
| `#[post_validate(fields = [...], validate = closure)]`                     | Single post-validator       |
| `#[post_validate(fields = [...], validate = [closure, ...])]`              | Multiple post-validators    |
| `#[post_validate(fields = [...], pre_validate = closure, validate = ...)]` | With optional pre-validator |

All closures have signature `|ctx, opts| async move { Option<PartialErrors> }`.

## Mapping from current builder API

Current:

```rust
ivo::schema::<UserInput, User>()
    .post_validate(["name", "email"], |b| b
        .pre_validate(|_ctx, _opts| async move { ... })
        .validate(|_ctx, _opts| async move { ... })
        .validate(|_ctx, _opts| async move { ... })
    )
```

New:

```rust
#[options]
mod options {
    #[post_validate(
        fields = ["name", "email"],
        pre_validate = |_ctx, _opts| async move { ... },
        validate = [
            |_ctx, _opts| async move { ... },
            |_ctx, _opts| async move { ... },
        ],
    )]
    const _: () = ();
}
```

## Generated code sketch

```rust
struct UserSchemaPostValidateConfig {
    fields: &'static [&'static str],
    pre_validator: Option<PostValidator<UserInput, User, UserCtxOptions, DefaultErrorSanitizer>>,
    validators: Vec<PostValidator<UserInput, User, UserCtxOptions, DefaultErrorSanitizer>>,
}

impl UserSchemaModel {
    async fn apply_pre_validators(
        &self,
        ctx: IvoContext<UserInput, User>,
        options: IvoRwCtxOptions<UserCtxOptions>,
        errors: &mut ErrorTool,
    ) {
        for config in &self.options.post_validate {
            if let Some(pre) = &config.pre_validator {
                if let Some(partial_errors) = pre(ctx.clone(), options.clone()).await {
                    for (field_name, error) in partial_errors.entries() {
                        if config.fields.contains(&field_name.as_str()) {
                            errors.set(&field_name, error);
                        }
                    }
                }
            }
        }
    }

    async fn apply_post_validators(
        &self,
        ctx: IvoContext<UserInput, User>,
        options: IvoRwCtxOptions<UserCtxOptions>,
        errors: &mut ErrorTool,
    ) {
        for config in &self.options.post_validate {
            for validator in &config.validators {
                if let Some(partial_errors) = validator(ctx.clone(), options.clone()).await {
                    for (field_name, error) in partial_errors.entries() {
                        if config.fields.contains(&field_name.as_str()) {
                            errors.set(&field_name, error);
                        }
                    }
                }
            }
        }
    }
}
```

## Notes

- `pre_validate` runs once per config before field validators.
- `validate` entries run once per config after field validators.
- Multiple `#[post_validate]` attributes are allowed.
- Field names are validated at macro expansion time.

## Implementation plan

1. Define `UserSchemaPostValidateConfig` with optional `pre_validator` and vector of `validators`.
2. Parse `#[post_validate(fields = [...], ...)]` attributes.
3. Support single closure and array of closures for `validate`.
4. Support optional `pre_validate`.
5. Collect all post-validate configs into a list.
6. Validate field names.
7. Generate `apply_pre_validators` (before field validation) and `apply_post_validators` (after field validation).

## Progress

- [ ] Define post-validate config type
- [ ] Parse `#[post_validate]` attributes
- [ ] Support `pre_validate` and multiple `validate` closures
- [ ] Collect multiple configs
- [ ] Validate field names at macro time
- [ ] Generate pre/post application logic
- [ ] Wire into create/update pipeline
- [ ] Write tests
