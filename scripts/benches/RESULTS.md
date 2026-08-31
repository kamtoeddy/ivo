# `ivo` `/rs` vs `/rs-next` Benchmark Comparison

**Date**: 2026-08-31
**Command**: `cargo bench` and `cargo bench --features validators --bench main_demo` run first in `/rs`, then in `/rs-next`
**Runtime**: Criterion + Tokio multi-thread runtime (`Runtime::new()`)

These are fresh, same-machine results for both implementations.

## Throughput

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |
| --- | --- | --- | --- | --- |
| minimal create | 1.07 µs | 19.15 ns | -98.2% | ~52.23M |
| user create | 3.92 µs | 536.59 ns | -86.3% | ~1.86M |
| create 20 required fields (sync validators) | 16.77 µs | 138.49 ns | -99.2% | ~7.22M |
| dependent chain length 10 | 4.24 µs | 1.26 µs | -70.2% | ~791.05k |
| create 10 readonly lax fields | 9.50 µs | 1.65 µs | -82.7% | ~607.78k |
| no-op update | 331.14 ns | 131.23 ns | -60.4% | ~7.62M |
| single field update | 1.56 µs | 407.12 ns | -73.9% | ~2.46M |

## Memory Stress

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New per-op |
| --- | --- | --- | --- | --- |
| memory minimal create x1000 | 1.06 ms | 18.25 µs | -98.3% | 18.25 ns |
| memory user create x1000 | 3.97 ms | 496.97 µs | -87.5% | 496.97 ns |
| memory 20 fields create x1000 | 16.67 ms | 137.23 µs | -99.2% | 137.23 ns |
| memory no-op update x1000 | 328.38 µs | 128.11 µs | -61.0% | 128.11 ns |

## Main Demo

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |
| --- | --- | --- | --- | --- |
| main_demo create [fail: required errors (email or phone_number)] | 1.21 µs | 292.41 ns | -75.9% | ~3.42M |
| main_demo create [fail: required errors (email or phone_number, username)] | 765.10 ns | 290.52 ns | -62.0% | ~3.44M |
| main_demo create [fail: validation error (email, slug_id, username)] | 2.92 µs | 659.08 ns | -77.5% | ~1.52M |
| main_demo create [fail: re_validation error (username taken)] | 2.05 µs | 653.13 ns | -68.2% | ~1.53M |
| main_demo create [fail: post-validation error (slug taken)] | 3.66 µs | 1.77 µs | -51.6% | ~564.66k |
| main_demo create [success: 2/4 inputs (a)] | 5.08 µs | 2.82 µs | -44.4% | ~354.14k |
| main_demo create [success: 2/4 inputs (b)] | 5.03 µs | 2.84 µs | -43.7% | ~352.69k |
| main_demo create [success: 3/4 inputs] | 6.11 µs | 3.48 µs | -43.0% | ~287.40k |
| main_demo create [success: 4/4 inputs] | 7.26 µs | 4.16 µs | -42.7% | ~240.28k |
| main_demo update [fail: required error (email or phone_number)] | 1.91 µs | 635.89 ns | -66.7% | ~1.57M |
| main_demo update [fail: validation error (email, slug_id, username)] | 2.66 µs | 740.61 ns | -72.1% | ~1.35M |
| main_demo update [fail: re_validation error (username taken)] | 1.91 µs | 598.86 ns | -68.6% | ~1.67M |
| main_demo update [fail: post-validation error (slug taken)] | 2.31 µs | 1.50 µs | -34.8% | ~664.64k |
| main_demo update [fail: nothing to update: 1/4 inputs (a)] | 582.03 ns | 230.64 ns | -60.4% | ~4.34M |
| main_demo update [fail: nothing to update: 1/4 inputs (b)] | 579.75 ns | 220.49 ns | -62.0% | ~4.54M |
| main_demo update [fail: nothing to update: 1/4 inputs (c)] | 3.15 µs | 1.62 µs | -48.7% | ~617.96k |
| main_demo update [fail: nothing to update: 1/4 inputs (d)] | 567.64 ns | 215.18 ns | -62.1% | ~4.65M |
| main_demo update [fail: nothing to update: 2/4 inputs] | 719.20 ns | 274.61 ns | -61.8% | ~3.64M |
| main_demo update [fail: nothing to update: 3/4 inputs] | 848.56 ns | 315.36 ns | -62.8% | ~3.17M |
| main_demo update [fail: nothing to update: 4/4 inputs] | 3.76 µs | 1.98 µs | -47.4% | ~506.26k |
| main_demo update [success: 1/4 inputs (a)] | 2.46 µs | 1.19 µs | -51.4% | ~837.05k |
| main_demo update [success: 1/4 inputs (b)] | 2.40 µs | 1.16 µs | -51.6% | ~862.05k |
| main_demo update [success: 1/4 inputs (c)] | 4.35 µs | 2.57 µs | -40.9% | ~388.76k |
| main_demo update [success: 1/4 inputs (d)] | 4.81 µs | 2.39 µs | -50.3% | ~418.69k |
| main_demo update [success: 3/4 inputs] | 5.13 µs | 2.64 µs | -48.4% | ~378.27k |
| main_demo update [success: 4/4 inputs] | 6.56 µs | 3.64 µs | -44.5% | ~274.76k |
| main_demo delete | 175.76 ns | 11.46 ns | -93.5% | ~87.29M |

## Notes

- Summary: 38/38 benchmarks matched on both sides. No regressions -- `/rs-next` ranges from -99.2% to -34.8% relative to `/rs` across everything measured.
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
- Criterion sanitizes benchmark names into directory names under
  `target/criterion/` (replacing filesystem-unsafe characters with `_` and
  truncating to 64 characters); `criterion_dir_name()` in this script
  replicates both so lookups for the longer `main_demo` names don't miss.
