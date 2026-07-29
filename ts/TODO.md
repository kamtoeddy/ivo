# TODO — `ts/src/schema/index.ts` runtime regressions

Found while porting the Rust-verified behavior suite into `tests-v2`. Each item
was confirmed with an isolated repro (schema + `create()`/`update()` call)
independent of `tests-v2`, so they should reproduce regardless of anything in
the test suite. P1–P3 and P5 are driving most of the current `tests-v2`
failure count; P4 is likely the single biggest contributor among the
"virtual"/"dependent" failure clusters.

- [ ] **P1 — Strictly-required fields not enforced at creation**
  `_evaluateMissingRequiredFields` (~line 1120) only iterates `this.propsRequiredBy`, never `this.requiredProps`. Fields declared `{ required: true, validator }` can be omitted from `create()` with no error.
  *Fix direction:* also iterate `requiredProps`, emitting the default `'${prop}' is required` message when missing.

- [ ] **P2 — `requiredBy` fields with a default never get defaulted**
  `_attachConstantsAndDefaults` (~line 753) only handles fields where `_isLaxProp(configName)` or `config.dependsOn` is true. `__isLax` (schema-core.ts) explicitly excludes any field with a `required` rule, so a conditionally-required field (`{ default, required: fn }`) omitted from input falls through every branch and is simply absent from the output — instead of resolving to its default.
  *Fix direction:* add a branch (or broaden the lax-prop check) to default `requiredBy` fields not present in input.

- [ ] **P3 — `allow` (allowed-values) unenforced on lax fields without a validator**
  `_runPrimaryValidators` (~line 842) shortcuts past `_validate` — where the `allow`/`propsToAllowedValuesMap` check lives — whenever a lax field has no explicit `validator`. Any value outside the `allow` list is accepted silently.
  *Fix direction:* run the allow-check even on the no-validator shortcut path, not just inside `_validate`.

- [ ] **P4 — Dependents on virtual fields never resolve**
  A `dependsOn: 'someVirtual'` resolver never fires even when that virtual is provided at creation — reproduced both with and without `alias`. Confirmed independent of P1–P3 with a minimal 2-field schema (virtual → dependent, no other rules).
  *Fix direction:* needs tracing through `_getFieldInfoCollection`/`_resolveDependentChanges` to see why a provided virtual isn't landing in `relevantDependentConfigNames` → `_getDependencies` lookup.

- [ ] **P5 — `createdAt` appears in update diffs**
  Timestamps config should make `createdAt` immutable and absent from every `update()` result; it currently shows up with a fresh value on every update.
  *Fix direction:* check the timestamp-attachment step (`_useConfigProps`) — likely re-adding `createdAt` unconditionally instead of only at creation.

- [ ] **P6 — `required` handler ctx is read-only despite being typed as mutable**
  `RequiredHandler` is typed to take `IvoContext` (has `updateOptions`), but `_evaluateMissingRequiredFields` passes `_getReadonlyCtx()`. Calling `ctx.updateOptions(...)` inside a `required` handler throws, and — per the model's general "handler errors are swallowed" behavior — that silently resolves the field to "not required" rather than surfacing anything.
  *Fix direction:* either pass a mutable ctx to `required` handlers, or narrow the `RequiredHandler` type to `ReadonlyIvoContext` to match reality.

- [ ] **P7 (doc/type gap, not a regression) — schema-level `required` option is a stub**
  `NS.Options.required` is declared in the type surface and `ALLOWED_OPTIONS`, but `SchemaCore._checkOptions` has no validation branch for it and `ModelTool` never reads `this._options.required` at runtime. No construction-time validation, no create/update effect.
  *Fix direction:* either implement validation + enforcement (mirroring `rs/tests/options/required.rs`: min 2 fields, lax/virtual-only, alias/timestamp exclusion) or mark the type as not-yet-implemented until it's wired up.
