import { writeFileSync } from 'node:fs';

export interface BenchResult {
  name: string;
  opsPerSecond: number;
  meanMs: number;
  samples: number;
  memoryDeltaMb?: number;
  retainedHeapMb?: number;
}

export interface BenchRun {
  name: string;
  date: string;
  runtime: string;
  results: BenchResult[];
}

const WARMUP_MS = 250;
const MIN_RUNTIME_MS = 1000;

export async function measureThroughput(
  name: string,
  fn: () => Promise<unknown> | unknown,
  opts: { warmupMs?: number; minRuntimeMs?: number } = {},
): Promise<BenchResult> {
  const warmupMs = opts.warmupMs ?? WARMUP_MS;
  const minRuntimeMs = opts.minRuntimeMs ?? MIN_RUNTIME_MS;

  // Warmup
  const warmupStart = performance.now();
  while (performance.now() - warmupStart < warmupMs) {
    await fn();
  }

  // Measurement
  let samples = 0;
  const start = performance.now();
  while (performance.now() - start < minRuntimeMs) {
    await fn();
    samples++;
  }
  const elapsed = performance.now() - start;
  const meanMs = elapsed / samples;

  return {
    name,
    opsPerSecond: (samples / elapsed) * 1000,
    meanMs,
    samples,
  };
}

export async function measureMemory(
  name: string,
  fn: () => Promise<unknown> | unknown,
  opts: { iterations?: number; forceGc?: boolean } = {},
): Promise<BenchResult> {
  const iterations = opts.iterations ?? 1000;
  const forceGc = opts.forceGc ?? true;

  if (forceGc && typeof gc === 'function') gc();
  const baseline = process.memoryUsage();

  for (let i = 0; i < iterations; i++) await fn();

  if (forceGc && typeof gc === 'function') gc();
  const after = process.memoryUsage();

  const memoryDeltaMb = (after.heapUsed - baseline.heapUsed) / 1024 / 1024;
  const retainedHeapMb = after.heapUsed / 1024 / 1024;

  const elapsed = 0; // Not timed for memory-only runs
  return {
    name,
    opsPerSecond: iterations / (elapsed || 1),
    meanMs: elapsed / iterations,
    samples: iterations,
    memoryDeltaMb,
    retainedHeapMb,
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

export function formatResult(result: BenchResult) {
  const ops = result.opsPerSecond.toLocaleString('en-US', {
    maximumFractionDigits: 0,
  });
  const mean = result.meanMs.toFixed(3);
  const mem =
    result.memoryDeltaMb !== undefined
      ? ` | mem: ${result.memoryDeltaMb.toFixed(2)} MB`
      : '';
  const retained =
    result.retainedHeapMb !== undefined
      ? ` | retained: ${result.retainedHeapMb.toFixed(2)} MB`
      : '';

  return `${padName(result.name, 55)} ${ops.padStart(12)} ops/s  mean: ${mean.padStart(6)} ms${mem}${retained}`;
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
