# `ivo` `/rs` vs `/rs-next` Benchmark Comparison

**Date**: 2026-08-31
**Command**: `cargo bench` and `cargo bench --features validators --bench main_demo` run first in `/rs`, then in `/rs-next`
**Runtime**: Criterion + Tokio multi-thread runtime (`Runtime::new()`)

These are fresh, same-machine results for both implementations.

## Throughput

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |
| --- | --- | --- | --- | --- |
| minimal create | 1.07 µs | 18.27 ns | -98.3% | ~54.74M |
| user create | 3.95 µs | 501.68 ns | -87.3% | ~1.99M |
| create 20 required fields (sync validators) | 17.09 µs | 138.71 ns | -99.2% | ~7.21M |
| dependent chain length 10 | 4.27 µs | 1.28 µs | -69.9% | ~778.74k |
| create 10 readonly lax fields | 9.61 µs | 1.63 µs | -83.0% | ~612.21k |
| no-op update | 333.74 ns | 129.65 ns | -61.2% | ~7.71M |
| single field update | 1.59 µs | 405.44 ns | -74.5% | ~2.47M |

## Memory Stress

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New per-op |
| --- | --- | --- | --- | --- |
| memory minimal create x1000 | 1.07 ms | 18.43 µs | -98.3% | 18.43 ns |
| memory user create x1000 | 3.93 ms | 514.09 µs | -86.9% | 514.09 ns |
| memory 20 fields create x1000 | 17.15 ms | 138.75 µs | -99.2% | 138.75 ns |
| memory no-op update x1000 | 328.10 µs | 131.86 µs | -59.8% | 131.86 ns |

## Main Demo

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |
| --- | --- | --- | --- | --- |
| main_demo create [fail: required errors (email or phone_number)] | - | - | - | - |
| main_demo create [fail: required errors (email or phone_number, username)] | - | - | - | - |
| main_demo create [fail: validation error (email, slug_id, username)] | - | - | - | - |
| main_demo create [fail: re_validation error (username taken)] | - | - | - | - |
| main_demo create [fail: post-validation error (slug taken)] | - | - | - | - |
| main_demo create [success: 2/4 inputs (a)] | - | - | - | - |
| main_demo create [success: 2/4 inputs (b)] | - | - | - | - |
| main_demo create [success: 3/4 inputs] | - | - | - | - |
| main_demo create [success: 4/4 inputs] | - | - | - | - |
| main_demo update [fail: required error (email or phone_number)] | - | - | - | - |
| main_demo update [fail: validation error (email, slug_id, username)] | - | - | - | - |
| main_demo update [fail: re_validation error (username taken)] | - | - | - | - |
| main_demo update [fail: post-validation error (slug taken)] | - | - | - | - |
| main_demo update [success: 1/4 inputs (d)] | - | - | - | - |
| main_demo update [success: 1/4 inputs (b)] | - | - | - | - |
| main_demo update [success: 1/4 inputs (c)] | - | - | - | - |
| main_demo update [success: 3/4 inputs] | - | - | - | - |
| main_demo update [success: 4/4 inputs] | - | - | - | - |
| main_demo delete | 178.46 ns | 11.56 ns | -93.5% | ~86.52M |

## Notes

- Lower time is better; negative `Change` means `/rs-next` is faster.
- The new `update` API takes the existing data by value, so the update
  harnesses clone the data each iteration. The old API borrowed the data.
- Both projects use the same release-profile tuning (`lto = true`,
  `codegen-units = 1`) and the same Tokio runtime.
- `main_demo` benchmarks exercise the same realistic schema (constants,
  lax/required/dependent/virtual fields, timestamps, grouped validation,
  post-validation, hooks) in both implementations.
