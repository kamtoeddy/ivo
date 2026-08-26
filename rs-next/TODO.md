# Pending validation / semantics work

## Ignore-attribute rules

- [x] Reject bare `#[ignore]`. `#[ignore]` must always be conditional (`#[ignore(|ctx, _| ...)]`).
- [x] Reject `#[ignore]` + `#[ignore_init]` on the same `lax` / `virtual` field.
- [x] Reject `#[ignore]` + `#[ignore_update]` on the same `lax` / `virtual` field.
- [x] Reject `#[ignore_init]` + bare `#[ignore_update]` on the same `lax` / `virtual` field.
  - Error: "init and update cannot be fully disabled".
- [x] Allow `#[ignore_init]` + `#[ignore_update(||resolver)]` on the same `lax` / `virtual` field.
- [x] Reject resolver form of `#[ignore_init]` (`#[ignore_init(||resolver)]`).
  - Error: "conditional #[ignore_init] is currently not accepted".
- [x] Grouped `#[ignore([...], ...)]` stays limited to `lax` / `virtual` fields.
- [x] Grouped `#[ignore_update([...], ...)]` limited to `required`, `lax`, and `virtual` fields.

## Required-field rules

- [x] Allow `#[readonly]` on required fields.
- [x] Allow `#[ignore_update(||resolver)]` on required fields.
- [x] Reject bare `#[ignore_update]` on required fields.
  - Error: "#[ignore_update] on a required field must be conditional; use #[readonly] to always ignore updates".
- [x] Reject `#[readonly]` + `#[ignore_update]` on required fields.
- [x] Keep `#[ignore]` and `#[ignore_init]` disallowed on required fields.

## Lax-field rules

- [x] Allow `#[readonly]` + `#[ignore]` on `lax` fields.
- [x] Allow `#[readonly]` + `#[ignore_init]` on `lax` fields.
- [x] Reject `#[readonly]` + `#[ignore_update]` on `lax` fields.

## Other

- [ ] Allow field-level `#[ignore]` and grouped `#[ignore([...], ...)]` to reference the same field.
- [ ] Allow `#[ignore(...)]` + `#[required(...)]` on the same `lax` / `virtual` field; evaluate independently.
- [ ] All hints are delivered as compile-error messages (no stable Rust warnings).

# Recently completed

- [x] Port `examples/lax_with_ignore_init.rs` to the `#[ivo_schema]` macro API and verify assertions / example output.
- [x] Port `examples/lax_with_ignore_update.rs` to the `#[ivo_schema]` macro API and verify assertions / example output.
- [x] Reconcile `#[ignore]` / `#[ignore_update]` semantics: field-level `#[ignore]` (with a resolver) applies to both create and update on `#[lax]` / `#[ivo_virtual]` fields; `#[ignore_init]` is create-only; `#[ignore_update]` is update-only.
- [x] Fix unconditional ignore-update flag bug that treated every `#[ignore_update(...)]` field as bare/always-ignore.
