import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

describe('extras.ctxOptions.constant', () => {
  it('should properly update ctx options in constant value resolver and provide those updates in onSuccess handlers', async () => {
    const CONSTANT_VALUE = 1;
    const MESSAGE = 'ctx_options updated in constant value resolver';
    let triggeredWith: string | undefined;

    const Model = new Schema<
      { lax: number },
      { id: number; lax: number },
      { messages: string[] }
    >((b) =>
      b
        .field(
          b
            .constant('id', (ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });
              return CONSTANT_VALUE;
            })
            .onSuccess((ctx) => {
              triggeredWith = ctx.options.messages[0];
            }),
        )
        .field(b.lax('lax', 2).validate(() => ({ valid: true }))),
    ).getModel();

    const value = 2;
    const { data, handleSuccess, options } = await Model.create(
      { lax: value },
      { messages: [] },
    );

    expect(data).toEqual({ id: CONSTANT_VALUE, lax: value });
    expect(options.messages[0]).toBe(MESSAGE);

    await handleSuccess?.();

    expect(triggeredWith).toBe(MESSAGE);
  });
});
