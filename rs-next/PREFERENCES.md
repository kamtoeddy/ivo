# Ivo Rust v2 API Design Preferences

This document captures the user preferences for the next-generation Ivo Rust API (`rs-v2`). It is the source of truth for syntax and architectural decisions.

## 1. Schema generation must be macro-driven

Schemas are declared, not built imperatively. A single proc macro (`#[ivo_schema]`) receives the field definitions and schema options, then generates the input/output structs, their partial/error structs, and a typed, schema-specific model.

- **Dual-struct entry shape** (required when the schema has input-only or output-only fields):

  ```rust
  #[ivo_schema(input = UserInput, output = User)]
  mod user_schema {
      #[fields]
      mod fields { ... }

      #[options]
      mod options { ... }
  }
  ```

- **Single-struct entry shape** (allowed when the schema contains only `#[required]` and/or `#[lax]` fields and no `#[timestamps]`):

  ```rust
  #[ivo_schema(input = User)]
  mod user_schema {
      #[fields]
      mod fields { ... }
  }
  ```

- The macro generates a `UserSchemaModel` (or similar) with typed field configs and generated `create`/`update`/`delete` methods.
- `input = ...` always names the input struct. `output = ...` names the output struct and is required only when the schema has input-only fields (`#[virtual]`) or output-only fields (`#[constant]`, `#[dependent]`, `#[timestamps]`).

## 2. Two-section layout inside the schema module

- `#[fields] mod fields { ... }` contains field declarations with attributes.
- `#[options] mod options { ... }` contains grouped schema options.
- Both sections live inside the same `#[ivo_schema]` module.
- The names `fields` and `options` are conventional; the macro identifies them by the `#[fields]` / `#[options]` attributes.

## 3. `const _: () = ()` is the preferred anchor for options

Inside `#[options]`, each grouped option is attached to an anonymous const item:

```rust
#[options]
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
#[ivo_schema(input = UserInput, output = User)]
mod user_schema {
    #[fields]
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

        #[dependent(on = [age])]
        #[resolve(|ctx, _opts| async move { format!("{}", ctx.values().age.unwrap_or(0)) })]
        age_label: String,

        #[virtual(alias = "raw_email")]
        #[sanitize(|email, _ctx, _opts| async move { email.to_lowercase() })]
        email: String,

        #[dependent(on = [email])]
        #[resolve(|ctx, _opts| async move { ctx.input().email.clone().unwrap() })]
        raw_email: String,
    }

    #[options]
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

The builder chain pattern `.ignore(...).post_validate(...).on_success(...)` is replaced by collecting all grouped option attributes in the `#[options]` module. The macro emits typed config lists for each option category.

## 9. Field groups reference declared fields by name

Options that target fields (`ignore`, `ignore_update`, `required`, `post_validate`, `on_success`) receive an array of field names. The macro validates at compile time that every referenced field exists in `#[fields]`.

`#[timestamps]` is a schema-level option that only provides the timestamp resolver; the actual timestamp fields (`#[created_at]`, `#[updated_at]`) are declared in `#[fields]` like any other output field.

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

If the schema contains **only** `#[required]` and/or `#[lax]` fields and no timestamp fields, there are no input-only fields and no output-only fields. The input struct is reused as the output struct. Only `input = User` is required; `output = ...` must not be provided.

```rust
#[ivo_schema(input = User)]
mod user_schema {
    #[fields]
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

If the schema contains any input-only field (`#[virtual]`) or output-only field (`#[constant]`, `#[dependent]`, or timestamp fields), both `input = ...` and `output = ...` are required. Providing only `input = ...` produces a compile error.

Additional rules:

- A virtual field without an alias must be referenced by at least one dependent field's `on = [...]`.
- A virtual alias must name a dependent field, and that dependent field must depend on the virtual field.
- Constant, dependent, and timestamp field names cannot appear on the input struct.
- Timestamp field names cannot be reused as field names or aliases.
- The timestamp resolver is declared once in `#[options]` via `#[timestamps(|| Utc::now())]`.
- A schema may declare zero or one `#[created_at]` fields and zero or one `#[updated_at]` fields.
- An `#[updated_at]` field typed as `Option<T>` is treated as optional (set only when a value is already present on update); an `#[updated_at]` field typed as `T` is always set on create/update.

