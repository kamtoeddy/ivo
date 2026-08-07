import { beforeEach, describe, expect, it } from 'bun:test';

import { Schema } from '../../src';
import {
  expectFailure,
  expectNoFailure,
  getValidSchema,
  makeFx,
  validator,
} from '../_utils';

describe('Schema.options.onDelete', () => {
  describe('behaviour', () => {
    type BookInput = { _setPrice?: number; name?: string };
    type BookOutput = { id: number; name: string; price: number | null };

    const values: BookOutput = { id: 1, name: 'Book name', price: 100 };
    let deletedValues: Record<string, unknown> = {};

    function onDelete_(field = '') {
      return (data: Readonly<BookOutput>) => {
        deletedValues[field] = data;
      };
    }

    beforeEach(() => {
      deletedValues = {};
    });

    describe('behaviour with other delete handlers', () => {
      const Book = new Schema<BookInput, BookOutput>(
        (b) =>
          b
            .field(b.constant('id', 1).onDelete(onDelete_('id')))
            .field(
              b
                .required('name')
                .validate(validator)
                .onDelete(onDelete_('name')),
            )
            .field(
              b
                .dependent('price', '_setPrice')
                .default(null)
                .resolve((ctx) => ctx.input._setPrice!)
                .onDelete(onDelete_('price')),
            )
            .field(b.virtual('_setPrice').validate(validator)),
        { onDelete: onDelete_('global') },
      ).getModel();

      it("should trigger all 'delete' handlers on properties an global handlers", async () => {
        await Book.delete(values, {});

        expect(deletedValues).toMatchObject({
          id: values,
          name: values,
          price: values,
          global: values,
        });
      });
    });

    describe('behaviour without other delete handlers', () => {
      const Book = new Schema<BookInput, BookOutput>(
        (b) =>
          b
            .field(b.constant('id', 1))
            .field(b.required('name').validate(validator))
            .field(
              b
                .dependent('price', '_setPrice')
                .default(null)
                .resolve((ctx) => ctx.input._setPrice!),
            )
            .field(b.virtual('_setPrice').validate(validator)),
        { onDelete: [onDelete_('global'), onDelete_('global-1')] },
      ).getModel();

      it("should trigger all global 'delete' handlers", async () => {
        await Book.delete(values, {});

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
        const toPass = makeFx(getValidSchema(), { onDelete });

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
        const toFail = makeFx(getValidSchema(), { onDelete });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err).toMatchObject({
            message: 'INVALID_SCHEMA',
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
