# Ivo Rust v2 API Design Preferences

> **Implementation progress: ~99%**
>
> Core schema parsing, struct generation, field-type handling, grouped options, attribute whitelist validation, dependency-graph validation, context/options wrapper integration, returned trigger futures, sync/async method inference for `create`/`update`/`delete`, `post_validate` grouped options, and smoke tests covering trigger execution are in place. `delete` returns plain `()`. Remaining work is mainly broader semantic-parity ports from `rs/tests` and extending `post_validate` to support optional partial updates / update-time validation.

This document captures the user preferences for the next-generation Ivo Rust API (`rs-v2`). It is the source of truth for syntax and architectural decisions.

## 1. Schema generation must be macro-driven

Schemas are declared, not built imperatively. A single proc macro (`#[ivo_schema]`) receives the field definitions and schema options, then generates the input/output structs, their partial/error structs, and a typed, schema-specific model.

- **Dual-struct entry shape** (required when the schema has input-only or output-only fields):

  ```rust
  #[ivo_schema(
      input(
          UserInput,
          derive(Debug, Clone, PartialEq),          // optional
          derive_partial(Deserialize),              // optional
      ),
      output(
          User,
          derive(Debug, Clone, PartialEq),          // optional
          derive_partial(Serialize),                // optional
      ),
      ctx_options(UserCtxOptions),                  // optional
      error_sanitizer(ErrorSanitizer),              // optional
  )]
  mod user_schema {
      struct Fields { ... }

      #[ignore(["secret"], async |_ctx, _opts| { true })]
      const _: () = ();

      #[required(["name", "email"], async |_ctx, _opts| { ... })]
      const _: () = ();
  }
  ```

- **Single-struct entry shape** (allowed when the schema contains only `#[required]` and/or `#[lax]` fields and no `#[timestamps]`):

  ```rust
  #[ivo_schema(
      input(
          User,
          derive(Debug, Clone, PartialEq),          // optional
          derive_partial(Deserialize, Serialize),   // optional
      ),
  )]
  mod user_schema {
      struct Fields { ... }
  }
  ```

- The macro generates a `{StructName}Model` unit type and a `pub const {StructName}Model: {StructName}ModelType = {StructName}ModelType;` singleton, so `user_schema::UserModel.create(...)` works without `.new()`.
- `input(...)` is always required and names the input struct. `output(...)` names the output struct and is required only when the schema has input-only fields (`#[ivo_virtual]`) or output-only fields (`#[constant]`, `#[depends_on(...)]` / `#[dependent]`, `#[timestamps]`).
- `derive(...)` inside `input(...)` / `output(...)` adds derives to the generated input/output struct. `derive_partial(...)` adds derives to the corresponding generated partial struct (`PartialUserInput` / `PartialUser`).
- `ctx_options(UserCtxOptions)` and `error_sanitizer(ErrorSanitizer)` are optional runtime types provided by the user.

## 2. Layout inside the schema module

- `struct Fields { ... }` contains field declarations with attributes. It is required because every schema must declare at least one field, and bare field declarations are not valid at module scope.
- Grouped schema options are attached to anonymous `const _: () = ()` items directly inside the schema module. They are optional; omit them when the schema has no options.
- The macro identifies the field container by the struct name `Fields` and option anchors by the `const _: () = ()` pattern. No additional attribute markers are needed.

## 3. `const _: () = ()` is the preferred anchor for options

Each grouped option is attached to an anonymous const item directly inside the schema module:

```rust
#[ivo_schema(...)]
mod user_schema {
    struct Fields { ... }

    #[ignore(["secret"], async |_ctx, _opts| { true })]
    const _: () = ();

    #[required(["name", "email"], async |_ctx, _opts| { ... })]
    const _: () = ();

    #[post_validate(
        ["password", "password_confirmation"],
        validate = async |ctx, _opts| {
            if ctx.input().password != ctx.input().password_confirmation {
                let mut errors = UserInputErrors::new();
                errors.set_password("passwords do not match", None);
                Err(errors)
            } else {
                Ok(None)
            }
        },
    )]
    const _: () = ();
}
```

- `const _: () = ()` allows duplicates and avoids inventing names.
- Multiple attributes of the same kind may be stacked on one const, or spread across multiple consts.
- The macro ignores the const body and type; only the attributes matter.

## 4. Field attributes map to the current builder API, with concise shorthands where possible

Each field is declared with a type and annotated with field-type attributes (`#[required]`, `#[lax]`, `#[constant]`, `#[depends_on(...)]`, `#[ivo_virtual]`) plus behavior attributes (`#[validate]`, `#[re_validate]`, `#[resolve]`, `#[sanitize]`, `#[default]`, etc.). `#[depends_on(...)]` both marks a field as dependent and declares its parent fields; `#[dependent]` is optional and accepted as an alias.

Because a constant field always requires a value, the field-type attribute and the value are combined into one:

```rust
#[constant(|| Uuid::new_v4())]
id: Uuid,
```

instead of the more verbose:

```rust
#[constant]
#[value(|| Uuid::new_v4())]
id: Uuid,
```

Lax fields, which commonly have defaults, may also use an inline default in the field-type attribute:

```rust
#[lax("user")]
role: String,
```

while still supporting a bare `#[lax]` when no default is needed.

Virtual fields may similarly declare an alias inline:

```rust
#[ivo_virtual("raw_email")]
email: String,
```

instead of a separate `#[alias("raw_email")]` attribute.

Example schema with both input and output structs:

```rust
#[ivo_schema(
    input(UserInput, derive(Debug, Clone, PartialEq), derive_partial(Deserialize)),
    output(User, derive(Debug, Clone, PartialEq), derive_partial(Serialize)),
)]
mod user_schema {
    struct Fields {
        #[required]
        #[validate(async |name, _ctx, _opts| { Ok(Some(name)) })]
        name: String,

        #[lax("user")]
        role: String,

        #[required]
        age: i32,

        #[constant(|| Uuid::new_v4())]
        id: Uuid,

        #[depends_on("age")]
        #[resolve(async |ctx, _opts| { format!("{}", ctx.values().age.unwrap_or(0)) })]
        age_label: String,

        #[ivo_virtual("raw_email")]
        #[sanitize(async |email, _ctx, _opts| { email.to_lowercase() })]
        email: String,

        #[depends_on("email")]
        #[resolve(async |ctx, _opts| { ctx.input().email.clone().unwrap() })]
        raw_email: String,
    }

    #[ignore(["secret"], async |_ctx, _opts| { true })]
    const _: () = ();
}
```

The macro generates:

```rust
#[derive(IvoInputStruct)]
pub struct UserInput {
    pub name: String,
    pub role: String,
    pub age: i32,          // dependency of "age_label"
    pub raw_email: String, // virtual alias; the virtual field "email" is not on input
}

#[derive(IvoStruct)]
pub struct User {
    pub name: String,
    pub role: String,
    pub age: i32,
    pub id: Uuid,          // constant
    pub age_label: String, // dependent
    pub raw_email: String, // dependent field that depends on the virtual "email"
}
```

## 5. Defaults support static, sync, async, and context-aware forms

The `#[default(...)]` attribute accepts all of these forms:

| Syntax                                    | Semantics                        |
| ----------------------------------------- | -------------------------------- |
| `#[default(expr)]`                        | Static default value             |
| `#[default(\|\| expr)]`                   | Sync resolver, no context        |
| `#[default(async \|\| expr)]`             | Async resolver, no context       |
| `#[default(async \|ctx, opts\| { ... })]` | Async resolver with full context |

The same forms apply to `#[value(...)]` for constants.

## 6. Eliminate `Box<dyn CloneableAny>` from the hot path

The long-term goal of the macro-generated schema is to move values as concrete types through the entire create/update/delete pipeline.

- `TypedFieldConfig<T>` stores validators/resolvers/defaults typed as `T`, not `ErasedValue`.
- Generated `validate_*` methods access partial structs by concrete field names (`self.name`) instead of `ivo_internal_set(..., &str)`.
- `ErasedValue` may remain for dynamic edge cases or interop, but the generated schema path should not allocate per field.

## 7. Keep `#[derive(IvoStruct)]` and `#[derive(IvoInputStruct)]`

The existing derive macros for input/output structs are retained, but they are applied by the `#[ivo_schema]` macro to the generated structs rather than written by hand. They still generate the partial structs and the methods the new schema model consumes.

Generated partial structs include utility methods such as `new()`, `is_empty()`, `into_option()`, and per-field `set_*`, `with_*`, and `unset_*` methods. `IvoStruct` also provides `append_updates` for merging a partial update back into a full output struct.

## 8. Grouped options are collected, not chained

The builder chain pattern `.ignore(...).post_validate(...).on_success(...)` is replaced by collecting all grouped option attributes on anonymous const items directly inside the schema module. The macro emits typed config lists for each option category.

## 9. Field groups reference declared fields by name

Options that target fields (`ignore`, `ignore_update`, `required`, `post_validate`, `on_success`) receive an array of field names. The macro validates at compile time that every referenced field exists in `struct Fields`.

`#[timestamps]` is a schema-level option that only provides the timestamp resolver; the actual timestamp fields (`#[created_at]`, `#[updated_at]`) are declared in the `fields` module like any other output field.

## 10. No required unique identifiers unless useful

When a group needs a stable identifier for error messages or debug output, an optional named const may be used:

```rust
#[required(["name", "email"], async |_ctx, _opts| { ... })]
const NAME_EMAIL_REQUIRED: () = ();
```

This is optional; `const _: () = ()` is the default.

## 11. Input/output structs are generated from the schema

The macro decides which fields belong on the input struct and which belong on the output struct based on field type:

| Field type                     | Input struct | Output struct               |
| ------------------------------ | ------------ | --------------------------- |
| `#[required]`, `#[lax]`        | field name   | field name                  |
| `#[constant]`                  | —            | field name                  |
| `#[depends_on(...)]`           | —            | field name                  |
| `#[ivo_virtual]` without alias | field name   | —                           |
| `#[ivo_virtual("X")]`          | alias `X`    | —                           |
| `#[created_at]`                | —            | field name                  |
| `#[updated_at]`                | —            | field name or `Option<...>` |

Single-struct mode:

If the schema contains **only** `#[required]` and/or `#[lax]` fields and no timestamp fields, there are no input-only fields and no output-only fields. The input struct is reused as the output struct. Only `input(...)` is required; `output(...)` must not be provided.

```rust
#[ivo_schema(input(User, derive(Debug, Clone)))]
mod user_schema {
    struct Fields {
        #[required]
        name: String,

        #[lax("user")]
        role: String,
    }
}
```

The macro derives both `IvoInputStruct` and `IvoStruct` on `User` and uses it for both input and output. This avoids generating the same impls twice.

Dual-struct mode:

If the schema contains any input-only field (`#[ivo_virtual]`) or output-only field (`#[constant]`, `#[depends_on(...)]` / `#[dependent]`, or timestamp fields), both `input(...)` and `output(...)` are required. Providing only `input(...)` produces a compile error.

Additional rules:

- A virtual field without an alias must be referenced by at least one dependent field's `#[depends_on(...)]`.
- A virtual alias must name a dependent field, and that dependent field must depend on the virtual field.
- Constant, dependent, and timestamp field names cannot appear on the input struct.
- Timestamp field names cannot be reused as field names or aliases.
- The timestamp resolver is declared once inside the schema module via `#[timestamps(|| Utc::now())]` on an anonymous const item.
- A schema may declare zero or one `#[created_at]` fields and zero or one `#[updated_at]` fields.
- An `#[updated_at]` field typed as `Option<T>` is treated as optional (set only when a value is already present on update); an `#[updated_at]` field typed as `T` is always set on create/update.