## 12. Field-type attributes are whitelisted at compile time

The macro enforces that each field type only accepts the attributes that the existing builder API supports. For example:

- `#[constant(value_or_resolver)]` accepts `#[on_delete]`, `#[on_success]`, `#[visibility(private)]`; it rejects bare `#[constant]`, `#[value]`, `#[validate]`, `#[on_failure]`, `#[default]`, `#[ignore_init]`.
- `#[required]` accepts `#[validate]`, `#[re_validate]`, `#[required_error]`, `#[ignore_update]`, `#[readonly]`, `#[on_delete]`, `#[on_success]`, `#[on_failure]`, `#[visibility(private)]`; it rejects `#[default]`, `#[ignore_init]`, `#[required]`, `#[ignore]`, `#[sanitize]`, `#[alias]`. `#[readonly]` requires `#[validate]`.
- `#[dependent]` accepts `#[depends_on]`, `#[resolve]`, `#[default]`, `#[readonly]`, `#[on_delete]`, `#[on_success]`, `#[visibility(private)]`; it rejects `#[validate]`, `#[re_validate]`, `#[on_failure]`, `#[ignore_init]`, `#[ignore_update]`, `#[ignore]`, `#[sanitize]`, `#[alias]`. `#[readonly]` requires a static `#[default]`.
- `#[virtual]` / `#[virtual(alias = "...")]` accepts `#[sanitize]`, `#[validate]`, `#[re_validate]`, `#[required]`, `#[ignore]`, `#[ignore_init]`, `#[ignore_update]`, `#[on_success]`, `#[on_failure]`, `#[visibility(private)]`; it rejects `#[alias]`, `#[on_delete]`, `#[default]`, `#[value]`, `#[readonly]`.
- `#[lax]` / `#[lax(default_or_resolver)]` accepts `#[validate]`, `#[re_validate]`, `#[required]`, `#[ignore]`, `#[ignore_init]`, `#[ignore_update]`, `#[readonly]`, `#[on_delete]`, `#[on_success]`, `#[on_failure]`, `#[visibility(private)]`; it rejects `#[default]`, `#[required_error]`, `#[value]`, `#[resolve]`, `#[sanitize]`, `#[alias]`. `#[readonly]` requires a static `#[lax(...)]` default.
- `#[created_at]` / `#[updated_at]` accept `#[visibility(private)]`; they reject all other field attributes. Their type must match the timestamp resolver's return type (`T` or `Option<T>` for `#[updated_at]`).

Standard Rust visibility keywords (`pub`, `pub(crate)`, `pub(super)`, etc.) written before the field name are also accepted on all field types and override the default `pub` field visibility.

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
- **Generated field visibility** defaults to `pub`. Users can override it by writing a standard Rust visibility keyword before the field name:
  - `#[required] name: String` → `pub name: String`.
  - `#[required] pub(crate) internal_id: String` → `pub(crate) internal_id: String`.
  - `#[lax] pub(super) secret: String` → `pub(super) secret: String`.
  - To make a field private in a public struct, use `#[visibility(private)]` on the field.
- **Timestamp fields** are declared like normal output fields with `#[created_at]` or `#[updated_at]`, so their visibility is set the same way as any other field. The shared timestamp resolver is declared once in `#[options]` via `#[timestamps(|| Utc::now())]`.
- **Partial structs, error structs, and the generated schema model** inherit the visibility of their corresponding input/output struct.

```rust
pub mod user_schema {
    #[fields]
    mod fields {
        #[required]
        pub name: String,

        #[required]
        pub(crate) tenant_id: Uuid,

        #[lax]
        #[visibility(private)]
        internal_note: Option<String>,

        #[created_at]
        pub(crate) created_at: Utc,

        #[updated_at]
        pub updated_at: Option<Utc>, // optional updated_at: only set if already present
    }

    #[options]
    mod options {
        #[timestamps(|| Utc::now())]
        const _: () = ();
    }
}
```
