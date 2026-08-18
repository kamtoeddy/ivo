# `ivo` Rust Baseline Benchmark Results

**Runtime**: Rust stable (cargo 1.97.1), Criterion, Tokio current-thread runtime  
**Date**: 2026-08-18  
**Command**: `cargo bench`

## Throughput

| Benchmark                                   | Mean time | Throughput (ops/s) |
| ------------------------------------------- | --------- | ------------------ |
| minimal create                              | 2.26 µs   | ~443,000           |
| user create                                 | 8.45 µs   | ~118,000           |
| create 20 required fields (sync validators) | 34.78 µs  | ~28,750            |
| dependent chain length 10                   | 9.32 µs   | ~107,000           |
| create 10 readonly lax fields               | 20.02 µs  | ~49,950            |
| no-op update                                | 1.06 µs   | ~944,000           |
| single field update                         | 3.72 µs   | ~269,000           |

## Memory Stress

| Benchmark                     | Mean time per 1,000 ops |
| ----------------------------- | ----------------------- |
| memory minimal create x1000   | 2.27 ms                 |
| memory user create x1000      | 8.38 ms                 |
| memory 20 fields create x1000 | 34.79 ms                |
| memory no-op update x1000     | 1.07 ms                 |

## Initial Observations

1. **No-op updates are very fast**: ~940k ops/s, suggesting the update-diffing path is already efficient.
2. **Field count dominates create cost**: 20 required fields takes ~15x longer than minimal create.
3. **Readonly fields are relatively expensive**: 10 readonly lax fields (~20 µs) is slower than a 10-field dependent chain (~9 µs), likely due to equality checks against static defaults.
4. **Dependent chains are efficient**: 10 chained dependents resolve in ~9 µs, indicating the loop overhead is moderate.

## Highest-Impact Optimization Opportunities

Based on code review and these baseline numbers:

1. **Cache `FieldInfoCollection` per schema** — it is rebuilt from scratch on every `create`/`update`.
2. **Avoid `Box<dyn CloneableAny>` per field** — every field value is heap-allocated and cloned repeatedly.
3. **Specialize synchronous handlers** — even `ready(...)` validators go through `BoxFuture` and `join_all`.
4. **Reduce partial-struct cloning** — context accessors clone on every call, and update rebuilds the full output struct after each phase.
5. **Pre-compute dependency order** per schema to avoid scanning all definitions each dependent-resolution round.
6. **Generate const field-name tables in `ivo-derive`** to replace runtime string matching in generated methods.
7. **Add release-profile tuning** (`lto`, `codegen-units=1`) for published builds.
