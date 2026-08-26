also add rules so that:

- [ ] `ignore + (ignore_init/ignore_update)` should be rejected
- [ ] `#[ignore_init] + #[ignore_update]` should be rejected
- [ ] `#[ignore_init(||resolver)]` + `#[ignore_update]` or `#[ignore_init]` + `#[ignore_update(||resolver)]` should be allowed for lax and virtual fields.
- [ ] `#[ignore_init(||resolver)]` + `#[ignore_update(||resolver)]` should warn and hint to use `#[ignore(||resolver)]` instead (is this feasible)?
- [ ] Required fields:
  - [ ] `[readonly]` is allowed to prevent any further updates to that required field
  - [ ] `[ignore_update(||resolver)]` is allowed.
  - [ ] `[readonly]` + `[ignore_update(||resolver)]` should be rejected.
  - [ ] they cannot have `ignore`, `ignore_init` or `ignore_init(||resolver)` or `#[ignore_update]`;
  - [ ] if `#[ignore_update]` is provided, reject and hint for `[readonly]` to be used instead (is this feasible)?

recently completed:

- [x] Port `examples/lax_with_ignore_init.rs` to the `#[ivo_schema]` macro API and verify assertions / example output.
- [x] Port `examples/lax_with_ignore_update.rs` to the `#[ivo_schema]` macro API and verify assertions / example output.
- [x] Reconcile `#[ignore]` / `#[ignore_update]` semantics: field-level `#[ignore]` (bare or with a resolver) applies to both create and update on `#[lax]` / `#[ivo_virtual]` fields; `#[ignore_init]` is create-only; `#[ignore_update]` is update-only. Bare `#[ignore]` / `#[ignore_update]` mean "always ignore"; resolver forms are evaluated at runtime.
- [x] Fix unconditional ignore-update flag bug that treated every `#[ignore_update(...)]` field as bare/always-ignore.
