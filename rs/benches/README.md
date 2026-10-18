# `ivo` Rust Benchmarks

This directory contains performance benchmarks for the Rust implementation of `ivo`.

## Running

```bash
# Throughput benchmarks
cargo bench --bench throughput

# Memory-stress benchmarks (iteration count based)
cargo bench --bench memory

# Realistic schema (needs the `validators` feature)
cargo bench --features validators --bench main_demo

# All benchmarks except main_demo
cargo bench
```

Benchmarks use Criterion with a Tokio multi-thread runtime (`Runtime::new()`).

## Recording results

`scripts/update_perf_results.py` runs every bench target (with the features it needs) and rewrites
`PERF_RESULTS.md` as a single report comparing the new results with ones recorded earlier:

- **HEAD**: the results in `PERF_RESULTS.md` as committed at git `HEAD`, so commit the report to
  make it the baseline of later runs.
- **Partial output**: the results recorded right after the API update that made `create` build a
  partial output, carried over from report to report.

```bash
python3 scripts/update_perf_results.py            # full Criterion runs
python3 scripts/update_perf_results.py --quick    # shorter runs (1s warm-up, 3s measurement)
python3 scripts/update_perf_results.py --skip-run # use Criterion's latest results without re-running
```

Benchmarks with no recorded result yet are listed without a comparison. New `bench_function`s are
picked up automatically, and the script fails if a benchmark's results weren't refreshed by the
run (e.g. after renaming it).

## Notes

- Results are written to `target/criterion/` by Criterion.
- For detailed allocation profiling, run with `dhat` or `heaptrack`:
  ```bash
  cargo install dhat
  DHAT_ROOT=. cargo +nightly bench --bench memory
  ```
