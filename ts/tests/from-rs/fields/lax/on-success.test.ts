import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

function laxValidator(v: unknown) {
  if (v === 'fail_validation')
    return { valid: false, reason: 'validation failed' } as const;
  return { valid: true, validated: v as string };
}

describe('fields.lax.onSuccess', () => {
  it('should trigger onSuccess handlers at creation if provided', async () => {
    let triggeredWith: string | undefined;

    const Model = new Schema<{ lax: string; lax_1: string }>((b) =>
      b
        .field(
          b
            .lax('lax', 'default_value')
            .validate(laxValidator)
            .ignoreUpdate()
            .onSuccess((ctx) => {
              triggeredWith = ctx.rawInput.lax;
            }),
        )
        .field(b.lax('lax_1', 'default_value').validate(laxValidator)),
    ).getModel();

    const data = { lax_1: 'lax_1', lax: 'lax1' };
    const { data: created, handleSuccess } = await Model.create(data, {});

    expect(created).toEqual(data);

    await handleSuccess?.();

    expect(triggeredWith).toBe('lax1');
  });

  it('should trigger onSuccess handlers at creation even if not provided', async () => {
    let secondHandlerCalled = false;
    let triggeredWith: string | undefined;
    const defaultLaxValue = 'default_lax_value';

    const Model = new Schema<{ lax: string; lax_1: string }>((b) =>
      b
        .field(
          b
            .lax('lax', defaultLaxValue)
            .validate(laxValidator)
            .ignoreUpdate()
            .onSuccess([
              async () => {
                secondHandlerCalled = true;
              },
              (ctx) => {
                triggeredWith = ctx.values.lax;
              },
            ]),
        )
        .field(b.lax('lax_1', 'default_lax_1_value').validate(laxValidator)),
    ).getModel();

    const lax1 = 'lax_1';
    const { data: created, handleSuccess } = await Model.create(
      { lax_1: lax1 },
      {},
    );

    expect(created).toEqual({ lax: defaultLaxValue, lax_1: lax1 });

    await handleSuccess?.();

    expect(secondHandlerCalled).toBe(true);
    expect(triggeredWith).toBe(defaultLaxValue);
  });

  it('should trigger onSuccess handlers at creation even if provided and ignored', async () => {
    let triggeredWith: string | undefined;
    const defaultLaxValue = 'default_lax_value';

    const Model = new Schema<{ lax: string; lax_1: string }>((b) =>
      b
        .field(
          b
            .lax('lax', defaultLaxValue)
            .validate(laxValidator)
            .ignoreInit()
            .onSuccess((ctx) => {
              triggeredWith = ctx.values.lax;
            }),
        )
        .field(b.lax('lax_1', 'default_lax_1_value').validate(laxValidator)),
    ).getModel();

    const lax1Value = 'lax_1_value';
    const { data: created, handleSuccess } = await Model.create(
      { lax: 'lax_value', lax_1: lax1Value },
      {},
    );

    expect(created).toEqual({ lax: defaultLaxValue, lax_1: lax1Value });

    await handleSuccess?.();

    expect(triggeredWith).toBe(defaultLaxValue);
  });

  it('should trigger onSuccess handlers during updates if provided', async () => {
    let triggeredWith: string | undefined;
    const defaultLaxValue = 'default_lax_value';

    const Model = new Schema<{ lax: string; lax_1: string }>((b) =>
      b
        .field(
          b
            .lax('lax', defaultLaxValue)
            .validate(laxValidator)
            .onSuccess((ctx) => {
              triggeredWith = ctx.values.lax;
            }),
        )
        .field(b.lax('lax_1', 'default_lax_1_value').validate(laxValidator)),
    ).getModel();

    const lax1 = 'lax_1';
    const data = { lax: defaultLaxValue, lax_1: lax1 };
    const updatedLaxValue = 'updated_lax_value';

    const { data: updated, handleSuccess } = await Model.update(
      data,
      { lax: updatedLaxValue, lax_1: lax1 },
      {},
    );

    expect(updated).toEqual({ lax: updatedLaxValue });

    await handleSuccess?.();

    expect(triggeredWith).toBe(updatedLaxValue);
  });

  it('should not trigger onSuccess handlers during updates if not provided', async () => {
    let triggered = false;
    const defaultLaxValue = 'default_lax_value';

    const Model = new Schema<{ lax: string; lax_1: string }>((b) =>
      b
        .field(
          b
            .lax('lax', defaultLaxValue)
            .validate(laxValidator)
            .onSuccess(() => {
              triggered = true;
            }),
        )
        .field(b.lax('lax_1', 'default_lax_1_value').validate(laxValidator)),
    ).getModel();

    const lax1 = 'lax_1';
    const data = { lax: defaultLaxValue, lax_1: lax1 };
    const updatedLax1Value = 'updated_lax_1_value';

    const { data: updated, handleSuccess } = await Model.update(
      data,
      { lax_1: updatedLax1Value },
      {},
    );

    expect(updated).toEqual({ lax_1: updatedLax1Value });

    await handleSuccess?.();

    expect(triggered).toBe(false);
  });

  it('should not trigger onSuccess handlers during updates if provided and ignored', async () => {
    let triggered = false;
    const defaultLaxValue = 'default_lax_value';

    const Model = new Schema<{ lax: string; lax_1: string }>((b) =>
      b
        .field(
          b
            .lax('lax', defaultLaxValue)
            .validate(laxValidator)
            .ignoreUpdate()
            .onSuccess(() => {
              triggered = true;
            }),
        )
        .field(b.lax('lax_1', 'default_lax_1_value').validate(laxValidator)),
    ).getModel();

    const lax1 = 'lax_1';
    const data = { lax: defaultLaxValue, lax_1: lax1 };
    const updatedLaxValue = 'updated_lax_value';
    const updatedLax1Value = 'updated_lax_1_value';

    const { data: updated, handleSuccess } = await Model.update(
      data,
      { lax: updatedLaxValue, lax_1: updatedLax1Value },
      {},
    );

    expect(updated).toEqual({ lax_1: updatedLax1Value });

    await handleSuccess?.();

    expect(triggered).toBe(false);
  });

  it('should trigger global success function handlers each time creation is successful', async () => {
    let triggered = false;
    const defaultLax = 1234;
    const defaultLax1 = 5678;

    const Model = new Schema<{ lax: number; lax_1: number }>(
      (b) =>
        b.field(b.lax('lax', defaultLax)).field(b.lax('lax_1', defaultLax1)),
      {
        onSuccess: () => {
          triggered = true;
        },
      },
    ).getModel();

    const { data, handleSuccess } = await Model.create({}, {});

    expect(data).toEqual({ lax: defaultLax, lax_1: defaultLax1 });

    await handleSuccess?.();

    expect(triggered).toBe(true);
  });

  it('should trigger global success function each time update is successful', async () => {
    let triggered = false;
    const defaultLax = 1234;
    const defaultLax1 = 5678;

    const Model = new Schema<{ lax: number; lax_1: number }>(
      (b) =>
        b.field(b.lax('lax', defaultLax)).field(b.lax('lax_1', defaultLax1)),
      {
        onSuccess: () => {
          triggered = true;
        },
      },
    ).getModel();

    const data = { lax: defaultLax, lax_1: defaultLax1 };
    const updatedLax1 = data.lax_1 + 1;

    const { data: updates, handleSuccess } = await Model.update(
      data,
      { lax_1: updatedLax1 },
      {},
    );

    expect(updates).toEqual({ lax_1: updatedLax1 });

    await handleSuccess?.();

    expect(triggered).toBe(true);
  });
});
