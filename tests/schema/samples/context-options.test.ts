import { afterEach, describe, expect, it } from 'vitest';

import {
  UserModel,
  UserModel_ErrorThrow,
  EUserModel,
  EUserModel_ErrorThrow
} from './custom-error-tool';
import Schema from '../../../dist';

const contextOptions = { lang: 'en' };

describe('Context options', () => {
  describe('Base Model', () => {
    describe('Schema.options.errors == silent', () => {
      it('should respect ctx options with Model.create', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await UserModel.create(
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });

      it('should respect ctx options with Model.update', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await UserModel.update(
          { firstName: 'John', fullName: '', lastName: 'lopa' },
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });
    });

    describe('Schema.options.errors == throw', () => {
      it('should respect ctx options with Model.create', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await UserModel_ErrorThrow.create(
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });

      it('should respect ctx options with Model.update', async () => {
        const firstName = 'name',
          lastName = 'lname';

        const { data, error } = await UserModel_ErrorThrow.update(
          { firstName: 'John', fullName: '', lastName: 'lopa' },
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });
    });
  });

  describe('Extended Model', () => {
    describe('Schema.options.errors == silent', () => {
      it('should respect ctx options with Model.create', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await EUserModel.create(
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });

      it('should respect ctx options with Model.update', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await EUserModel.update(
          { firstName: 'John', full_name: '', lastName: 'lopa' },
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });
    });

    describe('Schema.options.errors == throw', () => {
      it('should respect ctx options with Model.create', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await EUserModel_ErrorThrow.create(
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });

      it('should respect ctx options with Model.update', async () => {
        const firstName = 'name',
          lastName = 'lname';

        const { data, error } = await EUserModel_ErrorThrow.update(
          { firstName: 'John', full_name: '', lastName: 'lopa' },
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });
    });
  });

  describe('RequiredBy', () => {
    const contextOptions = { lang: 'en' };
    const validator = () => true;
    function handleRequired(prop: string) {
      return ({ context: { __getOptions__ } }) => {
        ctxOptions[prop] = __getOptions__();

        return false;
      };
    }

    const Model = new Schema<any, any, any, typeof contextOptions>({
      name: { default: '', required: handleRequired('name'), validator },
      price: { default: '', required: handleRequired('price'), validator }
    }).getModel();

    let ctxOptions: any = {};

    afterEach(() => {
      ctxOptions = {};
    });

    it('provided "contextOptions" should be accessible in requiredBy methods at creation', async () => {
      await Model.create({ name: 'test', price: 4 }, contextOptions);

      expect(ctxOptions).toEqual({
        name: contextOptions,
        price: contextOptions
      });
    });

    it('provided "contextOptions" should be accessible in requiredBy methods during updates', async () => {
      await Model.update(
        { name: 'test', price: 4 },
        { name: 'updateds', price: 4 },
        contextOptions
      );

      expect(ctxOptions).toEqual({
        name: contextOptions,
        price: contextOptions
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
      {},
      { ctxHandler: () => any }
    >({
      name: {
        default: '',
        validator(_, { context: { __getOptions__ } }) {
          __getOptions__().ctxHandler();
          return true;
        },
        onDelete({ __getOptions__ }) {
          __getOptions__().ctxHandler();
        },
        onSuccess({ context: { __getOptions__ } }) {
          __getOptions__().ctxHandler();
        }
      }
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
