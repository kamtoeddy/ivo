# `ivo` `/rs` vs `/rs-next` Benchmark Comparison

**Date**: 2026-09-01
**Command**: `cargo bench` and `cargo bench --features validators --bench main_demo` run first in `/rs`, then in `/rs-next`
**Runtime**: Criterion + Tokio multi-thread runtime (`Runtime::new()`)

These are fresh, same-machine results for both implementations.

## Throughput

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |
| --- | --- | --- | --- | --- |
| minimal create | 1.07 µs | 18.30 ns | -98.3% | ~54.64M |
| user create | 3.95 µs | 446.74 ns | -88.7% | ~2.24M |
| create 20 required fields (sync validators) | 16.85 µs | 138.79 ns | -99.2% | ~7.21M |
| dependent chain length 10 | 4.26 µs | 1.28 µs | -69.9% | ~780.39k |
| create 10 readonly lax fields | 9.58 µs | 1.41 µs | -85.3% | ~709.01k |
| no-op update | 329.85 ns | 164.09 ns | -50.3% | ~6.09M |
| single field update | 1.58 µs | 550.03 ns | -65.2% | ~1.82M |

## Memory Stress

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New per-op |
| --- | --- | --- | --- | --- |
| memory minimal create x1000 | 1.07 ms | 18.41 µs | -98.3% | 18.41 ns |
| memory user create x1000 | 3.95 ms | 438.46 µs | -88.9% | 438.46 ns |
| memory 20 fields create x1000 | 16.81 ms | 137.17 µs | -99.2% | 137.17 ns |
| memory no-op update x1000 | 327.98 µs | 163.26 µs | -50.2% | 163.26 ns |

## Main Demo

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |
| --- | --- | --- | --- | --- |
| main_demo create [fail: required errors (email or phone_number)] | 1.22 µs | 280.61 ns | -77.0% | ~3.56M |
| main_demo create [fail: required errors (email or phone_number, username)] | 768.48 ns | 282.31 ns | -63.3% | ~3.54M |
| main_demo create [fail: validation error (email, slug_id, username)] | 2.92 µs | 629.29 ns | -78.5% | ~1.59M |
| main_demo create [fail: re_validation error (username taken)] | 2.06 µs | 580.09 ns | -71.9% | ~1.72M |
| main_demo create [fail: post-validation error (slug taken)] | 3.71 µs | 1.61 µs | -56.7% | ~622.39k |
| main_demo create [success: 2/4 inputs (a)] | 5.14 µs | 2.47 µs | -51.9% | ~404.77k |
| main_demo create [success: 2/4 inputs (b)] | 5.06 µs | 2.44 µs | -51.8% | ~410.08k |
| main_demo create [success: 3/4 inputs] | 6.06 µs | 2.97 µs | -50.9% | ~336.15k |
| main_demo create [success: 4/4 inputs] | 7.27 µs | 3.70 µs | -49.1% | ~270.43k |
| main_demo update [fail: required error (email or phone_number)] | 1.92 µs | 930.00 ns | -51.5% | ~1.08M |
| main_demo update [fail: validation error (email, slug_id, username)] | 2.66 µs | 848.00 ns | -68.2% | ~1.18M |
| main_demo update [fail: re_validation error (username taken)] | 1.91 µs | 751.61 ns | -60.7% | ~1.33M |
| main_demo update [fail: post-validation error (slug taken)] | 2.33 µs | 1.81 µs | -22.3% | ~553.15k |
| main_demo update [fail: nothing to update: 1/4 inputs (a)] | 581.94 ns | 271.39 ns | -53.4% | ~3.68M |
| main_demo update [fail: nothing to update: 1/4 inputs (b)] | 590.89 ns | 266.93 ns | -54.8% | ~3.75M |
| main_demo update [fail: nothing to update: 1/4 inputs (c)] | 3.21 µs | 2.01 µs | -37.4% | ~497.74k |
| main_demo update [fail: nothing to update: 1/4 inputs (d)] | 583.85 ns | 264.97 ns | -54.6% | ~3.77M |
| main_demo update [fail: nothing to update: 2/4 inputs] | 734.23 ns | 317.32 ns | -56.8% | ~3.15M |
| main_demo update [fail: nothing to update: 3/4 inputs] | 862.38 ns | 365.69 ns | -57.6% | ~2.73M |
| main_demo update [fail: nothing to update: 4/4 inputs] | 3.80 µs | 2.36 µs | -38.0% | ~424.18k |
| main_demo update [success: 1/4 inputs (a)] | 2.48 µs | 1.57 µs | -36.5% | ~636.12k |
| main_demo update [success: 1/4 inputs (b)] | 2.39 µs | 1.55 µs | -35.1% | ~644.55k |
| main_demo update [success: 1/4 inputs (c)] | 4.41 µs | 3.06 µs | -30.7% | ~327.04k |
| main_demo update [success: 1/4 inputs (d)] | 4.88 µs | 2.99 µs | -38.8% | ~334.69k |
| main_demo update [success: 3/4 inputs] | 5.12 µs | 3.24 µs | -36.7% | ~308.49k |
| main_demo update [success: 4/4 inputs] | 6.57 µs | 4.29 µs | -34.7% | ~233.08k |
| main_demo delete | 177.50 ns | 11.98 ns | -93.3% | ~83.46M |

