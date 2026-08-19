# `ivo` Benchmarks

This directory contains performance and memory benchmarks for the TypeScript implementation of `ivo`.

## Running

### Full benchmark suite

```bash
bun run tests/bench/run.ts
```

This runs all throughput and memory benchmarks and writes the results to `tests/bench/results/baseline.json`.

### Bun-style benchmark files

Individual benchmark files can also be run with Bun's built-in benchmark runner:

```bash
bun test tests/bench/throughput.bench.ts
bun test tests/bench/memory.bench.ts
```

## Notes

- Benchmarks import from `../../src` so they exercise the current source directly.
- Memory benchmarks attempt to call `gc()` if exposed. Run with `--expose-gc` when available for more stable retained-heap numbers.
- Results are written to `tests/bench/results/` which is gitignored.
