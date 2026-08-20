# Ivo Rust v2 API Design Preferences

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
      mod fields { ... }

      mod options { ... }
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
      mod fields { ... }
  }
  ```

- The macro generates a `UserSchemaModel` (or similar) with typed field configs and generated `create`/`update`/`delete` methods.
- `input(...)` is always required and names the input struct. `output(...)` names the output struct and is required only when the schema has input-only fields (`#[virtual]`) or output-only fields (`#[constant]`, `#[dependent]`, `#[timestamps]`).
- `derive(...)` inside `input(...)` / `output(...)` adds derives to the generated input/output struct. `derive_partial(...)` adds derives to the corresponding generated partial struct (`PartialUserInput` / `PartialUser`).
- `ctx_options(UserCtxOptions)` and `error_sanitizer(ErrorSanitizer)` are optional runtime types provided by the user.

## 2. Two-section layout inside the schema module

- `mod fields { ... }` contains field declarations with attributes. It is required because every schema must declare at least one field.
- `mod options { ... }` contains grouped schema options. It is optional; omit it when the schema has no options.
- Both modules live inside the same `#[ivo_schema]` module.
- The macro identifies the field and options modules by their names (`fields` and `options`). No additional attribute markers are needed.

## 3. `const _: () = ()` is the preferred anchor for options

Inside the optional `options` module, each grouped option is attached to an anonymous const item:

```rust
mod options {
    #[ignore(["secret"], |_ctx, _opts| async move { true })]
    const _: () = ();

    #[required(["name", "email"], |_ctx, _opts| async move { ... })]
    const _: () = ();
}
```

- `const _: () = ()` allows duplicates and avoids inventing names.
- Multiple attributes of the same kind may be stacked on one const, or spread across multiple consts.
- The macro ignores the const body and type; only the attributes matter.

## 4. Field attributes map to the current builder API, with concise shorthands where possible

Each field is declared with a type and annotated with field-type attributes (`#[required]`, `#[lax]`, `#[constant]`, `#[dependent]`, `#[virtual]`) plus behavior attributes (`#[validate]`, `#[re_validate]`, `#[resolve]`, `#[sanitize]`, `#[default]`, etc.).

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
#[virtual(alias = "raw_email")]
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
    mod fields {
        #[required]
        #[validate(|name, _ctx, _opts| async move { Ok(Some(name)) })]
        name: String,

        #[lax("user")]
        role: String,

        #[required]
        age: i32,

        #[constant(|| Uuid::new_v4())]
        id: Uuid,

        #[depends_on(age)]
        #[resolve(|ctx, _opts| async move { format!("{}", ctx.values().age.unwrap_or(0)) })]
        age_label: String,

        #[virtual(alias = "raw_email")]
        #[sanitize(|email, _ctx, _opts| async move { email.to_lowercase() })]
        email: String,

        #[depends_on(email)]
        #[resolve(|ctx, _opts| async move { ctx.input().email.clone().unwrap() })]
        raw_email: String,
    }

    mod options { ... }
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

| Syntax                                         | Semantics                                                    |
| ---------------------------------------------- | ------------------------------------------------------------ |
| `#[default(expr)]`                             | Static default value                                         |
| `#[default(\|\| expr)]`                        | Sync resolver, no context (wrapped in `async move`)          |
| `#[default(async \|\| expr)]`                  | Async resolver, no context                                   |
| `#[default(\|ctx, opts\| async move { ... })]` | Async resolver with full context                             |
| `#[default(async \|ctx, opts\| { ... })]`      | Async resolver with full context (when compiler supports it) |

The same forms apply to `#[value(...)]` for constants.

## 6. Eliminate `Box<dyn CloneableAny>` from the hot path

The long-term goal of the macro-generated schema is to move values as concrete types through the entire create/update/delete pipeline.

- `TypedFieldConfig<T>` stores validators/resolvers/defaults typed as `T`, not `ErasedValue`.
- Generated `validate_*` methods access partial structs by concrete field names (`self.name`) instead of `ivo_internal_set(..., &str)`.
- `ErasedValue` may remain for dynamic edge cases or interop, but the generated schema path should not allocate per field.