## Main Demo Memory Stress

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New per-op |
| --- | --- | --- | --- | --- |
| memory main_demo create x1000 | 7.30 ms | 3.67 ms | -49.7% | 3.67 µs |
| memory main_demo update x1000 | 6.54 ms | 4.25 ms | -34.9% | 4.25 µs |
| memory main_demo nothing to update x1000 | 3.78 ms | 2.34 ms | -38.0% | 2.34 µs |
| memory main_demo delete x1000 | 177.40 µs | 12.03 µs | -93.2% | 12.03 ns |

## Notes

- Summary: 42/42 benchmarks matched on both sides. No regressions -- `/rs-next` ranges from -99.2% to -22.3% relative to `/rs` across everything measured.
- Lower time is better; negative `Change` means `/rs-next` is faster.
- The new `update` API takes the existing data by value, so the update
  harnesses clone the data each iteration. The old API borrowed the data.
- Both projects use the same release-profile tuning (`lto = true`,
  `codegen-units = 1`) and the same Tokio runtime.
- `main_demo` benchmarks exercise the same realistic schema (constants,
  lax/required/dependent/virtual fields, timestamps, grouped validation,
  post-validation, hooks) in both implementations, across every distinct
  outcome the schema can produce: each `create`/`update` failure mode
  (required/validation/re-validation/post-validation), each success shape
  (partial vs. full input), every `update [fail: nothing to update: ...]`
  no-op-resubmission case, and `delete`.
- `Main Demo Memory Stress` applies the same x1000-per-sample shape as
  `Memory Stress` (above) to `main_demo`'s realistic schema instead of
  the synthetic ones: full-input `create`, full-input `update`, an
  all-unchanged `update` (the no-op/"nothing to update" fast path), and
  `delete`, each looped 1000x per sample so `New per-op` isolates
  steady-state cost from one-off benchmark-harness overhead.
- Criterion sanitizes benchmark names into directory names under
  `target/criterion/` (replacing filesystem-unsafe characters with `_` and
  truncating to 64 characters); `criterion_dir_name()` in this script
  replicates both so lookups for the longer `main_demo` names don't miss.

## Partial output at creation (2026-10-18)

`create` now builds a partial output (`ctx.values()` is the partial output struct) and converts it
into the full output at the end; dependent defaults, constants and timestamps are only computed
once validation has passed. Same machine, `HEAD` before the change vs after, Criterion with
`--warm-up-time 1 --measurement-time 3`.

| Benchmark | Before | After | Change |
| --- | --- | --- | --- |
| minimal create | 56.35 ns | 56.91 ns | +1.0% |
| user create | 488.58 ns | 523.96 ns | +7.2% |
| create 20 required fields (sync validators) | 169.52 ns | 201.29 ns | +18.7% |
| dependent chain length 10 | 1.42 µs | 1.46 µs | +2.5% |
| create 10 readonly lax fields | 1.57 µs | 1.61 µs | +2.6% |
| no-op update | 218.06 ns | 205.81 ns | -5.6% |
| single field update | 594.96 ns | 576.87 ns | -3.0% |
| main_demo create [fail: required errors (email or phone_number)] | 276.41 ns | 292.66 ns | +5.9% |
| main_demo create [fail: validation error (email, slug_id, username)] | 632.87 ns | 612.29 ns | -3.3% |
| main_demo create [fail: re_validation error (username taken)] | 709.53 ns | 556.12 ns | -21.6% |
| main_demo create [fail: post-validation error (slug taken)] | 1.63 µs | 1.55 µs | -4.8% |
| main_demo create [success: 4/4 inputs] | 3.81 µs | 3.77 µs | -1.1% |

