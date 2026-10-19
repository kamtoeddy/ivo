# `ivo` (TypeScript) performance & correctness tasks

Findings from the 2026-10-19 review (benchmark baseline + three code reviews). Work proceeds
phase by phase, in order; each task is checked off with a short note on what changed and what it
measured.

Status: `[ ]` todo · `[~]` in progress · `[x]` done · `[-]` dropped (with reason)

## Decisions taken (override if needed)

- **Shared ctx (finding 1):** a ctx is reused only while the state it reads (input, values,
  previous values, options, update flag) is unchanged, and rebuilt when it changes, so handlers
  see exactly the values they saw before; only object identity is shared.
- **Sync handlers (finding 2):** handlers still run in the same order, and a handler that throws
  synchronously is caught on its own (like `Promise.allSettled` today), so later handlers still run.
- **`ignoreUpdate` (bug 5):** called once per field per update instead of twice, still including
  unchanged fields (no change in which fields it is called for).

## Baseline (Bun 1.4.3, before any change)

| Benchmark | Mean |
| --- | --- |
| minimal create | 5.1 µs |
| user create | 6.4 µs |
| create 50 / 100 required fields | 31 / 66 µs |
| 20 virtuals with sanitizers | 37 µs |
| create 50 dynamic ignore fields | 66 µs |
| 20 fields reading `ctx.options` | 20 µs |
| no-op update | 1.3 µs |
| single field update | 6.7 µs |
| update 50 fields unchanged | 12.5 µs |

## Baseline with the fixed harness (H6; Bun 1.4.3, after Phase 1)

Average of two runs, each benchmark in its own process; run-to-run difference: median 1.9%,
worst 6.1%. Raw results: `tests/bench/results/baseline-v2-run{1,2}.json` (gitignored).

| Benchmark | Median |
| --- | --- |
| minimal create | 5.16 µs |
| user create | 6.33 µs |
| create 50 required fields (sync validators) | 29.74 µs |
| create 50 required fields (async validators) | 29.48 µs |
| create 100 required fields | 62.48 µs |
| allow list validation (100 items) | 5.26 µs |
| dependent chain length 10 | 17.62 µs |
| dependent chain length 10 (defaults only) | 4.79 µs |
| wide dependency 20 parents | 15.54 µs |
| 20 virtuals with sanitizers | 34.94 µs |
| create 50 readonly lax fields | 26.32 µs |
| create 50 dynamic ignore fields | 60.11 µs |
| 20 fields reading ctx.options repeatedly | 17.69 µs |
| realistic create (success) | 12.76 µs |
| realistic create (username taken) | 5.16 µs |
| realistic update (username + dependents) | 10.55 µs |
| realistic update (nothing to update) | 1.86 µs |
| validator: string | 43 ns |
| validator: number | 39 ns |
| validator: email | 58 ns |
| validator: credit card | 634 ns |
| validator: array unique by key (200 objects) | 9.93 µs |
| validator: array unique (200 objects) | 30.22 µs |
| validator: array sort (200 numbers) | 13.17 µs |
| no-op update | 1.05 µs |
| single field update | 5.24 µs |
| update 50 fields unchanged | 10.09 µs |

| Leak check | Retained |
| --- | --- |
| minimal create retained memory | 21.8 B/op |
| user create retained memory | 1.1 B/op |
| create 100 fields retained memory | 0.0 B/op |
| handleSuccess retained memory | 18.1 B/op |
| ctx.options clone allocation | 2.7 B/op |
| update no-op retained memory | 5.0 B/op |
| realistic create retained memory | 9.0 B/op |

## Phase 1 — Bugs

- [x] **B1** `isEqual(5, {})` and `isEqual(new Date(), {})` return `true` (`src/utils/index.ts`)
  - done: different kinds (number vs object, Date vs non-Date, array vs object) are never equal
- [x] **B2** async array `filter` doesn't filter: the returned promise is always truthy (`src/validators.ts`)
  - done: predicates are awaited before filtering (`_filterBy`); same for `postModFilter`
- [x] **B3** `uniqueKey` treats `1` and `"1"` as the same key, keeps the last, and reorders (`src/utils/index.ts`)
  - done: `getUniqueBy` uses a Map: `1` and `"1"` stay apart, first-seen order kept, last item for a key wins
