import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

describe('extras.ctxOptions.required', () => {
  describe('ignoreUpdate', () => {
    it('should properly update ctx options in ignoreUpdate resolver and provide those updates in onSuccess handlers during updates', async () => {
      const DEFAULT_VALUE = 1;
      const MESSAGE = 'ctx_options updated in ignore_update resolver';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { required: number },
        { required: number },
        { messages: string[] }
      >((b) =>
        b.field(
          b
            .required('required')
            .validate(() => ({ valid: true }))
            .ignoreUpdate((ctx) => {
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

      const data = { required: DEFAULT_VALUE };
      const required = data.required + 1;

      const {
        data: updates,
        handleSuccess,
        options,
      } = await Model.update(data, { required }, { messages: [] });

      expect(updates).toEqual({ required });
      expect(options.messages[0]).toBe(MESSAGE);

      await handleSuccess?.();

      expect(triggeredWith).toBe(MESSAGE);
    });
  });

  describe('validate', () => {
    it('should properly update ctx options in validators and provide those updates in onFailure handlers at creation', async () => {
      const MESSAGE = 'ctx_options updated in validator';
      const MIN_LENGTH_ERROR =
        'expected required to be at least 2 characters long';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { required: string },
        { required: string },
        { messages: string[] }
      >((b) =>
        b.field(
          b
            .required('required')
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
        { required: ' ' },
        { messages: [] },
      );

      expect(error?.required?.reason).toBe(MIN_LENGTH_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);

      await handleFailure?.();

      expect(triggeredWith).toBe(MESSAGE);
    });

    it('should properly update ctx options in validators and provide those updates in onFailure handlers during updates', async () => {
      const DEFAULT_VALUE = 'default_value';
      const MESSAGE = 'ctx_options updated in validator';
      const MIN_LENGTH_ERROR =
        'expected required to be at least 2 characters long';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { required: string },
        { required: string },
        { messages: string[] }
      >((b) =>
        b.field(
          b
            .required('required')
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
        { required: DEFAULT_VALUE },
        { required: ' ' },
        { messages: [] },
      );

      expect(error?.payload?.required?.reason).toBe(MIN_LENGTH_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);

      await handleFailure?.();

      expect(triggeredWith).toBe(MESSAGE);
    });
  });

  describe('reValidate', () => {
    it('should properly update ctx options in re-validators and provide those updates in onFailure handlers at creation', async () => {
      const MESSAGE = 'ctx_options updated in re_validator';
      const MIN_LENGTH_ERROR =
        'expected required to be at least 2 characters long';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { required: string },
        { required: string },
        { messages: string[] }
      >((b) =>
        b.field(
          b
            .required('required')
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
        { required: ' ' },
        { messages: [] },
      );

      expect(error?.required?.reason).toBe(MIN_LENGTH_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);

      await handleFailure?.();

      expect(triggeredWith).toBe(MESSAGE);
    });

    it('should properly update ctx options in re-validators and provide those updates in onFailure handlers during updates', async () => {
      const DEFAULT_VALUE = 'default_value';
      const MESSAGE = 'ctx_options updated in re_validator';
      const MIN_LENGTH_ERROR =
        'expected required to be at least 2 characters long';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { required: string },
        { required: string },
        { messages: string[] }
      >((b) =>
        b.field(
          b
            .required('required')
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
        { required: DEFAULT_VALUE },
        { required: ' ' },
        { messages: [] },
      );

      expect(error?.payload?.required?.reason).toBe(MIN_LENGTH_ERROR);
      expect(options.messages[0]).toBe(MESSAGE);

      await handleFailure?.();

      expect(triggeredWith).toBe(MESSAGE);
    });
  });

  describe('o.postValidate & o.onSuccess', () => {
    it('should properly update ctx options in post-validators and provide those updates in grouped onSuccess handlers with no fields at creation', async () => {
      const MESSAGE = 'ctx_options updated in post_validator';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { required: number; required_1: number },
        { required: number; required_1: number },
        { messages: string[] }
      >(
        (b) =>
          b
            .field(b.required('required').validate(() => ({ valid: true })))
            .field(b.required('required_1').validate(() => ({ valid: true }))),
        {
          postValidate: {
            fields: ['required', 'required_1'],
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

      const required = 2;
      const required1 = 3;
      const { data, handleSuccess, options } = await Model.create(
        { required, required_1: required1 },
        { messages: [] },
      );

      expect(data).toEqual({ required, required_1: required1 });
      expect(options.messages[0]).toBe(MESSAGE);

      await handleSuccess?.();

      expect(triggeredWith).toBe(MESSAGE);
    });

    it('should properly update ctx options in post-validators and provide those updates in grouped onSuccess handlers during updates', async () => {
      const DEFAULT_VALUE = 1;
      const MESSAGE = 'ctx_options updated in post_validator';
      let triggeredWith: string | undefined;

      const Model = new Schema<
        { required: number; required_1: number },
        { required: number; required_1: number },
        { messages: string[] }
      >(
        (b) =>
          b
            .field(b.required('required').validate(() => ({ valid: true })))
            .field(b.required('required_1').validate(() => ({ valid: true }))),
        {
          postValidate: {
            fields: ['required', 'required_1'],
            validator: async (ctx) => {
              ctx.updateOptions({
                messages: [...ctx.options.messages, MESSAGE],
              });
              return undefined;
            },
          },
          onSuccess: {
            fields: ['required', 'required_1'],
            resolver: (ctx) => {
              triggeredWith = ctx.options.messages[0];
            },
          },
        },
      ).getModel();

      const data = { required: DEFAULT_VALUE, required_1: DEFAULT_VALUE };
      const required = data.required + 1;

      const {
        data: updates,
        handleSuccess,
        options,
      } = await Model.update(data, { required }, { messages: [] });

      expect(updates).toEqual({ required });
      expect(options.messages[0]).toBe(MESSAGE);

      await handleSuccess?.();

      expect(triggeredWith).toBe(MESSAGE);
    });
  });
});
