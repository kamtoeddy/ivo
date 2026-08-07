import { beforeEach, describe, expect, it } from 'bun:test';

import { Schema } from '../../src';
import type { IvoSuccessContext } from '../../src/utils/types';
import {
  expectFailure,
  expectNoFailure,
  getValidSchema,
  makeFx,
  validator,
} from '../_utils';

describe('Schema.options.onSuccess', () => {
  describe('signature', () => {
    describe('valid', () => {
      it("should allow valid 'onSuccess' config", () => {
        const values = [
          () => {},
          [() => {}],
          [() => {}, () => {}],
          {
            fields: ['fieldName1', 'fieldName2'],
            handler: () => {},
          },
          {
            fields: ['fieldName1', 'fieldName2'],
            handler: [() => {}, () => {}],
          },
          {
            fields: [
              'constant',
              'laxField',
              'fieldName2',
              'dependent',
              'virtual',
            ],
            handler: [() => {}, () => {}],
          },
          {
            fields: [
              'constant',
              'laxField',
              'fieldName2',
              'dependent',
              'virtual',
            ],
            handler: () => {},
          },
          [
            () => {},
            {
              fields: ['fieldName1', 'constant'],
              handler: [() => {}, () => {}],
            },
            {
              fields: ['laxField', 'fieldName2', 'dependent', 'virtual'],
              handler: () => {},
            },
          ],
          [
            () => {},
            {
              fields: ['fieldName1', 'fieldName1', 'constant'],
              handler: [() => {}, () => {}],
            },
            {
              fields: ['laxField', 'fieldName2', 'dependent', 'virtual'],
              handler: () => {},
            },
          ],
        ];

        for (const onSuccess of values) {
          const toPass = makeFx(
            (b) =>
              b
                .field(b.constant('constant', ''))
                .field(b.lax('fieldName1', ''))
                .field(b.lax('fieldName2', ''))
                .field(b.lax('laxField', ''))
                .field(
                  b
                    .dependent('dependent', ['laxField', 'virtual'])
                    .default('')
                    .resolve(() => {}),
                )
                .field(
                  b
                    .required('readonly')
                    .validate(() => false)
                    .readonly(),
                )
                .field(b.virtual('virtual').validate(() => false)),
            {
              onSuccess,
            },
          );

          expectNoFailure(toPass);

          toPass();
        }
      });

      it("should allow 'onSuccess' if a property or virtual is provided in more than 1 config or subsets if the configs don't have the same fields", () => {
        const toPass = makeFx(
          (b) =>
            b
              .field(b.constant('constant', ''))
              .field(b.lax('laxField', ''))
              .field(b.lax('fieldName1', ''))
              .field(b.lax('fieldName2', ''))
              .field(
                b
                  .dependent('dependent', ['laxField', 'virtual'])
                  .default('')
                  .resolve(() => {}),
              )
              .field(
                b
                  .required('readonly')
                  .validate(() => false)
                  .readonly(),
              )
              .field(b.virtual('virtual').validate(() => false)),
          {
            onSuccess: [
              {
                fields: ['fieldName1', 'laxField', 'dependent'],
                handler: () => {},
              },
              {
                fields: ['virtual', 'laxField'],
                handler: () => {},
              },
              {
                fields: ['dependent', 'fieldName1'],
                handler: () => {},
              },
            ],
          },
        );

        expectNoFailure(toPass);

        toPass();
      });
    });

    describe('invalid', () => {
      it('should reject if any of the fields passed in config object are not valid fields or virtuals', () => {
        const invalidFielderties = [
          1,
          0,
          -14,
          true,
          false,
          'invalid',
          '',
          null,
          undefined,
          [],
        ];

        const schemaWithInvalidFielderties = makeFx(getValidSchema(), {
          onSuccess: { fields: invalidFielderties, handler: () => {} },
        });

        expectFailure(schemaWithInvalidFielderties);

        try {
          schemaWithInvalidFielderties();
        } catch (err: any) {
          expect(err).toMatchObject({
            message: 'INVALID_SCHEMA',
            payload: {
              onSuccess: expect.arrayContaining(
                invalidFielderties.map(
                  (field) =>
                    `"${field}" is not a property or virtual on your schema`,
                ),
              ),
            },
          });
        }

        const schemaWithNestedInvalidFielderties = makeFx(getValidSchema(), {
          onSuccess: [{ fields: invalidFielderties, handler: () => {} }],
        });

        expectFailure(schemaWithNestedInvalidFielderties);

        try {
          schemaWithNestedInvalidFielderties();
        } catch (err: any) {
          expect(err).toMatchObject({
            message: 'INVALID_SCHEMA',
            payload: {
              onSuccess: expect.arrayContaining(
                invalidFielderties.map(
                  (field) =>
                    `Config at index 0: "${field}" is not a property or virtual on your schema`,
                ),
              ),
            },
          });
        }
      });
    });
  });

  describe('behaviour', () => {
    let successValues: Record<string, unknown> = {};

    type BookInput = { _setPrice?: number; name?: string };
    type BookOutput = { id: number; name: string; price: number | null };

    function onSuccess_(field = '') {
      return (summary: IvoSuccessContext<any>) => {
        successValues[field] = summary;
      };
    }

    beforeEach(() => {
      successValues = {};
    });

    describe('behaviour with other success listeners', () => {
      const Book = new Schema<BookInput, BookOutput>(
        (b) =>
          b
            .field(b.constant('id', 1).onSuccess(onSuccess_('id')))
            .field(
              b
                .required('name')
                .validate(validator)
                .onSuccess(onSuccess_('name')),
            )
            .field(
              b
                .dependent('price', '_setPrice')
                .default(null)
                .resolve((ctx) => ctx.input._setPrice!)
                .onSuccess(onSuccess_('price')),
            )
            .field(
              b
                .virtual('_setPrice')
                .validate(validator)
                .onSuccess(onSuccess_('_setPrice')),
            ),
        { onSuccess: onSuccess_('global') },
      ).getModel();

      it("should trigger all 'success' listeners at creation", async () => {
        const { data, handleSuccess } = await Book.create(
          {
            name: 'Book name',
            _setPrice: 100,
          },
          {},
        );

        await handleSuccess?.();

        const values = { id: 1, name: 'Book name', price: 100 };
        const summary = {
          changes: null,
          input: { name: 'Book name', _setPrice: 100 },
          isUpdate: false,
          previousValues: null,
          values: values,
        };

        expect(data).toEqual(values);
        expect(successValues).toMatchObject({
          id: summary,
          name: summary,
          price: summary,
          _setPrice: summary,
          global: summary,
        });
      });

      it("should trigger all 'success' listeners during updates ", async () => {
        const book = { id: 1, name: 'Book name', price: 100 };

        const { data, handleSuccess } = await Book.update(
          book,
          {
            _setPrice: 200,
          },
          {},
        );

        await handleSuccess?.();

        const values = { ...book, price: 200 };

        const summary = {
          changes: data,
          input: { _setPrice: 200 },
          isUpdate: true,
          previousValues: book,
          values: values,
        };

        expect(data).toEqual({ price: 200 });
        expect(successValues).toMatchObject({
          price: summary,
          _setPrice: summary,
          global: summary,
        });
      });
    });

    describe('behaviour without other success listeners', () => {
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
        { onSuccess: [onSuccess_('global'), onSuccess_('global-1')] },
      ).getModel();

      it("should trigger all 'success' listeners at creation", async () => {
        const { data, handleSuccess } = await Book.create(
          {
            name: 'Book name',
            _setPrice: 100,
          },
          {},
        );

        await handleSuccess?.();

        const values = { id: 1, name: 'Book name', price: 100 };
        const summary = {
          changes: null,
          input: { name: 'Book name', _setPrice: 100 },
          isUpdate: false,
          previousValues: null,
          values: values,
        };

        expect(data).toEqual(values);
        expect(successValues).toMatchObject({
          global: summary,
          'global-1': summary,
        });
      });

      it("should trigger all 'success' listeners during updates ", async () => {
        const book = { id: 1, name: 'Book name', price: 100 };

        const { data, handleSuccess } = await Book.update(
          book,
          {
            _setPrice: 200,
          },
          {},
        );

        await handleSuccess?.();

        const values = { ...book, price: 200 };

        const summary = {
          changes: data,
          input: { _setPrice: 200 },
          isUpdate: true,
          previousValues: book,
          values: values,
        };

        expect(data).toEqual({ price: 200 });
        expect(successValues).toMatchObject({
          global: summary,
          'global-1': summary,
        });
      });
    });

    describe('behaviour onSuccess config object', () => {
      let successValuesFromOptions: Record<string, number> = {};

      beforeEach(() => {
        successValuesFromOptions = {};
      });

      function onOptionSuccess(props: string[]) {
        return () => {
          props.forEach((field) => {
            successValuesFromOptions[field] =
              (successValuesFromOptions[field] ?? 0) + 1;
          });
        };
      }

      describe('constant fields', () => {
        const Model = new Schema<any>(
          (b) =>
            b
              .field(b.constant('const1', 1))
              .field(b.constant('const2', 2))
              .field(b.lax('lax', true)),
          {
            onSuccess: {
              fields: ['const1', 'const2'],
              // @ts-expect-error failed to properly infer
              handler: onOptionSuccess(['const1', 'const2']),
            },
          },
        ).getModel();

        it("should trigger all 'success' listeners of constant props at creation", async () => {
          const { data, handleSuccess } = await Model.create({}, {});

          await handleSuccess?.();

          expect(data).not.toBeNull();
          expect(successValuesFromOptions).toEqual({
            const1: 1,
            const2: 1,
          });
        });

        it("should not trigger 'success' listeners of constant props during updates", async () => {
          const initialData = { const1: 400, const2: 400, lax: 100 };

          // @ts-expect-error ikr
          const { data, handleSuccess } = await Model.update(initialData, {
            const1: 200,
            const2: 200,
            lax: 200,
          });

          await handleSuccess?.();

          expect(data).not.toBeNull();
          expect(successValuesFromOptions).toEqual({});
        });
      });

      describe('non-constant fields', () => {
        const Model = new Schema<any>(
          (b) =>
            b
              .field(b.constant('const', 1))
              .field(b.lax('lax', true))
              .field(b.lax('lax2', true))
              .field(b.required('required').validate(validator))
              .field(b.required('required2').validate(validator))
              .field(
                b
                  .dependent('dependent', ['lax2', 'virtual1', 'virtual2'])
                  .default(null)
                  .resolve(validator as never)
                  .onSuccess(onSuccess_('dependent')),
              )
              .field(b.virtual('virtual1').validate(validator))
              .field(b.virtual('virtual2').validate(validator)),
          {
            onSuccess: [
              onOptionSuccess(['dependent']),
              {
                fields: ['lax', 'lax2'],
                // @ts-expect-error failed to properly infer
                handler: [
                  onOptionSuccess(['lax', 'lax2']),
                  onOptionSuccess(['lax2']),
                ],
              },
              {
                fields: ['virtual1', 'virtual2'],
                // @ts-expect-error failed to properly infer
                handler: onOptionSuccess(['virtual1', 'virtual2']),
              },
              {
                fields: ['required', 'const'],
                // @ts-expect-error failed to properly infer
                handler: onOptionSuccess(['required', 'const']),
              },
              {
                fields: ['required2', 'dependent'],
                // @ts-expect-error failed to properly infer
                handler: onOptionSuccess(['required2', 'dependent']),
              },
            ],
          },
        ).getModel();

        it("should trigger all related 'success' listeners at creation", async () => {
          const { data, handleSuccess } = await Model.create(
            {
              required: 100,
              required2: 100,
            },
            {},
          );

          await handleSuccess?.();

          expect(data).not.toBeNull();
          expect(successValuesFromOptions).toEqual({
            const: 1,
            dependent: 2,
            lax: 1,
            lax2: 2,
            required: 1,
            required2: 1,
          });
        });

        it("should trigger 'success' listeners of virtual at creation if they are provided", async () => {
          const { data, handleSuccess } = await Model.create(
            {
              required: 100,
              required2: 100,
              virtual1: 4,
            },
            {},
          );

          await handleSuccess?.();

          expect(data).not.toBeNull();
          expect(successValuesFromOptions).toEqual({
            const: 1,
            dependent: 2,
            lax: 1,
            lax2: 2,
            required: 1,
            required2: 1,
            virtual1: 1,
            virtual2: 1,
          });
        });

        it("should trigger 'success' listeners of props provided during updates", async () => {
          const initialData = {
            const: 1,
            dependent: 2,
            lax: 1,
            lax2: 1,
            required: 1,
            required2: 1,
          };

          // @ts-expect-error ikr
          const { data, handleSuccess } = await Model.update(initialData, {
            const1: 200,
            const2: 200,
            required: 200,
          });

          await handleSuccess?.();

          expect(data).not.toBeNull();
          expect(successValuesFromOptions).toEqual({
            const: 1,
            dependent: 1,
            required: 1,
          });

          successValuesFromOptions = {};

          {
            // @ts-expect-error ikr
            const { data, handleSuccess } = await Model.update(initialData, {
              virtual1: 200,
            });

            await handleSuccess?.();

            expect(data).not.toBeNull();
            expect(successValuesFromOptions).toEqual({
              dependent: 2,
              required2: 1,
              virtual1: 1,
              virtual2: 1,
            });
          }
        });
      });
    });
  });
});
