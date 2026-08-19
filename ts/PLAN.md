# Performance & Memory Benchmarking Plan for `ivo` (TypeScript)

## 1. Goals

1. Establish reproducible baseline measurements for `Model.create`, `Model.update`, and validation throughput.
2. Measure peak and retained memory footprint under realistic and stress workloads.
3. Identify the highest-impact runtime bottlenecks in the current implementation.
4. Provide a harness that can be re-run after optimizations to verify gains.
5. Keep benchmarks separate from correctness tests so CI can run them optionally.

## 2. What to Benchmark

### 2.1 Throughput Scenarios

| Scenario | What it stresses | Relevant source areas |
|---|---|---|
| Minimal schema create | Baseline `ModelTool` + `FieldInfoCollection` allocation | `src/model/index.ts:92-139`, `:1651` |
| Many-field schema create (50, 100, 200 fields) | Per-call field map rebuild, multi-pass definition scans | `src/model/index.ts:231-260`, `:541-624` |
| No-op update | `_isValidUpdate` double pass, final diff loop | `src/model/index.ts:181`, `:194`, `:207-213`, `:1036-1081` |
| Update with many unchanged + few changed fields | `relevantFieldsProvided` filtering, equality cost | `src/model/index.ts:767-833`, `src/utils/index.ts:230-256` |
| All sync validators vs. all async validators | `Promise.allSettled` overhead vs. synchronous loops | `src/model/index.ts:1083-1200` |
| `allow` list validation | `values.some(isEqual(...))` per field | `src/model/index.ts:1132-1138` |
| Linear dependent chains (`A → B → C → D`) | `_resolveDependentChanges` loop and full-definition scans | `src/model/index.ts:1488-1548` |
| Wide dependency fan-out | `dependsOn.some(...)` checks | `src/model/index.ts:1496-1506` |
| Many virtuals with validators + sanitizers | `_sanitizeVirtuals` pass | `src/model/index.ts:1386-1421` |
| Many readonly lax fields | Static-default equality check during filtering | `src/model/index.ts:786-808` |
| Dynamic `ignore` / `ignoreUpdate` resolvers | Task arrays + `Promise.all` execution | `src/model/index.ts:675-679`, `:843-870` |
| Validators reading `ctx.options` repeatedly | `cloneWithMethods(options)` hot path | `src/model/index.ts:292-293`, `:315-316`, `src/utils/index.ts:86-131` |

### 2.2 Memory Scenarios

| Scenario | What it measures |
|---|---|
| Retained `handleSuccess` handles | Heap retained when callers store `handleSuccess` without invoking it |
| Large `ctx.options` with methods | Allocation rate from repeated `cloneWithMethods` inside validators |
| Deep/large payload cloning | `deepCloneValue` / `structuredClone` / JSON fallback cost |
| Tight creation loop | Short-lived garbage from `ModelTool`, `FieldInfoCollection`, `ErrorTool`, contexts |
| Circular / function-bearing inputs | Fallback `JSON.parse(JSON.stringify)` behavior and memory |

## 3. Benchmark Harness

- **Tooling**: `bench` from `bun:test` (already available in the Bun runtime used by the project). No new runtime dependencies.
- **Location**: `tests/bench/` directory, excluded from the default `bun test` run.
- **Entry**: `tests/bench/run.ts` or individual `*.bench.ts` files.
- **Metrics**:
  - Throughput: operations/sec (mean, p50, p99 where supported by the runner).
  - Memory: `process.memoryUsage()` before/after and `gc()`-forced retained heap where available.
  - Relative delta: compare baseline vs. optimized runs.

## 4. Baseline Procedure

1. Build the package (`bun run build`).
2. Run each benchmark scenario with a stable iteration count and warmup.
3. Record throughput and memory numbers in `tests/bench/results/baseline.json` (not committed, generated).
4. Run benchmarks at least 3 times and use the median to reduce noise.

## 5. Candidate Optimizations to Evaluate

After baseline measurements, investigate the following, in order of expected impact:

1. **Cache field metadata**: Build `FieldInfoCollection` once per schema instead of per `create/update` call.
2. **Avoid `Promise.allSettled` for sync-only handlers**: Detect synchronous validators/resolvers and run them inline when possible.
3. **Optimize `isEqual`**: Replace `JSON.stringify(sortKeys(...))` with a cheaper structural equality for common cases (primitives, shallow objects).
4. **Memoize / avoid `cloneWithMethods(options)`**: Cache the cloned options object or lazily clone only on mutation.
5. **Reduce `_isValidUpdate` to a single pass**: Combine the two equality passes into one.
6. **Avoid redundant `Object.entries` iterations**: Reuse filtered field lists instead of rebuilding.
7. **Reduce `ErrorTool` allocation**: Lazily create error tools only when an error is first added.
8. **Avoid `deepCloneValue` JSON fallback**: Use a faster clone path or skip cloning when safe (e.g., input is known to be plain).
9. **Batch dependent resolution**: Pre-compute dependency order once per schema instead of scanning all definitions each round.
10. **Reduce closure allocation in contexts**: Use object pools or lightweight context objects without getters.

## 6. Implementation Phases

### Phase 1 — Harness
- Create `tests/bench/` directory.
- Add shared utilities: schema factories, timing helpers, memory sampling, result serialization.
- Add a README for running benchmarks.

### Phase 2 — Baseline Benchmarks
- Implement throughput benchmarks for the scenarios listed in §2.1.
- Implement memory benchmarks for the scenarios listed in §2.2.
- Generate and save baseline results.

### Phase 3 — Profile & Optimize
- Use baseline results to rank bottlenecks.
- Apply targeted optimizations from §5, one at a time.
- Re-run benchmarks after each change and record deltas.

### Phase 4 — Report
- Summarize findings in `tests/bench/RESULTS.md`.
- Recommend which optimizations to merge based on measured gain vs. complexity.

## 7. Success Criteria

- Benchmarks run in under ~30 seconds total on a typical laptop.
- Baseline results are reproducible (coefficient of variation < 10% across runs).
- At least three optimizations are measured and documented with before/after numbers.
- No correctness regressions: `bun test` and `bun run typecheck` still pass.

## 8. Files to Create / Modify

- `tests/bench/README.md`
- `tests/bench/utils.ts`
- `tests/bench/schemas.ts`
- `tests/bench/throughput.bench.ts`
- `tests/bench/memory.bench.ts`
- `tests/bench/run.ts`
- `.gitignore` update to ignore `tests/bench/results/`

No existing source files will be modified during Phase 1 and Phase 2.