- [x] **B4** number validator throws on a `bigint` instead of rejecting it (`src/validators.ts`)
  - done: `isNaN` only checked for numbers; `10n` now validates to `10`
- [x] **B5** a required field's `ignoreUpdate` function runs twice per update (`src/model/index.ts`)
  - done: dropped the duplicate resolver task queued in the second loop
- [x] **B6** `delete()` freezes the caller's `data` and `options` when `onDelete` handlers exist (`src/model/index.ts`)
  - done: handlers get frozen copies (data spread, options via `cloneWithMethods`), built once per call

## Phase 2 — Benchmark harness (`tests/bench`)

- [x] **H1** force GC with `Bun.gc(true)` (fallback `globalThis.gc`); run the memory suite in its own process
  - done: `forceGc()` uses `Bun.gc(true)` (or `gc()` on Node); memory suite runs in its own process; reports retained B/op over 10k calls (a leak check)
- [x] **H2** several samples per benchmark; report the median and spread instead of one 1s mean
  - done: 7 windows of 200 ms after a 250 ms warmup, GC forced before each; median reported with min–max; each benchmark runs in its own process (JIT state from earlier benchmarks moved results by up to 52%)
- [x] **H3** "dependent chain length 10" must provide `field_0` so dependents actually resolve
  - done: `create({ field_0: 5 })` now resolves all 9 dependents (18 µs); the old variant kept as "(defaults only)"