## 12. Field-type attributes are whitelisted at compile time

The macro enforces that each field type only accepts the attributes that the existing builder API supports. For example:

- `#[constant(value_or_resolver)]` accepts `#[on_delete]`, `#[on_success]`; it rejects bare `#[constant]`, `#[value]`, `#[validate]`, `#[on_failure]`, `#[default]`, `#[ignore_init]`. `#[on_delete]` and `#[on_success]` may be provided multiple times.
- `#[required]` accepts `#[validate]`, `#[re_validate]`, `#[required_error]`, `#[ignore_update]`, `#[readonly]`, `#[on_delete]`, `#[on_success]`, `#[on_failure]`; it rejects `#[default]`, `#[ignore_init]`, `#[required]`, `#[ignore]`, `#[sanitize]`, `#[alias]`. A validator is optional (but `#[re_validate]` still requires `#[validate]`). `#[readonly]` disallows all updates. Lifecycle hooks may be provided multiple times.
- `#[depends_on(...)]` / `#[dependent]` accepts `#[resolve]`, `#[default]`, `#[readonly]`, `#[on_delete]`, `#[on_success]`; it rejects `#[validate]`, `#[re_validate]`, `#[on_failure]`, `#[ignore_init]`, `#[ignore_update]`, `#[ignore]`, `#[sanitize]`, `#[alias]`. `#[readonly]` requires a static `#[default]`. `#[on_delete]` and `#[on_success]` may be provided multiple times.
- `#[ivo_virtual]` / `#[ivo_virtual("alias_name")]` accepts `#[sanitize]`, `#[validate]`, `#[re_validate]`, `#[required]`, `#[ignore]`, `#[ignore_init]`, `#[ignore_update]`, `#[on_success]`, `#[on_failure]`; it rejects `#[alias]`, `#[on_delete]`, `#[default]`, `#[value]`, `#[readonly]`. A validator is optional (but `#[re_validate]` still requires `#[validate]`). `#[on_success]` and `#[on_failure]` may be provided multiple times.
- `#[lax]` / `#[lax(default_or_resolver)]` accepts `#[validate]`, `#[re_validate]`, `#[required]`, `#[ignore]`, `#[ignore_init]`, `#[ignore_update]`, `#[readonly]`, `#[on_delete]`, `#[on_success]`, `#[on_failure]`; it rejects `#[default]`, `#[required_error]`, `#[value]`, `#[resolve]`, `#[sanitize]`, `#[alias]`. `#[readonly]` requires a static `#[lax(...)]` default. `#[re_validate]` requires `#[validate]`. Lifecycle hooks may be provided multiple times. A lax field without a validator is still considered provided and copied to the output.
- `#[created_at]` / `#[updated_at]` reject all field attributes. Their type must match the timestamp resolver's return type (`T` or `Option<T>` for `#[updated_at]`).

Standard Rust visibility keywords (`pub`, `pub(crate)`, `pub(super)`, etc.) written before the field name are accepted on all field types and emitted unchanged on the generated struct fields.

Field-level attributes other than lifecycle hooks (`#[on_delete]`, `#[on_success]`, `#[on_failure]`) may not be repeated on the same field. Lifecycle hooks may be provided multiple times. When a disallowed attribute is used, the macro emits `compile_error!` with a clear message such as:

```text
error: `#[validate]` is not allowed on `#[constant]` fields
```

This replaces the runtime panics or silent no-ops that can occur with the current builder API when methods are called in the wrong order or on the wrong field type.

## 13. Generated structs and fields inherit visibility from the schema module and field declarations

Since `#[ivo_schema]` now generates the input/output structs, users need control over their visibility.

- **Generated struct visibility** matches the visibility of the `#[ivo_schema]` module:
  - `pub mod user_schema { ... }` emits `pub struct UserInput` / `pub struct User`.
  - `mod user_schema { ... }` emits structs with no visibility keyword (private to the containing module).
- **Generated field visibility** matches exactly what the user writes before the field name. A bare field is emitted as a private field; `pub`, `pub(crate)`, `pub(super)`, etc. are emitted unchanged:
  - `#[required] name: String` → `name: String` (private).
  - `#[required] pub name: String` → `pub name: String`.
  - `#[required] pub(crate) internal_id: String` → `pub(crate) internal_id: String`.
  - `#[lax] pub(super) secret: String` → `pub(super) secret: String`.
- **Timestamp fields** are declared like normal output fields with `#[created_at]` or `#[updated_at]`, so their visibility is set the same way as any other field. The shared timestamp resolver is declared once inside the schema module via `#[timestamps(|| Utc::now())]` on an anonymous const item.
- **Partial structs, error structs, and the generated schema model** inherit the visibility of their corresponding input/output struct.

```rust
pub mod user_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[required]
        pub(crate) tenant_id: Uuid,

        #[lax]
        internal_note: Option<String>, // private field in a public struct

        #[created_at]
        pub(crate) created_at: Utc,

        #[updated_at]
        pub updated_at: Option<Utc>, // optional updated_at: only set if already present
    }

    #[timestamps(|| Utc::now())]
    const _: () = ();
}
```

## 14. Custom derives and passthrough attributes on generated structs and fields

Users can attach derives to the generated input/output/partial structs and arbitrary attributes to individual generated fields. The macro emits these attributes verbatim on the corresponding generated item.

### Schema-level derives

Use `derive(...)` and `derive_partial(...)` inside the top-level `input(...)` / `output(...)` arguments. `derive(...)` targets the generated input/output struct; `derive_partial(...)` targets the corresponding generated partial struct (`PartialUserInput` / `PartialUser`).

```rust
#[ivo_schema(
    input(
        UserInput,
        derive(Debug, Clone, PartialEq),
        derive_partial(Deserialize),
    ),
    output(
        User,
        derive(Debug, Clone, PartialEq),
        derive_partial(Serialize),
    ),
)]
mod user_schema { ... }
```

