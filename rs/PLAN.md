# Performance & Memory Benchmarking Plan for `ivo` (Rust)

## 1. Goals

1. Establish reproducible baseline measurements for `IvoModel::create`, `IvoModel::update`, and validation throughput.
2. Measure peak and retained memory footprint under realistic and stress workloads.
3. Identify the highest-impact runtime bottlenecks in the current Rust implementation.
4. Provide a harness that can be re-run after optimizations to verify gains.
5. Keep benchmarks separate from correctness tests so CI can run them optionally.

## 2. What to Benchmark

### 2.1 Throughput Scenarios

| Scenario | What it stresses | Relevant source areas |
|---|---|---|
| Minimal schema create | Baseline `Model` + `FieldInfoCollection` + partial-struct overhead | `src/model/mod.rs:66`, `src/model/fields_collection.rs:37` |
| Many-field schema create (50, 100, 200 fields) | Per-call field map rebuild, `join_all` allocations, boxing/unboxing | `src/model/mod.rs:85`, `:700`, `:800` |
| All sync validators vs. all async validators | `BoxFuture` / `join_all` overhead vs. synchronous closures | `src/schema/fields/types.rs:231-292`, `src/model/mod.rs:700` |
| Deep dependent chain (`A → B → C → D`) | `resolve_dependent_values` loop and `HashSet` rebuilds | `src/model/mod.rs:1081`, `:223-235` |
| Wide dependency fan-out (one field depending on many) | `depends_on` scanning and resolver dispatch | `src/model/mod.rs:1081` |
| No-op update | `evaluate_update_validity`, full-value equality checks | `src/model/mod.rs:1582-1623`, `:262` |
| Update with many unchanged + few changed fields | Partial struct cloning and diffing | `src/model/mod.rs:268-276`, `:337,367,397,456,474` |
| Many readonly lax fields | Readonly equality checks during filtering | `src/model/mod.rs:1163` |
| Virtual fields with aliases + sanitizers | Alias lookup, sanitizer futures, input sanitization | `src/model/mod.rs:877`, `src/model/fields_collection.rs` |
| Post-validation with groups | O(n) string matches and `Vec`/`HashSet` growth | `src/model/mod.rs:933-999` |

### 2.2 Memory Scenarios

| Scenario | What it measures |
|---|---|
| Retained `handle_success` handles | Heap retained when callers store success/failure handlers without invoking them |
| `Box<dyn CloneableAny>` churn per create | Per-field boxing overhead |
| Repeated context access | Clones of partial structs every time `ctx.values()` etc. is read |
| 1M create loop | Total allocations, RSS growth, and GC-like fragmentation |
| Large `CtxOptions` with methods | Allocation rate from `Arc::make_mut` / clone of options |

### 2.3 Proc-Macro / Derive Scenarios

| Scenario | What it stresses |
|---|---|
| Compile-time macro expansion | `ivo-derive` code generation cost for structs with many fields |
| Runtime field access | Generated `match field_name { ... }` string comparison cost |
| Partial struct memory layout | `Option<T>` wrapper overhead per field |
| Enumeration of available fields | `ivo_internal_fields_available` and `ivo_internal_enumerate_fields_available` allocation cost |

## 3. Benchmark Harness

- **Tooling**: `criterion` (standard Rust benchmark crate) added as a dev-dependency.
- **Location**: `rs/benches/` directory.
- **Entry points**: `benches/throughput.rs`, `benches/memory.rs`.
- **Metrics**:
  - Throughput: iterations per second (mean, p50, p99 where supported).
  - Memory: peak RSS via `dhat` or custom allocator wrapping for allocation counts.
  - Compile-time: `cargo build --timings` for derive stress structs.
- **Runtime**: Use `tokio` current-thread runtime inside benchmarks to avoid multi-thread scheduler noise for the sync-heavy workloads.

## 4. Baseline Procedure

1. Run `cargo test` to confirm correctness.
2. Run each benchmark scenario with Criterion's default sample size and warmup.
3. Record throughput and memory numbers in `benches/results/baseline.json` (gitignored, generated).
4. Run benchmarks at least 3 times and use the median to reduce noise.

## 5. Candidate Optimizations to Evaluate

After baseline measurements, investigate the following, in order of expected impact:

1. **Cache `FieldInfoCollection` per schema** instead of rebuilding it on every `create`/`update` call.
2. **Avoid `Box<dyn CloneableAny>` boxing** for common value types by using an enum or generic storage.
3. **Specialize synchronous handlers** to avoid `BoxFuture` and `join_all` allocations when all validators/resolvers are synchronous.
4. **Reduce partial-struct cloning** by sharing read-only snapshots and copying only on write.
5. **Cache cloned `CtxOptions`** or switch to `Arc<CtxOptions>` without `make_mut` when options are read-only.
6. **Pre-compute dependency order** per schema so dependent resolution does not scan all fields each round.
7. **Generate const field-name tables** in `ivo-derive` to replace runtime string matching.
8. **Optimize `isEqual`** / `ivo_internal_is_value_equal` to avoid string-match lookups.
9. **Add release-profile tuning** (`lto`, `codegen-units=1`, `panic=abort`) for published builds.
10. **Use `ArrayVec` or small vecs** for small field collections to avoid heap allocations.

## 6. Implementation Phases

### Phase 1 — Harness
- Add `criterion` dev-dependency to root `Cargo.toml`.
- Create `benches/` directory.
- Add schema factories and benchmark fixtures.
- Add `benches/README.md`.

### Phase 2 — Baseline Benchmarks
- Implement throughput benchmarks for scenarios in §2.1.
- Implement memory benchmarks for scenarios in §2.2.
- Optionally add derive compile-time benchmarks.
- Generate and save baseline results.

### Phase 3 — Profile & Optimize
- Use baseline results to rank bottlenecks.
- Apply targeted optimizations from §5, one at a time.
- Re-run benchmarks after each change and record deltas.

### Phase 4 — Report
- Summarize findings in `benches/RESULTS.md`.
- Recommend which optimizations to merge based on measured gain vs. complexity.

## 7. Success Criteria

- Benchmarks run in under ~60 seconds total on a typical laptop.
- Baseline results are reproducible (coefficient of variation < 10% across runs).
- At least three optimizations are measured and documented with before/after numbers.
- No correctness regressions: `cargo test` still passes.

## 8. Files to Create / Modify

- `benches/README.md`
- `benches/common.rs`
- `benches/schemas.rs`
- `benches/throughput.rs`
- `benches/memory.rs`
- `Cargo.toml` (add `criterion` dev-dependency and bench targets)
- `.gitignore` update to ignore `benches/results/`

No existing source files will be modified during Phase 1 and Phase 2.
