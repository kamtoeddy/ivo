# `ivo` `/rs` vs `/rs-next` Benchmark Comparison

**Date**: 2026-08-27
**Command**: `cargo bench` run first in `/rs`, then in `/rs-next`
**Runtime**: Criterion + Tokio multi-thread runtime (`Runtime::new()`)

These are fresh, same-machine results for both implementations.

## Throughput

| Benchmark                                   | Old (`/rs`) | New (`/rs-next`) | Change | New ops/s |
| ------------------------------------------- | ----------- | ---------------- | ------ | --------- |
| minimal create                              | 2.08 µs     | 90.58 ns         | -95.6% | ~11.04M   |
| user create                                 | 7.65 µs     | 1.30 µs          | -83.0% | ~767.55k  |
| create 20 required fields (sync validators) | 32.48 µs    | 1.83 µs          | -94.4% | ~545.61k  |
| dependent chain length 10                   | 8.14 µs     | 2.17 µs          | -73.4% | ~461.10k  |
| create 10 readonly lax fields               | 18.28 µs    | 6.50 µs          | -64.4% | ~153.76k  |
| no-op update                                | 630.04 ns   | 789.77 ns        | +25.4% | ~1.27M    |
| single field update                         | 3.00 µs     | 859.87 ns        | -71.3% | ~1.16M    |

## Memory Stress

| Benchmark                     | Old (`/rs`) | New (`/rs-next`) | Change | New per-op |
| ----------------------------- | ----------- | ---------------- | ------ | ---------- |
| memory minimal create x1000   | 2.12 ms     | 73.57 µs         | -96.5% | 73.57 ns   |
| memory user create x1000      | 7.67 ms     | 1.30 ms          | -83.0% | 1.30 µs    |
| memory 20 fields create x1000 | 32.50 ms    | 1.83 ms          | -94.4% | 1.83 µs    |
| memory no-op update x1000     | 645.70 µs   | 777.55 µs        | +20.4% | 777.55 ns  |

## Notes

- Lower time is better; negative `Change` means `/rs-next` is faster.
- The new `update` API takes the existing data by value, so the update
  harnesses clone the data each iteration. The old API borrowed the data.
- Both projects use the same release-profile tuning (`lto = true`,
  `codegen-units = 1`) and the same Tokio runtime.