- The macro still applies `#[derive(IvoInputStruct)]` and `#[derive(IvoStruct)]` automatically; user-provided derives are appended.
- `derive_partial(...)` maps directly to the existing `#[ivo(derive(...))]` behavior of `IvoInputStruct` / `IvoStruct`: the derive is applied to the generated partial struct only.
- Non-derive struct attributes (e.g. `repr(C)`) are not supported at the schema level unless explicitly added later.

### Field-level passthrough attributes

On field declarations inside the `fields` module, use `#[input(...)]`, `#[output(...)]`, `#[partial(...)]`, `#[input_partial(...)]`, or `#[output_partial(...)]` to pass attributes through to the generated field on the corresponding struct(s).

```rust
struct Fields {
    #[required]
    #[input(serde(rename = "user_name"))]
    #[output(serde(skip_serializing_if = "Option::is_none"))]
    #[partial(serde(default = "default_name"))]
    name: String,
}
```

| Attribute                | Target generated field(s)                      |
| ------------------------ | ---------------------------------------------- |
| `#[input(...)]`          | Input struct field                             |
| `#[output(...)]`         | Output struct field                            |
| `#[partial(...)]`        | Both `PartialInput` and `PartialOutput` fields |
| `#[input_partial(...)]`  | `PartialInput` field only                      |
| `#[output_partial(...)]` | `PartialOutput` field only                     |

The macro emits the inner attributes verbatim on the corresponding generated struct field. The `IvoInputStruct` and `IvoStruct` derives then propagate partial-targeting attributes to the generated partial structs.

### Compile-time validation

Unknown or disallowed passthrough attributes are not validated by the macro; they are emitted as-is and left for the downstream derive or the Rust compiler to interpret. However, the macro must ensure that:

- `#[input(...)]` is not used on a field that does not exist on the input struct (e.g. `#[constant]`, `#[depends_on(...)]` / `#[dependent]`, output-only timestamp fields).
- `#[output(...)]` is not used on a field that does not exist on the output struct (e.g. a `#[ivo_virtual]` field without an alias).
- `#[input_partial(...)]` is not used on a field that does not have a `PartialInput` counterpart.
- `#[output_partial(...)]` is not used on a field that does not have a `PartialOutput` counterpart.

Violations produce a `compile_error!` such as:

```text
error: `#[input(...)]` cannot be applied to `#[constant]` field `id`, which is not present on the input struct
```

## 15. Sync/async handler inference

The macro determines whether each handler is synchronous or asynchronous at compile time and uses that information to generate the most efficient schema implementation. No runtime checks or extra `*_sync` APIs are needed.

### Handler classification

| Handler form                          | Classification |
| ------------------------------------- | -------------- |
| Closure returning a plain value       | **Sync**       |
| Async closure `async \|...\| { ... }` | **Async**      |
| Function item `handler_name`          | **Sync**       |
| Function item `async handler_name`    | **Async**      |

Examples:

```rust
struct Fields {
    // Sync validator
    #[validate(|name, _ctx, _opts| Ok(Some(name)))]
    name: String,

    // Async validator
    #[validate(async |email, _ctx, _opts| { validate_email(email).await })]
    email: String,

    // Sync resolver
    #[resolve(|ctx, _opts| format!("{}", ctx.values().age.unwrap_or(0)))]
    age_label: String,

    // Async function-item resolver
    #[resolve(async resolve_full_name)]
    full_name: String,
}
```

### Method-level consequence

The macro classifies every handler in the schema, including validators, re-validators, sanitizers, resolvers, defaults, value resolvers, timestamp resolvers, grouped-option handlers, and lifecycle hooks (`on_success`, `on_failure`, `on_delete`).

However, the sync/async nature of `create`/`update` is determined only by the handlers they **directly invoke**, not by `on_success` and `on_failure` handlers. `on_success` and `on_failure` are returned to the caller as separate handler triggers; their sync/async nature is determined by their own handlers.

| Method   | Handlers that determine its sync/async nature                                                                                                                                                            |
| -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `create` | Validators, sanitizers, defaults, value resolvers, resolvers, timestamp resolver, grouped options such as `ignore` / `required` / `post_validate` — everything **except** `on_success` and `on_failure`. |
| `update` | Same as `create`.                                                                                                                                                                                        |
| `delete` | `on_delete` handlers only (invoked directly inside `delete`).                                                                                                                                            |

If all relevant handlers are sync, the method is generated as a plain function; if any relevant handler is async, it is generated as `async fn`. The exact return type is an implementation detail, but it carries the operation result and any handler trigger(s) the caller must invoke.

In the mixed case, sync handlers are called directly without `BoxFuture` or `join_all` overhead; only async handlers go through the future path.

### Returned handler triggers

`create` and `update` return `Result<IvoSuccessHandle<...>, IvoFailureHandle<...>>`. The success handle exposes `handle_success` only when the schema declares at least one `on_success` handler. The failure handle exposes `handle_failure` only when the schema declares at least one `on_failure` handler. The sync/async nature of each method is determined only by the `on_success` / `on_failure` handlers it wraps:

```rust
// Core handlers are sync, so create is sync.
let handle = model.create(input)?;

