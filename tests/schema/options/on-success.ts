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
    describe.skip('behaviour', () => {
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
    });

    describe('valid', () => {
      it("should allow valid 'onSuccess' config", () => {
        const values = [
          () => {},
          [() => {}],
          [() => {}, () => {}],
          { properties: ['propertyName1', 'propertyName2'], handler: () => {} },
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
              properties: ['laxProp', 'propertyName2', 'dependent', 'virtual'],
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
};
