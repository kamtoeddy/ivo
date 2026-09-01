---
title: Execution Pipeline
---

# Execution Pipeline

`create` and `update` run through a fixed sequence of phases. Knowing the order matters for two
things in particular: what `ctx.values()` can see from inside a resolver (everything resolved in
an earlier phase, nothing from a later one), and when validation fails "fast" -- most phases that
can produce an error return immediately if one occurred, without running any later phase against
already-invalid data.

The macro only generates the phases a given schema actually needs -- a schema with no
`#[constant]` fields has no constant-attachment step at all, a schema with no `#[depends_on]`
fields has no dependent-resolution loop, and so on. Nothing described below is dead code or a
runtime branch for schemas that don't use a given feature.

## `create`

1. **Ignore** -- evaluate `#[ignore]`/`#[ignore_init]` (field-level and grouped together), then
   apply `#[ignore_init]` overrides.
2. **Required** -- evaluate conditional `#[required(...)]` and check bare `#[required]` fields for
   a missing value. *Fails fast.*
3. **Validate** -- `#[validate]` runs for required/lax and virtual fields together, as one phase.
   Defaults for unset `#[lax]` fields are applied here too. *Fails fast.*
4. **Re-validate** -- `#[re_validate]`, same grouping as validate, only for fields that validated
   successfully. *Fails fast.*
5. **Post-validate (pre)** -- every `#[post_validate(...)]` group's `pre_validate` handler, against
   a snapshot from before this phase. *Fails fast.*
6. **Post-validate (main)** -- every group's main `validate` handler; skipped entirely if step 5
   produced any error. *Fails fast.*
7. **Sanitize** -- `#[sanitize]` on virtual fields that were provided and not ignored, once step 6
   has succeeded.
8. **Resolve dependents** -- `#[resolve]`, one round per dependency-graph level, looping until
   nothing new changes. `ctx.values()` inside a resolver reflects everything resolved in earlier
   rounds.
9. **Attach constants** -- `#[constant]`, after dependents resolve, so a constant resolver can read
   resolved dependent values via `ctx.values()`.
10. **Attach timestamps** -- `#[created_at]`/`#[updated_at]`, after constants. The shared resolver
    is called at most once.
11. Prepare `on_success`/`on_failure` triggers for the returned tuple (see
    [Life Cycles - Triggering handlers](./life-cycles.md#triggering-handlers)).

## `update`

1. **Ignore** -- evaluate `#[ignore]`/`#[ignore_update]`, then apply bare `#[ignore_update]`
   overrides.
2. **Nothing-to-update checkpoint 1** -- if no required/lax/virtual field actually submitted
   survives ignore/`#[readonly]` filtering, fail immediately with "nothing to update", before the
   required check even runs.
3. **Required** -- conditional `#[required(...)]` only (bare `#[required]` is creation-only).
   *Fails fast.*
4. **Validate** -- same as create's step 3. *Fails fast.*
5. **Re-validate** -- same as create's step 4. *Fails fast.*
6. **Post-validate** -- `pre_validate` then main `validate`, gated the same way as create's steps
   5-6. *Fails fast.*
7. **Evaluate update validity** -- recompute the change set and drop any field whose value turned
   out unchanged, once, right after post-validate (`raw_input()` still shows what was actually
   submitted; `input()` reflects this filtering). *Fails fast.*
8. **Nothing-to-update checkpoint 2** -- if nothing is left to change after step 7 *and* no virtual
   field is still relevant (its dependent(s) haven't resolved yet), fail immediately, before
   dependent resolution runs.
9. **Sanitize** -- same condition as create's step 7.
10. **Resolve dependents** -- one pass per dependency-graph level.
11. **Nothing-to-update checkpoint 3** -- if the change set is still empty after dependent
    resolution, fail with "nothing to update".
12. **Attach timestamps** -- `#[updated_at]`/`#[optional_updated_at]`.
13. Prepare `on_success`/`on_failure` triggers for the returned tuple (see
    [Life Cycles - Triggering handlers](./life-cycles.md#triggering-handlers)).

## Why three "nothing to update" checkpoints?

Each catches a different way an update can turn out to be a no-op: submitting only fields that get
filtered out before validation even starts (checkpoint 1), submitting a field with its current,
unchanged value (checkpoint 2), or submitting a virtual field whose dependent resolves back to the
value it already had (checkpoint 3, since a virtual field's relevance can't be known until its
dependent actually resolves). All three surface the same way: `update`'s `Err` payload is `None`
-- not a validation failure, just nothing to do.

```rust
let (err, _ctx_options) = DataInputModel.update(existing, updates, ()).unwrap_err();
assert!(err.is_none()); // "nothing to update", not a validation error
```
