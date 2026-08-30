Found it — need `async-await-macro` (which brings `futures-macro`), not just `async-await`.

Works. Let's clean up the scratch example and proceed with the actual codegen changes.

Let me simplify this to remove the redundant/dead field:

Now let's build to check for errors before continuing further.

Builds cleanly. Let's run the full test suite to check for regressions from the create-side dependent parallelization.

Now let's append the concurrency test with schema at the end of the file.

Let's verify the test actually detects sequential execution (not a false positive) by temporarily forcing the sequential path.

Confirmed — the test correctly fails on the sequential path. Let's revert the threshold and re-verify everything passes.

Both compile cleanly. Let's run the full test suite.

Let's add an update-side concurrency test on the same schema (already has two independent async dependents).

Confirmed — fails on the update path too when forced sequential. Let's revert and re-run the full suite.

Now let's fix the 6 splice sites from `#(#x)*` to `#x`:

Now let's compile and test what we have so far before continuing to the non-virtual re_validate phase.

Good, compiles. Let's run the full suite.

Passes. Let's verify it's not a false positive by forcing sequential mode temporarily.

Confirmed as a genuine test. Let's revert and run the full suite.

Both create and update virtual-field parallelism confirmed. Now let's run the full suite once more, then proceed to the remaining phases: non-virtual `re_validate` and update's non-virtual field validate.

Let me clean this up properly:

Now let's fix the two splice sites for `re_validate_steps` (previously a Vec, now a single TokenStream):

Now let's build and test.

Let's verify it's not a false positive and run the full suite.

Confirmed genuine. Reverting and running the full suite.

Now let's replace this block:

Now let's compile and run the full test suite to catch this and any other regressions.

Let's verify it's genuine (not a false positive) then run the full suite.

Confirmed genuine. Reverting and running the full suite.

497 passing. Let's update TODO.md and task tracking to reflect the scope covered and what remains.

Parallelization work is done, tested, and stable. Here's the summary:

**What was done:**

- Added `futures-util` (`join!` macro only — no executor, no boxing/allocation) as a real dependency, replacing per-field sequential `.await` chains with genuine concurrent polling wherever ≥2 independent async handlers exist in a phase.
- Built a shared codegen helper (`emit_async_phase`) that: stays sequential when 0–1 handlers in a phase are async (zero overhead, per GOAL.md §15), and batches via `join!` when ≥2 are async.
- Applied it to: virtual validate/re-validate/sanitize (create + update), non-virtual re-validate, non-virtual field validate on update, and dependent-field resolution (create's dynamic rounds, update's static dependency levels).
- Each got a **rendezvous-based concurrency test** that only passes if the handlers genuinely overlap in flight — I verified every one of them by temporarily forcing the sequential path and confirming the test fails, then reverted.
- 497 tests passing (up from 481 at the start of this session), zero warnings.

**Deliberately left sequential** (documented in TODO.md as follow-ups, not silently dropped):

- `create`'s combined required/lax/constant/dependent-default pass — constant resolvers can read sibling `ctx.values()`, so ordering may be load-bearing there.
- `post_validate` across multiple `#[post_validate(...)]` groups — rs/'s reference batches all groups' handlers together, but rs-next threads `ctx` through groups sequentially; needs a small architecture change, not a mechanical port.
- `on_success`/`on_failure`/`on_delete` triggers — lower priority since they don't block the `create`/`update` call itself.

Both TODO.md and the task tracker are updated. Ready for the next item — want me to continue with the `SKIPPED:` marker audit next?
