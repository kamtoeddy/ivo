# Plan: `#[dependent]` fields

`#[dependent]` fields are output-only. A schema that contains any `#[dependent]` field cannot use single-struct mode and must declare both `input = ...` and `output = ...` in `#[ivo_schema]`. Field visibility defaults to `pub` and can be overridden with a standard Rust visibility keyword or `#[visibility(private)]`. See [`struct_generation.md`](../struct_generation.md) for the full struct-generation and visibility rules.

## New syntax

```rust
#[fields]
mod fields {
    #[dependent(on = [age])]
    #[resolve(|ctx, _opts| async move {
        format!("{} years old", ctx.values().age.unwrap_or(0))
    })]
    #[default(|| "unknown age".to_string())]
    age_label: String,

    #[dependent(on = [first_name, last_name])]
    #[resolve(|ctx, _opts| async move {
        format!("{} {}", ctx.values().first_name.unwrap_or_default(), ctx.values().last_name.unwrap_or_default())
    })]
    full_name: String,
}
```

## Supported attributes

| Attribute                                  | Required | Description                                               |
| ------------------------------------------ | -------- | --------------------------------------------------------- |
| `#[dependent(on = [field1, field2, ...])]` | yes      | Declares the field dependent on one or more parent fields |
| `#[resolve(closure)]`                      | yes      | Resolver that computes the value from parent fields       |
| `#[default(...)]`                          | optional | Fallback value when dependencies are not provided         |
| `#[readonly]`                              | optional | Treat as readonly on update (requires static default)     |
| `#[on_delete(closure)]`                    | optional | Delete handler for this field                             |
| `#[on_success(closure)]`                   | optional | Lifecycle handler                                         |

The `#[default(...)]` attribute supports the same forms as `#[lax]` defaults:

| Syntax                                         | Semantics                            |
| ---------------------------------------------- | ------------------------------------ |
| `#[default(expr)]`                             | Static fallback value                |
| `#[default(\|\| expr)]`                        | Sync fallback resolver               |
| `#[default(async \|\| expr)]`                  | Async fallback resolver              |
| `#[default(\|ctx, opts\| async move { ... })]` | Async fallback resolver with context |

## Mapping from current builder API

Current:

```rust
ivo::schema::<UserInput, User>()
    .dependent::<String>("age_label")
    .depends_on(["age"])
    .resolve(|ctx, _opts| async move {
        format!("{} years old", ctx.values().age.unwrap_or(0))
    })
    .default(|| "unknown age".to_string())
```

New:

```rust
#[fields]
mod fields {
    #[dependent(on = [age])]
    #[resolve(|ctx, _opts| async move {
        format!("{} years old", ctx.values().age.unwrap_or(0))
    })]
    #[default(|| "unknown age".to_string())]
    age_label: String,
}
```

## Generated code sketch

```rust
struct TypedFieldConfig_String {
    name: &'static str,
    depends_on: &'static [&'static str],
    resolver: Box<dyn Fn(IvoContext<UserInput, User>, IvoRwCtxOptions<UserCtxOptions>) -> BoxFuture<'static, String> + Send + Sync>,
    default: Option<DefaultValue<String, UserInput, UserCtxOptions>>,
    ignore_update: Option<IsFieldProvisionEnabled<UserInput, User, UserCtxOptions>>,
    on_delete: Option<Vec<DeleteHandler<User, UserCtxOptions>>>,
    on_success: Option<Vec<SuccessHandler<UserInput, User, UserCtxOptions>>>,
}

impl UserSchemaModel {
    async fn resolve_age_label(
        &self,
        validated_outputs: &mut UserPartial,
        previous_values: &User,
        ctx: IvoContext<UserInput, User>,
        options: IvoRwCtxOptions<UserCtxOptions>,
    ) {
        if ctx.is_update() {
            if let Some(IsFieldProvisionEnabled::Readonly) = self.fields.age_label.ignore_update {
                if let Some(DefaultValue::Static(default_value)) = &self.fields.age_label.default {
                    // readonly: only resolve if previous value equals the static default
                    if !previous_values.ivo_internal_is_value_equal("age_label", default_value) {
                        return;
                    }
                }
            }
        }

        // Check whether all dependencies are present
        let deps_available = self.fields.age_label.depends_on.iter()
            .all(|dep| validated_outputs.get(dep).is_some());

        let value = if deps_available {
            (self.fields.age_label.resolver)(ctx, options).await
        } else if let Some(default) = &self.fields.age_label.default {
            match default {
                DefaultValue::Static(v) => v.clone(),
                DefaultValue::Func(f) => f(ctx.default_ctx(), options).await,
            }
        } else {
            return;
        };

        validated_outputs.age_label = Some(value);
    }
}
```

