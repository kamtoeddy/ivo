import { beforeEach, describe, it, expect } from 'vitest';

import { ERRORS } from '../../../../dist';
import {
  expectFailure,
  expectNoFailure,
  getValidSchema,
  validator
} from '../_utils';

export const Test_SchemaOnSuccess = ({ Schema, fx }: any) => {
  describe('Schema.options.onSuccess', () => {
    describe('behaviour', () => {
      let successValues: any = {};

      function onSuccess_(prop = '') {
        return (summary: any) => (successValues[prop] = summary);
      }

      beforeEach(() => {
        successValues = {};
      });

      describe('behaviour with other success listeners', () => {
        const Book = new Schema(
          {
            id: { constant: true, value: 1, onSuccess: onSuccess_('id') },
            name: { required: true, validator, onSuccess: onSuccess_('name') },
            price: {
              default: null,
              dependsOn: '_setPrice',
              resolver: ({ context }: any) => context._setPrice,
              onSuccess: onSuccess_('price')
            },
            _setPrice: {
              virtual: true,
              validator,
              onSuccess: onSuccess_('_setPrice')
            }
          },
          { onSuccess: onSuccess_('global') }
        ).getModel();

        it("should trigger all 'success' listeners at creation", async () => {
          const { data, handleSuccess } = await Book.create({
            name: 'Book name',
            _setPrice: 100
          });

          await handleSuccess();

          const values = { id: 1, name: 'Book name', price: 100 };
          const summary = {
            changes: null,
            context: { ...values, _setPrice: 100 },
            operation: 'creation',
            previousValues: null,
            values: values
          };

          expect(data).toEqual(values);
          expect(successValues).toMatchObject({
            id: summary,
            name: summary,
            price: summary,
            _setPrice: summary,
            global: summary
          });
        });

        it("should trigger all 'success' listeners during updates ", async () => {
          const book = { id: 1, name: 'Book name', price: 100 };

          const { data, handleSuccess } = await Book.update(book, {
            _setPrice: 200
          });

          await handleSuccess();

          const values = { ...book, price: 200 };

          const summary = {
            changes: data,
            context: { ...values, _setPrice: 200 },
            operation: 'update',
            previousValues: book,
            values: values
          };

          expect(data).toEqual({ price: 200 });
          expect(successValues).toMatchObject({
            price: summary,
            _setPrice: summary,
            global: summary
          });
        });
      });

      describe('behaviour without other success listeners', () => {
        const Book = new Schema(
          {
            id: { constant: true, value: 1 },
            name: { required: true, validator },
            price: {
              default: null,
              dependsOn: '_setPrice',
              resolver: ({ context }: any) => context._setPrice
            },
            _setPrice: { virtual: true, validator }
          },
          { onSuccess: [onSuccess_('global'), onSuccess_('global-1')] }
        ).getModel();

        it("should trigger all 'success' listeners at creation", async () => {
          const { data, handleSuccess } = await Book.create({
            name: 'Book name',
            _setPrice: 100
          });

          await handleSuccess();

          const values = { id: 1, name: 'Book name', price: 100 };
          const summary = {
            changes: null,
            context: { ...values, _setPrice: 100 },
            operation: 'creation',
            previousValues: null,
            values: values
          };

          expect(data).toEqual(values);
          expect(successValues).toMatchObject({
            global: summary,
            'global-1': summary
          });
        });

        it("should trigger all 'success' listeners during updates ", async () => {
          const book = { id: 1, name: 'Book name', price: 100 };

          const { data, handleSuccess } = await Book.update(book, {
            _setPrice: 200
          });

          await handleSuccess();

          const values = { ...book, price: 200 };

          const summary = {
            changes: data,
            context: { ...values, _setPrice: 200 },
            operation: 'update',
            previousValues: book,
            values: values
          };

          expect(data).toEqual({ price: 200 });
          expect(successValues).toMatchObject({
            global: summary,
            'global-1': summary
          });
        });
      });
    });

    describe('valid', () => {
      it("should allow 'onSuccess' as (() => any) | ((() => any)[])", () => {
        const values = [() => {}, [() => {}]];

        for (const onSuccess of values) {
          const toPass = fx(getValidSchema(), { onSuccess });

          expectNoFailure(toPass);

          toPass();
        }
      });
    });

    describe('invalid', () => {
      it("should reject 'onSuccess' other than (() => any) | ((() => any)[])", () => {
        const invalidValues = [
          1,
          0,
          -14,
          true,
          false,
          'invalid',
          '',
          null,
          undefined
        ];

        for (const onSuccess of invalidValues) {
          const toFail = fx(getValidSchema(), { onSuccess });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err).toMatchObject({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                onSuccess: expect.arrayContaining([
                  "The 'onSuccess' handler @[0] is not a function"
                ])
              }
            });
          }
        }
      });
    });
  });
};
