- [ ] fix `## 17. Execution pipeline` in GOAL.md
  - [x] the validation, re_validation, and sanitization of virtual fields does not happen unless a virtual field is provided and accepted (i.e: not ignored) — fixed for both `create` and `update`; see `build_virtual_pipeline` in `crates/derive/src/lib.rs`. Covered by `should_only_sanitize_virtual_fields_that_were_provided` and `should_not_re_validate_virtual_fields_that_were_not_provided_or_were_ignored` in `tests/fields/virtuals/mod.rs`.
  - [x] re_validation does not happen unless a field has a revalidator and was successfully validated — virtual fields now support `#[re_validate]` (previously silently ignored); gated on the field's own validate having succeeded. Covered by `should_properly_use_re_validated_values` (create + update).
  - [x] the sanitization of virtuals only happens after a successful post-validation — sanitize step moved to run after `post_validate` and the errors check, for both `create` and `update`. Covered by `should_sanitize_virtual_fields_only_after_post_validate_succeeds`.
  - [ ] make sure every major operation in Model.create and Model.update should happen in parallel
  - [x] for every Model.create/Model.update call, the timestamp resolver should be called at most once — `create` now resolves a shared `__ivo_timestamp_value` once up front (when `#[created_at]`/non-optional `#[updated_at]` are present) instead of calling the resolver per field; `update` only ever had at most one timestamp field to begin with (schemas allow only one `#[updated_at]`), so it was already at most once. Covered by `should_call_the_timestamp_resolver_at_most_once_per_create_call` in `tests/fields/timestamps.rs`.
- [x] Bare `#[ignore]` is not a valid config and all associated logic should be removed
- [ ] find all `SKIPPED:` ports and make sure they are valid
- [ ] manually go through all tests in "/rs" and make sure they have been ported correctly
- [ ] update benchmarks, run benchmarks, compare results and improve performance
- [x] the ignore_update option should reject empty fields, and consider a closure without fields as an entity-level ignore update handler

# added later

- [x] Ported the same fix to `update` (turned out virtual fields got **zero** validate/re-validate/sanitize treatment during update before — a real gap beyond what TODO.md called out). Tests added: `should_properly_use_re_validated_values` and `should_respect_sanitizers_if_provided` both exercise create+update.
