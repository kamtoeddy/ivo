# Open

- [ ] manually go through all tests in `/rs` and make sure they have been ported correctly
- [ ] `create`'s `post_validate` groups still run unconditionally regardless of whether any of their fields were actually submitted (known gap left from the update-relevance fix below; `update` already gates this correctly)
- [ ] investigate potential performance (memory and speed) enhancements
  - [ ] limit allocations as much as possible
    - [ ] use static strs instead of `String` for schema fields
    - [ ] use arrays instead of `Vec` for schema fields where possible
- [ ] update docs (and README) of `rs-next`

# Done

- [x] fixed `## 17. Execution pipeline` in GOAL.md: virtual-field validate/re-validate/sanitize gating, parallel independent phases via `emit_async_phase` (with real ordering-bug fixes: constants/dependents/timestamps), timestamp resolver called once per call, dead dependent-resolution scaffold removed for schemas with no `#[depends_on]` fields, ignore/required phase splices condensed, GOAL.md §17 rewritten to match reality. Later folded validate/re-validate/post-validate/required phases together with their fail-fast checks into single self-contained phase variables (verified via `cargo expand` that check ordering is unchanged). Full suite: 809 + 46, stable throughout.
- [x] removed bare `#[ignore]` (not a valid config); `ignore_update` now rejects empty field arrays.
- [x] fail-fast: `create`/`update` return immediately after each phase if `errors` is non-empty.
- [x] `update` now matches all 3 of `rs/`'s `handle_nothing_to_update_error` checkpoints, not just the last.
- [x] audited stale `SKIPPED:` porting notes -- both described already-supported behavior, just untested; fixed in `tests/fields/required.rs`, `tests/options/mod.rs`, GOAL.md §19.
- [x] `IvoContext::raw_input()` now genuinely distinct from `input()` (previously an alias); `input()` also strips unchanged `#[required]`/`#[lax]` fields on update, matching `rs/`. Covered by `tests/extras/raw_input.rs`.
- [x] `#[post_validate(...)]` errors on an aliased virtual field were silently dropped (keyed by internal name instead of alias) -- fixed via `external_field_name()`.
- [x] audited alias (`#[ivo_virtual("alias")]`) handling at every phase. Found `tests/fields/virtuals/{ignore,on_success,on_failure}.rs` were never `mod`-declared (~276 tests silently never ran); wired them in, which surfaced 8 pre-existing "internal name instead of alias" assertion bugs plus the same bug class at 3 more derive-macro call sites -- all fixed. Full suite: 529+46 -> 805+46.
- [x] fixed `update` not detecting "nothing to update" when a resubmitted field's value was unchanged (was only checking "provided", never comparing against stored value) -- root-caused to a missing port of `rs/`'s `filter_input_fields_allowed`. Fixed across all downstream consumers: the early relevance checkpoint, `update_assignment_items`'s validate gate, `re_validate`, `post_validate` group gating (update only, see open item above), and dependent-resolution's parent guard -- all now read one shared `__update_relevant_*` flag per field. Verified against all 6 of `examples/main_demo`'s "nothing to update" scenarios; regression tests in `tests/extras/update_relevance.rs` cover both required-field and virtual-field (unaliased / unrelated alias / alias colliding with a dependent name) trigger shapes, each verified load-bearing by reverting the relevant guard and confirming failure. Full suite: 813 + 46.
- [x] ported the virtual-field validate/re-validate/sanitize gating fix to `update` (had zero such treatment before).
- [x] GOAL.md §10's named-const grouped-option anchors now actually work (were keying off const name instead of recognized option attribute).
- [x] `#[depends_on(...)]` and `#[ivo_virtual(...)]` now require string literals, not bare identifiers -- updated across 33 files + GOAL.md, with compile-fail coverage.