The remaining cost on all-integer schemas ("create 20 required fields") is the partial
representation itself (seeding, `Option` wrapping and the final conversion), about 1.5 ns per field.

## Fewer `ctx` rebuilds, clones and allocations (2026-10-18)

`create`/`update` codegen and `IvoContext` changes. Handler-facing APIs are unchanged; only the
hidden `IvoContext::new` (called by generated code) now takes the shared parts as `Arc`s:

- A `ctx` snapshot is only rebuilt where a later handler reads it, and after a phase's fail-fast
  check rather than before it.
- `IvoContext` shares the never-mutated raw input and previous values through `Arc`s instead of
  cloning them into every snapshot, and each method reassigns a single `ctx` binding so replaced
  snapshots are dropped right away instead of living on in the method's future.
- The context options are moved back out on return instead of locked and cloned, and the
  read-only handle is only created for triggers.
- Triggers build their `ctx` from the pipeline's state once (moving what's no longer needed)
  instead of cloning it twice, and grouped `on_success` relevance uses per-field `bool`s instead of
  a `HashSet`.
- Dependent resolution tracks changed parents in `[bool; N]` arrays instead of a `HashSet` per
  round, reuses `ctx` within a round, and skips rebuilds when nothing changed or nothing is left to
  read them.
- `update` defers copying the stored record and building its first `ctx` until after the early
  "nothing to update" check (when no ignore handler reads `ctx`), reuses its relevance flags, and
  recomputes changes in a single pass.
- Values handed to validators, sanitizers and post-validate merges are moved instead of cloned
  twice where the original isn't needed afterwards.

Same machine, `HEAD` (`289776de`) before the change vs after, run one after the other,
Criterion with `--warm-up-time 1 --measurement-time 3` (`cargo bench --bench throughput` and
`cargo bench --features validators --bench main_demo`). Times are Criterion's median estimate.

### Throughput

| Benchmark | Before | After | Change |
| --- | --- | --- | --- |
| minimal create | 56.71 ns | 28.09 ns | -50.5% |
| user create | 517.37 ns | 255.96 ns | -50.5% |
| create 20 required fields (sync validators) | 201.10 ns | 151.70 ns | -24.6% |
| dependent chain length 10 | 1.44 µs | 403.65 ns | -72.0% |
| create 10 readonly lax fields | 1.62 µs | 789.21 ns | -51.3% |
| no-op update | 202.75 ns | 56.65 ns | -72.1% |
| single field update | 564.10 ns | 181.01 ns | -67.9% |

### Main Demo

