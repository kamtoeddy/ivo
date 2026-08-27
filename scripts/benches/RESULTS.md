# `ivo` `/rs` vs `/rs-next` Benchmark Comparison

**Date**: 2026-08-27
**Command**: `cargo bench` and `cargo bench --features validators --bench main_demo` run first in `/rs`, then in `/rs-next`
**Runtime**: Criterion + Tokio multi-thread runtime (`Runtime::new()`)

These are fresh, same-machine results for both implementations.

## Throughput

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |
| --- | --- | --- | --- | --- |
| minimal create | 2.05 µs | 90.30 ns | -95.6% | ~11.07M |
| user create | 7.66 µs | 1.28 µs | -83.3% | ~779.63k |
| create 20 required fields (sync validators) | 32.64 µs | 1.83 µs | -94.4% | ~546.73k |
| dependent chain length 10 | 8.19 µs | 2.16 µs | -73.7% | ~463.84k |
| create 10 readonly lax fields | 18.74 µs | 6.34 µs | -66.1% | ~157.61k |
| no-op update | 662.71 ns | 757.63 ns | +14.3% | ~1.32M |
| single field update | 3.06 µs | 835.92 ns | -72.6% | ~1.20M |

## Memory Stress

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New per-op |
| --- | --- | --- | --- | --- |
| memory minimal create x1000 | 2.07 ms | 73.19 µs | -96.5% | 73.19 ns |
| memory user create x1000 | 7.63 ms | 1.28 ms | -83.3% | 1.28 µs |
| memory 20 fields create x1000 | 32.50 ms | 1.83 ms | -94.4% | 1.83 µs |
| memory no-op update x1000 | 632.56 µs | 759.71 µs | +20.1% | 759.71 ns |

## Main Demo

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |
| --- | --- | --- | --- | --- |
| main_demo create | 9.82 µs | 4.94 µs | -49.7% | ~202.58k |
| main_demo update | 14.49 µs | 5.99 µs | -58.6% | ~166.84k |
| main_demo delete | 332.03 ns | 22.13 ns | -93.3% | ~45.18M |

## Notes

- Lower time is better; negative `Change` means `/rs-next` is faster.
- The new `update` API takes the existing data by value, so the update
  harnesses clone the data each iteration. The old API borrowed the data.
- Both projects use the same release-profile tuning (`lto = true`,
  `codegen-units = 1`) and the same Tokio runtime.
- `main_demo` benchmarks exercise the same realistic schema (constants,
  lax/required/dependent/virtual fields, timestamps, grouped validation,
  post-validation, hooks) in both implementations.
