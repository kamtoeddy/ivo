# `ivo` `/rs` vs `/rs-next` Benchmark Comparison

**Date**: 2026-08-31
**Command**: `cargo bench` and `cargo bench --features validators --bench main_demo` run first in `/rs`, then in `/rs-next`
**Runtime**: Criterion + Tokio multi-thread runtime (`Runtime::new()`)

These are fresh, same-machine results for both implementations.

## Throughput

| Benchmark                                   | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |
| ------------------------------------------- | ----------- | ---------------- | ------ | --------- |
| minimal create                              | 1.07 µs     | 18.25 ns         | -98.3% | ~54.79M   |
| user create                                 | 3.99 µs     | 493.33 ns        | -87.6% | ~2.03M    |
| create 20 required fields (sync validators) | 17.03 µs    | 138.56 ns        | -99.2% | ~7.22M    |
| dependent chain length 10                   | 4.30 µs     | 1.27 µs          | -70.4% | ~786.90k  |
| create 10 readonly lax fields               | 9.56 µs     | 1.63 µs          | -82.9% | ~612.10k  |
| no-op update                                | 331.45 ns   | 129.51 ns        | -60.9% | ~7.72M    |
| single field update                         | 1.58 µs     | 408.36 ns        | -74.2% | ~2.45M    |

## Memory Stress

| Benchmark                     | Old (`/rs`) | New (`/rs-next`) | Change | New per-op |
| ----------------------------- | ----------- | ---------------- | ------ | ---------- |
| memory minimal create x1000   | 1.07 ms     | 18.17 µs         | -98.3% | 18.17 ns   |
| memory user create x1000      | 4.02 ms     | 501.57 µs        | -87.5% | 501.57 ns  |
| memory 20 fields create x1000 | 16.83 ms    | 138.64 µs        | -99.2% | 138.64 ns  |
| memory no-op update x1000     | 328.61 µs   | 128.67 µs        | -60.8% | 128.67 ns  |

## Main Demo

| Benchmark                                                                  | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |
| -------------------------------------------------------------------------- | ----------- | ---------------- | ------ | --------- |
| main_demo create [fail: required errors (email or phone_number)]           | 1.23 µs     | 289.11 ns        | -76.4% | ~3.46M    |
| main_demo create [fail: required errors (email or phone_number, username)] | 767.55 ns   | 289.20 ns        | -62.3% | ~3.46M    |
| main_demo create [fail: validation error (email, slug_id, username)]       | 2.97 µs     | 648.04 ns        | -78.2% | ~1.54M    |
| main_demo create [fail: re_validation error (username taken)]              | 2.08 µs     | 663.88 ns        | -68.0% | ~1.51M    |
| main_demo create [fail: post-validation error (slug taken)]                | 3.74 µs     | 1.78 µs          | -52.3% | ~561.00k  |
| main_demo create [success: 2/4 inputs (a)]                                 | 5.19 µs     | 2.85 µs          | -45.1% | ~351.04k  |
| main_demo create [success: 2/4 inputs (b)]                                 | 5.09 µs     | 2.80 µs          | -44.9% | ~356.56k  |
| main_demo create [success: 3/4 inputs]                                     | 6.14 µs     | 3.48 µs          | -43.3% | ~287.50k  |
| main_demo create [success: 4/4 inputs]                                     | 7.34 µs     | 4.17 µs          | -43.3% | ~240.08k  |
| main_demo update [fail: required error (email or phone_number)]            | 1.91 µs     | 635.35 ns        | -66.8% | ~1.57M    |
| main_demo update [fail: validation error (email, slug_id, username)]       | 2.71 µs     | 739.94 ns        | -72.7% | ~1.35M    |
| main_demo update [fail: re_validation error (username taken)]              | 1.91 µs     | 597.05 ns        | -68.7% | ~1.67M    |
| main_demo update [fail: post-validation error (slug taken)]                | 2.32 µs     | 1.48 µs          | -36.1% | ~674.55k  |
| main_demo update [fail: nothing to update: 1/4 inputs (a)]                 | 577.39 ns   | 224.79 ns        | -61.1% | ~4.45M    |
| main_demo update [fail: nothing to update: 1/4 inputs (b)]                 | 576.41 ns   | 221.37 ns        | -61.6% | ~4.52M    |
| main_demo update [fail: nothing to update: 1/4 inputs (c)]                 | 3.13 µs     | 1.63 µs          | -48.0% | ~614.73k  |
| main_demo update [fail: nothing to update: 1/4 inputs (d)]                 | 569.94 ns   | 216.48 ns        | -62.0% | ~4.62M    |
| main_demo update [fail: nothing to update: 2/4 inputs]                     | 720.70 ns   | 275.50 ns        | -61.8% | ~3.63M    |
| main_demo update [fail: nothing to update: 3/4 inputs]                     | 859.84 ns   | 327.05 ns        | -62.0% | ~3.06M    |
| main_demo update [fail: nothing to update: 4/4 inputs]                     | 3.74 µs     | 1.96 µs          | -47.5% | ~509.26k  |
| main_demo update [success: 1/4 inputs (a)]                                 | 2.43 µs     | 1.20 µs          | -50.5% | ~832.00k  |
| main_demo update [success: 1/4 inputs (b)]                                 | 2.38 µs     | 1.18 µs          | -50.6% | ~850.48k  |
| main_demo update [success: 1/4 inputs (c)]                                 | 4.35 µs     | 2.58 µs          | -40.6% | ~387.15k  |
| main_demo update [success: 1/4 inputs (d)]                                 | 4.82 µs     | 2.45 µs          | -49.2% | ~408.14k  |
| main_demo update [success: 3/4 inputs]                                     | 5.07 µs     | 2.70 µs          | -46.7% | ~370.08k  |
| main_demo update [success: 4/4 inputs]                                     | 6.57 µs     | 3.68 µs          | -44.1% | ~272.09k  |
| main_demo delete                                                           | 173.20 ns   | 12.13 ns         | -93.0% | ~82.44M   |

## Main Demo Memory Stress

| Benchmark                                | Old (`/rs`) | New (`/rs-next`) | Change | New per-op |
| ---------------------------------------- | ----------- | ---------------- | ------ | ---------- |
| memory main_demo create x1000            | 7.17 ms     | 4.21 ms          | -41.2% | 4.21 µs    |
| memory main_demo update x1000            | 6.43 ms     | 3.70 ms          | -42.5% | 3.70 µs    |
| memory main_demo nothing to update x1000 | 3.71 ms     | 1.98 ms          | -46.7% | 1.98 µs    |
| memory main_demo delete x1000            | 174.99 µs   | 11.98 µs         | -93.2% | 11.98 ns   |

## Notes

- Summary: 42/42 benchmarks matched on both sides. No regressions -- `/rs-next` ranges from -99.2% to -36.1% relative to `/rs` across everything measured.
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
