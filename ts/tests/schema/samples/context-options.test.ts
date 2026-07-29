import { afterEach, describe, expect, it } from 'bun:test';

import { type IvoContext, Schema } from '../../../src';

describe('Context options', () => {
  describe('RequiredBy', () => {
    const contextOptions = { lang: 'en' };
    const validator = () => true;
    function handleRequired(prop: string) {
      return ({
        options,
      }: IvoContext<
        { name: string; price: number },
        { name: string; price: number },
        typeof contextOptions
      >) => {
        ctxOptions[prop] = options;

        return false;
      };
    }

    const Model = new Schema<
      { name: string; price: number },
      { name: string; price: number },
      any,
      typeof contextOptions
    >({
      name: { default: '', required: handleRequired('name'), validator },
      price: { default: 0, required: handleRequired('price'), validator },
    }).getModel();

    let ctxOptions: any = {};

    afterEach(() => {
      ctxOptions = {};
    });

    it('provided "contextOptions" should be accessible in requiredBy methods at creation', async () => {
      await Model.create({ name: 'test', price: 4 }, contextOptions);

      expect(ctxOptions).toEqual({
        name: contextOptions,
        price: contextOptions,
      });
    });

    it('provided "contextOptions" should be accessible in requiredBy methods during updates', async () => {
      await Model.update(
        { name: 'test', price: 4 },
        { name: 'updateds', price: 4 },
        contextOptions,
      );

      expect(ctxOptions).toEqual({
        name: contextOptions,
        price: contextOptions,
      });
    });
  });

  describe('should accept functions properly', () => {
    let called = false;

    function ctxHandler() {
      called = true;
    }

    const Model = new Schema<
      { name: string },
      { name: string },
      { ctxHandler: () => any }
    >({
      name: {
        default: '',
        validator(_, { options }) {
          options.ctxHandler();
          return true;
        },
        onDelete(_, options) {
          options.ctxHandler();
        },
        onSuccess({ options }) {
          options.ctxHandler();
        },
      },
    }).getModel();

    afterEach(() => {
      called = false;
    });

    it('should allow provided function in ctx options at creation', async () => {
      expect(called).toBe(false);

      await Model.create({ name: 'lol' }, { ctxHandler });

      expect(called).toBe(true);
    });

    it('should allow provided function in ctx options onSuccess after creation', async () => {
      expect(called).toBe(false);

      const { handleSuccess } = await Model.create({}, { ctxHandler });

      expect(called).toBe(false);

      handleSuccess?.();

      expect(called).toBe(true);
    });

    it('should allow provided function in ctx options during updates', async () => {
      expect(called).toBe(false);

      await Model.update({ name: 'lol' }, { name: 'updated' }, { ctxHandler });

      expect(called).toBe(true);
    });

    it('should allow provided function in ctx options on deletion', async () => {
      expect(called).toBe(false);

      await Model.delete({ name: 'lol' }, { ctxHandler });

      expect(called).toBe(true);
    });
  });
});
