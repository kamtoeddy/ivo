import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

describe('fields.lax.onFailure', () => {
  it('should trigger onFailure handlers at creation', async () => {
    let triggeredWith: string | undefined;

    const Model = new Schema<{ lax: string }>((b) =>
      b.field(
        b
          .lax('lax', 'default_value')
          .validate((v: unknown) => {
            if (v === 'fail_validation')
              return { valid: false, reason: 'validation failed' };
            return { valid: true, validated: v as string };
          })
          .onFailure((ctx) => {
            triggeredWith = ctx.input.lax;
          }),
      ),
    ).getModel();

    const { error, handleFailure } = await Model.create(
      { lax: 'fail_validation' },
      {},
    );

    expect(error?.lax?.reason).toBe('validation failed');

    await handleFailure?.();

    expect(triggeredWith).toBe('fail_validation');
  });

  it('should trigger onFailure handlers at creation even if provided and ignored', async () => {
    let secondHandlerCalled = false;
    let triggeredWith: string | undefined;

    const Model = new Schema<{ lax: string; lax2: string }>((b) =>
      b
        .field(
          b
            .lax('lax', 'default_value')
            .ignoreInit()
            .onFailure([
              (ctx) => {
                triggeredWith = ctx.rawInput.lax;
              },
              async () => {
                secondHandlerCalled = true;
              },
            ]),
        )
        .field(
          b.lax('lax2', 'default_value').validate((v: unknown) => {
            if (v === 'fail_validation')
              return { valid: false, reason: 'validation failed' };
            return { valid: true, validated: v as string };
          }),
        ),
    ).getModel();

    const { error, handleFailure } = await Model.create(
      { lax: 'to be ignored', lax2: 'fail_validation' },
      {},
    );

    expect(error?.lax).toBeUndefined();
    expect(error?.lax2?.reason).toBe('validation failed');

    await handleFailure?.();

    expect(triggeredWith).toBe('to be ignored');
    expect(secondHandlerCalled).toBe(true);
  });

  it('should trigger onFailure handlers during updates', async () => {
    let triggeredWith: string | undefined;

    const Model = new Schema<{ lax: string }>((b) =>
      b.field(
        b
          .lax('lax', 'default_value')
          .validate((v: unknown) => {
            if (v === 'fail_validation')
              return { valid: false, reason: 'validation failed' };
            return { valid: true, validated: v as string };
          })
          .onFailure((ctx) => {
            triggeredWith = ctx.input.lax;
          }),
      ),
    ).getModel();

    const { error, handleFailure } = await Model.update(
      { lax: 'some value' },
      { lax: 'fail_validation' },
      {},
    );

    expect(error?.payload?.lax?.reason).toBe('validation failed');

    await handleFailure?.();

    expect(triggeredWith).toBe('fail_validation');
  });

  it('should trigger onFailure handlers during updates with unchanged values', async () => {
    let triggeredWithRaw: string | undefined;
    let triggeredWithInput: string | undefined;

    const Model = new Schema<{ lax: string }>((b) =>
      b.field(
        b
          .lax('lax', 'default_value')
          .validate((v: unknown) => {
            if (v === 'fail_validation')
              return { valid: false, reason: 'validation failed' };
            return { valid: true, validated: v as string };
          })
          .onFailure((ctx) => {
            triggeredWithRaw = ctx.rawInput.lax;
            triggeredWithInput = ctx.input.lax;
          }),
      ),
    ).getModel();

    const laxValue = 'some_value';

    const { error, handleFailure } = await Model.update(
      { lax: laxValue },
      { lax: laxValue },
      {},
    );

    expect(error).toEqual({ isNothingToUpdate: true, payload: null });

    await handleFailure?.();

    expect(triggeredWithRaw).toBe(laxValue);
    expect(triggeredWithInput).toBeUndefined();
  });

  it('should trigger onFailure handlers during updates even if provided and ignored', async () => {
    let triggeredWith: string | undefined;

    const Model = new Schema<{ lax: string; lax2: string }>((b) =>
      b
        .field(
          b
            .lax('lax', 'default_value')
            .validate((v: unknown) => {
              if (v === 'fail_validation')
                return { valid: false, reason: 'validation failed' };
              return { valid: true, validated: v as string };
            })
            .ignoreUpdate()
            .onFailure((ctx) => {
              triggeredWith = ctx.rawInput.lax;
            }),
        )
        .field(
          b.lax('lax2', 'default_value').validate((v: unknown) => {
            if (v === 'fail_validation')
              return { valid: false, reason: 'validation failed' };
            return { valid: true, validated: v as string };
          }),
        ),
    ).getModel();

    const { error, handleFailure } = await Model.update(
      { lax: 'lax1', lax2: 'lax2' },
      { lax: 'update to be ignored', lax2: 'fail_validation' },
      {},
    );

    expect(error?.payload?.lax).toBeUndefined();
    expect(error?.payload?.lax2?.reason).toBe('validation failed');

    await handleFailure?.();

    expect(triggeredWith).toBe('update to be ignored');
  });
});
