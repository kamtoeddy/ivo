# `ivo` `rs-next` Benchmark Results

**Runtime**: Rust stable, Criterion, Tokio multi-thread runtime (`Runtime::new()`)  
**Date**: 2026-08-27  
**Command**: `cargo bench` in `/rs-next/`

These numbers are for the new `#[ivo_schema]` macro-based implementation in `rs-next/`. They are compared against the final cumulative numbers from `/rs/benches/RESULTS.md` (old `IvoModel` implementation after its own optimizations).

## Throughput

| Benchmark                                   | Old (`/rs`) | New (`/rs-next`) | Change   | New throughput (ops/s) |
| ------------------------------------------- | ----------- | ---------------- | -------- | ---------------------- |
| minimal create                              | 2.05 µs     | 90.65 ns         | -95.6%   | ~11,030,000            |
| user create                                 | 7.58 µs     | 1.31 µs          | -82.8%   | ~763,000               |
| create 20 required fields (sync validators) | 32.65 µs    | 1.83 µs          | -94.4%   | ~546,000               |
| dependent chain length 10                   | 8.15 µs     | 2.17 µs          | -73.3%   | ~460,000               |
| create 10 readonly lax fields               | 18.31 µs    | 6.51 µs          | -64.5%   | ~154,000               |
| no-op update                                | 632 ns      | 784.60 ns        | +24.1%   | ~1,275,000             |
| single field update                         | 2.99 µs     | 855.19 ns        | -71.4%   | ~1,169,000             |

## Memory Stress

| Benchmark                     | Old (`/rs`) | New (`/rs-next`) | Change   | New per-op time |
| ----------------------------- | ----------- | ---------------- | -------- | --------------- |
| memory minimal create x1000   | 2.06 ms     | 73.51 µs         | -96.4%   | 73.51 ns        |
| memory user create x1000      | 7.58 ms     | 1.32 ms          | -82.6%   | 1.32 µs         |
| memory 20 fields create x1000 | 32.39 ms    | 1.84 ms          | -94.3%   | 1.84 µs         |
| memory no-op update x1000     | 630 µs      | 772.83 µs        | +22.7%   | 772.83 ns       |

## Notes

- All create paths are substantially faster in `rs-next` (between ~3x and ~22x faster). The macro-generated code avoids the dynamic per-field boxing and repeated map parsing of the old runtime implementation.
- Update paths are mixed: `single field update` is ~3.5x faster, while `no-op update` is ~24% slower. The new `update` API takes the existing data by value, so the benchmark clones the data each iteration; the old API borrowed the data and therefore avoided that explicit clone in the harness.
- Both suites use the same release-profile tuning (`lto = true`, `codegen-units = 1`) and the same Tokio multi-thread runtime for a fair comparison.
