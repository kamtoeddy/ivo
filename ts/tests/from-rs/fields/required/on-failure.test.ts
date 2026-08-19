import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

function requiredValidator(v: unknown) {
  if (v === 'fail_validation')
    return { valid: false, reason: 'validation failed' } as const;
  return { valid: true } as const;
}

describe('fields.required.onFailure', () => {
  it('should trigger onFailure handlers at creation', async () => {
    let triggeredWith: string | undefined;

    const Model = new Schema<{ required: string }>((b) =>
      b.field(
        b
          .required('required')
          .validate(requiredValidator)
          .onFailure((ctx) => {
            triggeredWith = ctx.input.required;
          }),
      ),
    ).getModel();

    const { error, handleFailure } = await Model.create(
      { required: 'fail_validation' },
      {},
    );

    expect(error?.required?.reason).toBe('validation failed');

    await handleFailure?.();

    expect(triggeredWith).toBe('fail_validation');
  });

  it('should trigger onFailure handlers during updates', async () => {
    let triggeredWith: string | undefined;

    const Model = new Schema<{ required: string }>((b) =>
      b.field(
        b
          .required('required')
          .validate(requiredValidator)
          .onFailure((ctx) => {
            triggeredWith = ctx.input.required;
          }),
      ),
    ).getModel();

    const { error, handleFailure } = await Model.update(
      { required: 'some value' },
      { required: 'fail_validation' },
      {},
    );

    expect(error?.payload?.required?.reason).toBe('validation failed');

    await handleFailure?.();

    expect(triggeredWith).toBe('fail_validation');
  });

  it('should trigger onFailure handlers during updates with unchanged values', async () => {
    let triggeredWithRaw: string | undefined;
    let triggeredWithInput: string | undefined;
    let secondHandlerCalled = false;

    const Model = new Schema<{ required: string }>((b) =>
      b.field(
        b
          .required('required')
          .validate(requiredValidator)
          .onFailure([
            (ctx) => {
              triggeredWithRaw = ctx.rawInput.required;
              triggeredWithInput = ctx.input.required;
            },
            () => {
              secondHandlerCalled = true;
            },
          ]),
      ),
    ).getModel();

    const requiredValue = 'some_value';

    const { error, handleFailure } = await Model.update(
      { required: requiredValue },
      { required: requiredValue },
      {},
    );

    expect(error).toEqual({ isNothingToUpdate: true, payload: null });

    await handleFailure?.();

    expect(triggeredWithRaw).toBe(requiredValue);
    expect(triggeredWithInput).toBeUndefined();
    expect(secondHandlerCalled).toBe(true);
  });

  it('should trigger onFailure handlers during updates even if provided and ignored', async () => {
    let triggeredWith: string | undefined;

    const Model = new Schema<{ required: string; required2: string }>((b) =>
      b
        .field(
          b
            .required('required')
            .validate(requiredValidator)
            .ignoreUpdate(() => true)
            .onFailure((ctx) => {
              triggeredWith = ctx.rawInput.required;
            }),
        )
        .field(b.required('required2').validate(requiredValidator)),
    ).getModel();

    const { error, handleFailure } = await Model.update(
      { required: 'required1', required2: 'required2' },
      { required: 'update to be ignored', required2: 'fail_validation' },
      {},
    );

    expect(error?.payload?.required).toBeUndefined();
    expect(error?.payload?.required2?.reason).toBe('validation failed');

    await handleFailure?.();

    expect(triggeredWith).toBe('update to be ignored');
  });

  it('should trigger onFailure handlers during updates even if provided and ignored as readonly', async () => {
    let triggeredWith: string | undefined;

    const Model = new Schema<{ required: string; required2: string }>((b) =>
      b
        .field(
          b
            .required('required')
            .validate(requiredValidator)
            .readonly()
            .onFailure((ctx) => {
              triggeredWith = ctx.rawInput.required;
            }),
        )
        .field(b.required('required2').validate(requiredValidator)),
    ).getModel();

    const { error, handleFailure } = await Model.update(
      { required: 'required1', required2: 'required2' },
      { required: 'update to be ignored', required2: 'fail_validation' },
      {},
    );

    expect(error?.payload?.required).toBeUndefined();
    expect(error?.payload?.required2?.reason).toBe('validation failed');

    await handleFailure?.();

    expect(triggeredWith).toBe('update to be ignored');
  });
});
