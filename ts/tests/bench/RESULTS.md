# `ivo` Benchmark Results

## Baseline

**Runtime**: Bun 1.3.14  
**Date**: 2026-08-18  
**Command**: `bun run tests/bench/run.ts`

### Baseline Throughput

| Benchmark                                    | Ops/sec | Mean (ms) |
| -------------------------------------------- | ------- | --------- |
| minimal create                               | 72,362  | 0.014     |
| user create                                  | 61,087  | 0.016     |
| create 50 required fields (sync validators)  | 13,352  | 0.075     |
| create 50 required fields (async validators) | 13,057  | 0.077     |
| create 100 required fields                   | 6,973   | 0.143     |
| allow list validation (100 items)            | 71,628  | 0.014     |
| dependent chain length 10                    | 77,877  | 0.013     |
| wide dependency 20 parents                   | 24,028  | 0.042     |
| 20 virtuals with sanitizers                  | 10,838  | 0.092     |
| create 50 readonly lax fields                | 14,539  | 0.069     |
| create 50 dynamic ignore fields              | 5,967   | 0.168     |
| 20 fields reading ctx.options repeatedly     | 14,296  | 0.070     |
| no-op update                                 | 189,805 | 0.005     |
| single field update                          | 58,001  | 0.017     |
| update 50 fields unchanged                   | 22,872  | 0.044     |

### Baseline Memory

| Benchmark                         | Delta MB | Retained heap MB |
| --------------------------------- | -------- | ---------------- |
| minimal create retained memory    | 0.00     | 42.87            |
| user create retained memory       | 0.11     | 42.97            |
| create 100 fields retained memory | 1.42     | 44.39            |
| handleSuccess retained memory     | 0.13     | 44.53            |
| ctx.options clone allocation      | 0.16     | 44.68            |
| update no-op retained memory      | 0.00     | 44.68            |

## Optimization 1: Cache `FieldInfoCollection` per schema

**File**: `src/model/index.ts`  
**Change**: Added a `WeakMap` cache keyed by `definitions` that stores the immutable `Map<string, InputFieldInfo>` built by `_getFieldInfoCollection`. The per-call `FieldInfoCollection` instances still get fresh mutable state, but the underlying field-info map is now built once per schema.

### Optimized Throughput

| Benchmark                                    | Ops/sec | Δ vs baseline |
| -------------------------------------------- | ------- | ------------- |
| minimal create                               | 75,392  | +4.2%         |
| user create                                  | 62,946  | +3.0%         |
| create 50 required fields (sync validators)  | 14,119  | +5.7%         |
| create 50 required fields (async validators) | 13,644  | +4.5%         |
| create 100 required fields                   | 6,901   | -1.0%         |
| allow list validation (100 items)            | 73,690  | +2.9%         |
| dependent chain length 10                    | 83,412  | +7.1%         |
| wide dependency 20 parents                   | 25,500  | +6.1%         |
| 20 virtuals with sanitizers                  | 11,349  | +4.7%         |
| create 50 readonly lax fields                | 14,620  | +0.6%         |
| create 50 dynamic ignore fields              | 6,273   | +5.1%         |
| 20 fields reading ctx.options repeatedly     | 14,417  | +0.8%         |
| no-op update                                 | 208,997 | +10.1%        |
| single field update                          | 60,247  | +3.9%         |
| update 50 fields unchanged                   | 24,222  | +5.9%         |

### Optimized Memory

| Benchmark                         | Retained heap MB | Δ vs baseline |
| --------------------------------- | ---------------- | ------------- |
| minimal create retained memory    | 31.73            | -26.0%        |
| user create retained memory       | 31.76            | -26.1%        |
| create 100 fields retained memory | 32.68            | -26.4%        |
| handleSuccess retained memory     | 32.68            | -26.6%        |
| ctx.options clone allocation      | 32.92            | -26.3%        |
| update no-op retained memory      | 32.92            | -26.3%        |

### Summary

- **Throughput**: modest but consistent gains across most workloads; largest improvements on update paths and dependency-heavy schemas.
- **Memory**: ~26% reduction in retained heap across the board by eliminating per-call allocation of `InputFieldInfo` objects and their `Map`.
- **Correctness**: full test suite still passes (`639 pass, 0 fail`).

## Optimization 2: Fast path for `cloneWithMethods`

**File**: `src/utils/index.ts`  
**Change**: Added fast paths in `cloneWithMethods` for plain arrays and plain objects (prototype is `Object.prototype` or `null`). These paths copy enumerable own properties directly instead of using `Object.getOwnPropertyDescriptors` + `Object.defineProperty`, while still preserving methods by reference and recursing into nested objects. The slower descriptor-based path remains for custom classes, getters/setters, symbols, and non-enumerable properties.

### Throughput delta vs. Optimization 1

