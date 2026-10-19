# `ivo` Benchmarks

This directory contains performance and memory benchmarks for the TypeScript implementation of `ivo`.

## Running

```bash
bun run tests/bench/run.ts [output.json]
```

This runs every throughput and memory benchmark and writes the results to
`tests/bench/results/baseline.json` (or `output.json`). The results directory is gitignored.

## How it measures

- **Throughput:** each benchmark runs in its own process, so the JIT state left by other
  benchmarks (they all share the same `ModelTool` code) can't skew it. After a 250 ms warmup it
  times 7 windows of 200 ms, forcing a GC before each, and reports the median window's mean time
  along with the fastest and slowest windows.
- **Memory:** a leak check, in its own process. It runs 10,000 calls whose results are dropped and
  reports the heap still retained per call between two forced GCs (`Bun.gc(true)`, or `gc()` on
  Node with `--expose-gc`). Values near 0 B/op mean nothing outlives the call.

## Scenarios

- Synthetic schemas (`schemas.ts`) that stress one feature each: many fields, sync vs async
  validators, allow-lists, dependent chains and fan-out, virtuals, readonly fields, dynamic
  ignores, `ctx.options` access, and updates.
- A realistic schema (`realisticUserSchema`, the counterpart of the Rust `main_demo`): create
  (success and failure) and update (with dependents, and with nothing to update).
- The built-in validators (string, number, email, credit card, array unique/sort).

Benchmarks import from `../../src`, so they exercise the current source directly.
