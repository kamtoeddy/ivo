# Plan: `#[required]` fields

`#[required]` fields appear on both the generated input and output structs. Field visibility defaults to `pub` and can be overridden with a standard Rust visibility keyword or `#[visibility(private)]`. See [`struct_generation.md`](../struct_generation.md) for struct-generation and visibility rules.

## New syntax

```rust
#[fields]
mod fields {
    #[required]
    #[validate(|name, _ctx, _opts| async move { Ok(Some(name)) })]
    #[re_validate(|name, _ctx, _opts| async move { Ok(Some(name)) })]
    #[required_error("name is required")]
    #[on_success(|_ctx, _opts| async move { ... })]
    #[on_failure(|_ctx, _opts| async move { ... })]
    name: String,
}
```

## Supported attributes

| Attribute                   | Required | Description                                       |
| --------------------------- | -------- | ------------------------------------------------- |
| `#[required]`               | yes      | Marks the field as required                       |
| `#[validate(closure)]`      | optional | Field validator for create                        |
| `#[re_validate(closure)]`   | optional | Field validator for update                        |
| `#[required_error(expr)]`   | optional | Static or computed required error message         |
| `#[ignore_update(closure)]` | optional | Ignore field on update when resolver returns true |
| `#[readonly]`               | optional | Treat as readonly on update (requires validator)  |
| `#[on_delete(closure)]`     | optional | Delete handler for this field                     |
| `#[on_success(closure)]`    | optional | Lifecycle handler called on successful validation |
| `#[on_failure(closure)]`    | optional | Lifecycle handler called on validation failure    |

## Mapping from current builder API

Current:

```rust
ivo::schema::<UserInput, User>()
    .required::<String>("name")
    .validate(|name, _ctx, _opts| async move { Ok(Some(name)) })
    .re_validate(|name, _ctx, _opts| async move { Ok(Some(name)) })
    .required_error("name is required")
    .on_success(|_ctx, _opts| async move { ... })
    .on_failure(|_ctx, _opts| async move { ... })
```

New:

```rust
#[fields]
mod fields {
    #[required]
    #[validate(|name, _ctx, _opts| async move { Ok(Some(name)) })]
    #[re_validate(|name, _ctx, _opts| async move { Ok(Some(name)) })]
    #[required_error("name is required")]
    #[on_success(|_ctx, _opts| async move { ... })]
    #[on_failure(|_ctx, _opts| async move { ... })]
    name: String,
}
```

## Generated code sketch

```rust
struct TypedFieldConfig_String {
    name: &'static str,
    validator: Option<Box<dyn Fn(String, IvoContext<UserInput, User>, IvoRwCtxOptions<UserCtxOptions>) -> BoxFuture<'static, ValidatorResponse<String, Metadata>> + Send + Sync>>,
    re_validator: Option<Box<dyn Fn(String, IvoContext<UserInput, User>, IvoRwCtxOptions<UserCtxOptions>) -> BoxFuture<'static, ValidatorResponse<String, Metadata>> + Send + Sync>>,
    required_error: Option<ComputableRequiredError<UserInput, UserCtxOptions>>,
    ignore_update: Option<IsFieldProvisionEnabled<UserInput, User, UserCtxOptions>>,
    on_delete: Option<Vec<DeleteHandler<User, UserCtxOptions>>>,
    on_success: Option<Vec<SuccessHandler<UserInput, User, UserCtxOptions>>>,
    on_failure: Option<Vec<FailureHandler<UserInput, User, UserCtxOptions>>>,
}

// Note: readonly on required fields sets ignore_update to Readonly.
// The runtime treats this as "always unset on update", so no extra field is needed in TypedFieldConfig.

impl UserSchemaModel {
    async fn validate_name(
        &self,
        raw_inputs: &UserInputPartial,
        validated_inputs: &mut UserInputPartial,
        validated_outputs: &mut UserPartial,
        ctx: IvoContext<UserInput, User>,
        options: IvoRwCtxOptions<UserCtxOptions>,
        errors: &mut ErrorTool,
    ) {
        let Some(value) = raw_inputs.name.clone() else {
            // required: emit error
            if let Some(error) = self.resolve_required_error("name", ctx, options).await {
                errors.set("name", error);
            }
            return;
        };

        if let Some(validator) = &self.fields.name.validator {
            match validator(value, ctx, options).await {
                Err(e) => errors.set("name", e),
                Ok(Some(v)) => {
                    validated_inputs.name = Some(v.clone());
                    validated_outputs.name = Some(v);
                }
                Ok(None) => {
                    validated_outputs.name = validated_inputs.name.clone();
                }
            }
        } else {
            validated_inputs.name = Some(value.clone());
            validated_outputs.name = Some(value);
        }
    }
}
```

## Notes

- Required fields are validated during create and update.
- If the field is missing and no `#[required_error]` is provided, a generic required error is emitted.
- Field-level `#[ignore_update]` and `#[readonly]` both control update-time behavior; they should not be combined in conflicting ways.
- **Struct placement:** the field name is added to the input struct. In single-struct mode it is also the output struct; in dual-struct mode it is added to both `input` and `output`. See [`struct_generation.md`](../struct_generation.md).

## Invariants enforced by the macro

The macro rejects invalid attribute combinations for required fields:

- `#[default(...)]` — required fields must be provided by the caller; defaults do not apply.
- `#[value(...)]` — constants use `#[value(...)]`, not required fields.
- `#[resolve(...)]` / `#[depends_on(...)]` — dependency resolution is for `#[dependent]` fields.
- `#[sanitize(...)]` — sanitization applies to virtual fields.
- `#[alias(...)]` — aliases are for virtual fields.
- `#[ignore_init]` — required fields cannot be ignored on create.
- `#[readonly]` without `#[validate]` — the current builder API requires a validator before `readonly()` can be called on a required field.

### Readonly

`#[readonly]` on a required field means the field is always unset during updates (it cannot be changed after creation). The current builder requires `validate(...)` before `readonly()`:

```rust
#[required]
#[validate(|name, _ctx, _opts| async move { Ok(Some(name)) })]
#[readonly]
name: String,
```

## Implementation plan

1. Define `TypedFieldConfig<T>` for required fields in the new schema crate.
2. Parse `#[required]` field declarations in the proc macro.
3. Parse `#[validate]`, `#[re_validate]`, `#[required_error]`, `#[ignore_update]`, `#[readonly]`, `#[on_delete]`, `#[on_success]`, `#[on_failure]` attributes.
4. Reject disallowed attributes for required fields, including `#[readonly]` without `#[validate]`.
5. Generate per-field `validate_<name>` and `re_validate_<name>` methods.
6. Wire required-error resolution into the generated create/update flow.
7. Add compile-time assertion that required fields exist on the input/output partial structs.

## Progress

- [ ] Design `TypedFieldConfig<T>`
- [ ] Implement parser for required field attributes
- [ ] Generate typed validator wrappers
- [ ] Generate required-error resolution
- [ ] Generate lifecycle handlers
- [ ] Write tests
