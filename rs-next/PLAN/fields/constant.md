# Plan: `#[constant]` fields

`#[constant]` fields are output-only. A schema that contains any `#[constant]` field cannot use single-struct mode and must declare both `input = ...` and `output = ...` in `#[ivo_schema]`. Field visibility defaults to `pub` and can be overridden with a standard Rust visibility keyword or `#[visibility(private)]`. See [`struct_generation.md`](../struct_generation.md) for the full struct-generation and visibility rules.

## New syntax

```rust
#[fields]
mod fields {
    #[constant(Uuid::new_v4())]
    id: Uuid,

    #[constant(|| Uuid::new_v4())]
    id: Uuid,

    #[constant(async || fetch_uuid().await)]
    id: Uuid,

    #[constant(|ctx, opts| async move { opts.tenant_id })]
    tenant_id: Uuid,
}
```

## Supported attributes

| Attribute                        | Required | Description                                                           |
| -------------------------------- | -------- | --------------------------------------------------------------------- |
| `#[constant(value_or_resolver)]` | yes      | Marks the field as constant and provides its static or resolved value |
| `#[on_delete(closure)]`          | optional | Delete handler for this field                                         |
| `#[on_success(closure)]`         | optional | Success handler for this field                                        |

The `#[constant(...)]` argument supports the same forms as `#[default(...)]`:

| Syntax                                          | Semantics                   |
| ----------------------------------------------- | --------------------------- |
| `#[constant(expr)]`                             | Static constant value       |
| `#[constant(\|\| expr)]`                        | Sync resolver, no context   |
| `#[constant(async \|\| expr)]`                  | Async resolver, no context  |
| `#[constant(\|ctx, opts\| async move { ... })]` | Async resolver with context |

## Mapping from current builder API

Current:

```rust
ivo::schema::<UserInput, User>()
    .constant::<Uuid>("id")
    .value(|| Uuid::new_v4())
```

New:

```rust
#[fields]
mod fields {
    #[constant(|| Uuid::new_v4())]
    id: Uuid,
}
```

## Generated code sketch

```rust
enum ConstantValue<T, I: IvoStruct, O: IvoStruct, CtxOptions> {
    Static(T),
    Func(Box<dyn Fn(IvoConstantCtx<I, O>, IvoRwCtxOptions<CtxOptions>) -> BoxFuture<'static, T> + Send + Sync>),
}

struct TypedFieldConfig_Uuid {
    name: &'static str,
    value: ConstantValue<Uuid, UserInput, User, UserCtxOptions>,
    on_delete: Option<Vec<DeleteHandler<User, UserCtxOptions>>>,
    on_success: Option<Vec<SuccessHandler<UserInput, User, UserCtxOptions>>>,
}

impl UserSchemaModel {
    async fn resolve_id(
        &self,
        validated_outputs: &mut UserPartial,
        ctx: IvoContext<UserInput, User>,
        options: IvoRwCtxOptions<UserCtxOptions>,
    ) {
        let value = match &self.fields.id.value {
            ConstantValue::Static(v) => v.clone(),
            ConstantValue::Func(f) => f(ctx.constant_ctx(), options).await,
        };

        validated_outputs.id = Some(value);
    }
}
```

## Notes

- Constant fields are output-only; they do not appear on the input partial struct.
- The macro should emit a compile-time error if a constant field is declared on the input struct.
- Static values are evaluated at schema construction time; dynamic resolvers are evaluated per create/update call.
- **Struct placement:** the field name is added only to the generated output struct. Because constants are output-only, their presence forces dual-struct mode. See [`struct_generation.md`](../struct_generation.md).

## Invariants enforced by the macro

The macro rejects invalid attribute combinations at compile time. For constant fields, the following are errors:

- `#[constant]` without an argument — a value or resolver is required.
- `#[validate(...)]` or `#[re_validate(...)]` — constants are output-only and have no input value to validate.
- `#[sanitize(...)]` — sanitization applies to virtual fields, not constants.
- `#[default(...)]` — constants use `#[constant(...)]`, not `#[default(...)]`.
- `#[value(...)]` — the value is passed directly to `#[constant(...)]`.
- `#[on_failure(...)]` — constants do not support failure handlers in the current builder API.
- `#[ignore_init]` / `#[ignore_update]` — constants are always computed; ignore flags do not apply.

Because the macro knows the field type (`#[constant]`), it can whitelist the allowed attributes and emit a clear `compile_error!` for any disallowed attribute.

## Implementation plan

1. Define `ConstantValue<T, I, O, CtxOptions>` with concrete type `T`.
2. Parse `#[constant(value_or_resolver)]` field declarations.
3. Parse the constant argument in static/sync/async/context forms.
4. Parse `#[on_delete(...)]` and `#[on_success(...)]` handlers.
5. Reject disallowed attributes for constant fields, including bare `#[constant]` and `#[value(...)]`.
6. Generate per-field constant resolvers.
7. Ensure constant fields are skipped during input parsing and populated during output assembly.
8. Add compile-time checks for output-only presence.

## Progress

- [ ] Define typed `ConstantValue<T>`
- [ ] Implement `#[constant]` and `#[value]` parser
- [ ] Generate constant resolver methods
- [ ] Enforce output-only semantics
- [ ] Wire into create/update pipeline
- [ ] Write tests