## Notes

- Dependent fields are computed from their parent fields; they are not provided by the caller.
- **Struct placement:** the field name is added only to the generated output struct. Because dependents are output-only, their presence forces dual-struct mode. Parent fields are added according to their own field type. See [`struct_generation.md`](../struct_generation.md).

## Dependency resolution order

- The macro builds a dependency graph from all `#[dependent(on = [...])]` declarations.
- It topologically sorts dependent fields and generates the resolution calls in dependency order.
- Cycles produce a compile-time error from the macro.
- The `dependent_children` cache from `rs` can be generated statically as a const array instead of computed at runtime.

## Generated dependency graph

```rust
const DEPENDENT_CHILDREN: &[(&'static str, &'static [&'static str])] = &[
    ("age", &["age_label"]),
    ("first_name", &["full_name"]),
    ("last_name", &["full_name"]),
];
```

## Invariants enforced by the macro

The macro rejects invalid attribute combinations for dependent fields:

- `#[validate(...)]` / `#[re_validate(...)]` — dependent values are computed, not validated directly.
- `#[required_error(...)]` — dependent fields are not required in the same sense as `#[required]` fields.
- `#[ignore_init]` / `#[ignore_update]` — dependent fields are computed from dependencies, not provided/ignored.
- `#[sanitize(...)]` / `#[alias(...)]` — sanitization and aliases are for virtual fields.
- `#[value(...)]` — constants use `#[value(...)]`; dependents use `#[resolve(...)]`.
- `#[on_failure(...)]` — the current builder API does not support failure handlers on dependent fields.
- `#[readonly]` with a computed default (`#[default(async \|\| ...)]`, `#[default(\|ctx, opts\| ...)]`) — readonly compares the previous value to the static default, so a computed default is not allowed.

### Readonly

`#[readonly]` on a dependent field means: during updates, the resolver is only called if the previous value equals the static default. If the value has been changed from its default, the update is ignored. Because the comparison is against the default value, `#[readonly]` requires a static default:

```rust
// OK
#[dependent(on = [age])]
#[resolve(|ctx, _opts| async move { format!("{} years old", ctx.values().age.unwrap_or(0)) })]
#[default(|| "unknown".to_string())]
#[readonly]
age_label: String,

// ERROR: computed default cannot be compared at update time
#[dependent(on = [age])]
#[resolve(|ctx, _opts| async move { format!("{} years old", ctx.values().age.unwrap_or(0)) })]
#[default(async || fetch_default_label().await)]
#[readonly]
age_label: String,
```

## Implementation plan

1. Define `TypedFieldConfig<T>` for dependent fields with `depends_on`, `resolver`, optional `default`, and `readonly`.
2. Parse `#[dependent(on = [...])]` and validate that every parent field exists.
3. Parse `#[resolve(...)]`, `#[default(...)]`, `#[readonly]`, `#[on_delete(...)]`, and `#[on_success(...)]`.
4. Reject disallowed attributes for dependent fields, including `#[readonly]` with computed defaults.
5. Build a dependency graph and topologically sort dependent fields at macro expansion time.
6. Generate per-field dependent resolvers in sorted order.
7. Generate a static `DEPENDENT_CHILDREN` table for efficient incremental updates.
8. Wire dependent resolution into create/update pipeline.

## Progress

- [ ] Define typed dependent field config
- [ ] Parse `#[dependent(on = [...])]`
- [ ] Parse typed resolvers and defaults
- [ ] Implement topological sort in the macro
- [ ] Generate static dependency tables
- [ ] Generate per-field dependent resolvers
- [ ] Wire into create/update pipeline
- [ ] Write tests
