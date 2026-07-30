# TODO — `ts/src/schema/index.ts` runtime regressions

Found while porting the Rust-verified behavior suite into a new test tree
(originally `tests-v2`, since merged back into `ts/tests` — see below). P1–P6
were fixed by closely re-reading the Rust `create`/`update` pipeline
(`rs/src/model/mod.rs`, `rs/src/model/fields_collection.rs`) and porting the
corresponding TS step. Verified against `rs/src/model/mod.rs` line-by-line;
after the fixes, the old and new suites together showed **zero tests that
regressed** (no test that passed before now fails) and a combined 103
previously-failing tests now pass.

**`tests-v2` has since been merged into `ts/tests`**: every old `tests/schema/{definitions,options}/*.ts` file superseded by its `tests-v2` counterpart was deleted, and everything unique to the old suite (`built-in-validators/`, `utils/`, `samples/`, `stress.test.ts`, `valid-configs.ts`, `validators.ts`, `values-parsing.ts`) was carried over into the merged `tests/` tree with import paths fixed for the new (flatter) layout. `validators.ts` and `samples/context-options.test.ts` carry pre-existing staleness (`shouldInit`/`shouldUpdate`/`readonly: 'lax'` — all removed rules — and `error.payload` instead of `error`) that predates this effort and wasn't in scope to fix during the merge; `samples/storeItem`/`samples/orderItem` are sample schemas with no `.test.ts` runner and aren't wired into anything (already true before the merge). `valid-configs.ts` has no `describe`/`it` blocks by design — it's a compile-time-only file (constructs schemas across several Input/Output shape combinations and accesses fields on the resulting `data` purely so `tsc` catches type-inference regressions; it has no runtime assertions and never did, going back to when it was introduced).

Verification: a first pass used a naive `comm`-based diff of failing-test *names*, which missed that `bun test`'s "errors" count (module-resolution failures etc.) isn't in that list — `tests/stress.test.ts` had a stale `../../src` import (missed when fixing paths after the flatten) that silently broke it. Caught on a second, more rigorous pass (`tsc -p tests/tsconfig.json`, which resolves every import in the tree at once) and fixed. Final check: a proper multiset diff of failing-test names (old `tests/` + old `tests-v2/`, combined, by count not just presence) against the merged suite, plus `tsc -p tests/tsconfig.json` showing zero "cannot find module" errors — zero new or more-frequent failures.

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
  Note: `tests/extras/ctx-options.ts` has a test documenting the *old* throwing behavior (`'calling ctx.updateOptions inside a "required" handler throws...'`) — now stale since the throw no longer happens; left as-is per "don't worry about tests for now."

- [x] **P7 — schema-level `required` option was a stub; now implemented, mirroring Rust**
  Implemented both halves, mirroring `rs/src/schema/mod.rs::make_options`'s `options.required` block and `rs/src/model/mod.rs::evaluate_missing_required_fields`'s grouped-config handling:
  - **Construction validation** (`SchemaCore._isRequiredOptionOk`, wired into `_checkOptions`): at least 2 properties, no duplicates, no aliases (must use the virtual's real name), and only lax/virtual fields — including conditionally-required "requiredBy" fields, which Rust classifies as plain `FieldType::Lax` — are allowed; constants, dependents, strictly-required fields, and timestamp fields are rejected. Panics on the first violation found, same as Rust (no accumulation).
  - **Runtime enforcement** (`ModelTool._evaluateMissingRequiredFields`): iterates `this._options.required` directly (no registry map needed, since Rust's own `evaluate_missing_required_fields` iterates its configs the same flat way). A config's handler(s) only run when *none* of its `properties` are in `relevantConfigNames`; the handler returns `undefined` (satisfied) or a per-field error map, which is filtered down to the group's own fields and alias-resolved before being set on the error tool — mirroring Rust's `field_names.contains(...)` filter and its use of the alias-resolved display name.
  - **Type change**: `RequiredConfigObject.handler` was typed as `RequiredHandler` (the field-level boolean/tuple handler) even though Rust's grouped resolver returns `Option<PartialErrors>` (a per-field map) — a pre-existing type/behavior mismatch. Introduced `RequiredOptionHandler` (reusing the already-existing `ResponseErrorObject`, the same return shape `postValidate` uses) and pointed `RequiredConfigObject.handler` at it instead. `RequiredHandler` itself (field-level `required: fn`) is untouched.
  - `tests/options/required.ts` (previously documenting the stub) was rewritten to test the real implementation — signature/invalid (construction) and behaviour (runtime) sections, mirroring `rs/tests/options/required.rs`.
  - Verified: zero new failures relative to the post-P1–P6 baseline (only expected change: the old required.ts stub tests were replaced, and the pre-existing P6-stale ctx-options.ts test's failure reason shifted, not newly broken).

## Remaining known test failures (not yet investigated)

The merged `ts/tests` currently has 103 failing tests (of 559), largest
clusters: `virtual` (~30), `requiredBy` (~22), `allowed values` (~10),
`Schema.options.postValidate`/`onSuccess` (8 each). A quick look at the top
`requiredBy` failures shows they're about whether a `required` handler
should still be *called* (for side-effect/observability) even when its
field was already provided — a finer-grained behavior than P1/P2 that
wasn't part of this pass. Plus the pre-existing staleness in
`validators.ts`/`samples/context-options.test.ts` noted above. Worth a
dedicated look later.