## 7. Keep `#[derive(IvoStruct)]` and `#[derive(IvoInputStruct)]`

The existing derive macros for input/output structs are retained, but they are applied by the `#[ivo_schema]` macro to the generated structs rather than written by hand. They still generate the partial structs and the methods the new schema model consumes.

## 8. Grouped options are collected, not chained

The builder chain pattern `.ignore(...).post_validate(...).on_success(...)` is replaced by collecting all grouped option attributes in the optional `options` module. The macro emits typed config lists for each option category.

## 9. Field groups reference declared fields by name

Options that target fields (`ignore`, `ignore_update`, `required`, `post_validate`, `on_success`) receive an array of field names. The macro validates at compile time that every referenced field exists in the `fields` module.

`#[timestamps]` is a schema-level option that only provides the timestamp resolver; the actual timestamp fields (`#[created_at]`, `#[updated_at]`) are declared in the `fields` module like any other output field.

## 10. No required unique identifiers unless useful

When a group needs a stable identifier for error messages or debug output, an optional named const may be used:

```rust
#[required(["name", "email"], |_ctx, _opts| async move { ... })]
const NAME_EMAIL_REQUIRED: () = ();
```

This is optional; `const _: () = ()` is the default.

## 11. Input/output structs are generated from the schema

The macro decides which fields belong on the input struct and which belong on the output struct based on field type:

| Field type                 | Input struct | Output struct               |
| -------------------------- | ------------ | --------------------------- |
| `#[required]`, `#[lax]`    | field name   | field name                  |
| `#[constant]`              | —            | field name                  |
| `#[dependent]`             | —            | field name                  |
| `#[virtual]` without alias | field name   | —                           |
| `#[virtual(alias = "X")]`  | alias `X`    | —                           |
| `#[created_at]`            | —            | field name                  |
| `#[updated_at]`            | —            | field name or `Option<...>` |

Single-struct mode:

If the schema contains **only** `#[required]` and/or `#[lax]` fields and no timestamp fields, there are no input-only fields and no output-only fields. The input struct is reused as the output struct. Only `input(...)` is required; `output(...)` must not be provided.

```rust
#[ivo_schema(input(User, derive(Debug, Clone)))]
mod user_schema {
    mod fields {
        #[required]
        name: String,

        #[lax("user")]
        role: String,
    }
}
```

The macro derives both `IvoInputStruct` and `IvoStruct` on `User` and uses it for both input and output. This avoids generating the same impls twice.

Dual-struct mode:

If the schema contains any input-only field (`#[virtual]`) or output-only field (`#[constant]`, `#[dependent]`, or timestamp fields), both `input(...)` and `output(...)` are required. Providing only `input(...)` produces a compile error.

Additional rules:

- A virtual field without an alias must be referenced by at least one dependent field's `#[depends_on(...)]`.
- A virtual alias must name a dependent field, and that dependent field must depend on the virtual field.
- Constant, dependent, and timestamp field names cannot appear on the input struct.
- Timestamp field names cannot be reused as field names or aliases.
- The timestamp resolver is declared once in the `options` module via `#[timestamps(|| Utc::now())]`.
- A schema may declare zero or one `#[created_at]` fields and zero or one `#[updated_at]` fields.
- An `#[updated_at]` field typed as `Option<T>` is treated as optional (set only when a value is already present on update); an `#[updated_at]` field typed as `T` is always set on create/update.

## 12. Field-type attributes are whitelisted at compile time

The macro enforces that each field type only accepts the attributes that the existing builder API supports. For example:

- `#[constant(value_or_resolver)]` accepts `#[on_delete]`, `#[on_success]`; it rejects bare `#[constant]`, `#[value]`, `#[validate]`, `#[on_failure]`, `#[default]`, `#[ignore_init]`.
- `#[required]` accepts `#[validate]`, `#[re_validate]`, `#[required_error]`, `#[ignore_update]`, `#[readonly]`, `#[on_delete]`, `#[on_success]`, `#[on_failure]`; it rejects `#[default]`, `#[ignore_init]`, `#[required]`, `#[ignore]`, `#[sanitize]`, `#[alias]`. `#[readonly]` requires `#[validate]`.
- `#[dependent]` accepts `#[depends_on]`, `#[resolve]`, `#[default]`, `#[readonly]`, `#[on_delete]`, `#[on_success]`; it rejects `#[validate]`, `#[re_validate]`, `#[on_failure]`, `#[ignore_init]`, `#[ignore_update]`, `#[ignore]`, `#[sanitize]`, `#[alias]`. `#[readonly]` requires a static `#[default]`.
- `#[virtual]` / `#[virtual(alias = "...")]` accepts `#[sanitize]`, `#[validate]`, `#[re_validate]`, `#[required]`, `#[ignore]`, `#[ignore_init]`, `#[ignore_update]`, `#[on_success]`, `#[on_failure]`; it rejects `#[alias]`, `#[on_delete]`, `#[default]`, `#[value]`, `#[readonly]`.
- `#[lax]` / `#[lax(default_or_resolver)]` accepts `#[validate]`, `#[re_validate]`, `#[required]`, `#[ignore]`, `#[ignore_init]`, `#[ignore_update]`, `#[readonly]`, `#[on_delete]`, `#[on_success]`, `#[on_failure]`; it rejects `#[default]`, `#[required_error]`, `#[value]`, `#[resolve]`, `#[sanitize]`, `#[alias]`. `#[readonly]` requires a static `#[lax(...)]` default.
- `#[created_at]` / `#[updated_at]` reject all field attributes. Their type must match the timestamp resolver's return type (`T` or `Option<T>` for `#[updated_at]`).

Standard Rust visibility keywords (`pub`, `pub(crate)`, `pub(super)`, etc.) written before the field name are accepted on all field types and emitted unchanged on the generated struct fields.

When a disallowed attribute is used, the macro emits `compile_error!` with a clear message such as:

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
- **Timestamp fields** are declared like normal output fields with `#[created_at]` or `#[updated_at]`, so their visibility is set the same way as any other field. The shared timestamp resolver is declared once in the `options` module via `#[timestamps(|| Utc::now())]`.
- **Partial structs, error structs, and the generated schema model** inherit the visibility of their corresponding input/output struct.

```rust
pub mod user_schema {
    mod fields {
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

    mod options {
        #[timestamps(|| Utc::now())]
        const _: () = ();
    }
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
mod fields {
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

- `#[input(...)]` is not used on a field that does not exist on the input struct (e.g. `#[constant]`, `#[dependent]`, output-only timestamp fields).
- `#[output(...)]` is not used on a field that does not exist on the output struct (e.g. a `#[virtual]` field without an alias).
- `#[input_partial(...)]` is not used on a field that does not have a `PartialInput` counterpart.
- `#[output_partial(...)]` is not used on a field that does not have a `PartialOutput` counterpart.

Violations produce a `compile_error!` such as:

```text
error: `#[input(...)]` cannot be applied to `#[constant]` field `id`, which is not present on the input struct
```

## 15. Sync/async handler inference

The macro determines whether each handler is synchronous or asynchronous at compile time and uses that information to generate the most efficient schema implementation. No runtime checks or extra `*_sync` APIs are needed.

### Handler classification

| Handler form                                              | Classification |
| --------------------------------------------------------- | -------------- |
| Closure returning a plain value                           | **Sync**       |
| Closure with `async move { ... }` or `async { ... }` body | **Async**      |
| Function item `handler_name`                              | **Sync**       |
| Function item `async handler_name`                        | **Async**      |

Examples:

```rust
#[fields]
mod fields {
    // Sync validator
    #[validate(|name, _ctx, _opts| Ok(Some(name)))]
    name: String,

    // Async validator
    #[validate(|email, _ctx, _opts| async move { validate_email(email).await })]
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

`create` and `update` return success and failure handler triggers alongside their result. A trigger is a callable (closure or function) whose sync/async nature is determined only by the `on_success` / `on_failure` handlers it wraps:

```rust
// Core handlers are sync, so create is sync.
let (user, handle_success, opts) = model.create(input)?;

// on_success handlers are async, so the trigger is async.
handle_success().await?;
```

On failure, the returned error includes a failure trigger instead:

```rust
let (error, handle_failure, opts) = model.create(input).unwrap_err();
handle_failure().await?;
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
