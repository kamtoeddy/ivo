# Plan: `#[virtual]` fields

`#[virtual]` fields are input-only (or, with `alias = "..."`, the alias is input-only). A schema that contains any `#[virtual]` field cannot use single-struct mode and must declare both `input = ...` and `output = ...` in `#[ivo_schema]`. Field visibility defaults to `pub` and can be overridden with a standard Rust visibility keyword or `#[visibility(private)]`. See [`struct_generation.md`](../struct_generation.md) for the full struct-generation and visibility rules.

## New syntax

```rust
#[fields]
mod fields {
    #[virtual]
    #[sanitize(|email, _ctx, _opts| async move { email.to_lowercase() })]
    #[validate(|email, _ctx, _opts| async move { Ok(Some(email)) })]
    email: String,

    #[virtual(alias = "raw_email")]
    #[sanitize(|raw_email, _ctx, _opts| async move { raw_email.trim().to_lowercase() })]
    email: String,
}
```

## Supported attributes

| Attribute                           | Required | Description                                         |
| ----------------------------------- | -------- | --------------------------------------------------- |
| `#[virtual]` or `#[virtual(alias)]` | yes      | Marks the field as virtual; optional alias argument |
| `#[sanitize(closure)]`              | optional | Sanitizer applied to the raw input value            |
| `#[validate(closure)]`              | optional | Validator for the sanitized value                   |
| `#[re_validate(closure)]`           | optional | Re-validator for updates                            |
| `#[required(closure)]`              | optional | Conditional required resolver                       |
| `#[ignore(closure)]`                | optional | Conditional ignore resolver (create + update)       |
| `#[ignore_init]`                    | optional | Ignore field on create                              |
| `#[ignore_update]`                  | optional | Ignore field on update                              |
| `#[on_success(closure)]`            | optional | Lifecycle handler                                   |
| `#[on_failure(closure)]`            | optional | Lifecycle handler                                   |

## Mapping from current builder API

Current:

```rust
ivo::schema::<UserInput, User>()
    .virtual_field::<String>("email")
    .alias("raw_email")
    .sanitize(|email, _ctx, _opts| async move { email.to_lowercase() })
    .validate(|email, _ctx, _opts| async move { Ok(Some(email)) })
```

New:

```rust
#[fields]
mod fields {
    #[virtual(alias = "raw_email")]
    #[sanitize(|email, _ctx, _opts| async move { email.to_lowercase() })]
    #[validate(|email, _ctx, _opts| async move { Ok(Some(email)) })]
    email: String,
}
```

## Generated code sketch

```rust
struct TypedFieldConfig_String {
    name: &'static str,
    alias: Option<&'static str>,
    sanitizer: Option<Box<dyn Fn(String, IvoContext<UserInput, User>, IvoRwCtxOptions<UserCtxOptions>) -> BoxFuture<'static, String> + Send + Sync>>,
    validator: Option<Box<dyn Fn(String, IvoContext<UserInput, User>, IvoRwCtxOptions<UserCtxOptions>) -> BoxFuture<'static, ValidatorResponse<String, Metadata>> + Send + Sync>>,
    re_validator: Option<Box<dyn Fn(String, IvoContext<UserInput, User>, IvoRwCtxOptions<UserCtxOptions>) -> BoxFuture<'static, ValidatorResponse<String, Metadata>> + Send + Sync>>,
    required_fn: Option<RequiredResolver<UserInput, User, UserCtxOptions>>,
    ignore: Option<BooleanResolver<UserInput, User, UserCtxOptions>>,
    ignore_init: bool,
    ignore_update: bool,
    on_success: Option<Vec<SuccessHandler<UserInput, User, UserCtxOptions>>>,
    on_failure: Option<Vec<FailureHandler<UserInput, User, UserCtxOptions>>>,
}

impl UserSchemaModel {
    // Sanitization phase: runs before per-field validation.
    async fn sanitize_email(
        &self,
        inputs: &mut UserInputPartial,
        ctx: IvoContext<UserInput, User>,
        options: IvoRwCtxOptions<UserCtxOptions>,
    ) {
        let source_field = self.fields.email.alias.unwrap_or("email");
        let raw_value = match source_field {
            "email" => inputs.email.clone(),
            "raw_email" => inputs.raw_email.clone(),
            _ => unreachable!(),
        };

        let Some(value) = raw_value else { return };

        let sanitized = if let Some(sanitizer) = &self.fields.email.sanitizer {
            sanitizer(value, ctx, options).await
        } else {
            value
        };

        inputs.email = Some(sanitized);
    }

    // Conditional required check. Called during field-provision filtering.
    async fn check_email_required(
        &self,
        ctx: IvoContext<UserInput, User>,
        options: IvoRwCtxOptions<UserCtxOptions>,
    ) -> Option<String> {
        if let Some(required_fn) = &self.fields.email.required_fn {
            return required_fn(ctx, options).await;
        }
        None
    }

    // Validation phase: runs after sanitization, on the sanitized input.
    async fn validate_email(
        &self,
        sanitized_inputs: &UserInputPartial,
        validated_inputs: &mut UserInputPartial,
        ctx: IvoContext<UserInput, User>,
        options: IvoRwCtxOptions<UserCtxOptions>,
        errors: &mut ErrorTool,
    ) {
        let Some(value) = sanitized_inputs.email.clone() else { return };

        if let Some(validator) = &self.fields.email.validator {
            match validator(value, ctx, options).await {
                Err(e) => errors.set("email", e),
                Ok(Some(v)) => validated_inputs.email = Some(v),
                Ok(None) => validated_inputs.email = Some(value),
            }
        } else {
            validated_inputs.email = Some(value);
        }
    }
}
```

