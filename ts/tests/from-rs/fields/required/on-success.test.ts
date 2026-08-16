import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

function requiredValidator(v: unknown) {
  if (v === 'fail_validation')
    return { valid: false, reason: 'validation failed' } as const;
  return { valid: true } as const;
}

describe('fields.required.onSuccess', () => {
  it('should trigger onSuccess handlers at creation', async () => {
    let triggeredWith: string | undefined;

    const Model = new Schema<{ required: string; required2: string }>((b) =>
      b
        .field(
          b
            .required('required')
            .validate(requiredValidator)
            .onSuccess((ctx) => {
              triggeredWith = ctx.rawInput.required;
            }),
        )
        .field(b.required('required2').validate(requiredValidator)),
    ).getModel();

    const data = { required2: 'required2', required: 'required1' };
    const { data: created, handleSuccess } = await Model.create(data, {});

    expect(created).toEqual(data);

    await handleSuccess?.();

    expect(triggeredWith).toBe('required1');
  });

  it('should trigger onSuccess handlers during updates if provided', async () => {
    let triggeredWith: string | undefined;

    const Model = new Schema<{ required: string; required2: string }>((b) =>
      b
        .field(
          b
            .required('required')
            .validate(requiredValidator)
            .onSuccess((ctx) => {
              triggeredWith = ctx.values.required;
            }),
        )
        .field(b.required('required2').validate(requiredValidator)),
    ).getModel();

    const required2 = 'required2';
    const data = { required2, required: 'required_value_value' };
    const updatedRequiredValue = 'updated_required_value';

    const { data: updated, handleSuccess } = await Model.update(
      data,
      { required: updatedRequiredValue, required2 },
      {},
    );

    expect(updated).toEqual({ required: updatedRequiredValue });

    await handleSuccess?.();

    expect(triggeredWith).toBe(updatedRequiredValue);
  });

  it('should not trigger onSuccess handlers during updates if not provided', async () => {
    let triggered = false;

    const Model = new Schema<{ required: string; required2: string }>((b) =>
      b
        .field(
          b
            .required('required')
            .validate(requiredValidator)
            .onSuccess(() => {
              triggered = true;
            }),
        )
        .field(b.required('required2').validate(requiredValidator)),
    ).getModel();

    const required2 = 'required2';
    const data = { required2, required: 'required_value_value' };
    const updatedRequired2Value = 'updated_required2_value';

    const { data: updated, handleSuccess } = await Model.update(
      data,
      { required2: updatedRequired2Value },
      {},
    );

    expect(updated).toEqual({ required2: updatedRequired2Value });

    await handleSuccess?.();

    expect(triggered).toBe(false);
  });

  it('should not trigger onSuccess handlers during updates if provided and ignored', async () => {
    let triggered = false;

    const Model = new Schema<{ required: string; required2: string }>((b) =>
      b
        .field(
          b
            .required('required')
            .validate(requiredValidator)
            .ignoreUpdate(() => true)
            .onSuccess(() => {
              triggered = true;
            }),
        )
        .field(b.required('required2').validate(requiredValidator)),
    ).getModel();

    const required2 = 'required2';
    const data = { required2, required: 'required_value_value' };
    const updatedRequiredValue = 'updated_required_value';
    const updatedRequired2Value = 'updated_required2_value';

    const { data: updated, handleSuccess } = await Model.update(
      data,
      { required2: updatedRequired2Value, required: updatedRequiredValue },
      {},
    );

    expect(updated).toEqual({ required2: updatedRequired2Value });

    await handleSuccess?.();

    expect(triggered).toBe(false);
  });

  it('should not trigger onSuccess handlers during updates if provided and ignored as readonly', async () => {
    let triggered = false;
    let secondHandlerCalled = false;

    const Model = new Schema<{ required: string; required2: string }>((b) =>
      b
        .field(
          b
            .required('required')
            .validate(requiredValidator)
            .readonly()
            .onSuccess([
              () => {
                triggered = true;
              },
              async () => {
                secondHandlerCalled = true;
              },
            ]),
        )
        .field(b.required('required2').validate(requiredValidator)),
    ).getModel();

    const required2 = 'required2';
    const data = { required2, required: 'required_value_value' };
    const updatedRequiredValue = 'updated_required_value';
    const updatedRequired2Value = 'updated_required2_value';

    const { data: updated, handleSuccess } = await Model.update(
      data,
      { required2: updatedRequired2Value, required: updatedRequiredValue },
      {},
    );

    expect(updated).toEqual({ required2: updatedRequired2Value });

    await handleSuccess?.();

    expect(triggered).toBe(false);
    expect(secondHandlerCalled).toBe(false);
  });

  it('should trigger global success function handlers each time creation is successful', async () => {
    let triggered = false;

    const Model = new Schema<{ required: number; required_1: number }>(
      (b) =>
        b
          .field(b.required('required').validate(() => ({ valid: true })))
          .field(b.required('required_1').validate(() => ({ valid: true }))),
      {
        onSuccess: () => {
          triggered = true;
        },
      },
    ).getModel();

    const requiredValue = 1234;
    const required1Value = 5678;

    const { data, handleSuccess } = await Model.create(
      { required: requiredValue, required_1: required1Value },
      {},
    );

    expect(data).toEqual({
      required: requiredValue,
      required_1: required1Value,
    });

    await handleSuccess?.();

    expect(triggered).toBe(true);
  });

  it('should trigger global success function handlers each time update is successful', async () => {
    let triggered = false;

    const Model = new Schema<{ required: number; required_1: number }>(
      (b) =>
        b
          .field(b.required('required').validate(() => ({ valid: true })))
          .field(b.required('required_1').validate(() => ({ valid: true }))),
      {
        onSuccess: () => {
          triggered = true;
        },
      },
    ).getModel();

    const data = { required: 1234, required_1: 5678 };
    const updatedRequired1 = data.required_1 + 1;

    const { data: updates, handleSuccess } = await Model.update(
      data,
      { required_1: updatedRequired1 },
      {},
    );

    expect(updates).toEqual({ required_1: updatedRequired1 });

    await handleSuccess?.();

    expect(triggered).toBe(true);
  });
});
