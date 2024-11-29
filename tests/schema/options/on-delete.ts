import { beforeEach, describe, expect, it } from 'bun:test';

import { ERRORS } from '../../../dist';
import {
  expectFailure,
  expectNoFailure,
  getValidSchema,
  validator,
} from '../_utils';

export const Test_SchemaOnDelete = ({ Schema, fx }: any) => {
  describe('Schema.options.onDelete', () => {
    describe('behaviour', () => {
      const values = { id: 1, name: 'Book name', price: 100 };
      let deletedValues: any = {};

      function onDelete_(prop = '') {
        return (values: any) => {
          deletedValues[prop] = values;
        };
      }

      beforeEach(() => {
        deletedValues = {};
      });

      describe('behaviour with other delete handlers', () => {
        const Book = new Schema(
          {
            id: { constant: true, value: 1, onDelete: onDelete_('id') },
            name: { required: true, validator, onDelete: onDelete_('name') },
            price: {
              default: null,
              dependsOn: '_setPrice',
              resolver: ({ context }: any) => context._setPrice,
              onDelete: onDelete_('price'),
            },
            _setPrice: { virtual: true, validator },
          },
          { onDelete: onDelete_('global') },
        ).getModel();

        it("should trigger all 'delete' handlers on properties an global handlers", async () => {
          await Book.delete(values);

          expect(deletedValues).toMatchObject({
            id: values,
            name: values,
            price: values,
            global: values,
          });
        });
      });

      describe('behaviour without other delete handlers', () => {
        const Book = new Schema(
          {
            id: { constant: true, value: 1 },
            name: { required: true, validator },
            price: {
              default: null,
              dependsOn: '_setPrice',
              resolver: ({ context }: any) => context._setPrice,
            },
            _setPrice: { virtual: true, validator },
          },
          { onDelete: [onDelete_('global'), onDelete_('global-1')] },
        ).getModel();

        it("should trigger all global 'delete' handlers", async () => {
          await Book.delete(values);

          expect(deletedValues).toMatchObject({
            global: values,
            'global-1': values,
          });
        });
      });
    });

    describe('valid', () => {
      it("should allow 'onDelete' as (() => any) | ((() => any)[])", () => {
        const values = [() => {}, [() => {}]];

        for (const onDelete of values) {
          const toPass = fx(getValidSchema(), { onDelete });

          expectNoFailure(toPass);

          toPass();
        }
      });
    });

    describe('invalid', () => {
      it("should reject 'onDelete' other than (() => any) | ((() => any)[])", () => {
        const invalidValues = [
          1,
          0,
          -14,
          true,
          false,
          'invalid',
          '',
          null,
          undefined,
        ];

        for (const onDelete of invalidValues) {
          const toFail = fx(getValidSchema(), { onDelete });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err).toMatchObject({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                onDelete: expect.arrayContaining([
                  "The 'onDelete' handler at index: 0 is not a function",
                ]),
              },
            });
          }
        }
      });
    });
  });
};
