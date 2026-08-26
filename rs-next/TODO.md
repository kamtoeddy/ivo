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

- [x] Allow field-level `#[ignore]` and grouped `#[ignore([...], ...)]` to reference the same field.
- [x] Allow `#[ignore(...)]` + `#[required(...)]` on the same `lax` / `virtual` field; evaluate independently.
- [x] All hints are delivered as compile-error messages (no stable Rust warnings).

# Recently completed

- [x] Port `examples/main_demo/` to the `#[ivo_schema]` macro API (folder example with `validators` feature) and verify it builds and runs with `--features validators`.
- [x] Support grouped `#[required([...], \|ctx, opts\| ...)]` returning `Option<{InputName}Errors>` for per-field custom error messages; invoke the handler only when none of the listed fields were provided.
- [x] Allow `#[timestamps(path::to_now)]` as a shorthand for `#[timestamps(|| path::to_now())]`, while keeping timestamps strictly synchronous and rejecting async resolvers with a clear macro error.
- [x] Replace `std::sync::RwLock` in `IvoRwCtxOptions` / `IvoCtxOptions` with `async-lock::RwLock`; expose async `.read()` / `.write()` and sync `.read_sync()` / `.write_sync()` so async resolvers no longer hold synchronous guards across await points.
- [x] Port `examples/lax_with_ignore_init.rs` to the `#[ivo_schema]` macro API and verify assertions / example output.
- [x] Port `examples/lax_with_ignore_update.rs` to the `#[ivo_schema]` macro API and verify assertions / example output.
- [x] Reconcile `#[ignore]` / `#[ignore_update]` semantics: field-level `#[ignore]` (with a resolver) applies to both create and update on `#[lax]` / `#[ivo_virtual]` fields; `#[ignore_init]` is create-only; `#[ignore_update]` is update-only.
- [x] Fix unconditional ignore-update flag bug that treated every `#[ignore_update(...)]` field as bare/always-ignore.
- [x] Port `examples/timestamps_with_default_names.rs` to the `#[ivo_schema]` macro API.
- [x] Port `examples/timestamps_with_custom_names.rs` to the `#[ivo_schema]` macro API.
- [x] Port `examples/virtuals.rs` to the `#[ivo_schema]` macro API (re-validator section omitted because `#[re_validate]` on virtual fields is not supported).
- [x] Port `examples/virtuals_with_ignore.rs` to the `#[ivo_schema]` macro API.
- [x] Port `examples/virtuals_with_ignore_init.rs` to the `#[ivo_schema]` macro API.
- [x] Port `examples/virtuals_with_ignore_update.rs` to the `#[ivo_schema]` macro API.
- [x] Support bare `#[ignore_update]` and conditional `#[ignore]` on `#[ivo_virtual]` fields during updates, so ignored virtual fields do not re-trigger dependent resolvers and updates consisting only of ignored virtual fields return the "nothing to update" failure.
- [x] Port `rs/tests/extras/error_sanitizer.rs` to `rs-next/tests/extras/error_sanitizer.rs`.
- [x] Port `rs/tests/extras/ctx_options/mod.rs` and its submodules (`constants`, `dependents`, `lax`, `required`, `virtuals`) to `rs-next/tests/extras/ctx_options/`.
- [x] Add `mod extras;` to the integration test root so the ported tests are included in the test suite.
- [x] Skip `re_validate` on virtual fields and virtual-field `validate`/`sanitize` during updates in the ported `ctx_options/virtuals.rs` tests because the new macro processes virtual fields only at creation time.
- [x] Add `trybuild` as a dev-dependency so compile-time macro validation tests can be expressed as compile-fail UI tests.
- [x] Port `rs/tests/options/ignore.rs` validation tests to `rs-next/tests/options/compile_fail/ignore.rs` (compile-time errors via trybuild).
- [x] Port `rs/tests/options/ignore_update.rs` validation tests to `rs-next/tests/options/compile_fail/ignore_update.rs`; port the runtime behavior tests to `rs-next/tests/options/ignore_update.rs`.
- [x] Port `rs/tests/options/post_validate.rs` validation tests to `rs-next/tests/options/compile_fail/post_validate.rs`.
- [x] Port `rs/tests/options/required.rs` validation tests to `rs-next/tests/options/compile_fail/required.rs`.
- [x] Port `rs/tests/options/mod.rs` runtime tests (`ignore_update` behavior, `on_delete`, and `on_success` allow-list) to `rs-next/tests/options/mod.rs`; port the invalid `on_success` config tests to `rs-next/tests/options/compile_fail/on_success.rs`.
- [x] Add `mod options;` to the integration test root so the ported option tests are included in the test suite.

- [x] Port the remaining `rs/tests/fields/lax` test cases (`ignore`, `on_delete`, `on_failure`, `on_success`) into `rs-next/tests/fields/lax.rs` using the `#[ivo_schema]` macro API, and verify the full `fields` test suite passes.

- [x] Fix field-level `#[on_success]` handlers firing unconditionally during updates; they now only run when the corresponding field actually changed (or, for virtual fields, when it was provided and not ignored).

- [x] Port `rs/tests/ivo_derive.rs` to `rs-next/tests/ivo_struct.rs` using the `#[ivo_schema]` macro API, adding `seahash`, `serde`, and `serde_json` as dev-dependencies.
