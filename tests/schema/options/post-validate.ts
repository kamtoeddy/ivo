import { describe, it, expect } from 'vitest';

import { ERRORS } from '../../../dist';
import {
  getInvalidPostValidateConfigMessage,
  getInvalidPostValidateConfigMessageForRepeatedProperties
} from '../../../src/schema/schema-core';
import { expectFailure, expectNoFailure, getValidSchema } from '../_utils';

export const Test_SchemaOptionPostValidate = ({ fx }: any) => {
  describe('Schema.options.postValidate', () => {
    describe('config', () => {
      describe('single config', () => {
        describe('valid', () => {
          it("should allow 'postValidate' as valid config", () => {
            const toPass = fx(getValidSchema(), {
              postValidate: {
                properties: ['propertyName1', 'propertyName2'],
                handler() {}
              }
            });

            expectNoFailure(toPass);

            toPass();
          });

          it("should allow 'postValidate' if some or all the properties to post validate are virtuals", () => {
            const values = [
              { properties: ['virtual', 'propertyName2'], handler() {} },
              [{ properties: ['virtual', 'virtual2'], handler() {} }]
            ];

            for (const postValidate of values) {
              const toPass = fx(
                getValidSchema(
                  {},
                  {
                    dependent: {
                      default: '',
                      dependsOn: ['virtual', 'virtual2'],
                      resolver() {}
                    },
                    virtual: { virtual: true, validator() {} },
                    virtual2: { virtual: true, validator() {} }
                  }
                ),
                { postValidate }
              );

              expectNoFailure(toPass);

              toPass();
            }
          });
        });

        describe('invalid', () => {
          it("should reject 'postValidate' as invaild config", () => {
            const invalidPostValidateConfigMessage =
              getInvalidPostValidateConfigMessage();

            const configs = [
              -1,
              0,
              1,
              null,
              undefined,
              true,
              false,
              '',
              'invalid',
              {},
              { properties: [] },
              { handler: [] },
              () => {}
            ];

            for (const postValidate of configs) {
              const toFail = fx(getValidSchema(), { postValidate });

              expectFailure(toFail);

              try {
                toFail();
              } catch (err: any) {
                expect(err).toMatchObject({
                  message: ERRORS.INVALID_SCHEMA,
                  payload: {
                    postValidate: expect.arrayContaining([
                      invalidPostValidateConfigMessage
                    ])
                  }
                });
              }
            }
          });

          it("should reject if 'properties' is not an array or does not contain valid input keys of schema", () => {
            const values = [
              -1,
              0,
              1,
              null,
              undefined,
              true,
              false,
              '',
              'invalid',
              {},
              { properties: [] },
              { handler: [] },
              () => {},
              [],
              ['lol'],
              ['lol', 'lolol'],
              ['propertyName1', 'lolol'],
              ['propertyName1', 'propertyName1'],
              ['propertyName1', 'dependent']
            ];

            for (const properties of values) {
              const toFail = fx(
                getValidSchema(
                  {},
                  {
                    dependent: {
                      default: '',
                      dependsOn: 'propertyName1',
                      resolver() {}
                    }
                  }
                ),
                {
                  postValidate: { properties, handler() {} }
                }
              );

              expectFailure(toFail);

              try {
                toFail();
              } catch (err: any) {
                expect(err).toMatchObject({
                  message: ERRORS.INVALID_SCHEMA,
                  payload: {
                    postValidate: expect.arrayContaining([
                      '"properties" must be an array of at least 2 input properties of your schema'
                    ])
                  }
                });
              }
            }
          });

          it("should reject if 'handler' is not a function", () => {
            const values = [
              -1,
              0,
              1,
              null,
              undefined,
              true,
              false,
              '',
              'invalid',
              {},
              []
            ];

            for (const handler of values) {
              const toFail = fx(
                getValidSchema(
                  {},
                  {
                    dependent: {
                      default: '',
                      dependsOn: 'propertyName1',
                      resolver() {}
                    }
                  }
                ),
                {
                  postValidate: {
                    properties: ['propertyName1', 'propertyName2'],
                    handler
                  }
                }
              );

              expectFailure(toFail);

              try {
                toFail();
              } catch (err: any) {
                expect(err).toMatchObject({
                  message: ERRORS.INVALID_SCHEMA,
                  payload: {
                    postValidate: expect.arrayContaining([
                      '"handler" must be a function'
                    ])
                  }
                });
              }
            }
          });

          it('should reject if config has any extra property', () => {
            const toFail = fx(getValidSchema(), {
              postValidate: {
                properties: ['propertyName1', 'propertyName2'],
                handler() {},
                lol: true
              }
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: {
                  postValidate: expect.arrayContaining([
                    'The "postValidate" option must be an object with keys "properties" and "handler" or an array of "PostValidateConfig"'
                  ])
                }
              });
            }
          });
        });
      });

      describe('multiple configs', () => {
        const validPostValidConfig = [
          { properties: ['propertyName1', 'propertyName2'], handler() {} },
          { properties: ['virtual', 'virtual2'], handler() {} },
          { properties: ['propertyName1', 'virtual2'], handler() {} },
          { properties: ['virtual', 'propertyName2'], handler() {} },
          {
            properties: ['propertyName1', 'propertyName2', 'virtual'],
            handler() {}
          },
          {
            properties: [
              'propertyName1',
              'propertyName2',
              'virtual',
              'virtual2'
            ],
            handler() {}
          }
        ];

        describe('valid', () => {
          it("should allow 'postValidate' as an array of valid configs", () => {
            const toPass = fx(
              getValidSchema(
                {},
                {
                  dependent: {
                    default: '',
                    dependsOn: ['virtual', 'virtual2'],
                    resolver() {}
                  },
                  virtual: { virtual: true, validator() {} },
                  virtual2: { virtual: true, validator() {} }
                }
              ),
              { postValidate: validPostValidConfig }
            );

            expectNoFailure(toPass);

            toPass();
          });
        });

        describe('invalid', () => {
          it("should reject 'postValidate' as invaild config", () => {
            const configs = [
              -1,
              0,
              1,
              null,
              undefined,
              true,
              false,
              '',
              'invalid',
              {},
              { properties: [] },
              { handler: [] },
              () => {}
            ];

            const reasons = configs.map((_, i) =>
              getInvalidPostValidateConfigMessage(i)
            );

            const toFail = fx(getValidSchema(), { postValidate: configs });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: {
                  postValidate: expect.arrayContaining(reasons)
                }
              });
            }
          });

          it("should reject if 'properties' are not arrays or do not contain valid input keys of schema", () => {
            const configs = [
              -1,
              0,
              1,
              null,
              undefined,
              true,
              false,
              '',
              'invalid',
              {},
              { properties: [] },
              { handler: [] },
              () => {},
              [],
              ['lol'],
              ['lol', 'lolol'],
              ['propertyName1', 'lolol'],
              ['propertyName1', 'propertyName1'],
              ['propertyName1', 'dependent']
            ].map((properties) => ({ properties, handler() {} }));

            const reasons = configs.map((_, i) =>
              getInvalidPostValidateConfigMessage(
                i,
                'properties-must-be-input-array'
              )
            );

            const toFail = fx(
              getValidSchema(
                {},
                {
                  dependent: {
                    default: '',
                    dependsOn: 'propertyName1',
                    resolver() {}
                  }
                }
              ),
              { postValidate: configs }
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: { postValidate: expect.arrayContaining(reasons) }
              });
            }
          });

          it("should reject if 'handler' are not functions", () => {
            const configs = [
              -1,
              0,
              1,
              null,
              undefined,
              true,
              false,
              '',
              'invalid',
              {},
              []
            ].map((handler) => ({
              handler,
              properties: ['propertyName1', 'propertyName2']
            }));

            const reasons = configs.map((_, i) =>
              getInvalidPostValidateConfigMessage(i, 'handler-must-be-function')
            );

            const toFail = fx(
              getValidSchema(
                {},
                {
                  dependent: {
                    default: '',
                    dependsOn: 'propertyName1',
                    resolver() {}
                  }
                }
              ),
              { postValidate: configs }
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: { postValidate: expect.arrayContaining(reasons) }
              });
            }
          });

          it('should reject if config have any extra properties', () => {
            const configs = [
              {
                properties: ['propertyName1', 'propertyName2'],
                handler() {},
                lol: true
              },
              {
                properties: ['propertyName1', 'propertyName2'],
                handler() {},
                hey: true
              }
            ];

            const reasons = configs.map((_, i) =>
              getInvalidPostValidateConfigMessage(i)
            );

            const toFail = fx(getValidSchema(), { postValidate: configs });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: {
                  postValidate: expect.arrayContaining(reasons)
                }
              });
            }
          });

          it('should reject if some configs have the same properties in any order', () => {
            const configs = [
              // valid
              ...validPostValidConfig.map((c, i) => [c, i]),

              // invalid because they're repeated
              ...validPostValidConfig.map((c, i) => [c, i]),

              // invalid because they're re-arranged
              [
                {
                  properties: ['propertyName2', 'propertyName1'],
                  handler() {}
                },
                0
              ],
              [{ properties: ['virtual2', 'virtual'], handler() {} }, 1],
              [{ properties: ['virtual2', 'propertyName1'], handler() {} }, 2],
              [
                {
                  properties: ['propertyName2', 'propertyName1', 'virtual'],
                  handler() {}
                },
                4
              ],
              [
                {
                  properties: ['propertyName1', 'virtual', 'propertyName2'],
                  handler() {}
                },
                4
              ]
            ] as [any, number][];

            const length = validPostValidConfig.length;

            const reasons = configs
              .slice(length)
              .map((ci, i) =>
                getInvalidPostValidateConfigMessageForRepeatedProperties(
                  i + length,
                  ci[1]
                )
              );

            const toFail = fx(
              getValidSchema(
                {},
                {
                  dependent: {
                    default: '',
                    dependsOn: ['virtual', 'virtual2'],
                    resolver() {}
                  },
                  virtual: { virtual: true, validator() {} },
                  virtual2: { virtual: true, validator() {} }
                }
              ),
              { postValidate: configs.map((ci) => ci[0]) }
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: {
                  postValidate: expect.arrayContaining(reasons)
                }
              });
            }
          });
        });
      });
    });
  });
};
