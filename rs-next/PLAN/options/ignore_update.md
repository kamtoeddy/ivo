# Plan: `#[ignore_update]` option

This option does not change whether the schema uses single-struct or dual-struct mode; see [`struct_generation.md`](../struct_generation.md) for those rules.

## New syntax

```rust
#[options]
mod options {
    #[ignore_update(["email"], |_input, _output, _opts| async move { true })]
    const _: () = ();

    #[ignore_update(["role", "status"], |_input, _output, _opts| async move { false })]
    const _: () = ();
}
```

## Description

`#[ignore_update]` decides whether to ignore a group of fields during update operations. The resolver receives the partial input and the full current output. This corresponds to the current builder's `.ignore_update(fields, resolver)` method.

## Supported forms

| Syntax                                                                    | Semantics                            |
| ------------------------------------------------------------------------- | ------------------------------------ |
| `#[ignore_update(["field"], \|input, output, opts\| async move { ... })]` | Async resolver with input and output |
| `#[ignore_update(["field"], \|\| async move { ... })]`                    | Async resolver with no arguments     |
| `#[ignore_update(["field"], \|\| true)]`                                  | Sync boolean                         |

## Mapping from current builder API

Current:

```rust
ivo::schema::<UserInput, User>()
    .ignore_update(["email"], |_input, _output, _opts| async move { true })
```

New:

```rust
#[options]
mod options {
    #[ignore_update(["email"], |_input, _output, _opts| async move { true })]
    const _: () = ();
}
```

## Generated code sketch

```rust
struct UserSchemaIgnoreUpdateConfig {
    fields: &'static [&'static str],
    resolver: Box<dyn Fn(UserInputPartial, User, IvoRwCtxOptions<UserCtxOptions>) -> BoxFuture<'static, bool> + Send + Sync>,
}

impl UserSchemaModel {
    async fn apply_ignore_update_options(
        &self,
        input: &mut UserInputPartial,
        previous_values: &User,
        options: IvoRwCtxOptions<UserCtxOptions>,
    ) {
        for config in &self.options.ignore_update {
            if (config.resolver)(input.clone(), previous_values.clone(), options.clone()).await {
                for field in config.fields {
                    input.unset(field);
                }
            }
        }
    }
}
```

## Notes

- `#[ignore_update]` only runs during update operations, not create.
- The resolver receives a clone of the partial input and the full previous output.
- Field names are validated at macro expansion time.

## Implementation plan

1. Define `UserSchemaIgnoreUpdateConfig`.
2. Parse `#[ignore_update([...], closure)]` attributes.
3. Collect all configs into a list.
4. Validate field names.
5. Generate `apply_ignore_update_options` and call it only during update, before per-field validation.

## Progress

- [ ] Define ignore-update config type
- [ ] Parse `#[ignore_update]` attributes
- [ ] Collect multiple configs
- [ ] Validate field names at macro time
- [ ] Generate application logic
- [ ] Wire into update pipeline only
- [ ] Write tests
