import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

describe('extras.ctxOptions.virtual', () => {
  describe('required', () => {
    it('should properly update ctx options in required resolver and provide those updates at creation', async () => {
      const DEFAULT_VALUE = 1;
      const MESSAGE = 'ctx_options updated in required resolver';
      const REQUIRED_ERROR = 'virtual_field is missing!';

      const Model = new Schema<
        { virtualField: number; virtualField1: number },
        { dependent: number },
        { messages: string[] }
      >((b) =>
        b
          .field(
            b
              .dependent('dependent', ['virtualField', 'virtualField1'])
              .default(DEFAULT_VALUE)
              .resolve((ctx) => ctx.input.virtualField! + 1),
          )
          .field(
            b
              .virtual('virtualField')
              .validate(() => ({ valid: true }))
              .required((ctx) => {
                ctx.updateOptions({
                  messages: [...ctx.options.messages, MESSAGE],
                });
                return [true, REQUIRED_ERROR];
              }),
          )
          .field(b.virtual('virtualField1').validate(() => ({ valid: true }))),
      ).getModel();

      const { error, options } = await Model.create(
        { virtualField1: 1 },
        { messages: [] },
      );

      expect(error?.virtualField?.reason).toBe(REQUIRED_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);
    });

    it('should properly update ctx options in required resolver and provide those updates during updates', async () => {
      const DEFAULT_VALUE = 1;
      const MESSAGE = 'ctx_options updated in required resolver';
      const REQUIRED_ERROR = 'virtual_field is missing!';

      const Model = new Schema<
        { virtualField: number; virtualField1: number },
        { dependent: number },
        { messages: string[] }
      >((b) =>
        b
          .field(
            b
              .dependent('dependent', ['virtualField', 'virtualField1'])
              .default(DEFAULT_VALUE)
              .resolve((ctx) => ctx.input.virtualField! + 1),
          )
          .field(
            b
              .virtual('virtualField')
              .validate(() => ({ valid: true }))
              .required((ctx) => {
                ctx.updateOptions({
                  messages: [...ctx.options.messages, MESSAGE],
                });
                return [true, REQUIRED_ERROR];
              }),
          )
          .field(b.virtual('virtualField1').validate(() => ({ valid: true }))),
      ).getModel();

      const { error, options } = await Model.update(
        { dependent: DEFAULT_VALUE },
        { virtualField1: 1 },
        { messages: [] },
      );

      expect(error?.payload?.virtualField?.reason).toBe(REQUIRED_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);
    });
  });

  describe('ignore', () => {
    it('should properly update ctx options in ignore resolver and provide those updates in onSuccess handlers during updates', async () => {
      const DEFAULT_VALUE = 1;
      const MESSAGE = 'ctx_options updated in ignore_update resolver';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { virtualField: number },
        { dependent: number },
        { messages: string[] }
      >((b) =>
        b
          .field(
            b
              .dependent('dependent', 'virtualField')
              .default(DEFAULT_VALUE)
              .resolve((ctx) => ctx.input.virtualField! + 1),
          )
          .field(
            b
              .virtual('virtualField')
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

      const data = { dependent: DEFAULT_VALUE };
      const value = data.dependent + 1;

      const {
        data: updates,
        handleSuccess,
        options,
      } = await Model.update(data, { virtualField: value }, { messages: [] });

      expect(updates).toEqual({ dependent: value + 1 });
      expect(options.messages[0]).toBe(MESSAGE);

      await handleSuccess?.();

      expect(triggeredWith).toBe(MESSAGE);
    });
  });

  describe('validate', () => {
    it('should properly update ctx options in validators and provide those updates in onFailure handlers at creation', async () => {
      const DEFAULT_VALUE = 'default_value';
      const MESSAGE = 'ctx_options updated in validator';
      const MIN_LENGTH_ERROR =
        'expected virtual_field to be at least 2 characters long';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { virtualField: string },
        { dependent: string },
        { messages: string[] }
      >((b) =>
        b
          .field(
            b
              .dependent('dependent', 'virtualField')
              .default(DEFAULT_VALUE)
              .resolve((ctx) => ctx.input.virtualField!),
          )
          .field(
            b
              .virtual('virtualField')
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
        { virtualField: ' ' },
        { messages: [] },
      );

      expect(error?.virtualField?.reason).toBe(MIN_LENGTH_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);

      await handleFailure?.();

      expect(triggeredWith).toBe(MESSAGE);
    });

    it('should properly update ctx options in validators and provide those updates in onFailure handlers during updates', async () => {
      const DEFAULT_VALUE = 'default_value';
      const MESSAGE = 'ctx_options updated in validator';
      const MIN_LENGTH_ERROR =
        'expected virtual_field to be at least 2 characters long';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { virtualField: string },
        { dependent: string },
        { messages: string[] }
      >((b) =>
        b
          .field(
            b
              .dependent('dependent', 'virtualField')
              .default(DEFAULT_VALUE)
              .resolve((ctx) => ctx.input.virtualField!),
          )
          .field(
            b
              .virtual('virtualField')
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
        { dependent: DEFAULT_VALUE },
        { virtualField: ' ' },
        { messages: [] },
      );

      expect(error?.payload?.virtualField?.reason).toBe(MIN_LENGTH_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);

      await handleFailure?.();

      expect(triggeredWith).toBe(MESSAGE);
    });
  });

  describe('reValidate', () => {
    it('should properly update ctx options in re-validators and provide those updates in onFailure handlers at creation', async () => {
      const DEFAULT_VALUE = 'default_value';
      const MESSAGE = 'ctx_options updated in re_validator';
      const MIN_LENGTH_ERROR =
        'expected virtual_field to be at least 2 characters long';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { virtualField: string },
        { dependent: string },
        { messages: string[] }
      >((b) =>
        b
          .field(
            b
              .dependent('dependent', 'virtualField')
              .default(DEFAULT_VALUE)
              .resolve((ctx) => ctx.input.virtualField!),
          )
          .field(
            b
              .virtual('virtualField')
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
        { virtualField: ' ' },
        { messages: [] },
      );

      expect(error?.virtualField?.reason).toBe(MIN_LENGTH_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);

      await handleFailure?.();

      expect(triggeredWith).toBe(MESSAGE);
    });

    it('should properly update ctx options in re-validators and provide those updates in onFailure handlers during updates', async () => {
      const DEFAULT_VALUE = 'default_value';
      const MESSAGE = 'ctx_options updated in re_validator';
      const MIN_LENGTH_ERROR =
        'expected virtual_field to be at least 2 characters long';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { virtualField: string },
        { dependent: string },
        { messages: string[] }
      >((b) =>
        b
          .field(
            b
              .dependent('dependent', 'virtualField')
              .default(DEFAULT_VALUE)
              .resolve((ctx) => ctx.input.virtualField!),
          )
          .field(
            b
              .virtual('virtualField')
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
        { dependent: DEFAULT_VALUE },
        { virtualField: ' ' },
        { messages: [] },
      );

      expect(error?.payload?.virtualField?.reason).toBe(MIN_LENGTH_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);

      await handleFailure?.();

      expect(triggeredWith).toBe(MESSAGE);
    });
  });

  describe('sanitize', () => {
    function sanitize(value: string) {
      return `sanitized-${value}`;
    }

    it('should properly update ctx options in sanitizers and provide those updates in onSuccess handlers at creation', async () => {
      const defaultDependentValue = 'default_dependent_value';
      const MESSAGE = 'ctx_options updated in sanitizer';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { virtualField: string },
        { dependent: string },
        { messages: string[] }
      >((b) =>
        b
          .field(
            b
              .dependent('dependent', 'virtualField')
              .default(defaultDependentValue)
              .resolve((ctx) => ctx.input.virtualField!),
          )
          .field(
            b
              .virtual('virtualField')
              .validate(() => ({ valid: true }))
              .sanitize((ctx) => {
                ctx.updateOptions({
                  messages: [...ctx.options.messages, MESSAGE],
                });
                return sanitize(ctx.input.virtualField!);
              })
              .onSuccess((ctx) => {
                triggeredWith = ctx.options.messages[0];
              }),
          ),
      ).getModel();

      const virtualValue = 'virtual_value';
      const { data, handleSuccess, options } = await Model.create(
        { virtualField: virtualValue },
        { messages: [] },
      );

      expect(data).toEqual({ dependent: sanitize(virtualValue) });
      expect(data).not.toEqual({ dependent: virtualValue });
      expect(options.messages[0]).toBe(MESSAGE);

      await handleSuccess?.();

      expect(triggeredWith).toBe(MESSAGE);
    });

    it('should properly update ctx options in sanitizers and provide those updates in onSuccess handlers during updates', async () => {
      const defaultDependentValue = 'default_dependent_value';
      const MESSAGE = 'ctx_options updated in sanitizer';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { virtualField: string },
        { dependent: string },
        { messages: string[] }
      >((b) =>
        b
          .field(
            b
              .dependent('dependent', 'virtualField')
              .default(defaultDependentValue)
              .resolve((ctx) => ctx.input.virtualField!),
          )
          .field(
            b
              .virtual('virtualField')
              .validate(() => ({ valid: true }))
              .sanitize((ctx) => {
                ctx.updateOptions({
                  messages: [...ctx.options.messages, MESSAGE],
                });
                return sanitize(ctx.input.virtualField!);
              })
              .onSuccess((ctx) => {
                triggeredWith = ctx.options.messages[0];
              }),
          ),
      ).getModel();

      const previous = { dependent: defaultDependentValue };
      const updatedVirtualValue = 'updated_virtual_value';

      const {
        data: updates,
        handleSuccess,
        options,
      } = await Model.update(
        previous,
        { virtualField: updatedVirtualValue },
        { messages: [] },
      );

      expect(updates).toEqual({ dependent: sanitize(updatedVirtualValue) });
      expect(updates).not.toEqual({ dependent: updatedVirtualValue });
      expect(options.messages[0]).toBe(MESSAGE);

      await handleSuccess?.();

      expect(triggeredWith).toBe(MESSAGE);
    });
  });

  describe('o.postValidate & o.onSuccess', () => {
    it('should properly update ctx options in post-validators and provide those updates in grouped onSuccess handlers with no fields at creation', async () => {
      const MESSAGE = 'ctx_options updated in post_validator';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { virtualField: string; virtualField1: string },
        { dependent: string },
        { messages: string[] }
      >(
        (b) =>
          b
            .field(
              b
                .dependent('dependent', ['virtualField', 'virtualField1'])
                .default('default_dependent_value')
                .resolve((ctx) => ctx.input.virtualField!),
            )
            .field(b.virtual('virtualField').validate(() => ({ valid: true })))
            .field(
              b.virtual('virtualField1').validate(() => ({ valid: true })),
            ),
        {
          postValidate: {
            fields: ['virtualField', 'virtualField1'],
            validator: async (ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });
              return undefined;
            },
          },
          onSuccess: {
            fields: [] as never,
            resolver: (ctx) => {
              triggeredWith = ctx.options.messages[0];
            },
          },
        },
      ).getModel();

      const virtualValue = 'virtual_value';
      const { data, handleSuccess, options } = await Model.create(
        { virtualField: virtualValue, virtualField1: 'other value' },
        { messages: [] },
      );

      expect(data).toEqual({ dependent: virtualValue });
      expect(options.messages[0]).toBe(MESSAGE);

      await handleSuccess?.();

      expect(triggeredWith).toBe(MESSAGE);
    });

    it('should properly update ctx options in post-validators and provide those updates in grouped onSuccess handlers during updates', async () => {
      const defaultDependentValue = 'default_dependent_value';
      const MESSAGE = 'ctx_options updated in post_validator';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { virtualField: string; virtualField1: string },
        { dependent: string },
        { messages: string[] }
      >(
        (b) =>
          b
            .field(
              b
                .dependent('dependent', ['virtualField', 'virtualField1'])
                .default(defaultDependentValue)
                .resolve((ctx) => ctx.input.virtualField!),
            )
            .field(b.virtual('virtualField').validate(() => ({ valid: true })))
            .field(
              b.virtual('virtualField1').validate(() => ({ valid: true })),
            ),
        {
          postValidate: {
            fields: ['virtualField', 'virtualField1'],
            validator: async (ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });
              return undefined;
            },
          },
          onSuccess: {
            fields: ['virtualField', 'virtualField1'],
            resolver: (ctx) => {
              triggeredWith = ctx.options.messages[0];
            },
          },
        },
      ).getModel();

      const previous = { dependent: defaultDependentValue };
      const virtualValue = 'updated_virtual_value';

      const {
        data: updates,
        handleSuccess,
        options,
      } = await Model.update(
        previous,
        { virtualField: virtualValue, virtualField1: 'other value' },
        { messages: [] },
      );

      expect(updates).toEqual({ dependent: virtualValue });
      expect(options.messages[0]).toBe(MESSAGE);

      await handleSuccess?.();

      expect(triggeredWith).toBe(MESSAGE);
    });
  });
});