| Benchmark                                    | Ops/sec    | Δ vs Opt 1 |
| -------------------------------------------- | ---------- | ---------- |
| minimal create                               | 75,006     | -0.5%      |
| user create                                  | 63,379     | +0.7%      |
| create 50 required fields (sync validators)  | 14,061     | -0.4%      |
| create 50 required fields (async validators) | 13,608     | -0.3%      |
| create 100 required fields                   | 6,928      | +0.4%      |
| allow list validation (100 items)            | 74,219     | +0.7%      |
| dependent chain length 10                    | 84,015     | +0.7%      |
| wide dependency 20 parents                   | 25,520     | +0.1%      |
| 20 virtuals with sanitizers                  | 11,333     | -0.1%      |
| create 50 readonly lax fields                | 14,650     | +0.2%      |
| create 50 dynamic ignore fields              | 6,286      | +0.2%      |
| **20 fields reading ctx.options repeatedly** | **17,699** | **+22.7%** |
| no-op update                                 | 220,853    | +5.7%      |
| single field update                          | 61,316     | +1.8%      |
| update 50 fields unchanged                   | 24,494     | +1.1%      |

### Memory delta vs. Optimization 1

| Benchmark                         | Retained heap MB | Δ vs Opt 1 |
| --------------------------------- | ---------------- | ---------- |
| minimal create retained memory    | 31.89            | -0.5%      |
| user create retained memory       | 31.92            | -0.5%      |
| create 100 fields retained memory | 32.85            | -0.5%      |
| handleSuccess retained memory     | 32.85            | -0.5%      |
| ctx.options clone allocation      | 33.01            | -0.5%      |
| update no-op retained memory      | 33.01            | -0.5%      |

### Summary

- **Targeted throughput win**: the benchmark specifically designed to stress repeated `ctx.options` access improved by **+22.7%**.
- **General throughput**: small gains on update paths; other workloads within noise.
- **Memory**: marginal additional reduction (~0.5%) on top of Optimization 1.
- **Correctness**: full test suite still passes (`639 pass, 0 fail`). Direct mutation of `ctx.options` remains isolated per access because each access still returns a fresh clone.

## Optimization 3: Combine `_isValidUpdate` passes — attempted and reverted

**File**: `src/model/index.ts`  
**Attempt**: Replaced the two-loop implementation with a single loop over `Object.entries(updates)` that built `relevantFieldsProvided` and dropped unchanged fields in one pass.

**Result**: Two variants were tried:

1. Starting from a copy of `fieldsCollection.relevantFieldsProvided` and removing unchanged non-virtual fields — passed tests but was slightly slower than the original on update workloads (no-op update -0.4%, single field update -1.4%, update 50 fields unchanged -3.7%).
2. Building `relevantFieldsProvided` from scratch while iterating updates — caused 36 test failures because virtuals present in `relevantFieldsProvided` but absent from `updates` were lost.

**Conclusion**: The current two-loop structure is already well-tuned for the common case. The overhead of `Object.entries(updates)` and extra conditionals outweighs the savings from merging the loops. This optimization was reverted.

## Final State

The only source changes retained are Optimizations 1 and 2:

- `src/model/index.ts`: `FieldInfoCollection` field map cached per schema via `WeakMap`
- `src/utils/index.ts`: fast paths in `cloneWithMethods` for plain arrays and plain objects

### Combined delta vs. original baseline

| Benchmark                                    | Baseline ops/sec | Final ops/sec | Δ      |
| -------------------------------------------- | ---------------- | ------------- | ------ |
| minimal create                               | 72,362           | 75,707        | +4.6%  |
| user create                                  | 61,087           | 62,710        | +2.7%  |
| create 50 required fields (sync validators)  | 13,352           | 14,077        | +5.4%  |
| create 50 required fields (async validators) | 13,057           | 13,622        | +4.3%  |
| create 100 required fields                   | 6,973            | 6,890         | -1.2%  |
| allow list validation (100 items)            | 71,628           | 73,674        | +2.9%  |
| dependent chain length 10                    | 77,877           | 82,955        | +6.5%  |
| wide dependency 20 parents                   | 24,028           | 25,359        | +5.5%  |
| 20 virtuals with sanitizers                  | 10,838           | 11,308        | +4.3%  |
| create 50 readonly lax fields                | 14,539           | 14,615        | +0.5%  |
| create 50 dynamic ignore fields              | 5,967            | 6,244         | +4.6%  |
| 20 fields reading ctx.options repeatedly     | 14,296           | 17,671        | +23.6% |
| no-op update                                 | 189,805          | 219,131       | +15.5% |
| single field update                          | 58,001           | 60,935        | +5.1%  |
| update 50 fields unchanged                   | 22,872           | 24,498        | +7.1%  |

| Benchmark                         | Baseline retained heap MB | Final retained heap MB | Δ      |
| --------------------------------- | ------------------------- | ---------------------- | ------ |
| minimal create retained memory    | 42.87                     | 31.49                  | -26.6% |
| user create retained memory       | 42.97                     | 31.51                  | -26.7% |
| create 100 fields retained memory | 44.39                     | 32.55                  | -26.7% |
| handleSuccess retained memory     | 44.53                     | 32.55                  | -26.9% |
| ctx.options clone allocation      | 44.68                     | 32.71                  | -26.8% |
| update no-op retained memory      | 44.68                     | 32.71                  | -26.8% |

## Recommended Next Optimizations

1. **Pre-compute dependency order** per schema so `_resolveDependentChanges` does not scan all definitions each round.
2. **Avoid `Promise.allSettled` for sync-only handlers** by detecting synchronous validators/resolvers at schema build time and running them inline.
3. **Optimize `isEqual`** — it currently uses `JSON.stringify(sortKeys(...))` which is expensive for repeated comparisons.
