# Plan: `#[lax]` fields

`#[lax]` fields appear on both the generated input and output structs. Field visibility defaults to `pub` and can be overridden with a standard Rust visibility keyword or `#[visibility(private)]`. See [`struct_generation.md`](../struct_generation.md) for struct-generation and visibility rules.

## New syntax

```rust
#[fields]
mod fields {
    #[lax("user")]
    role: String,

    #[lax(|| default_timezone())]
    timezone: String,

    #[lax(async || fetch_default_locale().await)]
    locale: String,

    #[lax(|ctx, opts| async move { opts.defaults.country.clone() })]
    country: String,

    #[lax]
    #[ignore_init]
    #[ignore_update]
    notes: Option<String>,
}
```

## Supported attributes

| Attribute                                 | Required | Description                                           |
| ----------------------------------------- | -------- | ----------------------------------------------------- |
| `#[lax]` or `#[lax(default_or_resolver)]` | yes      | Marks the field as lax; optional inline default       |
| `#[validate(closure)]`                    | optional | Field validator for create                            |
| `#[re_validate(closure)]`                 | optional | Field validator for update                            |
| `#[required(closure)]`                    | optional | Conditional required resolver                         |
| `#[ignore(closure)]`                      | optional | Conditional ignore resolver (create + update)         |
| `#[ignore_init]`                          | optional | Ignore field on create                                |
| `#[ignore_update]`                        | optional | Ignore field on update                                |
| `#[readonly]`                             | optional | Treat as readonly on update (requires static default) |
| `#[on_delete(closure)]`                   | optional | Delete handler for this field                         |
| `#[on_success(closure)]`                  | optional | Lifecycle handler                                     |
| `#[on_failure(closure)]`                  | optional | Lifecycle handler                                     |

The inline `#[lax(...)]` argument supports the same forms as `#[default(...)]`:

| Syntax                                     | Semantics                   |
| ------------------------------------------ | --------------------------- |
| `#[lax(expr)]`                             | Static default value        |
| `#[lax(\|\| expr)]`                        | Sync default resolver       |
| `#[lax(async \|\| expr)]`                  | Async default resolver      |
| `#[lax(\|ctx, opts\| async move { ... })]` | Async resolver with context |

## Mapping from current builder API

Current:

```rust
ivo::schema::<UserInput, User>()
    .lax::<String>("role")
    .default("user")
    .ignore_init()
    .ignore_update()
```

New:

```rust
#[fields]
mod fields {
    #[lax("user")]
    #[ignore_init]
    #[ignore_update]
    role: String,
}
```

## Generated code sketch

```rust
enum DefaultValue<T, I: IvoStruct, CtxOptions> {
    Static(T),
    Func(Box<dyn Fn(IvoDefaultCtx<I>, IvoRwCtxOptions<CtxOptions>) -> BoxFuture<'static, T> + Send + Sync>),
}

struct TypedFieldConfig_String {
    name: &'static str,
    default: Option<DefaultValue<String, UserInput, UserCtxOptions>>,
    validator: Option<Box<dyn Fn(String, IvoContext<UserInput, User>, IvoRwCtxOptions<UserCtxOptions>) -> BoxFuture<'static, ValidatorResponse<String, Metadata>> + Send + Sync>>,
    re_validator: Option<Box<dyn Fn(String, IvoContext<UserInput, User>, IvoRwCtxOptions<UserCtxOptions>) -> BoxFuture<'static, ValidatorResponse<String, Metadata>> + Send + Sync>>,
    required_fn: Option<RequiredResolver<UserInput, User, UserCtxOptions>>,
    ignore: Option<BooleanResolver<UserInput, User, UserCtxOptions>>,
    ignore_init: Option<IsFieldProvisionEnabled<UserInput, User, UserCtxOptions>>,
    ignore_update: Option<IsFieldProvisionEnabled<UserInput, User, UserCtxOptions>>,
    on_delete: Option<Vec<DeleteHandler<User, UserCtxOptions>>>,
    on_success: Option<Vec<SuccessHandler<UserInput, User, UserCtxOptions>>>,
    on_failure: Option<Vec<FailureHandler<UserInput, User, UserCtxOptions>>>,
}

impl UserSchemaModel {
    // Conditional required check. Called during field-provision filtering.
    async fn check_role_required(
        &self,
        ctx: IvoContext<UserInput, User>,
        options: IvoRwCtxOptions<UserCtxOptions>,
    ) -> Option<String> {
        if let Some(required_fn) = &self.fields.role.required_fn {
            return required_fn(ctx, options).await;
        }
        None
    }

    async fn resolve_role(
        &self,
        raw_inputs: &UserInputPartial,
        validated_inputs: &mut UserInputPartial,
        validated_outputs: &mut UserPartial,
        ctx: IvoContext<UserInput, User>,
        options: IvoRwCtxOptions<UserCtxOptions>,
        errors: &mut ErrorTool,
    ) {
        let value = if let Some(v) = raw_inputs.role.clone() {
            v
        } else if let Some(default) = &self.fields.role.default {
            match default {
                DefaultValue::Static(v) => v.clone(),
                DefaultValue::Func(f) => f(ctx.default_ctx(), options).await,
            }
        } else {
            return;
        };

        if let Some(validator) = &self.fields.role.validator {
            match validator(value, ctx, options).await {
                Err(e) => errors.set("role", e),
                Ok(Some(v)) => {
                    validated_inputs.role = Some(v.clone());
                    validated_outputs.role = Some(v);
                }
                Ok(None) => {
                    validated_outputs.role = validated_inputs.role.clone();
                }
            }
        } else {
            validated_inputs.role = Some(value.clone());
            validated_outputs.role = Some(value);
        }
    }
}
```

