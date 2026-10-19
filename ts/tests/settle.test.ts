import { describe, expect, it } from 'bun:test';

import { Schema } from '../src';
import { allOrThrow, settleAll, settleEach } from '../src/model/settle';

describe('settle helpers', () => {
  describe('settleEach', () => {
    it('should call every handler before applying any outcome, and stay sync when all handlers are', () => {
      const log: string[] = [];

      const result = settleEach(
        ['a', 'b'],
        (item) => {
          log.push(`call ${item}`);
          if (item === 'b') throw new Error('b failed');
          return item.toUpperCase();
        },
        (item, outcome) =>
          log.push(`settle ${item} ${outcome.ok ? outcome.value : 'error'}`),
      );

      expect(result).toBeUndefined();
      expect(log).toEqual(['call a', 'call b', 'settle a A', 'settle b error']);
    });

    it('should apply sync outcomes before awaiting, and async ones as they settle', async () => {
      const log: string[] = [];

      const result = settleEach(
        ['slow', 'sync', 'fast'],
        (item) =>
          item === 'sync'
            ? item
            : new Promise<string>((resolve) =>
                setTimeout(() => resolve(item), item === 'slow' ? 20 : 1),
              ),
        (item) => log.push(item),
      );

      expect(log).toEqual(['sync']);
      expect(result).toBeInstanceOf(Promise);

      await result;

      expect(log).toEqual(['sync', 'fast', 'slow']);
    });

    it('should swallow errors thrown while applying an outcome', () => {
      const applied: string[] = [];

      settleEach(
        ['a', 'b'],
        (item) => item,
        (item) => {
          if (item === 'a') throw new Error('apply failed');
          applied.push(item);
        },
      );

      expect(applied).toEqual(['b']);
    });
  });

  describe('settleAll', () => {
    it('should return outcomes in order, synchronously when every function is sync', () => {
      const outcomes = settleAll<unknown>([
        () => 1,
        () => {
          throw new Error('2 failed');
        },
        // a result that merely looks like a failure marker stays a value
        () => ({ ok: false, error: 'not a throw' }),
      ]);

      expect(Array.isArray(outcomes)).toBe(true);
      expect(outcomes).toEqual([
        { ok: true, value: 1 },
        { ok: false, error: new Error('2 failed') },
        { ok: true, value: { ok: false, error: 'not a throw' } },
      ]);
    });

    it('should wait for async functions and keep the order', async () => {
      const outcomes = settleAll<number>([
        () => Promise.resolve(1),
        () => 2,
        () => Promise.reject(new Error('3 failed')),
      ]);

      expect(outcomes).toBeInstanceOf(Promise);
      expect(await outcomes).toEqual([
        { ok: true, value: 1 },
        { ok: true, value: 2 },
        { ok: false, error: new Error('3 failed') },
      ]);
    });
  });

  describe('allOrThrow', () => {
    it('should call every function even when an earlier one throws, then throw the first error', () => {
      const called: number[] = [];

      expect(() =>
        allOrThrow([
          () => called.push(1),
          () => {
            called.push(2);
            throw new Error('2 failed');
          },
          () => called.push(3),
        ]),
      ).toThrow('2 failed');

      expect(called).toEqual([1, 2, 3]);
    });

    it('should reject when some function is async and one fails', async () => {
      const result = allOrThrow([
        () => Promise.resolve(1),
        () => {
          throw new Error('2 failed');
        },
      ]);

      expect(result).toBeInstanceOf(Promise);
      await expect(result).rejects.toThrow('2 failed');
    });
  });
});

describe('validation phase ordering', () => {
  it('should let an async validator see the values of sync validators after it awaits', async () => {
    let seenByAsync: unknown;

    const Model = new Schema<{ a: string; b: string }>((s) => {
      s.field(
        s
          .required('a')
          .validate((v) => ({ valid: true, validated: `${v}!` }) as never),
      );
      s.field(
        s.required('b').validate((async (v: unknown, ctx: any) => {
          await Promise.resolve();
          seenByAsync = ctx.values.a;
          return { valid: true, validated: v };
        }) as never),
      );
      return s;
    }).getModel();

    const { data } = await Model.create({ a: 'x', b: 'y' }, {});

    expect(data).toEqual({ a: 'x!', b: 'y' });
    expect(seenByAsync).toBe('x!');
  });
});