## Notes

- Virtual fields are input-only: their value comes from the raw input (or alias), is sanitized/validated, and then consumed by dependent fields.
- Without an alias, the field name is added to the generated input struct only.
- With an alias, the alias is added to the generated input struct; the virtual field name is not added to either struct.
- Because virtuals are input-only, their presence forces dual-struct mode.
- The alias allows the input struct to use a different field name (e.g., `raw_email`) than the schema field name (`email`).
- Sanitizers and validators operate on the concrete type `T`, not `ErasedValue`.
- See [`struct_generation.md`](../struct_generation.md) for the full rules.

## Pipeline order

Sanitization and validation are distinct phases:

1. **Sanitization** runs first, producing a sanitized input partial. Only virtual fields with a sanitizer participate; other fields are untouched.
2. **Validation** runs after sanitization, using the sanitized values. Virtual fields with a validator participate alongside required/lax fields.

This mirrors the current runtime behavior where `sanitize_virtuals` is called before `validate`.

## Alias handling

If `#[virtual(alias = "raw_email")]` is present, the generated sanitizer reads from `inputs.raw_email` and writes the sanitized value to `inputs.email`. Otherwise it reads from `inputs.email`. The macro emits a compile-time check that the alias field exists on the input partial struct.

## Invariants enforced by the macro

The macro rejects invalid attribute combinations for virtual fields:

- `#[default(...)]` / `#[value(...)]` — virtual fields sanitize and validate input, they do not produce values via defaults or constants.
- `#[alias(...)]` as a separate attribute — aliases are passed inline via `#[virtual(alias = "...")]`.
- `#[resolve(...)]` / `#[depends_on(...)]` — dependency resolution is for `#[dependent]` fields.
- `#[required_error(...)]` — virtual fields are not required fields.
- `#[on_delete(...)]` — the current builder API does not support delete handlers on virtual fields.
- `#[readonly]` — virtual fields do not support readonly in the current builder API.

### Conditional required

`#[required(closure)]` on a virtual field makes the field required only when the resolver returns `Some(reason)`:

```rust
#[virtual(alias = "raw_email")]
#[sanitize(|email, _ctx, _opts| async move { email.to_lowercase() })]
#[required(|ctx, opts| async move {
    if opts.require_email { Some("email is required".to_string()) } else { None }
})]
email: String,
```

### Conditional ignore

`#[ignore(closure)]` ignores the virtual field on both create and update when the resolver returns `true`:

```rust
#[virtual]
#[sanitize(|email, _ctx, _opts| async move { email.to_lowercase() })]
#[ignore(|ctx, opts| async move { opts.skip_email })]
email: String,
```

This is distinct from schema-level `#[ignore]` in the `#[options]` module.

## Implementation plan

1. Define `TypedFieldConfig<T>` for virtual fields with `alias`, `sanitizer`, `validator`, `re_validator`, `required_fn`, and `ignore`.
2. Parse `#[virtual]` and `#[virtual(alias = "...")]` field declarations.
3. Parse `#[sanitize(...)]`, `#[validate(...)]`, `#[re_validate(...)]`, `#[required(...)]`, `#[ignore(...)]`, `#[ignore_init]`, `#[ignore_update]`, `#[on_success]`, `#[on_failure]`.
4. Reject disallowed attributes for virtual fields, including separate `#[alias(...)]` and `#[readonly]`.
5. Generate source-field selection logic for aliases.
6. Generate a separate `sanitize_<name>` method for the sanitization phase.
7. Generate a separate `validate_<name>` method for the validation phase.
8. Ensure virtual fields are excluded from output assembly.
9. Add compile-time checks for alias existence on the input struct.

## Progress

- [ ] Define typed virtual field config
- [ ] Implement `#[virtual]` parser
- [ ] Implement alias parsing and validation
- [ ] Generate sanitizer/validator wrappers
- [ ] Generate source-field selection
- [ ] Wire into create/update pipeline
- [ ] Write tests
