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
