import { readFileSync, rmSync } from 'node:fs';
import {
  makeArrayValidator,
  makeNumberValidator,
  makeStringValidator,
  validateCreditCard,
  validateEmail,
} from '../../src';
import {
  allowListSchema,
  dependentChainSchema,
  dynamicIgnoreSchema,
  manyFieldSchema,
  minimalSchema,
  optionsReaderSchema,
  readonlyHeavySchema,
  realisticUserInput,
  realisticUserSchema,
  userSchema,
  virtualHeavySchema,
  wideDependencySchema,
} from './schemas';
import {
  type BenchResult,
  formatResult,
  measureMemory,
  measureThroughput,
  runSuite,
  saveResults,
} from './utils';

async function throughputBenchmarks(): Promise<
  Array<() => Promise<BenchResult>>
> {
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
        Model.create({ field_0: 5 }, {}),
      ),
    );
    benchmarks.push(() =>
      measureThroughput('dependent chain length 10 (defaults only)', () =>
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
    const Model = realisticUserSchema();
    const { data } = await Model.create(realisticUserInput, {});
    const user = data!;
    benchmarks.push(() =>
      measureThroughput('realistic create (success)', async () => {
        const { handleSuccess } = await Model.create(realisticUserInput, {});
        await handleSuccess?.();
      }),
    );
    benchmarks.push(() =>
      measureThroughput('realistic create (username taken)', () =>
        Model.create({ ...realisticUserInput, username: 'taken' }, {}),
      ),
    );
    benchmarks.push(() =>
      measureThroughput('realistic update (username + dependents)', () =>
        Model.update(user, { username: 'Ada_Lovelace' }, {}),
      ),
    );
    benchmarks.push(() =>
      measureThroughput('realistic update (nothing to update)', () =>
        Model.update(user, { username: user.username }, {}),
      ),
    );
  }

  {
    const validateString = makeStringValidator({ min: 3, max: 50 });
    const validateNumber = makeNumberValidator({ min: 0, max: 100 });
    const people = Array.from({ length: 200 }, (_, i) => ({
      id: i % 100,
      name: `person_${i}`,
    }));
    const numbers = Array.from({ length: 200 }, (_, i) => (i * 7919) % 200);
    const uniqueById = makeArrayValidator({
      filter: () => true,
      unique: true,
      uniqueKey: 'id',
    });
    const uniqueObjects = makeArrayValidator({
      filter: () => true,
      unique: true,
    });
    const sorted = makeArrayValidator({ filter: () => true, sort: true });

    benchmarks.push(() =>
      measureThroughput('validator: string', () =>
        validateString('hello world'),
      ),
    );
    benchmarks.push(() =>
      measureThroughput('validator: number', () => validateNumber(42)),
    );
    benchmarks.push(() =>
      measureThroughput('validator: email', () =>
        validateEmail('ada@example.com'),
      ),
    );
    benchmarks.push(() =>
      measureThroughput('validator: credit card', () =>
        validateCreditCard('5420596721435293'),
      ),
    );
    benchmarks.push(() =>
      measureThroughput('validator: array unique by key (200 objects)', () =>
        uniqueById(people),
      ),
    );
    benchmarks.push(() =>
      measureThroughput('validator: array unique (200 objects)', () =>
        uniqueObjects(people),
      ),
    );
    benchmarks.push(() =>
      measureThroughput('validator: array sort (200 numbers)', () =>
        sorted(numbers),
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

  return benchmarks;
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
        { iterations: 10_000 },
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

  {
    const Model = realisticUserSchema();
    benchmarks.push(() =>
      measureMemory('realistic create retained memory', () =>
        Model.create(realisticUserInput, {}),
      ),
    );
  }

  return runSuite('Memory benchmarks', benchmarks);
}

/**
 * Runs each throughput benchmark in its own process: every schema shares the
 * same `ModelTool` code, so in one process the JIT state left by earlier
 * benchmarks shifts later results (by up to ~50% between identical runs).
 */
async function runThroughput(): Promise<BenchResult[]> {
  const count = (await throughputBenchmarks()).length;

  console.log(`\n${'='.repeat(70)}`);
  console.log('Throughput benchmarks (one process each)');
  console.log('='.repeat(70));

  const results: BenchResult[] = [];
  for (let index = 0; index < count; index++) {
    const [result] = runChild(['--suite=throughput', `--index=${index}`]);
    results.push(result!);
    console.log(formatResult(result!));
  }

  return results;
}

/** runs this script with `args` and returns the results it saved */
function runChild(args: string[]): BenchResult[] {
  const path = `tests/bench/results/.child-${process.pid}.json`;
  const child = Bun.spawnSync(['bun', 'run', import.meta.path, ...args, path], {
    stdout: 'inherit',
    stderr: 'inherit',
  });
  if (!child.success) throw new Error(`benchmark child failed: ${args}`);

  const { results } = JSON.parse(readFileSync(path, 'utf8'));
  rmSync(path);

  return results;
}

async function main() {
  const args = process.argv.slice(2);
  const option = (name: string) =>
    args.find((a) => a.startsWith(`--${name}=`))?.slice(name.length + 3);
  const suite = option('suite');
  const outputPath =
    args.find((a) => !a.startsWith('--')) ??
    'tests/bench/results/baseline.json';
  const runName = outputPath.includes('/')
    ? outputPath.split('/').pop()!.replace('.json', '')
    : outputPath.replace('.json', '');
  const save = (results: BenchResult[]) =>
    saveResults(
      {
        name: runName,
        date: new Date().toISOString(),
        runtime: `Bun ${Bun.version}`,
        results,
      },
      outputPath,
    );

  if (suite === 'throughput') {
    const benchmarks = await throughputBenchmarks();
    return save([await benchmarks[Number(option('index'))]!()]);
  }

  if (suite === 'memory') return save(await runMemory());

  const throughput = await runThroughput();

  // The memory suite also runs in its own process, so no garbage left by
  // other benchmarks can skew its heap measurements.
  const memory = runChild(['--suite=memory']);

  save([...throughput, ...memory]);
  console.log(`\nResults saved to ${outputPath}`);
}

main();
