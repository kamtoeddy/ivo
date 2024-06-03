import { beforeEach, describe, it, expect } from 'bun:test';

import { ERRORS } from '../../../dist';
import {
  expectFailure,
  expectNoFailure,
  getValidSchema,
  validator,
} from '../_utils';
import {
  getInvalidOnSuccessConfigMessage,
  getInvalidOnSuccessConfigMessageForRepeatedProperties,
} from '../../../src/schema/schema-core';

export const Test_SchemaOnSuccess = ({ Schema, fx }: any) => {
  describe('Schema.options.onSuccess', () => {
    describe('signature', () => {
      describe('valid', () => {
        it("should allow valid 'onSuccess' config", () => {
          const values = [
            () => {},
            [() => {}],
            [() => {}, () => {}],
            {
              properties: ['propertyName1', 'propertyName2'],
              handler: () => {},
            },
            {
              properties: ['propertyName1', 'propertyName2'],
              handler: [() => {}, () => {}],
            },
            {
              properties: [
                'constant',
                'laxProp',
                'propertyName2',
                'dependent',
                'virtual',
              ],
              handler: [() => {}, () => {}],
            },
            {
              properties: [
                'constant',
                'laxProp',
                'propertyName2',
                'dependent',
                'virtual',
              ],
              handler: () => {},
            },
            [
              () => {},
              {
                properties: ['propertyName1', 'constant'],
                handler: [() => {}, () => {}],
              },
              {
                properties: [
                  'laxProp',
                  'propertyName2',
                  'dependent',
                  'virtual',
                ],
                handler: () => {},
              },
            ],
            [
              () => {},
              {
                properties: ['propertyName1', 'propertyName1', 'constant'],
                handler: [() => {}, () => {}],
              },
              {
                properties: [
                  'laxProp',
                  'propertyName2',
                  'dependent',
                  'virtual',
                ],
                handler: () => {},
              },
            ],
          ];

          for (const onSuccess of values) {
            const toPass = fx(
              getValidSchema(
                {},
                {
                  constant: { constant: true, value: '' },
                  laxProp: { default: '' },
                  dependent: {
                    default: '',
                    dependsOn: ['laxProp', 'virtual'],
                    resolver: () => {},
                  },
                  readonly: { readonly: true, validator: () => {} },
                  virtual: { virtual: true, validator: () => {} },
                },
              ),
              {
                onSuccess,
              },
            );

            expectNoFailure(toPass);

            toPass();
          }
        });
      });

      describe('invalid', () => {
        it("should reject 'onSuccess' if it's not a function, object or array", () => {
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

          invalidValues.forEach((onSuccess) => {
            const toFail = fx(getValidSchema(), { onSuccess });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: {
                  onSuccess: expect.arrayContaining([
                    getInvalidOnSuccessConfigMessage(),
                  ]),
                },
              });
            }
          });
        });

        it("should reject 'onSuccess' if invalid properties or handlers are passed in config object", () => {
          const invalidProperties = [
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

          invalidProperties.forEach((properties) => {
            const toFail = fx(getValidSchema(), {
              onSuccess: { properties, handler: () => {} },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: {
                  onSuccess: expect.arrayContaining([
                    '"properties" must be an array of at least 2 properties or virtuals of your schema',
                  ]),
                },
              });
            }
          });

          const invalidHandlers = [
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
          invalidHandlers.forEach((handler) => {
            const toFail = fx(getValidSchema(), {
              onSuccess: {
                properties: ['propertyName1', 'propertyName2'],
                handler,
              },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: {
                  onSuccess: expect.arrayContaining([
                    '"handler" must be a function or array of functions',
                  ]),
                },
              });
            }
          });

          const invalidNestedHandlers = [
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
          const schemaWithInvalidHandlers = fx(getValidSchema(), {
            onSuccess: {
              properties: ['propertyName1', 'propertyName2'],
              handler: invalidNestedHandlers,
            },
          });

          expectFailure(schemaWithInvalidHandlers);

          try {
            schemaWithInvalidHandlers();
          } catch (err: any) {
            expect(err).toMatchObject({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                onSuccess: expect.arrayContaining(
                  invalidNestedHandlers.map((_, i) =>
                    getInvalidOnSuccessConfigMessage(
                      undefined,
                      'handler-must-be-function',
                      i,
                    ),
                  ),
                ),
              },
            });
          }
        });

        it('should reject if any of the properties passed in config object are not valid properties or virtuals', () => {
          const invalidProperties = [
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

          const schemaWithInvalidProperties = fx(getValidSchema(), {
            onSuccess: { properties: invalidProperties, handler: () => {} },
          });

          expectFailure(schemaWithInvalidProperties);

          try {
            schemaWithInvalidProperties();
          } catch (err: any) {
            expect(err).toMatchObject({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                onSuccess: expect.arrayContaining(
                  invalidProperties.map(
                    (prop) =>
                      `"${prop}" is not a property or virtual on your schema`,
                  ),
                ),
              },
            });
          }

          const schemaWithNestedInvalidProperties = fx(getValidSchema(), {
            onSuccess: [{ properties: invalidProperties, handler: () => {} }],
          });

          expectFailure(schemaWithNestedInvalidProperties);

          try {
            schemaWithNestedInvalidProperties();
          } catch (err: any) {
            expect(err).toMatchObject({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                onSuccess: expect.arrayContaining(
                  invalidProperties.map(
                    (prop) =>
                      `Config at index 0: "${prop}" is not a property or virtual on your schema`,
                  ),
                ),
              },
            });
          }
        });

        it("should reject 'onSuccess' if a property or virtual is provided in more than 1 config", () => {
          const toFail = fx(
            getValidSchema(
              {},
              {
                constant: { constant: true, value: '' },
                laxProp: { default: '' },
                dependent: {
                  default: '',
                  dependsOn: ['laxProp', 'virtual'],
                  resolver: () => {},
                },
                readonly: { readonly: true, validator: () => {} },
                virtual: { virtual: true, validator: () => {} },
              },
            ),
            {
              onSuccess: [
                {
                  properties: ['propertyName1', 'laxProp'],
                  handler: () => {},
                },
                {
                  properties: ['virtual', 'laxProp'],
                  handler: () => {},
                },
              ],
            },
          );

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err).toMatchObject({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                onSuccess: expect.arrayContaining([
                  getInvalidOnSuccessConfigMessageForRepeatedProperties(
                    'laxProp',
                    1,
                    0,
                  ),
                ]),
              },
            });
          }
        });
      });
    });

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
              onSuccess: onSuccess_('price'),
            },
            _setPrice: {
              virtual: true,
              validator,
              onSuccess: onSuccess_('_setPrice'),
            },
          },
          { onSuccess: onSuccess_('global') },
        ).getModel();

        it("should trigger all 'success' listeners at creation", async () => {
          const { data, handleSuccess } = await Book.create({
            name: 'Book name',
            _setPrice: 100,
          });

          await handleSuccess();

          const values = { id: 1, name: 'Book name', price: 100 };
          const summary = {
            changes: null,
            context: { ...values, _setPrice: 100 },
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

          const { data, handleSuccess } = await Book.update(book, {
            _setPrice: 200,
          });

          await handleSuccess();

          const values = { ...book, price: 200 };

          const summary = {
            changes: data,
            context: { ...values, _setPrice: 200 },
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
          { onSuccess: [onSuccess_('global'), onSuccess_('global-1')] },
        ).getModel();

        it("should trigger all 'success' listeners at creation", async () => {
          const { data, handleSuccess } = await Book.create({
            name: 'Book name',
            _setPrice: 100,
          });

          await handleSuccess();

          const values = { id: 1, name: 'Book name', price: 100 };
          const summary = {
            changes: null,
            context: { ...values, _setPrice: 100 },
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

          const { data, handleSuccess } = await Book.update(book, {
            _setPrice: 200,
          });

          await handleSuccess();

          const values = { ...book, price: 200 };

          const summary = {
            changes: data,
            context: { ...values, _setPrice: 200 },
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
        let successValuesFromOptions: any = {};

        beforeEach(() => {
          successValuesFromOptions = {};
        });

        function onOptionSuccess(props: string[]) {
          return () => {
            props.forEach((prop) => {
              successValuesFromOptions[prop] =
                (successValuesFromOptions[prop] ?? 0) + 1;
            });
          };
        }

        describe('constant properties', () => {
          const Model = new Schema(
            {
              const1: { constant: true, value: 1 },
              const2: { constant: true, value: 2 },
              lax: { default: true },
            },
            {
              onSuccess: {
                properties: ['const1', 'const2'],
                handler: onOptionSuccess(['const1', 'const2']),
              },
            },
          ).getModel();

          it("should trigger all 'success' listeners of constant props at creation", async () => {
            const { data, handleSuccess } = await Model.create();

            await handleSuccess();

            expect(data).not.toBeNull();
            expect(successValuesFromOptions).toEqual({
              const1: 1,
              const2: 1,
            });
          });

          it("should not trigger 'success' listeners of constant props during updates", async () => {
            const initialData = { const1: 400, const2: 400, lax: 100 };

            const { data, handleSuccess } = await Model.update(initialData, {
              const1: 200,
              const2: 200,
              lax: 200,
            });

            await handleSuccess();

            expect(data).not.toBeNull();
            expect(successValuesFromOptions).toEqual({});
          });
        });

        describe('non-constant properties', () => {
          const Model = new Schema(
            {
              const: { constant: true, value: 1 },
              lax: { default: true },
              lax2: { default: true },
              required: { required: true, validator },
              required2: { required: true, validator },
              dependent: {
                default: null,
                dependsOn: ['lax2', 'virtual1', 'virtual2'],
                resolver: validator,
                onSuccess: onSuccess_('dependent'),
              },
              virtual1: { virtual: true, validator },
              virtual2: { virtual: true, validator },
            },
            {
              onSuccess: [
                onOptionSuccess(['dependent']),
                {
                  properties: ['lax', 'lax2'],
                  handler: [
                    onOptionSuccess(['lax', 'lax2']),
                    onOptionSuccess(['lax2']),
                  ],
                },
                {
                  properties: ['virtual1', 'virtual2'],
                  handler: onOptionSuccess(['virtual1', 'virtual2']),
                },
                {
                  properties: ['required', 'const'],
                  handler: onOptionSuccess(['required', 'const']),
                },
                {
                  properties: ['required2', 'dependent'],
                  handler: onOptionSuccess(['required2', 'dependent']),
                },
              ],
            },
          ).getModel();

          it("should trigger all related 'success' listeners at creation", async () => {
            const { data, handleSuccess } = await Model.create({
              required: 100,
              required2: 100,
            });

            await handleSuccess();

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
            const { data, handleSuccess } = await Model.create({
              required: 100,
              required2: 100,
              virtual1: 4,
            });

            await handleSuccess();

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

            const { data, handleSuccess } = await Model.update(initialData, {
              const1: 200,
              const2: 200,
              required: 200,
            });

            await handleSuccess();

            expect(data).not.toBeNull();
            expect(successValuesFromOptions).toEqual({
              const: 1,
              dependent: 1,
              required: 1,
            });

            successValuesFromOptions = {};

            {
              const { data, handleSuccess } = await Model.update(initialData, {
                virtual1: 200,
              });

              await handleSuccess();

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
};