// on_success handlers are async, so the trigger method is async.
handle.handle_success().await;
```

On failure, the returned handle carries a failure trigger instead:

```rust
let handle = model.create(input).unwrap_err();
handle.handle_failure().await;
```

This keeps the core create/update path free of async overhead when possible, while still allowing async side-effect hooks.

### Performance motivation

This design directly addresses the "specialize synchronous handlers" optimization opportunity. In sync-only schemas:

- No `BoxFuture` is allocated per field.
- No `join_all` machinery is invoked.
- No executor polling is needed for trivial `ready(...)` futures.

### Constraints

- Async handlers must use explicit `async` syntax. A closure that calls an async function but omits `.await` (and the surrounding `async` block) is classified as sync and will fail to compile against the generated sync signature.
- Function-item handlers use the `async fn_name` marker to indicate asynchrony; a bare `fn_name` is always treated as sync.

## 16. Model method signatures and data-flow philosophy

This section documents the generated methods on the schema model. `create` and `update` return `Result<IvoSuccessHandle<...>, IvoFailureHandle<...>>`. The concrete handle types are generated by the macro.

The core data-flow philosophy of Ivo is:

- **Create** moves from a partial description of the desired entity to a complete output: `PartialInput → Output`.
- **Update** moves from an existing complete output plus a partial set of changes to a partial output describing what actually changed: `Output + PartialInput → PartialOutput`.

### `create`

Conceptually, `create` accepts the partial input:

```rust
pub fn create(
    &self,
    input: PartialUserInput,
    options: UserCtxOptions,
) -> Result<
    IvoSuccessHandle<User, UserCtxOptions, IS_ASYNC>,
    IvoFailureHandle<ErrorSanitizer::Payload, UserCtxOptions, IS_ASYNC>,
>
```

For ergonomics, the generated method actually accepts `impl Into<PartialUserInput>`. The macro also emits an `Into<PartialUserInput>` implementation for the full `UserInput` struct, so callers may pass either `PartialUserInput` or `UserInput`. Passing `UserInput` is equivalent to converting it into a `PartialUserInput` first.

```rust
// These are equivalent:
let handle = model.create(PartialUserInput { name: Some("Ada".into()), ..Default::default() }, ())?;
let handle = model.create(UserInput { name: "Ada".into() }, ())?;
```

- If any directly-invoked handler is async, the method becomes `async fn`.
- Directly-invoked handlers include validators, re-validators, sanitizers, defaults, resolvers, timestamp resolver, and grouped options such as `ignore`, `required`, and `post_validate`.
- `on_success` / `on_failure` handlers do **not** make `create` async; they determine the sync/async nature of `handle_success` / `handle_failure`.

### `update`

```rust
pub fn update(
    &self,
    existing: User,
    updates: PartialUserInput,
    options: UserCtxOptions,
) -> Result<
    IvoSuccessHandle<PartialUser, UserCtxOptions, IS_ASYNC>,
    IvoFailureHandle<Option<ErrorSanitizer::Payload>, UserCtxOptions, IS_ASYNC>,
>
```

- `update` takes the existing `User` (the complete output) and a `PartialUserInput` of desired changes.
- It returns a `PartialUser` containing only the output fields whose values actually changed, i.e. `Output + PartialInput → PartialOutput`.
- The error payload is `Option<ErrorSanitizer::Payload>`:
  - `Some(payload)` means validation failed and `payload` contains the validation errors.
  - `None` means the update was rejected because nothing would change (the “nothing to update” case).

### `delete`

```rust
pub fn delete(&self, data: &User, options: UserCtxOptions)
```

- `delete` is generated only when the schema declares at least one field-level or schema-level `on_delete` handler.
- It invokes all such handlers directly and returns `()`.
- It is async if any `on_delete` handler is async.

### Handler triggers

`IvoSuccessHandle` and `IvoFailureHandle` wrap the operation result and any captured `on_success` / `on_failure` triggers. Call `handle_success` to invoke success triggers when `on_success` handlers exist; call `handle_failure` to invoke failure triggers when `on_failure` handlers exist. The handle methods consume `self` and return `()`; if any wrapped handler is async the method is `async`.

The sync/async nature of each method is determined only by the `on_success` / `on_failure` handlers it wraps:

- If **all** `on_success` handlers are sync, `handle_success` is a sync method returning `()`.
- If **any** `on_success` handler is async, `handle_success` is an `async` method returning `()`.
- The same rule applies to `handle_success` / `handle_failure`: each is only available when at least one corresponding handler exists.

```rust
// Core handlers are sync, so create is sync.
let handle = model.create(input)?;

// on_success handlers are async, so the trigger method is async.
handle.handle_success().await;
```

If all `on_success` handlers were sync, the call would be synchronous:

```rust
let handle = model.create(input)?;
handle.handle_success(); // no .await
```

On failure, the returned handle carries a failure trigger instead:

```rust
let handle = model.create(input).unwrap_err();
handle.handle_failure().await;
```

When a schema has no `on_success` handlers, `handle_success` is not present on `IvoSuccessHandle`. When a schema has no `on_failure` handlers, `handle_failure` is not present on `IvoFailureHandle`.

## 17. Execution pipeline

`create` and `update` execute the following phases in order. The macro omits any phase that does not apply to the schema, which eliminates runtime branches and dead code for simple schemas.

Every phase below that can produce an `errors`/field-error entry is followed
immediately by a fail-fast check: if any error was recorded, `create`/
`update` return right away rather than letting later phases run against
already-invalid data. Where a phase says "batched", every independent
handler in it (across every field it applies to, and across
required/lax/virtual fields together where noted) is resolved in one go via
`emit_async_phase`: synchronously if 0/1 of them is async, concurrently via
`join!` if 2+ are.

### Create

1. Ignore phase: evaluate `ignore`/`ignore_init` (batched: entity-level,
   field-level, and grouped resolvers together), then apply `ignore_init`
   overrides.
2. Required phase: evaluate conditional `#[required]` (batched, same scope as
   step 1) and check bare `#[required]` fields for a missing value. *Fail
   fast.*
3. Validate fields (`#[validate]`) -- required/lax and virtual fields'
   validators run as one combined batched phase, not two separate passes.
   Defaults for unset `#[lax]` fields are applied as part of this same step.
   *Fail fast.*
4. Re-validate fields (`#[re_validate]`) -- required/lax and virtual merged
   the same way as step 3, gated on each field having successfully validated.
   *Fail fast.*
5. Post-validate, `pre_validate` handlers only, all `#[post_validate]` groups
   batched together against a snapshot from before this phase. *Fail fast.*
