import { describe, it, expect } from 'bun:test';

import { ERRORS } from '../../../dist';
import {
  expectFailure,
  expectNoFailure,
  getValidSchema,
  validator,
} from '../_utils';

export const Test_SchemaShouldUpdateOption = ({ Schema, fx }: any) => {
  describe('Schema.options.shouldUpdate', () => {
    describe('behaviour', () => {
      const book = { name: 'book name', price: 10 };
      const update = { name: 'updated name' };

      const error = {
        message: ERRORS.NOTHING_TO_UPDATE,
        payload: {},
      };

      const options = [
        { data: null, error, shouldUpdate: false },
        { data: update, error: null, shouldUpdate: true },
        { data: null, error, shouldUpdate: () => false },
        { data: update, error: null, shouldUpdate: () => true },
        { data: null, error, shouldUpdate: () => ({ update: false }) },
        { data: update, error: null, shouldUpdate: () => ({ update: true }) },
        { data: null, error, shouldUpdate: () => Promise.resolve(false) },
        {
          data: update,
          error: null,
          shouldUpdate: () => Promise.resolve(true),
        },
        {
          data: null,
          error,
          shouldUpdate: () => Promise.resolve({ update: false }),
        },
        {
          data: update,
          error: null,
          shouldUpdate: () => Promise.resolve({ update: true }),
        },
        // falsy values
        { data: null, error, shouldUpdate: () => 0 },
        { data: null, error, shouldUpdate: () => 1 },
        { data: null, error, shouldUpdate: () => '' },
        { data: null, error, shouldUpdate: () => '1' },
        { data: null, error, shouldUpdate: () => [] },
        { data: null, error, shouldUpdate: () => {} },
        { data: null, error, shouldUpdate: () => ({}) },
        { data: null, error, shouldUpdate: () => null },
        { data: null, error, shouldUpdate: () => undefined },
      ];

      it('should respect the "shouldUpdate" rule accordingly', async () => {
        for (const option of options) {
          const Model = new Schema(
            {
              name: { required: true, validator },
              price: { required: true, validator },
            },
            { shouldUpdate: option.shouldUpdate },
          ).getModel();

          const { data, error } = await Model.update(book, update);

          expect(data).toEqual(option.data);
          expect(error).toEqual(option.error);
        }
      });

      it('should respect the "contextOptionsUpdate" returned if returned from shouldUpdate', async () => {
        const contextOptions = { lang: 'en' };

        let ctxOptionsStats: any = {};

        const options = [
          { data: null, error, shouldUpdate: shouldUpdate({ update: false }) },
          {
            ctx: contextOptions,
            data: update,
            error: null,
            shouldUpdate: shouldUpdate({ update: true }),
          },
          {
            ctx: { lang: '' },
            data: null,
            error,
            shouldUpdate: shouldUpdate({
              update: false,
              contextOptionsUpdate: { lang: '' },
            }),
          },
          {
            ctx: { lang: 'fr', ctx: true },
            data: update,
            error: null,
            shouldUpdate: shouldUpdate({
              update: true,
              contextOptionsUpdate: { lang: 'fr', ctx: true },
            }),
          },
          {
            ctx: { lang: 'de' },
            data: null,
            error,
            shouldUpdate: AsyncShouldUpdate({
              update: false,
              contextOptionsUpdate: { lang: 'de' },
            }),
          },
          {
            ctx: { lang: 'fr', async: true },
            data: update,
            error: null,
            shouldUpdate: AsyncShouldUpdate({
              update: true,
              contextOptionsUpdate: { lang: 'fr', async: true },
            }),
          },
        ];

        function AsyncShouldUpdate(v) {
          return ({ context }) => {
            ctxOptionsStats.initial = context.__getOptions__();

            return Promise.resolve(v);
          };
        }

        function shouldUpdate(v) {
          return ({ context }) => {
            ctxOptionsStats.initial = context.__getOptions__();

            return v;
          };
        }

        function validator(_, { context }) {
          ctxOptionsStats.final = context.__getOptions__();

          return true;
        }

        for (const option of options) {
          ctxOptionsStats = {};

          const Model = new Schema(
            {
              name: { required: true, validator },
              price: { required: true, validator },
            },
            { shouldUpdate: option.shouldUpdate },
          ).getModel();

          const { data, error } = await Model.update(
            book,
            update,
            contextOptions,
          );

          expect(data).toEqual(option.data);
          expect(error).toEqual(option.error);

          expect(ctxOptionsStats.initial).toEqual(contextOptions);

          if (option.data) expect(ctxOptionsStats.final).toEqual(option.ctx);
        }
      });
    });

    describe('valid', () => {
      it("should allow 'shouldUpdate' as boolean | () => boolean", () => {
        const values = [true, false, () => {}];

        for (const shouldUpdate of values) {
          const toPass = fx(getValidSchema(), { shouldUpdate });

          expectNoFailure(toPass);

          toPass();
        }
      });
    });

    describe('invalid', () => {
      it("should reject 'shouldUpdate' other than boolean or function", () => {
        const invalidValues = [
          1,
          0,
          -14,
          {},
          [],
          'invalid',
          '',
          null,
          undefined,
        ];

        for (const shouldUpdate of invalidValues) {
          const toFail = fx(getValidSchema(), { shouldUpdate });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err).toMatchObject({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                shouldUpdate: expect.arrayContaining([
                  "'shouldUpdate' should either be a 'boolean' or a 'function'",
                ]),
              },
            });
          }
        }
      });
    });
  });
};
