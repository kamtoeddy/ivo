import {
  allowListSchema,
  dependentChainSchema,
  dynamicIgnoreSchema,
  manyFieldSchema,
  minimalSchema,
  optionsReaderSchema,
  readonlyHeavySchema,
  userSchema,
  virtualHeavySchema,
  wideDependencySchema,
} from './schemas';
import {
  type BenchResult,
  measureMemory,
  measureThroughput,
  runSuite,
  saveResults,
} from './utils';

async function runThroughput(): Promise<BenchResult[]> {
  const benchmarks: Array<() => Promise<BenchResult>> = [];

  {
    const Model = minimalSchema();
    benchmarks.push(() =>
      measureThroughput('minimal create', () =>
        Model.create({ value: 42 }, {}),
      ),
    );
  }

  {
    const Model = userSchema();
    const input = { name: 'Alice', email: 'alice@example.com', age: 30 };
    benchmarks.push(() =>
      measureThroughput('user create', () => Model.create(input, {})),
    );
  }

  {
    const Model = manyFieldSchema(50);
    const input = Object.fromEntries(
      Array.from({ length: 50 }, (_, i) => [`field_${i}`, i]),
    );
    benchmarks.push(() =>
      measureThroughput('create 50 required fields (sync validators)', () =>
        Model.create(input, {}),
      ),
    );
  }

  {
    const Model = manyFieldSchema(50, true);
    const input = Object.fromEntries(
      Array.from({ length: 50 }, (_, i) => [`field_${i}`, i]),
    );
    benchmarks.push(() =>
      measureThroughput('create 50 required fields (async validators)', () =>
        Model.create(input, {}),
      ),
    );
  }

  {
    const Model = manyFieldSchema(100);
    const input = Object.fromEntries(
      Array.from({ length: 100 }, (_, i) => [`field_${i}`, i]),
    );
    benchmarks.push(() =>
      measureThroughput('create 100 required fields', () =>
        Model.create(input, {}),
      ),
    );
  }

  {
    const Model = allowListSchema(100);
    benchmarks.push(() =>
      measureThroughput('allow list validation (100 items)', () =>
        Model.create({ value: 50 }, {}),
      ),
    );
  }

  {
    const Model = dependentChainSchema(10);
    benchmarks.push(() =>
      measureThroughput('dependent chain length 10', () =>
        Model.create({}, {}),
      ),
    );
  }

  {
    const Model = wideDependencySchema(20);
    const input = Object.fromEntries(
      Array.from({ length: 20 }, (_, i) => [`parent_${i}`, i]),
    );
    benchmarks.push(() =>
      measureThroughput('wide dependency 20 parents', () =>
        Model.create(input, {}),
      ),
    );
  }

  {
    const Model = virtualHeavySchema(20);
    const input = {
      base: 1,
      ...Object.fromEntries(
        Array.from({ length: 20 }, (_, i) => [`virtual_${i}`, i]),
      ),
    };
    benchmarks.push(() =>
      measureThroughput('20 virtuals with sanitizers', () =>
        Model.create(input, {}),
      ),
    );
  }

  {
    const Model = readonlyHeavySchema(50);
    const input = Object.fromEntries(
      Array.from({ length: 50 }, (_, i) => [`readonly_${i}`, `value_${i}`]),
    );
    benchmarks.push(() =>
      measureThroughput('create 50 readonly lax fields', () =>
        Model.create(input, {}),
      ),
    );
  }

  {
    const Model = dynamicIgnoreSchema(50);
    const input = Object.fromEntries(
      Array.from({ length: 50 }, (_, i) => [`field_${i}`, i]),
    );
    benchmarks.push(() =>
      measureThroughput('create 50 dynamic ignore fields', () =>
        Model.create(input, {}),
      ),
    );
  }

  {
    const Model = optionsReaderSchema(20);
    const input = Object.fromEntries(
      Array.from({ length: 20 }, (_, i) => [`field_${i}`, i]),
    );
    benchmarks.push(() =>
      measureThroughput('20 fields reading ctx.options repeatedly', () =>
        Model.create(input, { tag: 'benchmark' }),
      ),
    );
  }

  {
    const Model = userSchema();
    const { data } = await Model.create(
      { name: 'Alice', email: 'alice@example.com', age: 30 },
      {},
    );
    const item = data!;
    benchmarks.push(() =>
      measureThroughput('no-op update', () =>
        Model.update(item, { name: 'Alice', age: 30 }, {}),
      ),
    );
    benchmarks.push(() =>
      measureThroughput('single field update', () =>
        Model.update(item, { age: 31 }, {}),
      ),
    );
  }

  {
    const Model = manyFieldSchema(50);
    const input = Object.fromEntries(
      Array.from({ length: 50 }, (_, i) => [`field_${i}`, i]),
    );
    const { data } = await Model.create(input, {});
    const item = data!;
    benchmarks.push(() =>
      measureThroughput('update 50 fields unchanged', () =>
        Model.update(item, input, {}),
      ),
    );
  }

  return runSuite('Throughput benchmarks', benchmarks);
}

