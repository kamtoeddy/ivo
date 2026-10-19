import { describe, expect, it } from 'bun:test';

import { Schema } from '../src';

type Values = { id: number; name: string; bio: string };

function captureContexts() {
  const seen: Record<string, any> = { validate: [] };

  const Model = new Schema<Values, Values, { tag: string }>((s) => {
    s.field(
      s.constant('id', (ctx) => {
        seen.constant = ctx;
        return 1;
      }),
    );
    s.field(
      s
        .required('name')
        .validate(((value: unknown, ctx: any) => {
          seen.validate.push(ctx);
          return { valid: true, validated: value };
        }) as never)
        .ignoreUpdate((ctx) => {
          seen.ignoreUpdate = ctx;
          return false;
        }),
    );
    s.field(
      s.lax('bio', (ctx) => {
        seen.default = ctx;
        return '';
      }),
    );
    return s;
  }).getModel();

  return { Model, seen };
}

describe('handler contexts', () => {
  it('should expose the same properties per kind of ctx', async () => {
    const { Model, seen } = captureContexts();

    const { data } = await Model.create({ name: 'a' }, { tag: 'x' });
    await Model.update(data!, { name: 'b' }, { tag: 'x' });

    const props = (ctx: object) =>
      ['changes', 'input', 'rawInput', 'isUpdate', 'previousValues', 'values']
        .concat(['options', 'updateOptions'])
        .filter((p) => p in ctx);

    expect(props(seen.constant)).toEqual([
      'input',
      'rawInput',
      'values',
      'options',
      'updateOptions',
    ]);
    expect(props(seen.default)).toEqual([
      'input',
      'rawInput',
      'options',
      'updateOptions',
    ]);
    expect(props(seen.ignoreUpdate)).toEqual([
      'input',
      'rawInput',
      'previousValues',
      'options',
      'updateOptions',
    ]);

    const [createCtx, updateCtx] = seen.validate;
    for (const ctx of [createCtx, updateCtx])
      expect(props(ctx)).toEqual([
        'changes',
        'input',
        'rawInput',
        'isUpdate',
        'previousValues',
        'values',
        'options',
        'updateOptions',
      ]);

    expect(createCtx.isUpdate).toBe(false);
    expect(createCtx.changes).toBeNull();
    expect(createCtx.previousValues).toBeNull();

    expect(updateCtx.isUpdate).toBe(true);
    expect(updateCtx.previousValues).toEqual(data);
    expect(updateCtx.changes).toEqual({ name: 'b' });
    expect(updateCtx.values).toEqual({ ...data, name: 'b' });
  });

  it('should be frozen', async () => {
    const { Model, seen } = captureContexts();

    await Model.create({ name: 'a' }, { tag: 'x' });

    const ctx = seen.validate[0];
    expect(Object.isFrozen(ctx)).toBe(true);
    expect(() => {
      ctx.values = {};
    }).toThrow();
    expect(() => {
      ctx.extra = 1;
    }).toThrow();
  });

  it('should return a fresh copy of the options on every read', async () => {
    const { Model, seen } = captureContexts();

    await Model.create({ name: 'a' }, { tag: 'x' });

    const ctx = seen.validate[0];
    expect(ctx.options).toEqual({ tag: 'x' });
    expect(ctx.options).not.toBe(ctx.options);

    ctx.options.tag = 'changed';
    expect(ctx.options).toEqual({ tag: 'x' });
  });
});
