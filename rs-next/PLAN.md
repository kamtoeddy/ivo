# Condense phase splices + fix dead-code-elimination gaps in generated `create`/`update`

## Context

GOAL.md §17 documents a specific design promise: "the macro omits any phase that
does not apply to the schema, which eliminates runtime branches and dead code
for simple schemas," and lists concrete guarantees (no dependent-resolution
loop without `#[depends_on]` fields, no constant attachment without
`#[constant]` fields, etc.). Separately, the final `create`/`update` method
bodies in `crates/derive/src/lib.rs` (~L5090-5250) currently splice several
adjacent, single-purpose token streams for what is conceptually one phase
(e.g. ignore-flag declarations + ignore evaluation + ignore-init assignment
are three separate splices), which makes the body harder to scan.

I audited every phase-producing variable against GOAL.md §17's checklist by
reading each one's construction in full. Most already collapse to `quote!{}`
correctly when inapplicable, because they're built via `emit_async_phase`
(`crates/derive/src/lib.rs:318`), which returns `quote!{}` immediately when
its `items: Vec<AsyncPhaseItem>` is empty — this already covers ignore
evaluation, required evaluation, validate, re-validate, post-validate (both
phases, both create and update), and sanitize. `create_constants_phase` and
`create_timestamps_phase` are likewise already correctly empty when there are
no constant/timestamp fields (verified by reading their construction).

One real, confirmed violation: **`dependent_create_block`** (create's
dependent-field resolution, `crates/derive/src/lib.rs:3003`) is a hand-built
`if/else` that _always_ emits a `HashSet` declaration + `loop { ... if
next_parents.is_empty() { break; } }` scaffold, even when the schema has zero
`#[depends_on]` fields — the loop body degenerates to nothing and breaks on
the first iteration, but the scaffold is still generated and compiled. This
contradicts GOAL.md §17's explicit "A schema with no `#[depends_on]` fields
does not generate the dependent-resolution loop" guarantee. Its sibling on
the update side, `dependent_update_assignments`, is a plain `Vec<TokenStream>`
(one entry per dependency level) that is naturally empty and splices to
nothing — it does **not** have this problem, so no fix needed there.

## User correction (incorporated below)

My first draft proposed keeping `#create_error_check`/`#update_error_check`
as their own visible splice between validate/re-validate/post-validate-pre/
post-validate-main. The user corrected this: those phases should each be
merged _with_ their trailing error check into one self-contained unit, and
virtual-field "provided" tracking (currently the `__virtual_provided_*` flags
computed once upfront in `build_virtual_pipeline`'s `setup`, ahead of and
separate from the merged validate/re-validate phases) should be tracked
_from within_ each phase instead — mirroring `rs/`'s structure, where
`validate()`/`re_validate()` (`rs/src/model/mod.rs:700`, `:800`) are each a
single self-contained function that determines field relevance, runs
validators, and checks errors together, rather than being split across
several independent top-level codegen splices the way rs-next currently is.
The user flagged this as probably a larger refactor than the rest of this
change and said it can be done **last** — so this plan is split into two
parts accordingly.

## Part 1 (do first): mechanical fixes, no behavior change

1. **Fix `dependent_create_block`** to short-circuit to `quote!{}` when there
   are no dependent fields (`dependent_infos.is_empty()`), matching the
   pattern `emit_async_phase` already uses.

2. **Condense the two flagged adjacent-splice groups** in both `create`'s and
   `update`'s literal method bodies into single named phase variables (each
   already naturally empty-when-inapplicable, since their constituent parts
   already are):
   - Ignore group: `ignore_flag_decls` + `ignore_evaluations` +
     `ignore_init_assignments` (create); `update_ignore_flag_decls` +
     `update_ignore_evaluations` + `bare_ignore_update_assignments` (update)
     → `create_ignore_phase` / `update_ignore_phase`.
   - Required group: `required_evaluations` + `required_field_checks`
     (create) → `create_required_phase`. (`update_required_evaluations` is
     already a single splice on the update side — confirm no merge needed.)
     `virtual_ignore_update_attempts` and `update_early_nothing_to_update_check`
     stay where they are (not part of the ignore group) since their pipeline
     position is deliberate.

3. **Refresh GOAL.md §17** ("Execution pipeline") to match the current
   generated order precisely: fold in the fail-fast checks after each phase,
   the ignore/required one-go batching, the two-phase post_validate
   pre/main split, and the 3-checkpoint nothing-to-update logic for update.
   Keep the existing "Dynamic pipeline generation" bullet list as-is — it
   remains accurate once (1) lands.

### Files (Part 1)

- `crates/derive/src/lib.rs` — the `dependent_create_block` fix, the two
  phase-variable consolidations, and updating the final `quote!{ ... }`
  bodies (~L5090-5250) to reference the new combined variable names.
- `GOAL.md` — refresh §17's Create/Update numbered lists.

### Verification (Part 1)

- `cargo build -p ivo-derive` then `cargo build --workspace --tests` — clean,
  zero-warning build (pure codegen-structure refactor; the only intended
  behavior change is dead-scaffold removal for zero-dependent-field schemas).
- `cargo test --workspace`, twice, at parity with the last known-good count
  (809 + 46).
- Confirm the `dependent_create_block` fix is load-bearing via `cargo expand`
  on a schema with zero `#[depends_on]` fields, before/after, confirming the
  `HashSet`/`loop` scaffold is actually gone — a compile-only check wouldn't
  catch a regression here since the old code was already correct at runtime,
  just wasteful in codegen.
- Re-run `examples/main_demo` to confirm end-to-end behavior is unchanged.

## Part 2 (do last, larger refactor): merge phases with their error checks + inline virtual-provided tracking

Restructure `build_virtual_pipeline` and the create/update validate/
re-validate/post-validate(pre/main) construction so that:

- Each phase (validate, re-validate, post-validate-pre, post-validate-main)
  becomes one self-contained variable that includes its own trailing
  error-check (fail fast) internally, instead of the caller splicing
  `#create_error_check`/`#update_error_check` separately after each.
- Virtual-field "provided" determination moves from the current upfront,
  shared `setup_stmts` block into each phase's own construction, matching
  `rs/`'s `validate()`/`re_validate()` shape (`rs/src/model/mod.rs:700`,
  `:800`) where relevance/provided-ness, running the validator, and error
  checking are all one unit rather than a pre-computed flag threaded through
  separately-spliced phases.

This needs its own dedicated investigation before implementation (how
`__virtual_provided_*` flags are currently consumed downstream — by sanitize,
by dependent-resolution guards, by the mid-pipeline nothing-to-update check —
since moving their computation will require auditing every downstream reader,
not just the phase that currently computes them). Given the user explicitly
scheduled this last, it will be scoped and planned in a follow-up pass after
Part 1 lands and is verified, rather than attempted in the same change.