## Notes

- Lax fields are optional on input; when missing, the default (if any) is applied.
- **Struct placement:** the field name is added to the input struct. In single-struct mode it is also the output struct; in dual-struct mode it is added to both `input` and `output`. See [`struct_generation.md`](../struct_generation.md).

## Default forms

| Syntax                                         | Generated behavior                                                                            |
| ---------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `#[default("user")]`                           | `DefaultValue::Static(String::from("user"))`                                                  |
| `#[default(\|\| default_timezone())]`          | `DefaultValue::Func(Box::new(\|_ctx, _opts\| Box::pin(async move { default_timezone() })))`   |
| `#[default(async \|\| fetch_locale().await)]`  | `DefaultValue::Func(Box::new(\|_ctx, _opts\| Box::pin(async move { fetch_locale().await })))` |
| `#[default(\|ctx, opts\| async move { ... })]` | `DefaultValue::Func(Box::new(\|ctx, opts\| Box::pin(async move { ... })))`                    |

## Invariants enforced by the macro

The macro rejects invalid attribute combinations for lax fields:

- `#[required_error(...)]` — required errors are for `#[required]` fields.
- `#[value(...)]` — constants use `#[constant(...)]`.
- `#[resolve(...)]` / `#[depends_on(...)]` — dependency resolution is for `#[dependent]` fields.
- `#[sanitize(...)]` / `#[alias(...)]` — sanitization and aliases are for virtual fields.
- `#[readonly]` with a computed default (`#[lax(async \|\| ...)]`, `#[lax(\|ctx, opts\| ...)]`) — readonly compares the previous value to the static default, so a computed default is not allowed.

### Conditional required

`#[required(closure)]` on a lax field makes the field required only when the resolver returns `Some(reason)`. The closure signature is the same as the field-level `#[required]` resolver in the current builder API:

```rust
#[lax]
#[required(|ctx, opts| async move {
    if ctx.input().age.is_none() && ctx.input().birth_year.is_none() {
        Some("age or birth_year is required".to_string())
    } else {
        None
    }
})]
age: Option<i32>,
```

### Conditional ignore

`#[ignore(closure)]` ignores the field on both create and update when the resolver returns `true`:

```rust
#[lax]
#[ignore(|ctx, opts| async move { opts.skip_optional_fields })]
notes: Option<String>,
```

This is distinct from schema-level `#[ignore]` in the `#[options]` module.

### Readonly

`#[readonly]` on a lax field means updates are only allowed when the previous value equals the static default. If the value has changed from its default, the update is ignored. Because the comparison is against the default value, `#[readonly]` requires a static default:

```rust
// OK
#[lax("user")]
#[readonly]
role: String,

// ERROR: computed default cannot be compared at update time
#[lax(|| fetch_role())]
#[readonly]
role: String,
```

- `#[default(...)]` — lax defaults are provided inline via `#[lax(...)]`.

## Implementation plan

1. Define `DefaultValue<T, I, CtxOptions>` with concrete type `T` instead of `ErasedValue`.
2. Parse `#[lax]` and `#[lax(default_or_resolver)]` fields.
3. Parse `#[validate]`, `#[re_validate]`, `#[required]`, `#[ignore]`, `#[ignore_init]`, `#[ignore_update]`, `#[readonly]`, `#[on_delete]`, `#[on_success]`, `#[on_failure]` attributes.
4. Reject disallowed attributes for lax fields, including separate `#[default(...)]` and `#[readonly]` with computed defaults.
5. Generate per-field `resolve_<name>` methods that apply defaults, conditional required checks, conditional ignore checks, and validators.
6. Integrate lax field resolution into the generated create/update flow.

## Progress

- [ ] Define typed `DefaultValue<T>`
- [ ] Implement `#[default]` parser for static/sync/async/context forms
- [ ] Implement `#[ignore_init]` / `#[ignore_update]` flags
- [ ] Generate per-field lax resolvers
- [ ] Wire into create/update pipeline
- [ ] Write tests
