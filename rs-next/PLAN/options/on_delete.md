# Plan: `#[on_delete]` option

This option does not change whether the schema uses single-struct or dual-struct mode; see [`struct_generation.md`](../struct_generation.md) for those rules.

## New syntax

```rust
#[options]
mod options {
    #[on_delete(|_data, _opts| async move { ... })]
    const _: () = ();

    #[on_delete(|data, opts| async move {
        // side effects on delete
    })]
    const _: () = ();
}
```

## Description

`#[on_delete]` registers handlers that run when a record is deleted. This is a global option, not tied to specific fields. It corresponds to the current builder's `.on_delete(handler)` method.

## Supported forms

| Syntax                                            | Semantics                                                  |
| ------------------------------------------------- | ---------------------------------------------------------- |
| `#[on_delete(\|data, opts\| async move { ... })]` | Async handler receiving the full output struct and options |
| `#[on_delete(\|\| async move { ... })]`           | Async handler with no arguments                            |
| `#[on_delete(\|\| { ... })]`                      | Sync handler (wrapped in async)                            |

## Mapping from current builder API

Current:

```rust
ivo::schema::<UserInput, User>()
    .on_delete(|_data, _opts| async move { ... })
    .on_delete(|data, opts| async move { ... })
```

New:

```rust
#[options]
mod options {
    #[on_delete(|_data, _opts| async move { ... })]
    const _: () = ();

    #[on_delete(|data, opts| async move { ... })]
    const _: () = ();
}
```

## Generated code sketch

```rust
struct UserSchemaModel {
    on_delete_handlers: Vec<Box<dyn Fn(User, IvoRwCtxOptions<UserCtxOptions>) -> BoxFuture<'static, ()> + Send + Sync>>,
}

impl UserSchemaModel {
    pub async fn delete(
        &self,
        data: User,
        options: UserCtxOptions,
    ) -> Result<(), IvoError> {
        for handler in &self.on_delete_handlers {
            handler(data.clone(), options.clone()).await;
        }
        Ok(())
    }
}
```

## Notes

- `#[on_delete]` is global and can be specified multiple times.
- Handlers receive the full output struct (`User`) and the context options.
- Handlers run sequentially; if one panics, the rest do not run.
- Unlike field-level `#[on_failure]`, delete handlers are not tied to validation errors.

## Implementation plan

1. Define the delete handler type in the generated model.
2. Parse `#[on_delete(closure)]` attributes.
3. Collect all delete handlers into a list.
4. Generate the `delete` method on the schema model.
5. Call all handlers in order before returning success.

## Progress

- [ ] Define delete handler type
- [ ] Parse `#[on_delete]` attributes
- [ ] Collect multiple handlers
- [ ] Generate `delete` method
- [ ] Wire handlers into delete pipeline
- [ ] Write tests
