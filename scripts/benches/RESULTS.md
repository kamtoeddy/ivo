# `ivo` `/rs` vs `/rs-next` Benchmark Comparison

**Date**: 2026-08-27
**Command**: `cargo bench` run first in `/rs`, then in `/rs-next`
**Runtime**: Criterion + Tokio multi-thread runtime (`Runtime::new()`)

These are fresh, same-machine results for both implementations.

## Throughput

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |
| --- | --- | --- | --- | --- |
| minimal create | 1.08 µs | 47.00 ns | -95.6% | ~21.28M |
| user create | 3.98 µs | 675.01 ns | -83.0% | ~1.48M |
| create 20 required fields (sync validators) | 16.73 µs | 956.04 ns | -94.3% | ~1.05M |
| dependent chain length 10 | 4.24 µs | 1.16 µs | -72.6% | ~861.31k |
| create 10 readonly lax fields | 9.55 µs | 3.70 µs | -61.2% | ~270.07k |
| no-op update | 333.06 ns | 402.00 ns | +20.7% | ~2.49M |
| single field update | 1.58 µs | 432.48 ns | -72.7% | ~2.31M |

## Memory Stress

| Benchmark | Old (`/rs`) | New (`/rs-next`) | Change | New per-op |
| --- | --- | --- | --- | --- |
| memory minimal create x1000 | 1.06 ms | 38.16 µs | -96.4% | 38.16 ns |
| memory user create x1000 | 3.93 ms | 667.37 µs | -83.0% | 667.37 ns |
| memory 20 fields create x1000 | 16.74 ms | 954.90 µs | -94.3% | 954.90 ns |
| memory no-op update x1000 | 325.90 µs | 388.90 µs | +19.3% | 388.90 ns |

## Notes

- Lower time is better; negative `Change` means `/rs-next` is faster.
- The new `update` API takes the existing data by value, so the update
  harnesses clone the data each iteration. The old API borrowed the data.
- Both projects use the same release-profile tuning (`lto = true`,
  `codegen-units = 1`) and the same Tokio runtime.
