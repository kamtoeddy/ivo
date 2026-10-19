import { describe, expect, it } from 'bun:test';
import { isDeepStrictEqual } from 'node:util';

import { deepCloneValue } from '../../src/utils';

class Point {
  constructor(public x = 1) {}
}

function referenceClone(value: unknown) {
  try {
    return structuredClone(value);
  } catch {
    return JSON.parse(JSON.stringify(value));
  }
}

describe('deepCloneValue', () => {
  const shared = { x: 1 };
  const cyclic: Record<string, unknown> = { name: 'c' };
  cyclic.self = cyclic;

  const cases: Record<string, unknown> = {
    primitives: { a: 1, b: 'two', c: true, d: null, e: undefined, f: 10n },
    nested: { a: [1, { b: [2, 3] }], d: new Date(5) },
    shared: { a: shared, b: shared },
    cyclic,
    'null prototype': Object.assign(Object.create(null), { a: 1 }),
    // biome-ignore lint/suspicious/noSparseArray: cloning sparse arrays is what's tested
    'sparse array': [1, , 3],
    'class instance': { p: new Point(2) },
    map: { m: new Map([[1, 2]]) },
    'with a function': { f: () => 1, a: 1 },
  };

  for (const [name, value] of Object.entries(cases))
    it(`should clone ${name} like structuredClone`, () => {
      const clone = deepCloneValue(value);

      expect(clone).not.toBe(value);
      expect(isDeepStrictEqual(clone, referenceClone(value))).toBe(true);
    });

  it('should keep shared references and cycles shared', () => {
    const sharedClone = deepCloneValue(cases.shared) as any;
    expect(sharedClone.a).toBe(sharedClone.b);
    expect(sharedClone.a).not.toBe(shared);

    const cyclicClone = deepCloneValue(cyclic) as any;
    expect(cyclicClone.self).toBe(cyclicClone);
  });

  it('should return primitives as they are', () => {
    for (const value of [1, 'a', true, null, undefined, 10n])
      expect(deepCloneValue(value)).toBe(value);
  });
});
