import { writeFileSync } from 'node:fs';

export interface BenchResult {
  name: string;
  /** derived from `meanMs` (the median sample) */
  opsPerSecond: number;
  /** median of the per-sample mean times */
  meanMs: number;
  /** fastest and slowest per-sample mean times */
  minMs?: number;
  maxMs?: number;
  /** number of samples (throughput) or iterations (memory) */
  samples: number;
  /** heap retained per operation after a forced GC (memory benchmarks) */
  retainedBytesPerOp?: number;
  /** total heap in use after the run (memory benchmarks) */
  retainedHeapMb?: number;
}

export interface BenchRun {
  name: string;
  date: string;
  runtime: string;
  results: BenchResult[];
}

const WARMUP_MS = 250;
const SAMPLES = 7;
const SAMPLE_MS = 200;

/**
 * Times `fn` in `samples` separate windows of `sampleMs` each (after a warmup)
 * and reports the median window's mean time, so a single noisy window
 * (GC, scheduling) can't skew the result. `minMs`/`maxMs` show the spread.
 */
export async function measureThroughput(
  name: string,
  fn: () => Promise<unknown> | unknown,
  opts: { warmupMs?: number; samples?: number; sampleMs?: number } = {},
): Promise<BenchResult> {
  const warmupMs = opts.warmupMs ?? WARMUP_MS;
  const samples = opts.samples ?? SAMPLES;
  const sampleMs = opts.sampleMs ?? SAMPLE_MS;

  // every window starts from a collected heap (GC time itself isn't measured)
  forceGc();
  const warmupStart = performance.now();
  while (performance.now() - warmupStart < warmupMs) await fn();

  const sampleMeans: number[] = [];
  for (let s = 0; s < samples; s++) {
    forceGc();
    let iterations = 0;
    const start = performance.now();
    let elapsed = 0;
    while (elapsed < sampleMs) {
      await fn();
      iterations++;
      elapsed = performance.now() - start;
    }
    sampleMeans.push(elapsed / iterations);
  }

  sampleMeans.sort((a, b) => a - b);
  const meanMs = sampleMeans[Math.floor(sampleMeans.length / 2)]!;

  return {
    name,
    opsPerSecond: 1000 / meanMs,
    meanMs,
    minMs: sampleMeans[0],
    maxMs: sampleMeans[sampleMeans.length - 1],
    samples,
  };
}

/** Bun exposes `Bun.gc`, Node/V8 a global `gc` (with `--expose-gc`). */
export function forceGc() {
  const bun = (globalThis as { Bun?: { gc(force: boolean): void } }).Bun;
  if (bun) return bun.gc(true);

  const gc = (globalThis as { gc?: () => void }).gc;
  if (typeof gc === 'function') return gc();

  throw new Error('cannot force a GC: run with Bun, or Node with --expose-gc');
}

/**
 * Leak check: heap still retained per operation after `iterations` calls whose
 * results are dropped, measured between two forced GCs. ~0 means nothing
 * outlives the call.
 */
export async function measureMemory(
  name: string,
  fn: () => Promise<unknown> | unknown,
  opts: { iterations?: number } = {},
): Promise<BenchResult> {
  const iterations = opts.iterations ?? 10_000;

  // let lazy, one-off allocations (caches, JIT data) happen before the baseline
  for (let i = 0; i < 100; i++) await fn();

  forceGc();
  const before = process.memoryUsage().heapUsed;

  for (let i = 0; i < iterations; i++) await fn();

  forceGc();
  const after = process.memoryUsage().heapUsed;

  return {
    name,
    opsPerSecond: 0,
    meanMs: 0,
    samples: iterations,
    retainedBytesPerOp: Math.max(0, after - before) / iterations,
    retainedHeapMb: after / 1024 / 1024,
  };
}

export function saveResults(
  run: BenchRun,
  path = 'tests/bench/results/baseline.json',
) {
  writeFileSync(path, JSON.stringify(run, null, 2));
}

export function padName(name: string, width: number) {
  return name.length > width
    ? `${name.slice(0, width - 3)}...`
    : name.padEnd(width);
}

function formatTime(ms: number) {
  return ms >= 1 ? `${ms.toFixed(2)} ms` : `${(ms * 1000).toFixed(2)} µs`;
}

export function formatResult(result: BenchResult) {
  const name = padName(result.name, 55);

  if (result.retainedBytesPerOp !== undefined)
    return `${name} retained: ${result.retainedBytesPerOp.toFixed(1).padStart(8)} B/op  (heap after: ${result.retainedHeapMb!.toFixed(2)} MB)`;

  const ops = result.opsPerSecond.toLocaleString('en-US', {
    maximumFractionDigits: 0,
  });
  const spread =
    result.minMs !== undefined && result.maxMs !== undefined
      ? `  [${formatTime(result.minMs)} – ${formatTime(result.maxMs)}]`
      : '';

  return `${name} ${ops.padStart(12)} ops/s  median: ${formatTime(result.meanMs).padStart(10)}${spread}`;
}

export async function runSuite(
  name: string,
  benchmarks: Array<() => Promise<BenchResult>>,
): Promise<BenchResult[]> {
  console.log(`\n${'='.repeat(70)}`);
  console.log(name);
  console.log('='.repeat(70));

  const results: BenchResult[] = [];
  for (const benchmark of benchmarks) {
    const result = await benchmark();
    results.push(result);
    console.log(formatResult(result));
  }

  return results;
}
