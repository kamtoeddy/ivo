import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

describe('extras.ctxOptions.lax', () => {
  it('should properly update ctx options in default resolver and provide those updates in onSuccess handlers', async () => {
    const DEFAULT_VALUE = 1;
    const MESSAGE = 'ctx_options updated in default value resolver';
    let triggeredWith: string | undefined;

    const Model = new Schema<
      { lax: number },
      { lax: number },
      { messages: string[] }
    >((b) =>
      b.field(
        b
          .lax('lax', (ctx) => {
            ctx.updateOptions({
              messages: [...ctx.options.messages, MESSAGE],
            });
            return DEFAULT_VALUE;
          })
          .validate(() => ({ valid: true }))
          .onSuccess((ctx) => {
            triggeredWith = ctx.options.messages[0];
          }),
      ),
    ).getModel();

    const { data, handleSuccess, options } = await Model.create(
      {},
      { messages: [] },
    );

    expect(data).toEqual({ lax: DEFAULT_VALUE });
    expect(options.messages[0]).toBe(MESSAGE);

    await handleSuccess?.();

    expect(triggeredWith).toBe(MESSAGE);
  });

  describe('ignore', () => {
    it('should properly update ctx options in ignore resolver and provide those updates in onSuccess handlers at creation', async () => {
      const DEFAULT_VALUE = 1;
      const MESSAGE = 'ctx_options updated in ignore resolver';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { lax: number },
        { lax: number },
        { messages: string[] }
      >((b) =>
        b.field(
          b
            .lax('lax', DEFAULT_VALUE)
            .validate(() => ({ valid: true }))
            .ignore((ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });
              return false;
            })
            .onSuccess((ctx) => {
              triggeredWith = ctx.options.messages[0];
            }),
        ),
      ).getModel();

      const lax = DEFAULT_VALUE + 1;
      const { data, handleSuccess, options } = await Model.create(
        { lax },
        { messages: [] },
      );

      expect(data).toEqual({ lax });
      expect(options.messages[0]).toBe(MESSAGE);

      await handleSuccess?.();

      expect(triggeredWith).toBe(MESSAGE);
    });

    it('should properly update ctx options in ignore resolver and provide those updates in onSuccess handlers during updates', async () => {
      const DEFAULT_VALUE = 1;
      const MESSAGE = 'ctx_options updated in ignore resolver';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { lax: number },
        { lax: number },
        { messages: string[] }
      >((b) =>
        b.field(
          b
            .lax('lax', DEFAULT_VALUE)
            .validate(() => ({ valid: true }))
            .ignore((ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });
              return false;
            })
            .onSuccess((ctx) => {
              triggeredWith = ctx.options.messages[0];
            }),
        ),
      ).getModel();

      const data = { lax: DEFAULT_VALUE };
      const lax = data.lax + 1;

      const {
        data: updates,
        handleSuccess,
        options,
      } = await Model.update(data, { lax }, { messages: [] });

      expect(updates).toEqual({ lax });
      expect(options.messages[0]).toBe(MESSAGE);

      await handleSuccess?.();

      expect(triggeredWith).toBe(MESSAGE);
    });
  });

  describe('required', () => {
    it('should properly update ctx options in required resolver and provide those updates at creation', async () => {
      const DEFAULT_VALUE = 1;
      const MESSAGE = 'ctx_options updated in required resolver';
      const REQUIRED_ERROR = 'lax is missing!';

      const Model = new Schema<
        { lax: number },
        { lax: number },
        { messages: string[] }
      >((b) =>
        b.field(
          b
            .lax('lax', DEFAULT_VALUE)
            .validate(() => ({ valid: true }))
            .required((ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });
              return [true, REQUIRED_ERROR];
            }),
        ),
      ).getModel();

      const { error, options } = await Model.create({}, { messages: [] });

      expect(error?.lax?.reason).toBe(REQUIRED_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);
    });

    it('should properly update ctx options in required resolver and provide those updates during updates', async () => {
      const DEFAULT_VALUE = 1;
      const MESSAGE = 'ctx_options updated in required resolver';
      const REQUIRED_ERROR = 'lax is missing!';

      const Model = new Schema<
        { lax: number; lax_1: number },
        { lax: number; lax_1: number },
        { messages: string[] }
      >((b) =>
        b
          .field(
            b
              .lax('lax', DEFAULT_VALUE)
              .validate(() => ({ valid: true }))
              .required((ctx) => {
                ctx.updateOptions({
                  messages: [...ctx.options.messages, MESSAGE],
                });
                return [true, REQUIRED_ERROR];
              }),
          )
          .field(
            b.lax('lax_1', DEFAULT_VALUE).validate(() => ({ valid: true })),
          ),
      ).getModel();

      const { error, options } = await Model.update(
        { lax: DEFAULT_VALUE, lax_1: DEFAULT_VALUE },
        { lax_1: DEFAULT_VALUE + 1 },
        { messages: [] },
      );

      expect(error?.payload?.lax?.reason).toBe(REQUIRED_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);
    });
  });

  describe('validate', () => {
    it('should properly update ctx options in validators and provide those updates in onFailure handlers at creation', async () => {
      const DEFAULT_VALUE = 'default_value';
      const MESSAGE = 'ctx_options updated in validator';
      const MIN_LENGTH_ERROR = 'expected lax to be at least 2 characters long';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { lax: string },
        { lax: string },
        { messages: string[] }
      >((b) =>
        b.field(
          b
            .lax('lax', DEFAULT_VALUE)
            .validate((v: unknown, ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });

              const validated = String(v).trim();
              if (validated.length < 2)
                return { valid: false, reason: MIN_LENGTH_ERROR };
              return { valid: true, validated };
            })
            .onFailure((ctx) => {
              triggeredWith = ctx.options.messages[0];
            }),
        ),
      ).getModel();

      const { error, handleFailure, options } = await Model.create(
        { lax: ' ' },
        { messages: [] },
      );

      expect(error?.lax?.reason).toBe(MIN_LENGTH_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);

      await handleFailure?.();

      expect(triggeredWith).toBe(MESSAGE);
    });

    it('should properly update ctx options in validators and provide those updates in onFailure handlers during updates', async () => {
      const DEFAULT_VALUE = 'default_value';
      const MESSAGE = 'ctx_options updated in validator';
      const MIN_LENGTH_ERROR = 'expected lax to be at least 2 characters long';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { lax: string },
        { lax: string },
        { messages: string[] }
      >((b) =>
        b.field(
          b
            .lax('lax', DEFAULT_VALUE)
            .validate((v: unknown, ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });

              const validated = String(v).trim();
              if (validated.length < 2)
                return { valid: false, reason: MIN_LENGTH_ERROR };
              return { valid: true, validated };
            })
            .onFailure((ctx) => {
              triggeredWith = ctx.options.messages[0];
            }),
        ),
      ).getModel();

      const { error, handleFailure, options } = await Model.update(
        { lax: DEFAULT_VALUE },
        { lax: ' ' },
        { messages: [] },
      );

      expect(error?.payload?.lax?.reason).toBe(MIN_LENGTH_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);

      await handleFailure?.();

      expect(triggeredWith).toBe(MESSAGE);
    });
  });

  describe('reValidate', () => {
    it('should properly update ctx options in re-validators and provide those updates in onFailure handlers at creation', async () => {
      const DEFAULT_VALUE = 'default_value';
      const MESSAGE = 'ctx_options updated in re_validator';
      const MIN_LENGTH_ERROR = 'expected lax to be at least 2 characters long';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { lax: string },
        { lax: string },
        { messages: string[] }
      >((b) =>
        b.field(
          b
            .lax('lax', DEFAULT_VALUE)
            .validate(() => ({ valid: true }))
            .reValidate((v, ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });

              const validated = v.trim();
              if (validated.length < 2)
                return { valid: false, reason: MIN_LENGTH_ERROR };
              return { valid: true, validated };
            })
            .onFailure((ctx) => {
              triggeredWith = ctx.options.messages[0];
            }),
        ),
      ).getModel();

      const { error, handleFailure, options } = await Model.create(
        { lax: ' ' },
        { messages: [] },
      );

      expect(error?.lax?.reason).toBe(MIN_LENGTH_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);

      await handleFailure?.();

      expect(triggeredWith).toBe(MESSAGE);
    });

    it('should properly update ctx options in re-validators and provide those updates in onFailure handlers during updates', async () => {
      const DEFAULT_VALUE = 'default_value';
      const MESSAGE = 'ctx_options updated in re_validator';
      const MIN_LENGTH_ERROR = 'expected lax to be at least 2 characters long';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { lax: string },
        { lax: string },
        { messages: string[] }
      >((b) =>
        b.field(
          b
            .lax('lax', DEFAULT_VALUE)
            .validate(() => ({ valid: true }))
            .reValidate((v, ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });

              const validated = v.trim();
              if (validated.length < 2)
                return { valid: false, reason: MIN_LENGTH_ERROR };
              return { valid: true, validated };
            })
            .onFailure((ctx) => {
              triggeredWith = ctx.options.messages[0];
            }),
        ),
      ).getModel();

      const { error, handleFailure, options } = await Model.update(
        { lax: DEFAULT_VALUE },
        { lax: ' ' },
        { messages: [] },
      );

      expect(error?.payload?.lax?.reason).toBe(MIN_LENGTH_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);

      await handleFailure?.();

      expect(triggeredWith).toBe(MESSAGE);
    });
  });

  describe('o.postValidate & o.onSuccess', () => {
    it('should properly update ctx options in post-validators and provide those updates in global success function handlers at creation', async () => {
      const DEFAULT_VALUE = 1;
      const MESSAGE = 'ctx_options updated in post_validator';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { lax: number; lax_1: number },
        { lax: number; lax_1: number },
        { messages: string[] }
      >(
        (b) =>
          b
            .field(
              b.lax('lax', DEFAULT_VALUE).validate(() => ({ valid: true })),
            )
            .field(
              b.lax('lax_1', DEFAULT_VALUE).validate(() => ({ valid: true })),
            ),
        {
          postValidate: {
            fields: ['lax', 'lax_1'],
            validator: async (ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });
              return undefined;
            },
          },
          onSuccess: (ctx) => {
            triggeredWith = ctx.options.messages[0];
          },
        },
      ).getModel();

      const lax = DEFAULT_VALUE + 1;
      const { data, handleSuccess, options } = await Model.create(
        { lax },
        { messages: [] },
      );

      expect(data).toEqual({ lax, lax_1: DEFAULT_VALUE });
      expect(options.messages[0]).toBe(MESSAGE);

      await handleSuccess?.();

      expect(triggeredWith).toBe(MESSAGE);
    });

    it('should properly update ctx options in post-validators and provide those updates in grouped onSuccess handlers during updates', async () => {
      const DEFAULT_VALUE = 1;
      const MESSAGE = 'ctx_options updated in post_validator';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { lax: number; lax_1: number },
        { lax: number; lax_1: number },
        { messages: string[] }
      >(
        (b) =>
          b
            .field(
              b.lax('lax', DEFAULT_VALUE).validate(() => ({ valid: true })),
            )
            .field(
              b.lax('lax_1', DEFAULT_VALUE).validate(() => ({ valid: true })),
            ),
        {
          postValidate: {
            fields: ['lax', 'lax_1'],
            validator: async (ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });
              return undefined;
            },
          },
          onSuccess: {
            fields: ['lax', 'lax_1'],
            handler: (ctx) => {
              triggeredWith = ctx.options.messages[0];
            },
          },
        },
      ).getModel();

      const data = { lax: DEFAULT_VALUE, lax_1: DEFAULT_VALUE };
      const lax = data.lax + 1;

      const {
        data: updates,
        handleSuccess,
        options,
      } = await Model.update(data, { lax }, { messages: [] });

      expect(updates).toEqual({ lax });
      expect(options.messages[0]).toBe(MESSAGE);

      await handleSuccess?.();

      expect(triggeredWith).toBe(MESSAGE);
    });
  });
});
