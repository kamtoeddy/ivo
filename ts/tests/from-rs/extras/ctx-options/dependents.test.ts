import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

describe('extras.ctxOptions.dependent', () => {
  it('should properly update ctx options in default resolver and provide those updates in onSuccess handlers', async () => {
    const DEFAULT_DEPENDENT_VALUE = 1;
    const DEFAULT_LAX_VALUE = 1;
    const MESSAGE = 'ctx_options updated in default value resolver';
    let triggeredWith: string | undefined;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number },
      { messages: string[] }
    >((b) =>
      b
        .field(
          b
            .dependent('dependent', 'lax')
            .default((ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });
              return DEFAULT_DEPENDENT_VALUE;
            })
            .resolve((ctx) => ctx.values.dependent! + 1)
            .onSuccess((ctx) => {
              triggeredWith = ctx.options.messages[0];
            }),
        )
        .field(
          b.lax('lax', DEFAULT_LAX_VALUE).validate(() => ({ valid: true })),
        ),
    ).getModel();

    const { data, handleSuccess, options } = await Model.create(
      {},
      { messages: [] },
    );

    expect(data).toEqual({
      dependent: DEFAULT_DEPENDENT_VALUE,
      lax: DEFAULT_LAX_VALUE,
    });
    expect(options.messages[0]).toBe(MESSAGE);

    await handleSuccess?.();

    expect(triggeredWith).toBe(MESSAGE);
  });

  it('should properly update ctx options in value resolver and provide those updates in onSuccess handlers at creation', async () => {
    const DEFAULT_DEPENDENT_VALUE = 1;
    const DEFAULT_LAX_VALUE = 1;
    const MESSAGE = 'ctx_options updated in value resolver';
    let triggeredWith: string | undefined;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number },
      { messages: string[] }
    >((b) =>
      b
        .field(
          b
            .dependent('dependent', 'lax')
            .default(DEFAULT_DEPENDENT_VALUE)
            .resolve((ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });
              return ctx.values.dependent! + 1;
            })
            .onSuccess((ctx) => {
              triggeredWith = ctx.options.messages[0];
            }),
        )
        .field(
          b.lax('lax', DEFAULT_LAX_VALUE).validate(() => ({ valid: true })),
        ),
    ).getModel();

    const value = DEFAULT_LAX_VALUE + 1;
    const { data, handleSuccess, options } = await Model.create(
      { lax: value },
      { messages: [] },
    );

    expect(data).toEqual({
      dependent: DEFAULT_DEPENDENT_VALUE + 1,
      lax: value,
    });
    expect(options.messages[0]).toBe(MESSAGE);

    await handleSuccess?.();

    expect(triggeredWith).toBe(MESSAGE);
  });

  it('should properly update ctx options in value resolver and provide those updates in onSuccess handlers during updates', async () => {
    const DEFAULT_DEPENDENT_VALUE = 1;
    const DEFAULT_LAX_VALUE = 1;
    const MESSAGE = 'ctx_options updated in value resolver';
    let triggeredWith: string | undefined;

    const Model = new Schema<
      { lax: number },
      { dependent: number; lax: number },
      { messages: string[] }
    >((b) =>
      b
        .field(
          b
            .dependent('dependent', 'lax')
            .default(DEFAULT_DEPENDENT_VALUE)
            .resolve((ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });
              return ctx.values.dependent! + 1;
            })
            .onSuccess((ctx) => {
              triggeredWith = ctx.options.messages[0];
            }),
        )
        .field(
          b.lax('lax', DEFAULT_LAX_VALUE).validate(() => ({ valid: true })),
        ),
    ).getModel();

    const data = {
      dependent: DEFAULT_DEPENDENT_VALUE,
      lax: DEFAULT_LAX_VALUE,
    };

    const lax = data.lax + 1;
    const {
      data: updates,
      handleSuccess,
      options,
    } = await Model.update(data, { lax }, { messages: [] });

    expect(updates).toEqual({ dependent: data.dependent + 1, lax });
    expect(options.messages[0]).toBe(MESSAGE);

    await handleSuccess?.();

    expect(triggeredWith).toBe(MESSAGE);
  });
});
