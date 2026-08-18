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

## Optimization #1 — Cache `FieldInfoCollection` per schema

**Change**: Store a pre-parsed `HashMap<&'static str, InputFieldInfo<'static>>` on `IvoModel` at construction time, and have `FieldInfoCollection` borrow it instead of re-parsing `field_configs` on every `create`/`update`.

**Files modified**:

- `src/types/mod.rs` — added `field_infos` to `IvoModel`.
- `src/schema/mod.rs` — populate `field_infos` during `IvoModel::new`.
- `src/model/fields_collection.rs` — `FieldInfoCollection` now borrows the cached map; removed per-call parsing and unused generic parameters.
- `src/model/mod.rs` — `create`/`update` pass `&self.field_infos` to `FieldInfoCollection::new`.

### Results vs. baseline

| Benchmark                                   | Baseline | Optimized | Change |
| ------------------------------------------- | -------- | --------- | ------ |
| minimal create                              | 2.26 µs  | 2.19 µs   | -3.1%  |
| user create                                 | 8.45 µs  | 8.14 µs   | -3.7%  |
| create 20 required fields (sync validators) | 34.78 µs | 33.61 µs  | -3.4%  |
| dependent chain length 10                   | 9.32 µs  | 8.76 µs   | -6.0%  |
| create 10 readonly lax fields               | 20.02 µs | 19.55 µs  | -2.3%  |
| no-op update                                | 1.06 µs  | 750 ns    | -29.2% |
| single field update                         | 3.72 µs  | 3.28 µs   | -11.8% |

| Memory stress benchmark       | Baseline | Optimized | Change |
| ----------------------------- | -------- | --------- | ------ |
| memory minimal create x1000   | 2.27 ms  | 2.14 ms   | -5.7%  |
| memory user create x1000      | 8.38 ms  | 8.14 ms   | -2.9%  |
| memory 20 fields create x1000 | 34.79 ms | 33.59 ms  | -3.4%  |
| memory no-op update x1000     | 1.07 ms  | 735 µs    | -31.3% |

`cargo test`: 892 passed; 0 failed.

The biggest wins are on update paths, where the per-call field map rebuild was pure overhead. `no-op update` and `single field update` improve by ~29% and ~12% respectively. Create paths see modest gains (~3-6%) because the field map is only one part of their total cost. The `dependent chain length 10` benchmark also improves, partly benefiting from the `dependent_children` cache added in optimization #2.

## Optimization #2 — Cache dependent children per schema

**Change**: Pre-compute a `dependent_children: HashMap<&'static str, Vec<&'static str>>` map at schema build time, mapping each field config name to the list of dependent fields that declare it as a parent. `resolve_dependent_values` now walks only the dependents of currently-active parents instead of scanning the entire `field_configs` map on every resolution wave.

**Files modified**:

- `src/types/mod.rs` — added `dependent_children` to `IvoModel`.
- `src/schema/mod.rs` — build `dependent_children` from `field_configs` in `IvoModel::new`.
- `src/model/fields_collection.rs` — expose `relevant_dependent_config_names()`; removed now-unused `is_relevant_dependent_config_name`.
- `src/model/mod.rs` — `resolve_dependent_values` uses `self.dependent_children` to find candidates.

This optimization is included in the final numbers above. Its effect is most visible on dependent-chain workloads (e.g. `dependent chain length 10`), where each wave no longer iterates over the whole schema. The improvement there is ~6% vs. the original baseline.

`cargo test`: 892 passed; 0 failed.

## Highest-Impact Optimization Opportunities (Remaining)

Based on code review and these baseline numbers:

1. ~~**Cache `FieldInfoCollection` per schema** — it is rebuilt from scratch on every `create`/`update`.~~ ✅ Applied.
2. ~~**Pre-compute dependency order** per schema to avoid scanning all definitions each dependent-resolution round.~~ ✅ Applied (cached `dependent_children`).
3. **Avoid `Box<dyn CloneableAny>` per field** — every field value is heap-allocated and cloned repeatedly.
4. **Specialize synchronous handlers** — even `ready(...)` validators go through `BoxFuture` and `join_all`.
5. **Reduce partial-struct cloning** — context accessors clone on every call, and update rebuilds the full output struct after each phase.
6. **Generate const field-name tables in `ivo-derive`** to replace runtime string matching in generated methods.
7. **Add release-profile tuning** (`lto`, `codegen-units=1`) for published builds.
