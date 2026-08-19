import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../src';

describe('fields.dependent', () => {
  it('should use static default value of dependent if resolver is not run at creation', async () => {
    const dependent = 1234;
    const lax = 20;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number }
    >((b) =>
      b
        .field(
          b
            .dependent('dependent', 'lax')
            .default(dependent)
            .resolve((ctx) => ctx.values.dependent! + 1),
        )
        .field(b.lax('lax', lax)),
    ).getModel();

    const { data } = await Model.create({}, {});

    expect(data).toEqual({ dependent, lax });
  });

  it('should use computed default value of dependent if resolver is not run at creation', async () => {
    const dependent = 1234;
    const lax = 20;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number }
    >((b) =>
      b
        .field(
          b
            .dependent('dependent', 'lax')
            .default(() => dependent)
            .resolve((ctx) => ctx.values.dependent! + 1),
        )
        .field(b.lax('lax', lax)),
    ).getModel();

    const { data } = await Model.create({}, {});

    expect(data).toEqual({ dependent, lax });
  });

  it('should properly run dependent resolver', async () => {
    const defaultDependentValue = 1234;
    const defaultLaxValue = 20;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number }
    >((b) =>
      b
        .field(
          b
            .dependent('dependent', 'lax')
            .default(defaultDependentValue)
            .resolve((ctx) => ctx.values.dependent! + 1),
        )
        .field(b.lax('lax', defaultLaxValue)),
    ).getModel();

    let { data } = await Model.create({ lax: defaultLaxValue }, {});

    expect(data).toEqual({
      dependent: defaultDependentValue + 1,
      lax: defaultLaxValue,
    });

    const lax = 700;
    ({ data } = await Model.create({ lax }, {}));

    expect(data).toEqual({ dependent: defaultDependentValue + 1, lax });

    const previous = data!;
    const { data: updates } = await Model.update(previous, { lax: 200 }, {});

    expect(updates).toEqual({ dependent: previous.dependent + 1, lax: 200 });
  });

  it('should properly run dependent resolver even with multiple parents', async () => {
    const defaultDependentValue = 1234;
    const defaultLaxValue = 20;

    const Model = new Schema<
      { lax: number; lax_1: number },
      { dependent: number; lax: number; lax_1: number }
    >((b) =>
      b
        .field(
          b
            .dependent('dependent', ['lax', 'lax_1'])
            .default(defaultDependentValue)
            .resolve((ctx) => ctx.values.dependent! + 1),
        )
        .field(b.lax('lax', defaultLaxValue))
        .field(b.lax('lax_1', defaultLaxValue)),
    ).getModel();

    let { data } = await Model.create(
      { lax: defaultLaxValue, lax_1: defaultLaxValue + 1 },
      {},
    );

    expect(data).toEqual({
      dependent: defaultDependentValue + 1,
      lax: defaultLaxValue,
      lax_1: defaultLaxValue + 1,
    });

    const lax = 700;
    ({ data } = await Model.create({ lax, lax_1: lax }, {}));

    expect(data).toEqual({
      dependent: defaultDependentValue + 1,
      lax,
      lax_1: lax,
    });

    const previous = data!;
    const { data: updates } = await Model.update(previous, { lax: 200 }, {});

    expect(updates).toEqual({ dependent: previous.dependent + 1, lax: 200 });
  });

  it('should properly run dependent resolver even with dependency on other dependents', async () => {
    const defaultDependentValue = 1234;
    const defaultLaxValue = 20;

    const Model = new Schema<
      { lax: number; lax_1: number },
      {
        dependent: number;
        dependent_1: number;
        lax: number;
        lax_1: number;
      }
    >((b) =>
      b
        .field(
          b
            .dependent('dependent', ['lax', 'lax_1'])
            .default(defaultDependentValue)
            .resolve((ctx) => ctx.values.dependent! + 1),
        )
        .field(
          b
            .dependent('dependent_1', 'dependent')
            .default(defaultDependentValue)
            .resolve((ctx) => ctx.values.dependent! + 10),
        )
        .field(b.lax('lax', defaultLaxValue))
        .field(b.lax('lax_1', defaultLaxValue)),
    ).getModel();

    let { data } = await Model.create(
      { lax: defaultLaxValue, lax_1: defaultLaxValue + 1 },
      {},
    );

    let dependent = defaultDependentValue + 1;
    let dependent_1 = dependent + 10;

    expect(data).toEqual({
      dependent,
      dependent_1,
      lax: defaultLaxValue,
      lax_1: defaultLaxValue + 1,
    });

    const lax = 700;
    ({ data } = await Model.create({ lax, lax_1: lax }, {}));

    dependent = defaultDependentValue + 1;
    dependent_1 = dependent + 10;

    expect(data).toEqual({ dependent, dependent_1, lax, lax_1: lax });

    const previous = data!;
    const { data: updates } = await Model.update(previous, { lax: 200 }, {});

    dependent = previous.dependent + 1;
    dependent_1 = dependent + 10;

    expect(updates).toEqual({ dependent, dependent_1, lax: 200 });
  });

  it('should not run dependent resolver if readonly is provided and value is different from default value', async () => {
    const defaultDependentValue = 1234;
    const defaultLaxValue = 20;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number }
    >((f) =>
      f
        .field(
          f
            .dependent('dependent', 'lax')
            .default(defaultDependentValue)
            .resolve((ctx) => ctx.values.dependent! + 1)
            .readonly(),
        )
        .field(f.lax('lax', defaultLaxValue)),
    ).getModel();

    let { data } = await Model.create({ lax: defaultLaxValue }, {});

    expect(data).toEqual({
      dependent: defaultDependentValue + 1,
      lax: defaultLaxValue,
    });

    let previous = data!;
    let lax = previous.lax + 1;
    let { data: updates } = await Model.update(previous, { lax }, {});

    expect(updates).toEqual({ lax });

    ({ data } = await Model.create({}, {}));

    expect(data).toEqual({
      dependent: defaultDependentValue,
      lax: defaultLaxValue,
    });

    previous = data!;
    lax = previous.lax + 1;
    ({ data: updates } = await Model.update(previous, { lax }, {}));

    expect(updates).toEqual({ dependent: previous.dependent + 1, lax });

    previous = Object.assign({}, previous, updates);
    lax = previous.lax + 1;

    ({ data: updates } = await Model.update(previous, { lax }, {}));

    expect(updates).toEqual({ lax });
  });

  it('should trigger onDelete handlers with static default values', async () => {
    const dependent = 1234;
    let triggeredWith: number | undefined;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number }
    >((b) =>
      b
        .field(
          b
            .dependent('dependent', 'lax')
            .default(dependent)
            .resolve((ctx) => ctx.values.dependent! + 1)
            .onDelete((data) => {
              triggeredWith = data.dependent;
            }),
        )
        .field(b.lax('lax', 20)),
    ).getModel();

    await Model.delete({ dependent, lax: 400 }, {});

    expect(triggeredWith).toBe(dependent);
  });

  it('should trigger onDelete handlers with computed default values', async () => {
    const dependent = 1234;
    let triggeredWith: number | undefined;
    let secondHandlerCalled = false;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number }
    >((b) =>
      b
        .field(
          b
            .dependent('dependent', 'lax')
            .default(dependent)
            .resolve((ctx) => ctx.values.dependent! + 1)
            .onDelete([
              (data) => {
                triggeredWith = data.dependent;
              },
              () => {
                secondHandlerCalled = true;
              },
            ]),
        )
        .field(b.lax('lax', 20)),
    ).getModel();

    await Model.delete({ dependent, lax: 400 }, {});

    expect(triggeredWith).toBe(dependent);
    expect(secondHandlerCalled).toBe(true);
  });

  it('should trigger onSuccess handlers if resolver is run at creation', async () => {
    const defaultDependentValue = 1234;
    const defaultLaxValue = 20;
    let triggeredWith: number | undefined;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number }
    >((b) =>
      b
        .field(
          b
            .dependent('dependent', 'lax')
            .default(defaultDependentValue)
            .resolve((ctx) => ctx.values.dependent! + 1)
            .onSuccess((ctx) => {
              triggeredWith = ctx.values.dependent;
            }),
        )
        .field(b.lax('lax', defaultLaxValue)),
    ).getModel();

    const { data, handleSuccess } = await Model.create(
      { lax: defaultLaxValue },
      {},
    );

    const resolvedValue = defaultDependentValue + 1;

    expect(data).toEqual({ dependent: resolvedValue, lax: defaultLaxValue });

    await handleSuccess?.();

    expect(triggeredWith).toBe(resolvedValue);
  });

  it('should trigger onSuccess handlers even if resolver is not run at creation', async () => {
    const defaultDependentValue = 1234;
    const defaultLaxValue = 20;
    let triggeredWith: number | undefined;
    let secondHandlerCalled = false;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number }
    >((b) =>
      b
        .field(
          b
            .dependent('dependent', 'lax')
            .default(defaultDependentValue)
            .resolve((ctx) => ctx.values.dependent! + 1)
            .onSuccess([
              (ctx) => {
                triggeredWith = ctx.values.dependent;
              },
              async () => {
                secondHandlerCalled = true;
              },
            ]),
        )
        .field(b.lax('lax', defaultLaxValue)),
    ).getModel();

    const { data, handleSuccess } = await Model.create(
      { lax: defaultLaxValue },
      {},
    );

    const resolvedValue = defaultDependentValue + 1;

    expect(data).toEqual({ dependent: resolvedValue, lax: defaultLaxValue });

    await handleSuccess?.();

    expect(triggeredWith).toBe(resolvedValue);
    expect(secondHandlerCalled).toBe(true);
  });

  it('should trigger onSuccess handlers if resolver is run during updates', async () => {
    const defaultDependentValue = 1234;
    const defaultLaxValue = 20;
    let triggeredWith: number | undefined;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number }
    >((b) =>
      b
        .field(
          b
            .dependent('dependent', 'lax')
            .default(defaultDependentValue)
            .resolve((ctx) => ctx.values.dependent! + 1)
            .onSuccess((ctx) => {
              triggeredWith = ctx.values.dependent;
            }),
        )
        .field(b.lax('lax', defaultLaxValue)),
    ).getModel();

    const { data, handleSuccess } = await Model.update(
      { dependent: defaultDependentValue, lax: defaultLaxValue },
      { lax: defaultLaxValue + 1 },
      {},
    );

    const resolvedValue = defaultDependentValue + 1;

    expect(data).toEqual({
      dependent: resolvedValue,
      lax: defaultLaxValue + 1,
    });

    await handleSuccess?.();

    expect(triggeredWith).toBe(resolvedValue);
  });

  it('should not trigger onSuccess handlers if resolver is not run during updates', async () => {
    const defaultDependentValue = 1234;
    const defaultLaxValue = 20;
    let triggered = false;

    const Model = new Schema<
      { lax: number; lax_1: number },
      { dependent: number; lax: number; lax_1: number }
    >((b) =>
      b
        .field(
          b
            .dependent('dependent', 'lax')
            .default(defaultDependentValue)
            .resolve((ctx) => ctx.values.dependent! + 1)
            .onSuccess(() => {
              triggered = true;
            }),
        )
        .field(b.lax('lax', defaultLaxValue))
        .field(b.lax('lax_1', defaultLaxValue)),
    ).getModel();

    const updatedLax1 = defaultDependentValue + 1;

    const { data, handleSuccess } = await Model.update(
      {
        dependent: defaultDependentValue,
        lax: defaultLaxValue,
        lax_1: defaultLaxValue,
      },
      { lax_1: updatedLax1 },
      {},
    );

    expect(data).toEqual({ lax_1: updatedLax1 });

    await handleSuccess?.();

    expect(triggered).toBe(false);
  });

  it('should trigger grouped onSuccess at creation if resolved', async () => {
    const defaultDependentValue = 1234;
    const defaultLaxValue = 20;
    let triggered = false;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number }
    >(
      (b) =>
        b
          .field(
            b
              .dependent('dependent', 'lax')
              .default(defaultDependentValue)
              .resolve((ctx) => ctx.values.dependent! + 1),
          )
          .field(b.lax('lax', defaultLaxValue)),
      {
        onSuccess: {
          fields: ['dependent'] as never,
          handler: () => {
            triggered = true;
          },
        },
      },
    ).getModel();

    const { data, handleSuccess } = await Model.create(
      { lax: defaultLaxValue },
      {},
    );

    const resolvedValue = defaultDependentValue + 1;

    expect(data).toEqual({ dependent: resolvedValue, lax: defaultLaxValue });

    await handleSuccess?.();

    expect(triggered).toBe(true);
  });

  it('should trigger grouped onSuccess at creation even if not resolved', async () => {
    const defaultDependentValue = 1234;
    const defaultLaxValue = 20;
    let triggered = false;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number }
    >(
      (b) =>
        b
          .field(
            b
              .dependent('dependent', 'lax')
              .default(defaultDependentValue)
              .resolve((ctx) => ctx.values.dependent! + 1),
          )
          .field(b.lax('lax', defaultLaxValue)),
      {
        onSuccess: {
          fields: ['dependent'] as never,
          handler: () => {
            triggered = true;
          },
        },
      },
    ).getModel();

    const { data, handleSuccess } = await Model.create({}, {});

    expect(data).toEqual({
      dependent: defaultDependentValue,
      lax: defaultLaxValue,
    });

    await handleSuccess?.();

    expect(triggered).toBe(true);
  });

  it('should trigger grouped onSuccess during updates if resolved', async () => {
    const defaultDependentValue = 1234;
    const defaultLaxValue = 20;
    let triggered = false;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number }
    >(
      (b) =>
        b
          .field(
            b
              .dependent('dependent', 'lax')
              .default(defaultDependentValue)
              .resolve((ctx) => ctx.values.dependent! + 1),
          )
          .field(b.lax('lax', defaultLaxValue)),
      {
        onSuccess: {
          fields: ['dependent'] as never,
          handler: () => {
            triggered = true;
          },
        },
      },
    ).getModel();

    const lax = defaultLaxValue + 1;

    const { data, handleSuccess } = await Model.update(
      { dependent: defaultDependentValue, lax: defaultLaxValue },
      { lax },
      {},
    );

    const resolvedValue = defaultDependentValue + 1;

    expect(data).toEqual({ dependent: resolvedValue, lax });

    await handleSuccess?.();

    expect(triggered).toBe(true);
  });

  it('should not trigger grouped onSuccess during updates if not resolved because it is readonly', async () => {
    const defaultDependentValue = 1234;
    const defaultLaxValue = 20;
    let triggered = false;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number }
    >(
      (b) =>
        b
          .field(
            b
              .dependent('dependent', 'lax')
              .default(defaultDependentValue)
              .resolve((ctx) => ctx.values.dependent! + 1)
              .readonly(),
          )
          .field(b.lax('lax', defaultLaxValue)),
      {
        onSuccess: {
          fields: ['dependent'] as never,
          handler: () => {
            triggered = true;
          },
        },
      },
    ).getModel();

    const lax = defaultLaxValue + 1;

    const { data, handleSuccess } = await Model.update(
      { dependent: defaultDependentValue + 1, lax: defaultLaxValue },
      { lax },
      {},
    );

    expect(data).toEqual({ lax });

    await handleSuccess?.();

    expect(triggered).toBe(false);
  });

  it('should not trigger grouped onSuccess during updates if not resolved', async () => {
    const defaultDependentValue = 1234;
    const defaultLaxValue = 20;
    let triggered = false;

    const Model = new Schema<
      { lax: number; lax_1: number },
      { dependent: number; lax: number; lax_1: number }
    >(
      (b) =>
        b
          .field(
            b
              .dependent('dependent', 'lax')
              .default(defaultDependentValue)
              .resolve((ctx) => ctx.values.dependent! + 1),
          )
          .field(b.lax('lax', defaultLaxValue))
          .field(b.lax('lax_1', defaultLaxValue)),
      {
        onSuccess: {
          fields: ['dependent'] as never,
          handler: () => {
            triggered = true;
          },
        },
      },
    ).getModel();

    const lax_1 = defaultLaxValue + 1;

    const { data, handleSuccess } = await Model.update(
      {
        dependent: defaultDependentValue + 1,
        lax: defaultLaxValue,
        lax_1: defaultLaxValue,
      },
      { lax_1 },
      {},
    );

    expect(data).toEqual({ lax_1 });

    await handleSuccess?.();

    expect(triggered).toBe(false);
  });
});