- [x] **H4** add a realistic end-to-end scenario (like Rust's `main_demo`) and an update that does real work
  - done: `realisticUserSchema` (mirrors Rust `main_demo`): create success / username taken, update with dependents / nothing to update
- [x] **H5** add benchmarks for the built-in validators
  - done: string, number, email, credit card, array unique by key / unique / sort
- [x] **H6** record a new baseline with the fixed harness
  - done: recorded above

## Phase 3 — Main performance findings (`src/model/index.ts`, `src/utils/index.ts`)

- [x] **P1** build the ctx once per state instead of on every `_getContext`/`_getReadonlyCtx` call
  - done: ctx built as one frozen literal (no descriptor copying) and reused while the 6 references it captures are unchanged; 1.1–2.5× (minimal create 5.16→2.40 µs, dynamic ignores 60→24 µs, realistic create 12.8→7.8 µs, single field update 5.2→2.7 µs)
- [x] **P2** skip phases with nothing to do; call sync handlers directly instead of `Promise.allSettled`/`Promise.all`
  - done: `src/model/settle.ts` (`settleEach` / `settleAll` / `allOrThrow`) replaces every phase's `Promise.all(Settled)`, keeping each phase's semantics (handlers called first and in order; results applied per item as they settle, or all at once, as before); phases return plain values when all handlers are sync and `create`/`update` only `await` real promises; empty phases (re-validate, sanitize, post-validate, required, defaults, constants) return before building a ctx. 8 new tests (`tests/settle.test.ts`). With P1: 1.6–4.8× (minimal create 5.16→1.12 µs, dependent chain 17.6→3.6 µs, realistic create 12.8→4.7 µs, single field update 5.2→1.55 µs)
- [x] **P3** per-schema plan: definitions by kind, dependency graph (parent → dependents), `onDelete` list
  - done: `SchemaPlan` cached per schema (like the field-info map): constants, fields that get defaults, fields that can be required, dependents with their parents, fields' `onDelete` handlers — all in definition order, so handlers run in the same order; replaces five per-call scans of every definition. Another 5–42% on creates (defaults-only chain 1.48→0.86 µs, 50 required 13.7→10.6 µs); 1.9–6.1× in total
- [x] **P4** fast clone for plain data instead of `structuredClone` of the input / stored record
  - done: `deepCloneValue` clones plain data in JS (Dates, plain arrays/objects; shared references and cycles kept shared, like `structuredClone`), arrays of primitives natively with `structuredClone`, and falls back to the old `structuredClone`/JSON path for anything else; 11 tests compare it with `structuredClone` (`tests/utils/deep-clone.test.ts`). 5–59% faster (realistic nothing-to-update 1.90→0.78 µs, 50 fields unchanged 10.1→7.2 µs, realistic update 3.9→2.7 µs); an early version made `array sort` 22% slower (`Object.keys` on a 200-number array costs 3 µs), fixed by cloning primitive arrays natively
- [x] **P5** update filtering: one pass writing only relevant fields, instead of add-then-`delete`
  - done: `_cleanUpdateInput` builds `input`/`output` in one pass that never writes ignored or unchanged fields (their `ignoreUpdate` resolvers only run after filtering, so nothing can see the difference); `_cleanInput` iterates with `for…in` instead of `Object.entries`. Update 50 fields unchanged 7.2→2.9 µs (−60%), no-op update −16%, creates another 3–14%

## Phase 4 — Remaining findings

- [x] **P6** `FieldInfoCollection`: drop redundant Set rebuilds and the dead assignment (`model:776`)
  - done: dropped the dead `relevantFieldsProvided` assignment; config names of relevant fields only built (as a Set) when grouped `ignore`/`ignoreUpdate` exist; without aliases (`SchemaPlan.hasAliases`) the setter copies the set instead of a Map lookup per field
- [x] **P7** cheaper "nothing to update" / failure paths (no ctx just to read options; lazy handles)
  - done: `_handleUpdateError` clones the options directly instead of building a ctx to read them; `handleFailure`/`handleSuccess` are a shared no-op when there's no handler to run (`SchemaPlan.hasSuccessHandlers`), and the failure ctx is only built when there are failure handlers
- [x] **P8** `isEqual`: identity fast path and key-by-key comparison instead of sort + `JSON.stringify`
  - done: key sets compared with a length check + `hasOwn` instead of sorting and `JSON.stringify`-ing both key lists; same results. 3-key nested object 1167→762 ns, 5-key flat object 299→56 ns. No `a === b` shortcut: it would make an object holding `NaN` equal to itself, which it isn't today
- [x] **P9** validators: `unique` without a JSON round-trip, `sort` without a deep clone, single-pass credit card check
  — credit card: one `charCodeAt` Luhn pass, 634 → 57 ns (11×), identical verdicts to HEAD on 649,740
  inputs. `unique`: keeps the first original item per serialized key, 30.2 → 16.0 µs (1.9×).
  `sort`: sorts the already-fresh array in place, 13.2 → 11.7 µs. **Behaviour change:** `unique`
  now returns the original items (not `JSON.parse`d copies, so `Date`s stay `Date`s).

## Log

- **2026-10-19 · Phase 1 done.** B1–B6 fixed, each with a regression test (`tests/utils/index.ts`,
  `tests/built-in-validators/validate-{array,number}.test.ts`, `tests/regressions.test.ts`).
  648 tests pass (642 before + 6 new); typecheck and Biome clean.
- **2026-10-19 · Phase 2 done.** The harness takes medians over GC-reset windows, isolates each
  benchmark in its own process, and has realistic and validator benchmarks; the memory suite is a
  real leak check now. New baseline recorded above.
- **2026-10-19 · Phase 3 done.** P1–P5: 1.6–6.6× across every model benchmark (minimal create
  5.16→0.85 µs, dependent chain 17.6→2.7 µs, realistic create 12.8→3.6 µs, realistic update
  10.6→2.7 µs, single field update 5.2→1.2 µs, 50 unchanged fields 10.1→2.9 µs), nothing slower.
  667 tests pass.
- **2026-10-19 · Phase 4 done.** P6–P9. Cumulative vs the fixed-harness baseline: 2.7–8.5× on every
  model benchmark (minimal create 5.16 µs→731 ns, dependent chain 17.6→2.5 µs, 20 virtuals
  34.9→5.0 µs, 50 dynamic ignore fields 60.1→7.1 µs, realistic create 12.8→3.5 µs, realistic
  update 10.6→2.6 µs, single field update 5.2→1.0 µs, no-op update 1.05 µs→395 ns); credit card
  11×, array unique 1.9×; scalar validators and `unique by key` unchanged (within noise). Leak check
  clean (≤ 20 B/op everywhere); the `handleSuccess` bench now runs 10k iterations instead of 100,
  whose ~28 KB totals had read as 230–375 B/op of noise since P3. 668 tests pass; typecheck and
  Biome clean.
