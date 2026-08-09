import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../src';

describe('fields.constant', () => {
  it('should respect constants with static values', async () => {
    const constant = 1234;

    const Model = new Schema<
      { lax: number },
      { constant: number; lax: number }
    >((b) =>
      b.field(b.constant('constant', constant)).field(b.lax('lax', 20)),
    ).getModel();

    let lax = 400;
    let { data } = await Model.create({ lax }, {});
    expect(data).toEqual({ constant, lax });

    lax = 700;
    ({ data } = await Model.create({ lax }, {}));
    expect(data).toEqual({ constant, lax });

    const previous = data!;
    lax = 200;
    const { data: updates } = await Model.update(previous, { lax }, {});
    expect(updates).toEqual({ lax });
  });

  it('should respect constants with computed values', async () => {
    const constant = 1234;

    const Model = new Schema<
      { lax: number },
      { constant: number; lax: number }
    >((b) =>
      b.field(b.constant('constant', () => constant)).field(b.lax('lax', 20)),
    ).getModel();

    let lax = 400;
    let { data } = await Model.create({ lax }, {});
    expect(data).toEqual({ constant, lax });

    lax = 700;
    ({ data } = await Model.create({ lax }, {}));
    expect(data).toEqual({ constant, lax });

    const previous = data!;
    lax = 200;
    const { data: updates } = await Model.update(previous, { lax }, {});
    expect(updates).toEqual({ lax });
  });

  it('should trigger onDelete handlers with static values', async () => {
    const constant = 1234;
    let triggeredWith: number | undefined;

    const Model = new Schema<
      { lax: number },
      { constant: number; lax: number }
    >((b) =>
      b
        .field(
          b.constant('constant', constant).onDelete((data) => {
            triggeredWith = data.constant;
          }),
        )
        .field(b.lax('lax', 20)),
    ).getModel();

    await Model.delete({ constant, lax: 400 }, {});

    expect(triggeredWith).toBe(constant);
  });

  it('should trigger onDelete handlers with computed values', async () => {
    const constant = 1234;
    let triggeredWith: number | undefined;
    let secondHandlerCalled = false;

    const Model = new Schema<
      { lax: number },
      { constant: number; lax: number }
    >((b) =>
      b
        .field(
          b
            .constant('constant', () => constant)
            .onDelete([
              (data) => {
                triggeredWith = data.constant;
              },
              () => {
                secondHandlerCalled = true;
              },
            ]),
        )
        .field(b.lax('lax', 20)),
    ).getModel();

    await Model.delete({ constant, lax: 400 }, {});

    expect(triggeredWith).toBe(constant);
    expect(secondHandlerCalled).toBe(true);
  });

  it('should trigger onSuccess handlers with static values', async () => {
    const constant = 1234;
    let triggeredWith: number | undefined;

    const Model = new Schema<
      { lax: number },
      { constant: number; lax: number }
    >((b) =>
      b
        .field(
          b.constant('constant', constant).onSuccess((ctx) => {
            triggeredWith = ctx.values.constant;
          }),
        )
        .field(b.lax('lax', 20)),
    ).getModel();

    const lax = 400;
    const { data, handleSuccess } = await Model.create({ lax }, {});

    expect(data).toEqual({ constant, lax });

    await handleSuccess?.();

    expect(triggeredWith).toBe(constant);
  });

  it('should trigger onSuccess handlers with computed values', async () => {
    const constant = 1234;
    let triggeredWith: number | undefined;
    let secondHandlerCalled = false;

    const Model = new Schema<
      { lax: number },
      { constant: number; lax: number }
    >((b) =>
      b
        .field(
          b
            .constant('constant', () => constant)
            .onSuccess([
              (ctx) => {
                triggeredWith = ctx.values.constant;
              },
              async () => {
                secondHandlerCalled = true;
              },
            ]),
        )
        .field(b.lax('lax', 20)),
    ).getModel();

    const lax = 400;
    const { data, handleSuccess } = await Model.create({ lax }, {});

    expect(data).toEqual({ constant, lax });

    await handleSuccess?.();

    expect(triggeredWith).toBe(constant);
    expect(secondHandlerCalled).toBe(true);
  });
});
