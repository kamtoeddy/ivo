# TODO — `ts/src/schema/index.ts` runtime regressions

Found while porting the Rust-verified behavior suite into `tests-v2`. P1–P6
were fixed by closely re-reading the Rust `create`/`update` pipeline
(`rs/src/model/mod.rs`, `rs/src/model/fields_collection.rs`) and porting the
corresponding TS step. Verified against `rs/src/model/mod.rs` line-by-line;
after the fixes, `bun test ./tests/` and `bun test ./tests-v2/` show **zero
tests that regressed** (no test that passed before now fails) and a
combined 103 previously-failing tests now pass.

- [x] **P1 — Strictly-required fields not enforced at creation**
  `_evaluateMissingRequiredFields` only iterated `this.propsRequiredBy`, never `this.requiredProps`. Fixed: now also evaluates `requiredProps`, but only at creation (`!isUpdate`) — mirroring Rust's `FieldType::Required` match arm, which is guarded by `if !is_update` in `evaluate_missing_required_fields`. Also switched the "already provided, skip" check from raw `fieldsCollection.fieldsProvided` to `fieldsCollection.relevantConfigNames`, matching Rust's `is_relevant_config_name` — a field that was provided but got ignored (or, on update, whose value didn't actually change) is correctly treated as "not provided" for required-checks.

- [x] **P2 — `requiredBy` fields with a default never get defaulted**
  `_attachConstantsAndDefaults` only handled `_isLaxProp`/`dependsOn` fields. In Rust, conditionally-required ("requiredBy") fields are just `FieldType::Lax` with an extra `required_fn` — same classification as plain lax fields — so they default the same way. TS's `laxProps` set excludes anything with a `required` rule (`__isLax` in schema-core.ts), so the defaulting branch now also explicitly includes `_isRequiredBy(configName)` fields (excluding virtuals, which have no Output slot to default).

- [x] **P3 — `allow` (allowed-values) unenforced on lax fields without a validator**
  `_runPrimaryValidators` had a shortcut that bypassed `_validate` (where the `allow` check lives) whenever a lax field had no explicit `validator`. Removed the shortcut — `_validate` already returns `{valid: true, validated: value}` as-is when there's no validator, so behavior is unchanged for fields without `allow`, while fields with `allow` now correctly get checked. (No Rust equivalent exists for `allow` — it's TS-only — so this was fixed by internal-consistency reasoning rather than a Rust port.)

- [x] **P4 — Dependents on virtual fields never resolve**
  Root cause: `FieldInfoCollection.clonedFromRelevantFieldsProvided()` seeded `relevantDependentConfigNames` from `_relevantFieldsProvided`, which the `relevantFieldsProvided` setter deliberately narrows to **output fields only** (`if (info.isOutput) outputFieldsChanged.add(...)`). Virtuals have `isOutput: false`, so a dependent whose only parent is a virtual never saw that parent as "relevant" and never resolved. Rust's equivalent (`cloned_from_relevant_dependent_fields`) seeds from the *unfiltered* `relevant_fields_provided` (Rust doesn't narrow that field to outputs at all). Fixed by seeding from TS's `_relevantConfigNames` instead — the already-existing unfiltered, config-name-mapped set — which is the correct analog.

- [x] **P5 — `createdAt` appears in update diffs**
  `update()`'s timestamp-attachment step called `this._useConfigProps()` with no argument, defaulting `isUpdate` to `false` inside `_useConfigProps`, which both re-added `createdAt` and mis-resolved optional/nullable `updatedAt` handling on every update. Fixed by passing `true` explicitly at the `update()` call site.

- [x] **P6 — `required` handler ctx is read-only despite being typed as mutable**
  `_evaluateMissingRequiredFields` passed `_getReadonlyCtx()` (no `updateOptions`) to `required` handlers, though `RequiredHandler` is typed to receive a full `IvoContext`. Fixed by switching to `_getContext()` (a strict superset — same shape plus `updateOptions`), so `required` handlers can now legitimately call `ctx.updateOptions(...)` like every other handler kind (validator, resolver, sanitizer, ignore/ignoreUpdate, post-validator).
  Note: `tests-v2/extras/ctx-options.ts` has a test documenting the *old* throwing behavior (`'calling ctx.updateOptions inside a "required" handler throws...'`) — now stale since the throw no longer happens; left as-is per "don't worry about tests for now."

- [ ] **P7 (doc/type gap, not a regression) — schema-level `required` option is a stub**
  `NS.Options.required` is declared in the type surface and `ALLOWED_OPTIONS`, but `SchemaCore._checkOptions` has no validation branch for it and `ModelTool` never reads `this._options.required` at runtime. No construction-time validation, no create/update effect. Not touched — implementing it is a feature addition, not a regression fix.
  *Fix direction:* either implement validation + enforcement (mirroring `rs/tests/options/required.rs`: min 2 fields, lax/virtual-only, alias/timestamp exclusion) or mark the type as not-yet-implemented until it's wired up.

## Remaining known test failures (not yet investigated)

After P1–P6, `tests-v2` still has ~94 failing tests, largest clusters:
`virtual` (21), `requiredBy` (21), `allowed values` (10), `Schema.options.postValidate`/`onSuccess` (8 each). A quick look at the top `requiredBy`
failures shows they're about whether a `required` handler should still be
*called* (for side-effect/observability) even when its field was already
provided — a finer-grained behavior than P1/P2 that wasn't part of this
pass; worth a dedicated look later.
