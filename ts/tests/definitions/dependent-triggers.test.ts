import { afterEach, describe, expect, it } from 'bun:test';
import { Schema } from '../../src';

/**
 * When dependent resolvers run (from bug-repro/scenarios.ts):
 *
 * create -- a dependent's resolver runs iff at least one parent:
 *   - virtual or required: was provided & accepted
 *   - lax: was provided & accepted with a value other than its static
 *     default, or has a resolver default
 *   - dependent: was resolved to a value other than its default, or has a
 *     resolver default
 *
 * update -- a dependent's resolver runs iff at least one parent:
 *   - virtual: was provided & accepted (always)
 *   - required / lax: was provided & accepted, with a value other than its
 *     previous one
 *   - dependent: was resolved to a value other than its previous one
 *
 * A dependent resolves at most once per operation, whatever its parents.
 */
describe('dependent resolvers trigger', () => {
  type Input = {
    title: string; // required
    note: string; // lax, static default
    region: string; // lax, resolver default
    flag: boolean; // virtual -> flagged
    touch: boolean; // virtual -> tag (always "set") -> tag_child
    seed: boolean; // virtual -> stamp (resolver default) -> stamp_child
  };

  type Output = {
    title: string;
    note: string;
    region: string;
    from_required: string;
    from_lax_static: string;
    from_lax_dynamic: string;
    flagged: boolean;
    tag: string;
    tag_child: string;
    stamp: string;
    stamp_child: string;
    summary: string;
  };

  const calls: Record<string, number> = {};

  const hit = <T>(name: string, value: T): T => {
    calls[name] = (calls[name] ?? 0) + 1;
    return value;
  };

  const ok = () => true;

  const Model = new Schema<Input, Output>((b) =>
    b
      .field(b.required('title').validate(ok))
      .field(b.lax('note', '').validate(ok))
      .field(b.lax('region', () => 'CM').validate(ok))
      .field(b.virtual('flag').validate(ok))
      .field(b.virtual('touch').validate(ok))
      .field(b.virtual('seed').validate(ok))
      .field(
        b
          .dependent('from_required', 'title')
          .default('')
          .resolve(({ values }) => hit('from_required', `r:${values.title}`)),
      )
      .field(
        b
          .dependent('from_lax_static', 'note')
          .default('')
          .resolve(({ values }) => hit('from_lax_static', `n:${values.note}`)),
      )
      .field(
        b
          .dependent('from_lax_dynamic', 'region')
          .default('')
          .resolve(({ values }) =>
            hit('from_lax_dynamic', `g:${values.region}`),
          ),
      )
      .field(
        b
          .dependent('flagged', 'flag')
          .default(false)
          .resolve(({ input }) => hit('flagged', input.flag === true)),
      )
      .field(
        b
          .dependent('tag', 'touch')
          .default('none')
          .resolve(() => hit('tag', 'set')),
      )
      .field(
        b
          .dependent('tag_child', 'tag')
          .default('')
          .resolve(({ values }) => hit('tag_child', `c:${values.tag}`)),
      )
      .field(
        b
          .dependent('stamp', 'seed')
          .default(() => 'auto')
          .resolve(() => hit('stamp', 'seeded')),
      )
      .field(
        b
          .dependent('stamp_child', 'stamp')
          .default('')
          .resolve(({ values }) => hit('stamp_child', `s:${values.stamp}`)),
      )
      .field(
        b
          .dependent('summary', ['title', 'note', 'flag'])
          .default('')
          .resolve(({ values }) =>
            hit('summary', `${values.title}|${values.note}`),
          ),
      ),
  ).getModel();

  afterEach(() => {
    for (const name of Object.keys(calls)) delete calls[name];
  });

  async function created(input: Partial<Input>) {
    const { data } = await Model.create(input, {});

    for (const name of Object.keys(calls)) delete calls[name];

    return data as Output;
  }

  describe('creation', () => {
    it('C1: required only', async () => {
      await Model.create({ title: 'Ann' }, {});

      expect(calls).toEqual({
        from_required: 1, // required provided
        from_lax_dynamic: 1, // lax parent with a resolver default
        stamp_child: 1, // dependent parent with a resolver default
        summary: 1, // via title
      });
    });

    it('C2: every input provided', async () => {
      await Model.create(
        {
          title: 'Ann',
          note: 'hi',
          region: 'NG',
          flag: true,
          touch: true,
          seed: true,
        },
        {},
      );

      expect(calls).toEqual({
        from_required: 1,
        from_lax_static: 1,
        from_lax_dynamic: 1,
        flagged: 1,
        tag: 1,
        tag_child: 1,
        stamp: 1,
        stamp_child: 1, // its parent has a resolver default and resolves
        summary: 1, // three parents, still once
      });
    });

    it('a lax parent provided with its static default', async () => {
      await Model.create({ title: 'Ann', note: '' }, {});

      expect(calls.from_lax_static).toBeUndefined();
      expect(calls.summary).toBe(1); // via title only
    });
  });

  describe('updates', () => {
    it('U1: required changed', async () => {
      const base = await created({ title: 'Ann' });

      await Model.update(base, { title: 'Bob' }, {});

      expect(calls).toEqual({ from_required: 1, summary: 1 });
    });

    it('U2: required with the same value', async () => {
      const base = await created({ title: 'Ann' });

      const { error } = await Model.update(base, { title: 'Ann' }, {});

      expect(calls).toEqual({});
      expect(error?.isNothingToUpdate).toBe(true);
    });

    it('U3: lax (static default) changed', async () => {
      const base = await created({ title: 'Ann' });

      await Model.update(base, { note: 'x' }, {});

      expect(calls).toEqual({ from_lax_static: 1, summary: 1 });
    });

    it('U4: lax fields with the same values', async () => {
      const base = await created({ title: 'Ann' });

      const { error } = await Model.update(
        base,
        { note: '', region: 'CM' },
        {},
      );

      expect(calls).toEqual({});
      expect(error?.isNothingToUpdate).toBe(true);
    });

    it('U5: lax (resolver default) changed', async () => {
      const base = await created({ title: 'Ann' });

      await Model.update(base, { region: 'NG' }, {});

      expect(calls).toEqual({ from_lax_dynamic: 1 });
    });

    it('U6: virtual provided, dependents resolve to new values', async () => {
      const base = await created({ title: 'Ann' });

      await Model.update(base, { flag: true }, {});

      expect(calls).toEqual({ flagged: 1, summary: 1 });
    });

    it('U7: virtual provided, dependents resolve to the same values', async () => {
      const base = await created({ title: 'Ann' });

      const { error } = await Model.update(base, { flag: false }, {});

      expect(calls).toEqual({ flagged: 1, summary: 1 });
      expect(error?.isNothingToUpdate).toBe(true);
    });

    it('U8: a dependent parent resolving to a new value triggers its child', async () => {
      const base = await created({ title: 'Ann' });

      await Model.update(base, { touch: true }, {});

      expect(calls).toEqual({ tag: 1, tag_child: 1 });
    });

    it('U9: a dependent parent resolving to the same value does not trigger its child', async () => {
      const touched = await created({ title: 'Ann', touch: true });

      const { error } = await Model.update(touched, { touch: true }, {});

      expect(calls).toEqual({ tag: 1 });
      expect(error?.isNothingToUpdate).toBe(true);
    });

    it('U10: a dependent parent (resolver default) changing triggers its child', async () => {
      const base = await created({ title: 'Ann' });

      await Model.update(base, { seed: true }, {});

      expect(calls).toEqual({ stamp: 1, stamp_child: 1 });
    });

    it('U11: a dependent parent (resolver default) resolving to the same value does not trigger its child', async () => {
      const seeded = await created({ title: 'Ann', seed: true });

      const { error } = await Model.update(seeded, { seed: true }, {});

      expect(calls).toEqual({ stamp: 1 });
      expect(error?.isNothingToUpdate).toBe(true);
    });

    it('U12: many parents changing at once resolve their dependent once', async () => {
      const base = await created({ title: 'Ann' });

      await Model.update(base, { title: 'Zed', note: 'z', flag: true }, {});

      expect(calls).toEqual({
        from_required: 1,
        from_lax_static: 1,
        flagged: 1,
        summary: 1,
      });
    });
  });

  // from bug-repro/minimal.ts
  it('should not resolve a child of a dependent that resolves to its previous value', async () => {
    const Model = new Schema<
      { a: string; v: boolean },
      { a: string; d: number; e: number }
    >((b) =>
      b
        .field(b.required('a').validate(ok))
        .field(b.virtual('v').validate(ok))
        .field(
          b
            .dependent('d', 'v')
            .default(0)
            .resolve(() => 1),
        )
        .field(
          b
            .dependent('e', 'd')
            .default(0)
            .resolve(({ isUpdate }) => (isUpdate ? 99 : 0)),
        ),
    ).getModel();

    const { error } = await Model.update(
      { a: 'x', d: 1, e: 0 },
      { v: true },
      {},
    );

    expect(error?.isNothingToUpdate).toBe(true);
  });

  // `x` depends on `a` and on `e`, itself a dependent of `b`: with both
  // provided, `x` must wait for `e` instead of resolving once per parent
  describe('a dependent with parents on different levels', () => {
    let resolvedX = 0;

    const Model = new Schema<
      { a: string; b: string },
      { a: string; b: string; e: string; x: string }
    >((s) =>
      s
        .field(s.required('a').validate(ok))
        .field(s.required('b').validate(ok))
        .field(
          s
            .dependent('e', 'b')
            .default('')
            .resolve(({ values }) => `e:${values.b}`),
        )
        .field(
          s
            .dependent('x', ['a', 'e'])
            .default('')
            .resolve(({ values }) => {
              resolvedX++;
              return `x:${values.a}:${values.e}`;
            }),
        ),
    ).getModel();

    afterEach(() => {
      resolvedX = 0;
    });

    it('should resolve once at creation, after its dependent parent', async () => {
      const { data } = await Model.create({ a: '1', b: '1' }, {});

      expect(resolvedX).toBe(1);
      expect(data).toEqual({ a: '1', b: '1', e: 'e:1', x: 'x:1:e:1' });
    });

    it('should resolve once during updates, after its dependent parent', async () => {
      const previous = { a: '1', b: '1', e: 'e:1', x: 'x:1:e:1' };

      const { data } = await Model.update(previous, { a: '2', b: '2' }, {});

      expect(resolvedX).toBe(1);
      expect(data).toEqual({ a: '2', b: '2', e: 'e:2', x: 'x:2:e:2' });
    });
  });
});
