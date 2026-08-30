Here's my proposal for each, in order of increasing effort/risk:

### 1. Lifecycle hook triggers (`on_success`/`on_failure`/`on_delete`) — lowest effort, do this first

Mechanically this is the same pattern I already built (`AsyncPhaseItem` + `emit_async_phase`), just not wired up yet. Right now the 4 builder functions (`create_success_stmts`, `create_failure_stmts`, `update_success_stmts`, `update_failure_stmts`, plus the grouped-`on_success` one) track only an _aggregate_ "is anything async" bool and emit `Vec<TokenStream>` of `if condition { call.await; }` statements that `make_trigger` awaits one after another inside a single `async move { ... }` block.

Proposal: change those builders to return `Vec<AsyncPhaseItem>` (per-handler `is_async`, `value_expr = if condition { call }`, `apply = {}` since the side effect already happened during evaluation), then have `make_trigger` call `emit_async_phase` on the merged list instead of splatting `#(#stmts)*`. Each hook is already documented as independent (no declared ordering between multiple `on_success` handlers), so this is safe by design, not just by absence-of-evidence. Rough effort: touching ~5 call sites, no new semantic questions to resolve. I'd want one rendezvous test per trigger kind (create-success, create-failure, update-success, on_delete) to prove it, same as before.

### 2. `create`'s required/lax/constant/dependent-default pass — medium effort, but the risk is narrower than I first described

I need to correct something: after re-reading `rs/`'s reference `validate()` more closely, required/lax field validators are _already_ documented and implemented there as safe to parallelize among themselves (no cross-field visibility contract) — I was overcautious lumping them in with constants. The actual risk is isolated to **constants**, because their ctx type (`IvoConstantCtx<I,O>`) exposes `ctx.values()`, i.e. sibling output.

Proposal:

- Split the current single `create_steps` loop into two separate phases, matching GOAL.md §17's own ordering (which rs-next currently violates by interleaving them): a **required/lax phase** (unchanged position, batched via `emit_async_phase` against one pre-phase `ctx` snapshot — same "gather then apply" pattern already used for virtual fields), and a **constants phase** moved to run _after_ dependent resolution (step 9, not step 4), also batched via `emit_async_phase` among themselves.
- This fixes a real, separate pre-existing ordering bug I noticed while investigating this (constants currently get attached _before_ dependents resolve, contradicting the spec's own step order) as a side effect of doing the split correctly — worth flagging to you now rather than silently bundling it in.
- Risk: this changes when constants get attached relative to dependent resolution, which could be observable if any existing test/schema has a constant resolver reading a dependent's freshly-resolved value. I'd run the full suite and watch for that specifically.

### 3. `post_validate` across multiple groups — highest effort, and involves a real behavior decision

`rs/`'s reference implementation batches **all** groups' `pre_validate` handlers together (one `join_all`, against a single shared ctx snapshot), applies all their results, rebuilds ctx once, _then_ batches all groups' main `validate` handlers together the same way — a strict two-phase, all-groups-at-once structure.

rs-next currently processes groups one at a time, sequentially, and — this is the important part — **group 2 currently sees group 1's already-applied changes** (ctx gets rebuilt after each group). Restructuring to match `rs/`'s two-phase batching would remove that cross-group visibility. That's arguably _more correct_ relative to the documented spec (post_validate groups aren't specified as ordered/dependent on each other), but it's a genuine behavior change I shouldn't make silently.

Proposal: build it as `rs/` does (collect all groups' `pre_validate` into one `emit_async_phase` batch → apply all → single ctx rebuild → early-return check → collect all groups' `validate` into a second batch → apply → ctx rebuild), but I'd want to explicitly test the "does group 2 need to see group 1's changes" question against the current test suite first, and flag it to you if anything depends on that ordering, before treating it as done.

**My recommended order:** 1 → 2 → 3, stopping to check in with you if #3's cross-group visibility question turns out to matter for real schemas. Want me to proceed on that basis?
