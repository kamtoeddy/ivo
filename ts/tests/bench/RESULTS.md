# `ivo` Baseline Benchmark Results

**Runtime**: Bun 1.3.14  
**Date**: 2026-08-18  
**Command**: `bun run tests/bench/run.ts`

## Throughput

| Benchmark | Ops/sec | Mean (ms) |
|---|---|---|
| minimal create | 75,959 | 0.013 |
| user create | 62,249 | 0.016 |
| create 50 required fields (sync validators) | 13,258 | 0.075 |
| create 50 required fields (async validators) | 12,804 | 0.078 |
| create 100 required fields | 6,904 | 0.145 |
| allow list validation (100 items) | 72,139 | 0.014 |
| dependent chain length 10 | 78,257 | 0.013 |
| wide dependency 20 parents | 24,273 | 0.041 |
| 20 virtuals with sanitizers | 11,041 | 0.091 |
| create 50 readonly lax fields | 14,625 | 0.068 |
| create 50 dynamic ignore fields | 6,069 | 0.165 |
| 20 fields reading ctx.options repeatedly | 14,429 | 0.069 |
| no-op update | 193,718 | 0.005 |
| single field update | 59,306 | 0.017 |
| update 50 fields unchanged | 22,937 | 0.044 |

## Memory

| Benchmark | Delta MB | Retained heap MB |
|---|---|---|
| minimal create retained memory | 0.08 | 42.80 |
| user create retained memory | 0.03 | 42.82 |
| create 100 fields retained memory | 1.36 | 44.18 |
| handleSuccess retained memory | 0.00 | 44.18 |
| ctx.options clone allocation | 0.23 | 44.41 |
| update no-op retained memory | 0.00 | 44.41 |

## Initial Observations

1. **Field count is the dominant cost**: throughput roughly halves from 50 to 100 required fields, confirming that per-field allocations and multi-pass scans dominate runtime.
2. **Async validator overhead is small**: 50 sync vs. 50 async validators differ by only ~3%, suggesting the `Promise.allSettled` machinery is already fairly efficient for all-async workloads.
3. **Dynamic ignore is expensive**: 50 dynamic ignore resolvers drop throughput to ~6k ops/sec, likely due to task array construction and closure invocation.
4. **`ctx.options` cloning is measurable**: 20 fields each reading `ctx.options` three times results in ~0.23 MB of allocations per 1,000 ops, confirming the per-access clone cost.
5. **No-op updates are very fast**: the `_isValidUpdate` early-exit path is well optimized for unchanged values.
6. **Wide dependencies are slower than chains**: a dependent depending on 20 parents is ~3x slower than a chain of 10, due to the `dependsOn.some(...)` scan each resolution round.

## Recommended First Optimizations

1. **Cache `FieldInfoCollection` per schema** instead of rebuilding it on every `create`/`update`. Expected to improve large-schema workloads significantly.
2. **Short-circuit `cloneWithMethods(options)`** so repeated reads of `ctx.options` return a cached clone. Should reduce memory pressure in options-heavy validators.
3. **Combine the two `_isValidUpdate` passes** into a single loop. Likely to help update-heavy workloads.
4. **Pre-compute dependency order** per schema so `_resolveDependentChanges` does not scan all definitions each round.
5. **Avoid `Promise.allSettled` when all handlers are synchronous** by detecting sync vs. async paths at schema build time.
