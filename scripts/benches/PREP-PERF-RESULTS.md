# `ivo` `/rs` vs `/rs-next` Benchmark Comparison

**Date**: 2026-08-27
**Command**: `cargo bench` and `cargo bench --features validators --bench main_demo` run first in `/rs`, then in `/rs-next`
**Runtime**: Criterion + Tokio multi-thread runtime (`Runtime::new()`)

These are fresh, same-machine results for both implementations.

## Throughput

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |
| --- | --- | --- | --- | --- |
| minimal create | 2.27 µs | 91.67 ns | -96.0% | ~10.91M |
| user create | 7.63 µs | 1.33 µs | -82.6% | ~753.32k |
| create 20 required fields (sync validators) | 32.59 µs | 1.84 µs | -94.4% | ~543.88k |
| dependent chain length 10 | 8.13 µs | 2.20 µs | -72.9% | ~454.88k |
| create 10 readonly lax fields | 18.35 µs | 6.42 µs | -65.0% | ~155.88k |
| no-op update | 647.73 ns | 796.32 ns | +22.9% | ~1.26M |
| single field update | 3.05 µs | 883.57 ns | -71.0% | ~1.13M |

## Memory Stress

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New per-op |
| --- | --- | --- | --- | --- |
| memory minimal create x1000 | 2.07 ms | 73.89 µs | -96.4% | 73.89 ns |
| memory user create x1000 | 7.60 ms | 1.30 ms | -82.8% | 1.30 µs |
| memory 20 fields create x1000 | 33.08 ms | 1.86 ms | -94.4% | 1.86 µs |
| memory no-op update x1000 | 674.69 µs | 781.22 µs | +15.8% | 781.22 ns |

## Main Demo

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |
| --- | --- | --- | --- | --- |
| main_demo create | 9.77 µs | 5.11 µs | -47.8% | ~195.82k |
| main_demo update | 14.43 µs | 6.12 µs | -57.6% | ~163.48k |
| main_demo delete | 342.36 ns | 25.72 ns | -92.5% | ~38.88M |

## Notes

- Lower time is better; negative `Change` means `/rs-next` is faster.
- The new `update` API takes the existing data by value, so the update
  harnesses clone the data each iteration. The old API borrowed the data.
- Both projects use the same release-profile tuning (`lto = true`,
  `codegen-units = 1`) and the same Tokio runtime.
- `main_demo` benchmarks exercise the same realistic schema (constants,
  lax/required/dependent/virtual fields, timestamps, grouped validation,
  post-validation, hooks) in both implementations.