| Benchmark | Before | After | Change |
| --- | --- | --- | --- |
| main_demo create [fail: required errors (email or phone_number)] | 310.31 ns | 246.25 ns | -20.6% |
| main_demo create [fail: required errors (email or phone_number, username)] | 281.74 ns | 245.22 ns | -13.0% |
| main_demo create [fail: validation error (email, slug_id, username)] | 670.43 ns | 410.86 ns | -38.7% |
| main_demo create [fail: re_validation error (username taken)] | 582.93 ns | 425.43 ns | -27.0% |
| main_demo create [fail: post-validation error (slug taken)] | 1.77 µs | 1.29 µs | -27.4% |
| main_demo create [success: 2/4 inputs (a)] | 2.71 µs | 1.53 µs | -43.4% |
| main_demo create [success: 2/4 inputs (b)] | 2.64 µs | 1.49 µs | -43.6% |
| main_demo create [success: 3/4 inputs] | 3.13 µs | 1.80 µs | -42.7% |
| main_demo create [success: 4/4 inputs] | 3.98 µs | 2.19 µs | -45.0% |
| main_demo update [fail: required error (email or phone_number)] | 961.92 ns | 564.16 ns | -41.4% |
| main_demo update [fail: validation error (email, slug_id, username)] | 924.14 ns | 503.04 ns | -45.6% |
| main_demo update [fail: re_validation error (username taken)] | 892.80 ns | 513.25 ns | -42.5% |
| main_demo update [fail: post-validation error (slug taken)] | 1.92 µs | 1.37 µs | -28.6% |
| main_demo update [fail: nothing to update: 1/4 inputs (a)] | 341.27 ns | 242.59 ns | -28.9% |
| main_demo update [fail: nothing to update: 1/4 inputs (b)] | 368.72 ns | 242.38 ns | -34.3% |
| main_demo update [fail: nothing to update: 1/4 inputs (c)] | 2.34 µs | 1.43 µs | -38.9% |
| main_demo update [fail: nothing to update: 1/4 inputs (d)] | 340.10 ns | 237.96 ns | -30.0% |
| main_demo update [fail: nothing to update: 2/4 inputs] | 391.72 ns | 272.27 ns | -30.5% |
| main_demo update [fail: nothing to update: 3/4 inputs] | 431.55 ns | 304.62 ns | -29.4% |
| main_demo update [fail: nothing to update: 4/4 inputs] | 2.81 µs | 1.56 µs | -44.5% |
| main_demo update [success: 1/4 inputs (a)] | 1.76 µs | 942.06 ns | -46.4% |
| main_demo update [success: 1/4 inputs (b)] | 1.84 µs | 912.62 ns | -50.4% |
| main_demo update [success: 1/4 inputs (c)] | 3.43 µs | 2.39 µs | -30.4% |
| main_demo update [success: 1/4 inputs (d)] | 3.18 µs | 1.93 µs | -39.3% |
| main_demo update [success: 3/4 inputs] | 3.54 µs | 1.86 µs | -47.4% |
| main_demo update [success: 4/4 inputs] | 4.57 µs | 2.58 µs | -43.6% |
| main_demo delete | 13.70 ns | 12.05 ns | -12.1% |

### Main Demo Memory Stress

| Benchmark | Before | After | Change |
| --- | --- | --- | --- |
| memory main_demo create x1000 | 3.98 ms | 2.21 ms | -44.4% |
| memory main_demo update x1000 | 4.75 ms | 2.57 ms | -45.8% |
| memory main_demo nothing to update x1000 | 2.49 ms | 1.53 ms | -38.5% |
| memory main_demo delete x1000 | 15.28 µs | 11.85 µs | -22.5% |

### Allocations per call

Heap allocations (count and bytes requested) during one call, counted with a global allocator
wrapper in a release build, plus the size of the returned future. Building the input (or cloning
the stored record for `update`) is not counted.

| Call | Before | After |
| --- | --- | --- |
| minimal create | 3 allocs, 200 B, 240 B future | 2 allocs, 80 B, 200 B future |
| user create | 39 allocs, 476 B, 2000 B future | 17 allocs, 267 B, 1240 B future |
| no-op update | 12 allocs, 269 B, 2192 B future | 1 alloc, 56 B, 1424 B future |
| single field update | 39 allocs, 476 B, 2192 B future | 9 allocs, 294 B, 1424 B future |
| dependent chain length 10 create | 13 allocs, 960 B, 5584 B future | 2 allocs, 80 B, 4448 B future |
| main_demo create [success: 4/4 inputs] | 206 allocs, 6964 B, 5480 B future | 114 allocs, 4174 B, 2528 B future |
| main_demo create [fail: required errors] | 15 allocs, 988 B, 5480 B future | 13 allocs, 949 B, 2528 B future |
| main_demo update [success: 4/4 inputs] | 250 allocs, 7779 B, 5768 B future | 115 allocs, 4779 B, 3336 B future |

### Notes

- Every benchmark is faster; `main_demo` creates and updates run 13-50% faster, the no-op update
  and the 10-level dependent chain about 72% faster.
- Sharing the raw input through an `Arc` costs one allocation per call, which shows on schemas
  whose input has no heap data at all (`minimal create` would allocate once rather than twice
  without it); it removes far more on schemas whose input holds `String`s (`main_demo`).
- `main_demo`'s "nothing to update" cases can't defer the stored record's copy and the first
  `ctx`, since its schema has ignore handlers that read `ctx` before the early check.