async function runMemory(): Promise<BenchResult[]> {
  const benchmarks: Array<() => Promise<BenchResult>> = [];

  {
    const Model = minimalSchema();
    benchmarks.push(() =>
      measureMemory('minimal create retained memory', () =>
        Model.create({ value: 42 }, {}),
      ),
    );
  }

  {
    const Model = userSchema();
    const input = { name: 'Alice', email: 'alice@example.com', age: 30 };
    benchmarks.push(() =>
      measureMemory('user create retained memory', () =>
        Model.create(input, {}),
      ),
    );
  }

  {
    const Model = manyFieldSchema(100);
    const input = Object.fromEntries(
      Array.from({ length: 100 }, (_, i) => [`field_${i}`, i]),
    );
    benchmarks.push(() =>
      measureMemory('create 100 fields retained memory', () =>
        Model.create(input, {}),
      ),
    );
  }

  {
    const Model = readonlyHeavySchema(50);
    const input = Object.fromEntries(
      Array.from({ length: 50 }, (_, i) => [`readonly_${i}`, `value_${i}`]),
    );
    benchmarks.push(() =>
      measureMemory(
        'handleSuccess retained memory',
        async () => {
          const { handleSuccess } = await Model.create(input, {});
          (globalThis as any).__retainedHandle = handleSuccess;
        },
        { iterations: 100 },
      ),
    );
  }

  {
    const Model = optionsReaderSchema(20);
    const input = Object.fromEntries(
      Array.from({ length: 20 }, (_, i) => [`field_${i}`, i]),
    );
    benchmarks.push(() =>
      measureMemory('ctx.options clone allocation', () =>
        Model.create(input, { tag: 'benchmark' }),
      ),
    );
  }

  {
    const Model = userSchema();
    const { data } = await Model.create(
      { name: 'Alice', email: 'alice@example.com', age: 30 },
      {},
    );
    const item = data!;
    benchmarks.push(() =>
      measureMemory('update no-op retained memory', () =>
        Model.update(item, { name: 'Alice', age: 30 }, {}),
      ),
    );
  }

  return runSuite('Memory benchmarks', benchmarks);
}

async function main() {
  const outputPath = process.argv[2] ?? 'tests/bench/results/baseline.json';
  const runName = outputPath.includes('/')
    ? outputPath.split('/').pop()!.replace('.json', '')
    : outputPath.replace('.json', '');

  const throughput = await runThroughput();
  const memory = await runMemory();

  const run = {
    name: runName,
    date: new Date().toISOString(),
    runtime: `Bun ${Bun.version}`,
    results: [...throughput, ...memory],
  };

  saveResults(run, outputPath);
  console.log(`\nResults saved to ${outputPath}`);
}

main();