6. Post-validate, main `validate` handlers, same batching -- does not run at
   all if step 5 produced any error. *Fail fast.*
7. Sanitize virtual fields (`#[sanitize]`) -- only for virtual fields that
   were provided and not ignored; runs only once step 6 has succeeded.
8. Resolve dependent fields (`#[resolve]`), one round per dependency-graph
   level, looping until no new changes; each round's independent resolvers
   are batched via `emit_async_phase`.
9. Attach constants (`#[constant]`) -- runs after dependent resolution so
   constant resolvers can read resolved dependent values via `ctx.values()`.
10. Attach timestamps (`#[created_at]` / `#[updated_at]`) -- after constants;
    the resolver is called at most once and shared across both fields.
11. Prepare success / failure triggers.

### Update

1. Ignore phase: evaluate `ignore`/`ignore_update` (batched, same scope as
   create's step 1), then apply bare `#[ignore_update]` overrides.
2. Nothing-to-update checkpoint 1: if no `#[required]`/`#[lax]`/virtual field
   actually present in the update survives ignore/`#[readonly]` filtering,
   fail immediately with "nothing to update" (before the required check ever
   runs).
3. Evaluate missing required fields (conditional `#[required]` only -- bare
   `#[required]` is creation-only). *Fail fast.*
4. Validate fields (`#[validate]`), batched like create's step 3. *Fail fast.*
5. Re-validate fields (`#[re_validate]`), batched like create's step 4.
   *Fail fast.*
6. Post-validate: `pre_validate` then main `validate`, batched and gated the
   same way as create's steps 5-6, against a scratch `input` seeded from
   only the fields each group covers. *Fail fast.*
7. Evaluate update validity: recompute `changes` and strip any field whose
   value turned out unchanged from both `changes` and `input` (matching
   `rs/`'s `evaluate_update_validity`; `raw_input()` still shows what was
   submitted). Runs once, right after post-validate -- not after every
   phase. *Fail fast.*
8. Nothing-to-update checkpoint 2: if nothing is left in `changes` after step
   7 *and* no virtual field is still relevant (its dependent(s) haven't
   resolved yet), fail immediately, before dependent resolution runs.
9. Sanitize virtual fields (`#[sanitize]`), same condition as create's step 7.
10. Resolve dependent fields (`#[resolve]`), one round per dependency-graph
    level (a single topological pass, not a convergence loop, since update's
    dependency guards read from `updates`/`__original_output` directly).
11. Nothing-to-update checkpoint 3: if `changes` is still empty after
    dependent resolution, fail with "nothing to update".
12. Attach timestamps (`#[updated_at]`).
13. Prepare success / failure triggers.

### Dynamic pipeline generation

Because the macro has full knowledge of the schema, it generates only the code paths that are actually needed. For example:

- A schema with no `#[constant]` fields does not generate constant attachment.
- A schema with no `#[default]` attributes (i.e. no dependent and no lax fields) does not generate step to resolve default values.
- A schema with no `#[depends_on]` fields does not generate the dependent-resolution loop.
- A schema with no `#[validate]` attributes does not generate validation logic.
- A schema with no `#[re_validate]` attributes does not generate re-validation logic.
- A schema with no `#[sanitize]` attributes (i.e. no virtual with `#[sanitize]`) does not generate sanitization logic.
- A schema with no required fields and no conditional required fields or grouped required options does not generate logic to evaluate missing required fields.
- A schema with no `ignore`, `ignore_init`, `ignore_update` on either fields or options should not generate the logic for those checks.
- A schema with no `post_validate` option should not generate the logic for those checks.
- A schema with no timestamps does not generate timestamp attachment.

This is a key performance advantage of the macro-driven design: the generated `create`/`update` methods are specialized to the schema rather than being generic interpreters.

## 18. Field attribute matrix

The following matrix lists every field-level attribute, the field types it may appear on, its signature or form, and its semantics.

| Attribute                    | Allowed on                                | Signature / form                                          | Notes                                                                                                                                                               |
| ---------------------------- | ----------------------------------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `#[required]`                | `#[lax]`, `#[ivo_virtual]`                | `async \|ctx, opts\| { Option<String> }`                  | Conditional required check. Distinct from the `#[required]` field type.                                                                                             |
| `#[required_error(...)]`     | `#[required]`                             | static string **or** `\|raw_input, opts\| -> String`      | Error when field is missing at create. Accepts a static string or a closure for dynamic messages.                                                                   |
| `#[ignore]`                  | `#[lax]`, `#[ivo_virtual]`                | `\|ctx, opts\| -> bool`                                   | Must be conditional; bare `#[ignore]` is rejected. Applies to both create and update. Not allowed with `#[ignore_init]` or `#[ignore_update]` on the same field.    |
| `#[ignore_init]`             | `#[lax]`, `#[ivo_virtual]`                | none                                                      | Ignore field during create only. Resolver form is rejected. Not allowed with `#[ignore]` or bare `#[ignore_update]`; allowed with `#[ignore_update(\|\|resolver)]`. |
| `#[ignore_update]`           | `#[required]`, `#[lax]`, `#[ivo_virtual]` | bare or `\|partial_input, full_output, rw_opts\| -> bool` | On `#[required]` only the resolver form is allowed (bare is rejected; use `#[readonly]`). Not allowed with `#[readonly]`, `#[ignore]`, or bare `#[ignore_init]`.    |
| `#[readonly]`                | `#[required]`, `#[lax]`, `#[dependent]`   | none                                                      | Required: no updates allowed. Lax/dependent: update only if current value equals static default. Cannot be combined with `#[ignore_update]`.                        |
| `#[validate(...)]`           | `#[required]`, `#[lax]`, `#[ivo_virtual]` | `\|value, ctx, opts\| -> Result<Option<T>, ...>`          | Primary validation. `Ok(None)` uses the input as-is; `Ok(Some(value))` replaces it.                                                                                 |
| `#[re_validate(...)]`        | `#[required]`, `#[lax]`, `#[ivo_virtual]` | same as validate                                          | Secondary validation after primary. Only allowed when `#[validate]` is also present.                                                                                |
| `#[sanitize(...)]`           | `#[ivo_virtual]`                          | `\|value, ctx, opts\| -> T`                               | Mutates virtual input value. Runs after validate/re-validate/post_validate. Virtual values do not appear on the output partial.                                     |
| `#[resolve(...)]`            | `#[dependent]`                            | `\|ctx, opts\| -> T`                                      | Resolver run when any parent changes.                                                                                                                               |
| `#[default(...)]`            | `#[dependent]`; also inline for `#[lax]`  | static or resolver                                        | Static default or context-aware resolver. For `#[lax]`, defaults attach only when the field is missing; for `#[dependent]`, defaults always attach if configured.   |
| `#[value(...)]`              | `#[constant]`                             | static or resolver                                        | Static value or context-aware resolver.                                                                                                                             |
| `#[depends_on(...)]`         | `#[dependent]`                            | list of string literals (field names)                     | Required; at least one parent.                                                                                                                                      |
| `#[ivo_virtual("alias_name")]` | `#[ivo_virtual]`                        | string literal                                             | Alias name for the generated input field; a dependent field may depend on the virtual via either the alias or the declared field name.                              |
| `#[on_delete]`               | per §12 whitelist                         | `\|ctx, opts\| -> ()`                                     | Lifecycle hook invoked directly by `delete`. May be provided multiple times.                                                                                        |
| `#[on_success]`              | per §12 whitelist                         | `\|ctx, opts\| -> ()`                                     | Returned as a trigger from `create`/`update`. May be provided multiple times.                                                                                       |
| `#[on_failure]`              | per §12 whitelist                         | `\|ctx, opts\| -> ()`                                     | Returned as a trigger from `create`/`update`. May be provided multiple times.                                                                                       |

Field-level attributes other than lifecycle hooks (`#[on_delete]`, `#[on_success]`, `#[on_failure]`) may not be repeated on the same field.

### Ignore attribute combinations

The following combinations are enforced at compile time:

- `#[ignore]` may not appear on the same field as `#[ignore_init]` or `#[ignore_update]`.
- Bare `#[ignore_update]` may not appear on the same field as `#[ignore_init]`; use `#[ignore]` instead.
- `#[ignore_update]` may not appear on the same field as `#[readonly]` (on `#[required]` or `#[lax]` fields).
- `#[ignore_init]` may be combined with `#[readonly]` on `#[lax]` fields.
- `#[ignore_init]` may be combined with `#[ignore_update(||resolver)]` on `#[lax]` / `#[ivo_virtual]` fields.
- `#[ignore(...)]` may be combined with `#[required(...)]` on the same `#[lax]` / `#[ivo_virtual]` field; both resolvers are evaluated independently.
- Field-level `#[ignore]` and grouped `#[ignore([...], ...)]` may reference the same field.

## 19. Schema options reference

Grouped options and their constraints.

| Option                                                        | Min fields   | Allowed field types                                                             | Notes                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ------------------------------------------------------------- | ------------ | ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `#[ignore([...], \|ctx, opts\| ...)]`                         | ≥2           | `#[lax]`, `#[ivo_virtual]`                                                      | Skip fields when resolver returns true (applies to both create and update).                                                                                                                                                                                                                                                                                                                                                           |
| `#[ignore_update([...], \|partial, full, opts\| ...)]` / `#[ignore_update(\|partial, full, opts\| ...)]` | ≥2 with an array; entity-level form omits the array entirely | `#[required]`, `#[lax]`, `#[ivo_virtual]`                                       | Skip fields during update only. `#[ignore_update([...], handler)]` requires at least *two* fields -- unlike `#[on_success]`/`#[on_delete]`, a single-field array is rejected (use the field's own `#[ignore_update(...)]` option instead of grouping just one field), and an empty array is rejected too. To skip *every* field on update (a global update-ignore switch), omit the array and use the bare `#[ignore_update(handler)]` entity-level form instead.                                                                                                                                                                                                                                                                                                                                                        |
| `#[required([...], \|ctx, opts\| ...)]`                       | ≥2           | `#[lax]`, `#[ivo_virtual]`                                                      | Group requirement check. The handler is invoked only when **none** of the listed fields were provided; it returns `Option<{InputName}Errors>`. `Some(errors)` merges the per-field errors into the payload, `None` means the requirement does not apply.                                                                                                                                                                              |
| `#[post_validate([...], validate = ..., pre_validate = ...)]` | ≥2           | `#[lax]`, `#[required]`, `#[ivo_virtual]`                                       | Cross-field validation running after `re_validate` and during `update`. Both handlers return `Result<Option<PartialInput>, InputErrors>`; `Ok(Some(partial))` applies updates to the built output/input and refreshes the context, `Err(errors)` merges the errors into the returned payload. `pre_validate` runs before `validate` and can feed updated values into it. Errors are typed via a generated `{InputName}Errors` struct. |
| `#[on_success([...], \|b\| ...)]` / `#[on_success(\|b\| ...)]` | ≥1 with an array; entity-level form omits the array entirely | Output fields, including `#[constant]` / `#[dependent]`, but **not** timestamps | `#[on_success([...], handler)]` still requires at least one field -- an empty array is rejected. To always fire on success regardless of which fields changed, omit the array and use the bare `#[on_success(handler)]` entity-level form instead.                                                                                                                                                                                                                                                                                                                                                                                             |
| `#[on_delete([...], \|data, opts\| ...)]`                     | schema-level | —                                                                               | Invoked directly inside `delete`.                                                                                                                                                                                                                                                                                                                                                                                                     |
| `#[timestamps(\|\| ...)]`                                     | schema-level | —                                                                               | Resolver must be **synchronous**. Accepts either a zero-arg closure (`&#124;&#124; now()`) or a function path (`now`). The function-path shorthand is intentionally limited to this built-in lifecycle option; all other handlers continue to require closures.                                                                                                                                                                       |

Additional rules:

- Duplicate field names within a grouped option are rejected.
- Grouped option arrays must reference declared field names directly. Virtual aliases are **not** accepted; the user must reference the alias target field name instead.
- `#[on_success]` referencing a virtual alias is rejected with a compile error.

## 20. Context types and error sanitizer

Handlers receive different context/options types depending on their role.

| Handler                                                   | Context                | Options                       | Notes                                           |
| --------------------------------------------------------- | ---------------------- | ----------------------------- | ----------------------------------------------- |
| Validators, re-validators, sanitizers, resolvers          | `IvoContext<I, O>`     | `IvoRwCtxOptions<CtxOptions>` | Core handlers can mutate options.               |
| Lax / dependent defaults                                  | `IvoDefaultCtx<I>`     | `IvoRwCtxOptions<CtxOptions>` | Access to input and raw input.                  |
| Constant value resolvers                                  | `IvoConstantCtx<I, O>` | `IvoRwCtxOptions<CtxOptions>` | Access to input, raw input, and current values. |
| Lifecycle hooks (`on_success`, `on_failure`, `on_delete`) | `IvoContext<I, O>`     | `IvoCtxOptions<CtxOptions>`   | Read-only options.                              |

Both options wrappers are backed by `async-lock::RwLock` so guards can safely be held across `.await` points. Async handlers use `.read().await` / `.write().await`; sync handlers use `.read_sync()` / `.write_sync()`.

Mixing sync and async ctx_options access within one `create`/`update` call is safe by construction, not by accident: every phase is batched by `emit_async_phase` (`crates/derive/src/lib.rs`), which always runs every *sync* handler in a phase to completion first, sequentially, before the `join!` for that phase's *async* handlers even begins polling (this holds regardless of how many of each kind are present, and applies to lifecycle-hook triggers too, since `make_trigger` reuses the same function). So a sync handler's `.read_sync()`/`.write_sync()` and an async handler's `.read().await`/`.write().await` are never actually in flight at the same instant -- the sync call always runs against an uncontended lock. This guarantee only covers concurrency ivo itself orchestrates; if a handler spawns its own independent task (e.g. `tokio::spawn` without awaiting it) that touches the same options handle, that task is no longer sequenced by `emit_async_phase`, and mixing a blocking `.read_sync()`/`.write_sync()` call with such a task reintroduces the usual async-Rust hazard of blocking an executor thread another task needs freed to make progress -- a general async-Rust footgun, not specific to ivo, and on the caller to avoid.

### `IvoContext<I, O>` accessors

- `input()` — current partial input.
- `raw_input()` — original partial input.
- `values()` — current full output values being built (creation or update).
- `changes()` — `Option<&O::Partial>`, the subset of output values changed by this update; `None` at creation.
- `previous_values()` — `Option<&O>`, the record as it was before this update was applied; `None` at creation.
- `is_update()` — whether the context is for an update (equivalent to `previous_values().is_some()`).

### Error sanitizer trait

```rust
trait IvoErrorSanitizer<CtxOptions> {
    type Metadata: Clone + Send + Sync;
    type Payload;

    fn sanitize(
        payload: IvoErrorPayload<Self::Metadata>,
        ctx_options: &CtxOptions,
    ) -> Self::Payload;
}
```

- `Metadata` is attached to individual field errors by validators.
- `Payload` is the final error shape returned by `create`/`update`.
- `DefaultErrorSanitizer` uses `Metadata = ()` and `Payload = IvoErrorPayload<()>`.

## 21. Dependency graph validation rules

Rules enforced at schema build time:

- Every dependent field must declare at least one parent via `#[depends_on(...)]`. `#[dependent]` is optional; `#[depends_on(...)]` alone marks the field as dependent.
- Parent fields can be `#[required]`, `#[lax]`, `#[ivo_virtual]`, or other `#[dependent]` fields.
- Parents **cannot** be `#[constant]` or timestamp fields (`#[created_at]` / `#[updated_at]`).
- No circular dependencies.
- No redundant (transitive) dependencies. For example, if `a` depends on `[b, c]` and `b` depends on `[c]`, then `a` should not list `c`.
- A `#[ivo_virtual]` field must be referenced by at least one dependent field's `#[depends_on(...)]`, regardless of whether it has an alias.
- A `#[ivo_virtual("X")]` alias must be unique within the schema and must not collide with any field name or timestamp field name. It may match a dependent field's name only if that dependent field depends on the virtual field. The alias appears on the generated input struct in place of the virtual field name.
- Timestamp field names cannot be reused as field names or aliases.
- Duplicate `#[depends_on(...)]` entries and self-dependencies are rejected.

## 22. Readonly semantics

`#[readonly]` behaves differently depending on the field type:

- On `#[required]`: the field is never allowed in updates. It is stripped from update input before validation.
- On `#[lax(...)]` / `#[dependent]` with a static default: an update is allowed only if the current stored value equals the static default. The field's resolver runs only when this condition holds.

This distinction must be preserved in the macro-generated code.

## 23. Derive compatibility / passthrough mapping

The existing `IvoStruct` and `IvoInputStruct` derives understand passthrough attributes via `#[ivo(...)]`. The new field-level passthrough attributes (`#[input(...)]`, `#[output(...)]`, `#[partial(...)]`, `#[input_partial(...)]`, `#[output_partial(...)]`) are not natively recognized by the existing derives.

The `#[ivo_schema]` macro translates the new passthrough attributes into `#[ivo(...)]` before applying `#[derive(IvoStruct)]` or `#[derive(IvoInputStruct)]`. This keeps the existing derives unchanged and preserves a clean separation between schema syntax and partial-struct generation.
