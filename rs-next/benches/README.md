# `ivo` Rust Benchmarks

This directory contains performance benchmarks for the Rust implementation of `ivo`.

## Running

```bash
# Throughput benchmarks
cargo bench --bench throughput

# Memory-stress benchmarks (iteration count based)
cargo bench --bench memory

# All benchmarks
cargo bench
```

Benchmarks use Criterion with a Tokio current-thread runtime.

## Notes

- Results are written to `target/criterion/` by Criterion.
- For detailed allocation profiling, run with `dhat` or `heaptrack`:
  ```bash
  cargo install dhat
  DHAT_ROOT=. cargo +nightly bench --bench memory
  ```
