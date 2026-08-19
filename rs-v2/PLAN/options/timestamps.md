# Plan: `#[timestamps]` option

Timestamp fields are declared as normal output fields inside `#[fields]` using `#[created_at]` and `#[updated_at]`. The shared timestamp resolver is declared once in `#[options]` via `#[timestamps(resolver)]`.

Because timestamp fields are output-only, a schema that declares them cannot use single-struct mode and must declare both `input = ...` and `output = ...` in `#[ivo_schema]`. See [`struct_generation.md`](../struct_generation.md) for the full struct-generation rules.

## New syntax

```rust
#[fields]
mod fields {
    #[created_at]
    pub created_at: DateTime<Utc>,

    #[updated_at]
    pub updated_at: DateTime<Utc>,
}

#[options]
mod options {
    #[timestamps(|| Utc::now())]
    const _: () = ();
}
```

Custom field names and visibilities are controlled by the field declaration itself:

```rust
#[fields]
mod fields {
    #[created_at]
    pub(crate) created_on: DateTime<Utc>,

    #[updated_at]
    pub updated_on: Option<DateTime<Utc>>, // optional updated_at
}

#[options]
mod options {
    #[timestamps(|| Utc::now())]
    const _: () = ();
}
```

## Supported arguments

`#[timestamps]` in `#[options]` accepts a single resolver:

| Argument             | Required | Description                                 |
| -------------------- | -------- | ------------------------------------------- |
| `resolver = closure` | yes      | Sync resolver returning the timestamp value |

The shorthand `#[timestamps(|| Utc::now())]` is equivalent to `#[timestamps(resolver = || Utc::now())]`.

## Optional `updated_at`

An `#[updated_at]` field whose type is exactly `Option<T>` is treated as optional: on update, the timestamp is written only if the field already has a value. An `#[updated_at]` field whose type is `T` is always populated on create and update.

The macro recognizes `Option<T>` syntactically; it does not resolve type aliases.

## Mapping from current builder API

Current:

```rust
ivo::schema::<UserInput, User>()
    .timestamps(|t| t
        .resolve(|| Utc::now())
        .created_at(None)
        .updated_at(None)
    )
```

New:

```rust
#[fields]
mod fields {
    #[created_at]
    pub created_at: DateTime<Utc>,

    #[updated_at]
    pub updated_at: DateTime<Utc>,
}

#[options]
mod options {
    #[timestamps(|| Utc::now())]
    const _: () = ();
}
```

## Generated code sketch

```rust
struct UserSchemaTimestampConfig<Timestamp> {
    created_at: Option<&'static str>,
    updated_at: Option<(&'static str, bool)>, // (name, is_optional)
    resolver: fn() -> Timestamp,
}

impl UserSchemaModel {
    fn timestamp_config(&self) -> Option<&UserSchemaTimestampConfig<DateTime<Utc>>> {
        Some(&self.options.timestamps)
    }
}
```

The generated `create`/`update` methods use this config to populate timestamp fields on the output struct:

```rust
impl UserSchemaModel {
    async fn apply_timestamps(
        &self,
        output: &mut UserPartial,
        is_create: bool,
    ) {
        let Some(config) = self.timestamp_config() else { return };

        let now = (config.resolver)();

        if is_create {
            if let Some(name) = config.created_at {
                output.set(name, &erase_value(now.clone()));
            }
        }

        if let Some((name, is_optional)) = config.updated_at {
            if !is_optional || output.contains(name) {
                output.set(name, &erase_value(now));
            }
        }
    }
}
```

## Validation performed by the macro

1. A schema may declare at most one `#[created_at]` field and at most one `#[updated_at]` field.
2. `#[created_at]` and `#[updated_at]` fields must appear in `#[fields]`, not in `#[options]`.
3. Timestamp field names cannot clash with schema field names or virtual aliases.
4. Dependent fields cannot depend on timestamp fields.
5. `#[created_at]` and `#[updated_at]` fields are output-only and must not appear on the input struct.
6. The `#[timestamps]` resolver must be declared exactly once if any timestamp field is present.
7. `#[updated_at]` optional detection is based on the field type being `Option<T>`; type aliases are not resolved.
