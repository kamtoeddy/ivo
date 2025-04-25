import { afterEach, beforeEach, describe, expect, it, test } from 'bun:test';

import { ERRORS } from '../../../dist';
import {
  getInvalidConfigMessageForRepeatedProperties,
  getInvalidPostValidateConfigMessage,
} from '../../../src/schema/schema-core';
import {
  expectFailure,
  expectNoFailure,
  getValidSchema,
  validator,
} from '../_utils';

export const Test_SchemaOptionPostValidate = ({ Schema, fx }: any) => {
  describe('Schema.options.postValidate', () => {
    describe('signature', () => {
      describe('single config', () => {
        describe('valid', () => {
          it("should allow 'postValidate' as valid config", () => {
            const validators = [
              () => {},
              [() => {}, () => {}],
              [() => {}, [() => {}, () => {}]],
              [() => {}, [() => {}, () => {}], () => {}],
            ];

            for (const validator of validators) {
              const toPass = fx(getValidSchema(), {
                postValidate: {
                  properties: ['propertyName1', 'propertyName2'],
                  validator,
                },
              });

              expectNoFailure(toPass);

              toPass();
            }
          });

          it("should allow 'postValidate' if some or all the properties to post validate are virtuals", () => {
            const values = [
              { properties: ['virtual', 'propertyName2'], validator() {} },
              [{ properties: ['virtual', 'virtual2'], validator() {} }],
            ];

            for (const postValidate of values) {
              const toPass = fx(
                getValidSchema(
                  {},
                  {
                    dependent: {
                      default: '',
                      dependsOn: ['virtual', 'virtual2'],
                      resolver() {},
                    },
                    virtual: { virtual: true, validator() {} },
                    virtual2: { virtual: true, validator() {} },
                  },
                ),
                { postValidate },
              );

              expectNoFailure(toPass);

              toPass();
            }
          });
        });

        describe('invalid', () => {
          it("should reject 'postValidate' as invalid config", () => {
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
              () => {},
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
                      invalidPostValidateConfigMessage,
                    ]),
                  },
                });
              }
            }
          });

          it("should reject 'postValidate' has never repeated properties", () => {
            const toFail = fx(
              getValidSchema(
                {},
                {
                  p1: { default: true, validator() {} },
                  p2: { default: true, validator() {} },
                },
              ),
              {
                postValidate: {
                  properties: [
                    'propertyName1',
                    'propertyName2',
                    'propertyName1',
                  ],
                  validator: () => {},
                },
              },
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: {
                  postValidate: expect.arrayContaining([
                    getInvalidPostValidateConfigMessage(
                      undefined,
                      'properties-array-must-contain-unique-values',
                    ),
                  ]),
                },
              });
            }
          });

          describe("should reject if 'properties' is not an array or does not contain valid input keys of schema", () => {
            const commonError = [
              '"properties" must be an array of at least 2 input properties of your schema',
            ];

            const values = [
              [-1, commonError],
              [0, commonError],
              [1, commonError],
              [null, commonError],
              [undefined, commonError],
              [true, commonError],
              [false, commonError],
              ['', commonError],
              ['invalid', commonError],
              [{}, commonError],
              [{ properties: [] }, commonError],
              [{ validator: [] }, commonError],
              [() => {}, commonError],
              [[], commonError],
              [['lol'], commonError],
              [
                ['lol', 'lolol'],
                [
                  '"lol" cannot be post-validated',
                  '"lolol" cannot be post-validated',
                ],
              ],
              [
                ['propertyName1', 'lolol'],
                ['"lolol" cannot be post-validated'],
              ],
              [['propertyName1', 'propertyName1'], commonError],
              [
                ['propertyName1', 'propertyName2', 'lol'],
                ['"lol" cannot be post-validated'],
              ],
              [
                ['propertyName1', 'dependent'],
                ['"dependent" cannot be post-validated'],
              ],
            ] as const;

            test.each(values)('', (properties, errors) => {
              const toFail = fx(
                getValidSchema(
                  {},
                  {
                    dependent: {
                      default: '',
                      dependsOn: 'propertyName1',
                      resolver() {},
                    },
                  },
                ),
                { postValidate: { properties, validator() {} } },
              );

              expectFailure(toFail);

              try {
                toFail();
              } catch (err: any) {
                expect(err).toMatchObject({
                  message: ERRORS.INVALID_SCHEMA,
                  payload: { postValidate: expect.arrayContaining(errors) },
                });
              }
            });
          });

          it("should reject if 'validator' is not a function or array", () => {
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
            ];

            for (const validator of values) {
              const toFail = fx(
                getValidSchema(
                  {},
                  {
                    dependent: {
                      default: '',
                      dependsOn: 'propertyName1',
                      resolver() {},
                    },
                  },
                ),
                {
                  postValidate: {
                    properties: ['propertyName1', 'propertyName2'],
                    validator,
                  },
                },
              );

              expectFailure(toFail);

              try {
                toFail();
              } catch (err: any) {
                expect(err).toMatchObject({
                  message: ERRORS.INVALID_SCHEMA,
                  payload: {
                    postValidate: expect.arrayContaining([
                      '"validator" must be a function or array of functions',
                    ]),
                  },
                });
              }
            }
          });

          it('should reject if config has never extra property', () => {
            const toFail = fx(getValidSchema(), {
              postValidate: {
                properties: ['propertyName1', 'propertyName2'],
                validator() {},
                lol: true,
              },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: {
                  postValidate: expect.arrayContaining([
                    'Config must be an object with keys "properties" and "validator" or an array of "PostValidateConfig"',
                  ]),
                },
              });
            }
          });

          describe('validator array', () => {
            it('should reject if array is empty', () => {
              const toFail = fx(
                getValidSchema(
                  {},
                  {
                    dependent: {
                      default: '',
                      dependsOn: 'propertyName1',
                      resolver() {},
                    },
                  },
                ),
                {
                  postValidate: {
                    properties: ['propertyName1', 'propertyName2'],
                    validator: [],
                  },
                },
              );

              expectFailure(toFail);

              try {
                toFail();
              } catch (err: any) {
                expect(err).toMatchObject({
                  message: ERRORS.INVALID_SCHEMA,
                  payload: {
                    postValidate: expect.arrayContaining([
                      '"validator" cannot be an empty array',
                    ]),
                  },
                });
              }
            });

            it('should reject if any of the validators is not a function', () => {
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
                // [],
              ];

              const toFail = fx(
                getValidSchema(
                  {},
                  {
                    dependent: {
                      default: '',
                      dependsOn: 'propertyName1',
                      resolver() {},
                    },
                  },
                ),
                {
                  postValidate: {
                    properties: ['propertyName1', 'propertyName2'],
                    validator: values,
                  },
                },
              );

              expectFailure(toFail);

              try {
                toFail();
              } catch (err: any) {
                expect(err).toMatchObject({
                  message: ERRORS.INVALID_SCHEMA,
                  payload: {
                    postValidate: expect.arrayContaining(
                      values.map(
                        (_, i) =>
                          `"validator" at index ${i} must be a function or array of functions`,
                      ),
                    ),
                  },
                });
              }
            });
          });
        });
      });

      describe('multiple configs', () => {
        const validConfigs = [
          { properties: ['propertyName1', 'propertyName2'], validator() {} },
          { properties: ['virtual', 'virtual2'], validator() {} },
          { properties: ['propertyName1', 'virtual2'], validator() {} },
          { properties: ['virtual', 'propertyName2'], validator() {} },
          {
            properties: ['propertyName1', 'propertyName2', 'virtual'],
            validator() {},
          },
          {
            properties: [
              'propertyName1',
              'propertyName2',
              'virtual',
              'virtual2',
            ],
            validator() {},
          },
        ];

        describe('valid', () => {
          it("should allow 'postValidate' as an array of subset and non-subset valid configs", () => {
            const configs = [
              ['propertyName1', 'p1', 'p3'],
              ['propertyName1', 'p1'],
              ['propertyName1', 'p3'],
              ['p1', 'p3'],
              ['propertyName1', 'p1', 'p2'],
              ['p1', 'p2', 'p3'],
              ['v1', 'v2'],
              ['p1', 'v2'],
              ['v1', 'p2'],
            ];

            const toPass = fx(
              getValidSchema(
                {},
                {
                  dependent: {
                    default: '',
                    dependsOn: ['v1', 'v2'],
                    resolver() {},
                  },
                  p1: { default: true },
                  p2: { default: true },
                  p3: { default: true },
                  v1: { virtual: true, validator() {} },
                  v2: { virtual: true, validator() {} },
                },
              ),
              {
                postValidate: configs.map((properties, i) => ({
                  properties,
                  validator: i % 2 === 0 ? validator : [validator, validator],
                })),
              },
            );

            expectNoFailure(toPass);

            toPass();
          });
        });

        describe('invalid', () => {
          it("should reject 'postValidate' as invalid config", () => {
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
              { validator: [] },
              () => {},
            ];

            const reasons = configs.map((_, i) =>
              getInvalidPostValidateConfigMessage(i),
            );

            const toFail = fx(getValidSchema(), { postValidate: configs });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: {
                  postValidate: expect.arrayContaining(reasons),
                },
              });
            }
          });

          it("should reject 'postValidate' has never repeated properties", () => {
            const configs = [
              {
                properties: ['propertyName1', 'propertyName2', 'propertyName2'],
                validator,
              },
              { properties: ['p1', 'p1', 'p2'], validator },
            ];

            const reasons = configs.map((_, i) =>
              getInvalidPostValidateConfigMessage(
                i,
                'properties-array-must-contain-unique-values',
              ),
            );

            const toFail = fx(
              getValidSchema(
                {},
                {
                  p1: { default: true, validator() {} },
                  p2: { default: true, validator() {} },
                },
              ),
              { postValidate: configs },
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: { postValidate: expect.arrayContaining(reasons) },
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
              { validator: [] },
              () => {},
              [],
              ['lol'],
              ['propertyName1', 'propertyName1'],
            ].map((properties) => ({ properties, validator() {} }));

            const reasons = configs.map((_, i) =>
              getInvalidPostValidateConfigMessage(
                i,
                'properties-must-be-input-array',
              ),
            );

            const toFail = fx(
              getValidSchema(
                {},
                {
                  dependent: {
                    default: '',
                    dependsOn: 'propertyName1',
                    resolver() {},
                  },
                },
              ),
              { postValidate: configs },
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: { postValidate: expect.arrayContaining(reasons) },
              });
            }
          });

          it("should reject if 'properties' of any config a property that cannot be post-validated", () => {
            const values = [
              [
                ['lol', 'lolol'],
                [
                  'Config at index 0: "lol" cannot be post-validated',
                  'Config at index 0: "lolol" cannot be post-validated',
                ],
              ],
              [
                ['propertyName1', 'lolol'],
                ['Config at index 1: "lolol" cannot be post-validated'],
              ],
              [
                ['propertyName1', 'propertyName2', 'lol'],
                ['Config at index 2: "lol" cannot be post-validated'],
              ],
              [
                ['propertyName1', 'dependent'],
                ['Config at index 3: "dependent" cannot be post-validated'],
              ],
            ] as const;

            // @ts-ignore
            const { configs, errors } = values.reduce(
              // @ts-ignore
              (acc, [properties, err]) => {
                return {
                  configs: [...acc.configs, { properties, validator() {} }],
                  errors: [...acc.errors, ...err],
                };
              },
              { configs: [], errors: [] },
            );

            const toFail = fx(
              getValidSchema(
                {},
                {
                  dependent: {
                    default: '',
                    dependsOn: 'propertyName1',
                    resolver() {},
                  },
                },
              ),
              { postValidate: configs },
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: { postValidate: expect.arrayContaining(errors) },
              });
            }
          });

          it("should reject if 'validator' are not functions or arrays", () => {
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
            ].map((validator) => ({
              validator,
              properties: ['propertyName1', 'propertyName2'],
            }));

            const reasons = configs.map((_, i) =>
              getInvalidPostValidateConfigMessage(
                i,
                'validator-must-be-function',
              ),
            );

            const toFail = fx(
              getValidSchema(
                {},
                {
                  dependent: {
                    default: '',
                    dependsOn: 'propertyName1',
                    resolver() {},
                  },
                },
              ),
              { postValidate: configs },
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: { postValidate: expect.arrayContaining(reasons) },
              });
            }
          });

          it('should reject if config have any extra properties', () => {
            const configs = [
              {
                properties: ['propertyName1', 'propertyName2'],
                validator() {},
                lol: true,
              },
              {
                properties: ['propertyName1', 'propertyName2'],
                validator() {},
                hey: true,
              },
            ];

            const reasons = configs.map((_, i) =>
              getInvalidPostValidateConfigMessage(i),
            );

            const toFail = fx(getValidSchema(), { postValidate: configs });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: {
                  postValidate: expect.arrayContaining(reasons),
                },
              });
            }
          });

          it('should reject if some configs have the same properties in any order', () => {
            const configs = [
              // valid
              ...validConfigs.map((c, i) => [c, i]),

              // invalid because they're repeated
              ...validConfigs.map((c, i) => [c, i]),

              // invalid because they're re-arranged
              [
                {
                  properties: ['propertyName2', 'propertyName1'],
                  validator() {},
                },
                0,
              ],
              [{ properties: ['virtual2', 'virtual'], validator() {} }, 1],
              [
                { properties: ['virtual2', 'propertyName1'], validator() {} },
                2,
              ],
              [
                {
                  properties: ['propertyName2', 'propertyName1', 'virtual'],
                  validator() {},
                },
                4,
              ],
              [
                {
                  properties: ['propertyName1', 'virtual', 'propertyName2'],
                  validator() {},
                },
                4,
              ],
            ] as [any, number][];

            const length = validConfigs.length;

            const reasons = configs
              .slice(length)
              .map((ci, i) =>
                getInvalidConfigMessageForRepeatedProperties(i + length, ci[1]),
              );

            const toFail = fx(
              getValidSchema(
                {},
                {
                  dependent: {
                    default: '',
                    dependsOn: ['virtual', 'virtual2'],
                    resolver() {},
                  },
                  virtual: { virtual: true, validator() {} },
                  virtual2: { virtual: true, validator() {} },
                },
              ),
              { postValidate: configs.map((ci) => ci[0]) },
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: {
                  postValidate: expect.arrayContaining(reasons),
                },
              });
            }
          });

          describe('validator array', () => {
            it('should reject if array is empty', () => {
              const toFail = fx(
                getValidSchema(
                  {},
                  {
                    dependent: {
                      default: '',
                      dependsOn: 'propertyName1',
                      resolver() {},
                    },
                    lax: {
                      default: '',
                    },
                  },
                ),
                {
                  postValidate: [
                    {
                      properties: ['propertyName1', 'propertyName2'],
                      validator: [validator],
                    },
                    {
                      properties: ['propertyName1', 'lax'],
                      validator: [],
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
                    postValidate: expect.arrayContaining([
                      'Config at index 1:  "validator" cannot be an empty array',
                    ]),
                  },
                });
              }
            });

            it('should reject if any of the validators is not a function', () => {
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
              ];

              const toFail = fx(
                getValidSchema(
                  {},
                  {
                    dependent: {
                      default: '',
                      dependsOn: 'propertyName1',
                      resolver() {},
                    },
                  },
                ),
                {
                  postValidate: [
                    {
                      properties: ['propertyName1', 'propertyName2'],
                      validator,
                    },
                    {
                      properties: ['propertyName1', 'propertyName2'],
                      validator: values,
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
                    postValidate: expect.arrayContaining(
                      values.map(
                        (_, i) =>
                          `Config at index 1:  "validator" at index ${i} must be a function or array of functions`,
                      ),
                    ),
                  },
                });
              }
            });
          });
        });
      });
    });

    describe('behaviour', () => {
      describe('should properly trigger post-validators', () => {
        let providedPropertiesStats = {} as never;
        let summaryStats = {} as never;

        function handlePostValidate(
          prop: string,
          summary: any,
          propsProvided: string[],
        ) {
          summaryStats[prop] = summary;

          if (propsProvided.includes(prop))
            providedPropertiesStats[prop] =
              (providedPropertiesStats[prop] ?? 0) + 1;
        }

        function makePostValidationConfig(properties: string[]) {
          return {
            properties,
            validator(summary: any, propsProvided: string[]) {
              for (const prop of properties)
                handlePostValidate(prop, summary, propsProvided);
            },
          };
        }

        afterEach(() => {
          summaryStats = {};
          providedPropertiesStats = {};
        });

        describe('behaviour with single post-validators', () => {
          const Model = new Schema(
            {
              dependent: {
                default: '',
                dependsOn: ['virtual', 'virtual2'],
                resolver: validator,
              },
              lax: { default: '' },
              readonly: { readonly: true, validator },
              readonlyLax: { default: '', readonly: 'lax', validator },
              required: { required: true, validator },
              virtual: { virtual: true, validator },
              virtual2: { virtual: true, validator },
            },
            {
              postValidate: makePostValidationConfig([
                'lax',
                'required',
                'readonly',
                'readonlyLax',
                'virtual',
                'virtual2',
              ]),
            },
          ).getModel();

          it('should trigger all post-validators at creation', async () => {
            const { error } = await Model.create();

            expect(error).toBeNull();
            expect(providedPropertiesStats).toEqual({
              lax: 1,
              required: 1,
              readonly: 1,
              readonlyLax: 1,
            });
          });

          it('should not trigger post-validators of virtuals not provided at creation', async () => {
            const { error } = await Model.create({ virtual2: true });

            expect(error).toBeNull();
            expect(providedPropertiesStats).toEqual({
              lax: 1,
              required: 1,
              readonly: 1,
              readonlyLax: 1,
              virtual2: 1,
            });
          });

          it('should only trigger post-validators of props that change during updates', async () => {
            const { error } = await Model.update(
              { lax: 2, required: 1, readonly: 1, readonlyLax: 1 },
              {
                lax: true,
                required: 1,
                readonly: true,
                readonlyLax: true,
                virtual: true,
                virtual2: true,
              },
            );

            expect(error).toBeNull();
            expect(providedPropertiesStats).toEqual({
              lax: 1,
              virtual: 1,
              virtual2: 1,
            });
          });

          it('should only trigger post-validators of readonly props that have not changed during updates', async () => {
            const { error } = await Model.update(
              { lax: 2, required: 1, readonly: 1, readonlyLax: '' },
              {
                readonly: true,
                readonlyLax: true,
              },
            );

            expect(error).toBeNull();
            expect(providedPropertiesStats).toEqual({ readonlyLax: 1 });
          });

          describe('behaviour with post-validators that have validator arrays', () => {
            const properties = [
              'lax',
              'required',
              'readonly',
              'readonlyLax',
              'virtual',
              'virtual2',
            ];

            const Model = new Schema(
              {
                dependent: {
                  default: '',
                  dependsOn: ['virtual', 'virtual2'],
                  resolver: validator,
                },
                lax: { default: '' },
                readonly: { readonly: true, validator },
                readonlyLax: { default: '', readonly: 'lax', validator },
                required: { required: true, validator },
                virtual: { virtual: true, validator },
                virtual2: { virtual: true, validator },
              },
              {
                postValidate: {
                  properties,
                  validator: [makePostValidator(), makePostValidator()],
                },
              },
            ).getModel();

            function makePostValidator() {
              return (summary: any, propsProvided: string[]) => {
                for (const prop of properties)
                  handlePostValidate(prop, summary, propsProvided);
              };
            }

            it('should trigger all post-validators at creation', async () => {
              const { error } = await Model.create();

              expect(error).toBeNull();
              expect(providedPropertiesStats).toEqual({
                lax: 2,
                required: 2,
                readonly: 2,
                readonlyLax: 2,
              });
            });

            it('should not trigger post-validators of virtuals not provided at creation', async () => {
              const { error } = await Model.create({ virtual2: true });

              expect(error).toBeNull();
              expect(providedPropertiesStats).toEqual({
                lax: 2,
                required: 2,
                readonly: 2,
                readonlyLax: 2,
                virtual2: 2,
              });
            });

            it('should only trigger post-validators of props that change during updates', async () => {
              const { error } = await Model.update(
                { lax: 2, required: 1, readonly: 1, readonlyLax: 1 },
                {
                  lax: true,
                  required: 1,
                  readonly: true,
                  readonlyLax: true,
                  virtual: true,
                  virtual2: true,
                },
              );

              expect(error).toBeNull();
              expect(providedPropertiesStats).toEqual({
                lax: 2,
                virtual: 2,
                virtual2: 2,
              });
            });

            it('should only trigger post-validators of readonly props that have not changed during updates', async () => {
              const { error } = await Model.update(
                { lax: 2, required: 1, readonly: 1, readonlyLax: '' },
                { readonly: true, readonlyLax: true },
              );

              expect(error).toBeNull();
              expect(providedPropertiesStats).toEqual({ readonlyLax: 2 });
            });
          });
        });

        describe('behaviour with multiple post-validators', () => {
          const Model = new Schema(
            {
              dependent: {
                default: '',
                dependsOn: ['virtual', 'virtual2'],
                resolver: validator,
              },
              lax: { default: '' },
              readonly: { readonly: true, validator },
              readonlyLax: { default: '', readonly: 'lax', validator },
              required: { required: true, validator },
              virtual: { virtual: true, validator },
              virtual2: { virtual: true, validator },
            },
            {
              postValidate: [
                makePostValidationConfig([
                  'lax',
                  'required',
                  'readonly',
                  'readonlyLax',
                ]),
                makePostValidationConfig(['lax', 'virtual']),
                makePostValidationConfig(['virtual', 'virtual2']),
              ],
            },
          ).getModel();

          afterEach(() => {
            summaryStats = {};
            providedPropertiesStats = {};
          });

          it('should trigger all post-validators at creation', async () => {
            const { error } = await Model.create();

            expect(error).toBeNull();
            expect(providedPropertiesStats).toEqual({
              lax: 2,
              required: 1,
              readonly: 1,
              readonlyLax: 1,
            });
          });

          it('should not trigger post-validators of virtuals not provided at creation', async () => {
            const { error } = await Model.create({ virtual2: true });

            expect(error).toBeNull();
            expect(providedPropertiesStats).toEqual({
              lax: 2,
              required: 1,
              readonly: 1,
              readonlyLax: 1,
              virtual2: 1,
            });
          });

          it('should only trigger post-validators of props provided during updates', async () => {
            const { error } = await Model.update(
              { lax: 2, required: 1, readonly: 1, readonlyLax: 1 },
              {
                lax: true,
                required: true,
                readonly: true,
                readonlyLax: true,
                virtual: true,
                virtual2: true,
              },
            );

            expect(error).toBeNull();
            expect(providedPropertiesStats).toEqual({
              lax: 2,
              required: 1,
              virtual: 2,
              virtual2: 1,
            });
          });

          it('should only trigger post-validators of readonly props that have not changed during updates', async () => {
            const { error } = await Model.update(
              { lax: 2, required: 1, readonly: 1, readonlyLax: '' },
              {
                readonly: true,
                readonlyLax: true,
              },
            );

            expect(error).toBeNull();
            expect(providedPropertiesStats).toEqual({ readonlyLax: 1 });
          });

          describe('behaviour with post-validators that have validator arrays', () => {
            const properties1 = ['lax', 'required', 'readonly', 'readonlyLax'],
              properties2 = ['virtual', 'virtual2'];

            const Model = new Schema(
              {
                dependent: {
                  default: '',
                  dependsOn: ['virtual', 'virtual2'],
                  resolver: validator,
                },
                lax: { default: '' },
                readonly: { readonly: true, validator },
                readonlyLax: { default: '', readonly: 'lax', validator },
                required: { required: true, validator },
                virtual: { virtual: true, validator },
                virtual2: { virtual: true, validator },
              },
              {
                postValidate: [
                  {
                    properties: properties1,
                    validator: [
                      makePostValidator(properties1),
                      makePostValidator(properties1),
                    ],
                  },
                  makePostValidationConfig(['lax', 'virtual']),
                  {
                    properties: properties2,
                    validator: [
                      makePostValidator(properties2),
                      makePostValidator(properties2),
                      makePostValidator(properties2),
                    ],
                  },
                ],
              },
            ).getModel();

            function makePostValidator(properties: string[]) {
              return (summary: any, propsProvided: string[]) => {
                for (const prop of properties)
                  handlePostValidate(prop, summary, propsProvided);
              };
            }

            it('should trigger all post-validators at creation', async () => {
              const { error } = await Model.create();

              expect(error).toBeNull();
              expect(providedPropertiesStats).toEqual({
                lax: 3,
                required: 2,
                readonly: 2,
                readonlyLax: 2,
              });
            });

            it('should not trigger post-validators of virtuals not provided at creation', async () => {
              const { error } = await Model.create({ virtual2: true });

              expect(error).toBeNull();
              expect(providedPropertiesStats).toEqual({
                lax: 3,
                required: 2,
                readonly: 2,
                readonlyLax: 2,
                virtual2: 3,
              });
            });

            it('should only trigger post-validators of props provided during updates', async () => {
              const { error } = await Model.update(
                { lax: 2, required: 1, readonly: 1, readonlyLax: 1 },
                {
                  lax: true,
                  required: true,
                  readonly: true,
                  readonlyLax: true,
                  virtual: true,
                  virtual2: true,
                },
              );

              expect(error).toBeNull();
              expect(providedPropertiesStats).toEqual({
                lax: 3,
                required: 2,
                virtual: 4,
                virtual2: 3,
              });
            });

            it('should only trigger post-validators of readonly props that have not changed during updates', async () => {
              const { error } = await Model.update(
                { lax: 2, required: 1, readonly: 1, readonlyLax: '' },
                {
                  readonly: true,
                  readonlyLax: true,
                },
              );

              expect(error).toBeNull();
              expect(providedPropertiesStats).toEqual({ readonlyLax: 2 });
            });
          });
        });
      });

      describe('values returned from post-validators should be handled accordingly', () => {
        it('should ignore non-object-like values', async () => {
          const values = [-1, 0, 1, '', 'lol', undefined, null, () => {}, []];

          for (const value of values) {
            const Model = new Schema(
              {
                p1: { default: '' },
                p2: { default: '' },
              },
              {
                postValidate: {
                  properties: ['p1', 'p2'],
                  validator: () => value,
                },
              },
            ).getModel();

            const { data, error } = await Model.create();

            expect(error).toBeNull();
            expect(data).toEqual({ p1: '', p2: '' });

            const updates = { p1: 'updated', p2: 'updated' };

            const { data: updated, error: error2 } = await Model.update(
              data,
              updates,
            );

            expect(error2).toBeNull();
            expect(updated).toEqual(updates);
          }
        });

        it('should respect errors returned in post-validators(sync & async)', async () => {
          const resolver = ({ context }: any) => context.v;

          const Model = new Schema(
            {
              p1: { default: '' },
              p2: { default: '' },
              p3: { default: '' },
              p4: { default: '' },
              d1: { default: '', dependsOn: 'v', resolver },
              d2: { default: '', dependsOn: 'v', resolver },
              v: { alias: 'd1', virtual: true, validator },
            },
            {
              postValidate: [
                {
                  properties: ['p1', 'v'],
                  validator({ context: { v }, isUpdate }: any) {
                    if (v === 'allow') return;

                    if (v === 'throw') throw new Error('lol');

                    return isUpdate
                      ? { d1: 'lolz' }
                      : {
                          p2: 'p2',
                          p3: 'error1',
                          p4: null,
                          v: { reason: 'error', metadata: { lol: true } },
                        };
                  },
                },
                {
                  properties: ['p1', 'p2'],
                  validator: ({ context: { v } }: any) => {
                    if (v === 'throw') throw new Error('lol');

                    return Promise.resolve(
                      v === 'allow' ? false : { p1: 'failed to validate' },
                    );
                  },
                },
              ],
            },
          ).getModel();

          const res = await Model.create();

          expect(res.data).toBeNull();
          expect(res.error.payload).toMatchObject({
            p1: expect.objectContaining({
              reason: 'failed to validate',
            }),
            // p2: expect.objectContaining({
            //   reason: "p2",
            // }),
            // p3: expect.objectContaining({
            //   reason: "error1",
            // }),
            // p4: expect.objectContaining({
            //   reason: "validation failed",
            // }),
            // v: expect.objectContaining({
            //   reason: "error",
            //   metadata: { lol: true },
            // }),
          });

          const res2 = await Model.update({}, { p2: 'updated', v: 'updated' });
          expect(res2.data).toBeNull();
          expect(res2.error.payload).toMatchObject({
            p1: expect.objectContaining({ reason: 'failed to validate' }),
            d1: expect.objectContaining({ reason: 'lolz' }),
          });

          const res3 = await Model.create({ v: 'allow' });
          expect(res3.error).toBeNull();
          expect(res3.data).toEqual({
            p1: '',
            p2: '',
            p3: '',
            p4: '',
            d1: 'allow',
            d2: 'allow',
          });

          const res4 = await Model.update(res3.data, {
            p1: 'data',
            v: 'allow',
          });
          expect(res4.error).toBeNull();
          expect(res4.data).toEqual({ p1: 'data' });

          const res5 = await Model.create({
            p1: 'provided',
            p2: 'provided',
            p4: 'provided',
            v: 'throw',
          });
          expect(res5.data).toBeNull();
          expect(Object.keys(res5.error.payload).length).toBe(3);
          expect(res5.error.payload).toMatchObject({
            p1: expect.objectContaining({ reason: 'validation failed' }),
            p2: expect.objectContaining({ reason: 'validation failed' }),
            v: expect.objectContaining({ reason: 'validation failed' }),
          });

          const res6 = await Model.update(
            {},
            { p1: 'updated', p2: 'updated', v: 'throw' },
          );
          expect(res6.data).toBeNull();
          expect(Object.keys(res6.error.payload).length).toBe(3);
          expect(res6.error.payload).toEqual({
            p1: expect.objectContaining({
              reason: 'validation failed',
            }),
            p2: expect.objectContaining({
              reason: 'validation failed',
            }),
            v: expect.objectContaining({
              reason: 'validation failed',
            }),
          });

          const res7 = await Model.create({ d1: 'throw' });
          expect(res7.data).toBeNull();
          expect(Object.keys(res7.error.payload).length).toBe(1);
          expect(res7.error.payload).toMatchObject({
            d1: expect.objectContaining({ reason: 'validation failed' }),
          });

          const res8 = await Model.update({}, { d1: 'throw' });
          expect(res8.data).toBeNull();
          expect(Object.keys(res8.error.payload).length).toBe(1);
          expect(res8.error.payload).toEqual({
            d1: expect.objectContaining({ reason: 'validation failed' }),
          });
        });

        it('should properly update revalidated values returned from post-validators', async () => {
          const Model = new Schema(
            {
              p1: { default: '' },
              p2: { default: '' },
            },
            {
              postValidate: {
                properties: ['p1', 'p2'],
                validator: ({ isUpdate }) =>
                  isUpdate
                    ? {
                        p1: { validated: 're updated' },
                        p2: { validated: 'also re updated' },
                      }
                    : {
                        p1: { validated: true },
                        p2: { validated: 'also revalidated' },
                      },
              },
            },
          ).getModel();

          const { data, error } = await Model.create();

          expect(error).toBeNull();
          expect(data).toEqual({ p1: true, p2: 'also revalidated' });

          const updates = { p1: 'updated', p2: 'updated' };

          const { data: updated, error: error2 } = await Model.update(
            data,
            updates,
          );

          expect(error2).toBeNull();
          expect(updated).toEqual({
            p1: 're updated',
            p2: 'also re updated',
          });
        });

        it('should properly update revalidated values returned from post-validators with virtuals', async () => {
          const Model = new Schema(
            {
              p1: { default: '' },
              p2: { default: '' },
              dependent: {
                default: '',
                dependsOn: ['v', 'v1'],
                resolver: ({ context: { v, v1 } }) => `${v} ${v1}`,
              },
              v: { virtual: true, validator: () => true },
              v1: { virtual: true, validator: () => true },
            },
            {
              postValidate: {
                properties: ['v', 'v1'],
                validator: ({ isUpdate }) =>
                  isUpdate
                    ? {
                        v: { validated: 're updated' },
                        v1: { validated: 'also re updated' },
                      }
                    : {
                        v: { validated: true },
                        v1: { validated: 'also revalidated' },
                      },
              },
            },
          ).getModel();

          const { data, error } = await Model.create({ v1: false });

          expect(error).toBeNull();
          expect(data).toMatchObject({ dependent: 'true also revalidated' });

          const updates = { v: true };

          const { data: updated, error: error2 } = await Model.update(
            data,
            updates,
          );

          expect(error2).toBeNull();
          expect(updated).toEqual({
            dependent: 're updated also re updated',
          });
        });

        it('should not revalidate props not related to validator', async () => {
          const Model = new Schema(
            {
              p1: { default: '' },
              p2: { default: '' },
              dependent: {
                default: '',
                dependsOn: ['v', 'v1'],
                resolver: ({ context: { v, v1 } }) => `${v} ${v1}`,
              },
              v: { virtual: true, validator: () => true },
              v1: { virtual: true, validator: () => true },
            },
            {
              postValidate: [
                {
                  properties: ['v', 'v1'],
                  validator: ({ isUpdate }) =>
                    isUpdate
                      ? {
                          v: { validated: 're updated' },
                          v1: { validated: 'also re updated' },
                        }
                      : {
                          v: { validated: true },
                          v1: { validated: 'also revalidated' },
                        },
                },
                {
                  properties: ['p1', 'p2', 'v'],
                  validator: ({ isUpdate }) =>
                    isUpdate
                      ? {
                          p2: { validated: 're updated' },
                          v: { validated: 'successfully re updated' },
                          v1: { validated: 'also re updated' },
                        }
                      : {
                          p1: { validated: true },
                          p2: { validated: 'also revalidated' },
                          v1: { validated: 'also revalidated' },
                        },
                },
              ],
            },
          ).getModel();

          const { data, error } = await Model.create();

          expect(error).toBeNull();
          expect(data).toEqual({
            dependent: '',
            p1: true,
            p2: 'also revalidated',
          });

          const updates = { p1: false };

          const { data: updated, error: error2 } = await Model.update(
            data,
            updates,
          );

          expect(error2).toBeNull();
          expect(updated).toEqual({
            p1: false,
            p2: 're updated',
            dependent: 'successfully re updated undefined',
          });
        });

        describe('behaviour with validator array', () => {
          it('should ignore non-object-like values', async () => {
            const values = [-1, 0, 1, '', 'lol', undefined, null, () => {}, []];

            for (const value of values) {
              const Model = new Schema(
                {
                  p1: { default: '' },
                  p2: { default: '' },
                },
                {
                  postValidate: {
                    properties: ['p1', 'p2'],
                    validator: [() => value, () => value],
                  },
                },
              ).getModel();

              const { data, error } = await Model.create();

              expect(error).toBeNull();
              expect(data).toEqual({ p1: '', p2: '' });

              const updates = { p1: 'updated', p2: 'updated' };

              const { data: updated, error: error2 } = await Model.update(
                data,
                updates,
              );

              expect(error2).toBeNull();
              expect(updated).toEqual(updates);
            }
          });

          it('should process errors of first validator to return errors and stop validating', async () => {
            const resolver = ({ context }: any) => context.v;

            let validatorRunCount: Record<string, number> = {};

            function incrementValidatorCount(key: string) {
              validatorRunCount[key] = (validatorRunCount?.[key] ?? 0) + 1;
            }

            const Model = new Schema(
              {
                p1: { default: '' },
                p2: { default: '' },
                p3: { default: '' },
                p4: { default: '' },
                d1: { default: '', dependsOn: 'v', resolver },
                d2: { default: '', dependsOn: 'v', resolver },
                v: { alias: 'd1', virtual: true, validator },
              },
              {
                postValidate: [
                  {
                    properties: ['p1', 'v'],
                    validator: [
                      () => {
                        incrementValidatorCount('p1-v');
                      },
                      ({ context: { v } }: any) => {
                        if (v === 'return error')
                          return { d1: 'error returned' };
                      },
                      ({ context: { v } }: any) => {
                        incrementValidatorCount('p1-v');
                        if (v === 'throw') throw new Error('lol');
                      },
                      ({ isUpdate }: any) => {
                        incrementValidatorCount('p1-v');

                        return isUpdate
                          ? { d1: 'lolz' }
                          : {
                              p1: 'p1',
                              p2: 'p2',
                              p3: 'error1',
                              p4: null,
                              v: { reason: 'error', metadata: { lol: true } },
                            };
                      },
                    ],
                  },
                  {
                    properties: ['p1', 'p2'],
                    validator: [
                      ({ context: { v } }: any) => {
                        incrementValidatorCount('p1-p2');
                        if (v === 'throw') throw new Error('lol');
                      },
                      ({ context: { v } }: any) => {
                        incrementValidatorCount('p1-p2');
                        return Promise.resolve(
                          v === 'allow' ? false : { p1: 'failed to validate' },
                        );
                      },
                    ],
                  },
                ],
              },
            ).getModel();

            const createRes = await Model.create();

            expect(createRes.data).toBeNull();
            expect(createRes.error.payload).toMatchObject({
              p1: expect.objectContaining({ reason: 'failed to validate' }),
              // p2: expect.objectContaining({ reason: 'p2' }),
              // p3: expect.objectContaining({ reason: 'error1' }),
              // p4: expect.objectContaining({ reason: 'validation failed' }),
              // v: expect.objectContaining({
              //   reason: 'error',
              //   metadata: { lol: true },
              // }),
            });

            expect(validatorRunCount).toMatchObject({
              'p1-v': 3,
              'p1-p2': 2,
            });

            validatorRunCount = {};

            const createRes1 = await Model.create({ v: 'throw', p2: true });

            expect(createRes1.data).toBeNull();
            expect(createRes1.error.payload).toEqual({
              p2: expect.objectContaining({ reason: 'validation failed' }),
              v: expect.objectContaining({ reason: 'validation failed' }),
            });

            expect(validatorRunCount).toMatchObject({
              'p1-v': 2,
              'p1-p2': 1,
            });

            validatorRunCount = {};

            const createRes11 = await Model.create({ v: 'return error' });

            expect(createRes11.data).toBeNull();
            expect(createRes11.error.payload).toMatchObject({
              d1: expect.objectContaining({ reason: 'error returned' }),
            });

            expect(validatorRunCount).toEqual({ 'p1-v': 1, 'p1-p2': 2 });

            validatorRunCount = {};

            const updateRes = await Model.update(
              {},
              { p2: 'updated', v: 'updated' },
            );
            expect(updateRes.data).toBeNull();
            expect(updateRes.error.payload).toMatchObject({
              p1: expect.objectContaining({ reason: 'failed to validate' }),
              d1: expect.objectContaining({ reason: 'lolz' }),
            });

            expect(validatorRunCount).toMatchObject({
              'p1-v': 3,
              'p1-p2': 2,
            });

            validatorRunCount = {};

            const updateRes1 = await Model.update(
              {},
              { p2: 'updated', v: 'throw' },
            );

            expect(updateRes1.data).toBeNull();
            expect(updateRes1.error.payload).toMatchObject({
              p2: expect.objectContaining({ reason: 'validation failed' }),
              v: expect.objectContaining({ reason: 'validation failed' }),
            });

            expect(validatorRunCount).toMatchObject({
              'p1-v': 2,
              'p1-p2': 1,
            });

            validatorRunCount = {};

            const updateRes2 = await Model.update({}, { v: 'return error' });

            expect(updateRes2.data).toBeNull();
            expect(updateRes2.error.payload).toMatchObject({
              d1: expect.objectContaining({ reason: 'error returned' }),
            });

            expect(validatorRunCount).toMatchObject({ 'p1-v': 1 });
          });

          it('should process parallel and sequential validators accordingly', async () => {
            const resolver = ({ context }: any) => context.v;

            let validatorRunCount: Record<string, number> = {};

            function incrementValidatorCount(key: string) {
              validatorRunCount[key] = (validatorRunCount?.[key] ?? 0) + 1;
            }

            const Model = new Schema(
              {
                p1: { default: '' },
                p2: { default: '' },
                p3: { default: '' },
                p4: { default: '' },
                d1: { default: '', dependsOn: 'v', resolver },
                d2: { default: '', dependsOn: 'v', resolver },
                v: { alias: 'd1', virtual: true, validator },
              },
              {
                postValidate: [
                  {
                    properties: ['p1', 'v'],
                    validator: [
                      ({ context: { v } }: any) => {
                        incrementValidatorCount('p1-v');

                        if (v === 'return error')
                          return { d1: 'error returned' };

                        if (v === 'throw') throw new Error('lol');
                      },
                      [
                        ({ context: { v } }: any) => {
                          incrementValidatorCount('p1-v-parallel');

                          if (v === 'throw-parallel') throw new Error('lol');
                        },
                        ({ context: { v } }: any) => {
                          incrementValidatorCount('p1-v-parallel');

                          if (v === 'return error-parallel')
                            return { v: 'error returned' };
                        },
                      ],
                      () => {
                        incrementValidatorCount('p1-v');
                      },
                      [
                        () => {
                          incrementValidatorCount('p1-v-parallel-1');
                        },
                        () => {
                          incrementValidatorCount('p1-v-parallel-1');
                        },
                      ],
                    ],
                  },
                ],
              },
            ).getModel();

            const createRes = await Model.create();

            expect(createRes.error).toBeNull();
            expect(validatorRunCount).toMatchObject({
              'p1-v': 2,
              'p1-v-parallel': 2,
              'p1-v-parallel-1': 2,
            });

            validatorRunCount = {};

            const createRes1 = await Model.create({ v: 'throw', p2: true });

            expect(createRes1.data).toBeNull();
            expect(createRes1.error.payload).toMatchObject({
              v: expect.objectContaining({ reason: 'validation failed' }),
            });

            expect(validatorRunCount).toMatchObject({ 'p1-v': 1 });

            validatorRunCount = {};

            const createRes2 = await Model.create({ d1: 'return error' });

            expect(createRes2.data).toBeNull();
            expect(createRes2.error.payload).toMatchObject({
              d1: expect.objectContaining({ reason: 'error returned' }),
            });

            expect(validatorRunCount).toMatchObject({ 'p1-v': 1 });

            validatorRunCount = {};

            const createRes3 = await Model.create({
              v: 'throw-parallel',
              p2: true,
            });

            expect(createRes3.data).toBeNull();
            expect(createRes3.error.payload).toMatchObject({
              v: expect.objectContaining({ reason: 'validation failed' }),
            });

            expect(validatorRunCount).toMatchObject({
              'p1-v': 1,
              'p1-v-parallel': 2,
            });

            validatorRunCount = {};

            const createRes4 = await Model.create({
              v: 'return error-parallel',
              p2: true,
            });

            expect(createRes4.data).toBeNull();
            expect(createRes4.error.payload).toMatchObject({
              v: expect.objectContaining({ reason: 'error returned' }),
            });

            expect(validatorRunCount).toMatchObject({
              'p1-v': 1,
              'p1-v-parallel': 2,
            });

            validatorRunCount = {};

            const updateRes = await Model.update({}, { v: 'valid' });
            expect(updateRes.error).toBeNull();

            expect(validatorRunCount).toMatchObject({
              'p1-v': 2,
              'p1-v-parallel': 2,
              'p1-v-parallel-1': 2,
            });

            validatorRunCount = {};

            const updateRes1 = await Model.update({}, { v: 'throw' });
            expect(updateRes1.data).toBeNull();
            expect(updateRes1.error.payload).toMatchObject({
              v: expect.objectContaining({ reason: 'validation failed' }),
            });

            expect(validatorRunCount).toMatchObject({ 'p1-v': 1 });

            validatorRunCount = {};

            const updateRes2 = await Model.update({}, { d1: 'return error' });

            expect(updateRes2.data).toBeNull();
            expect(updateRes2.error.payload).toMatchObject({
              d1: expect.objectContaining({ reason: 'error returned' }),
            });

            expect(validatorRunCount).toMatchObject({ 'p1-v': 1 });

            validatorRunCount = {};

            const updateRes3 = await Model.update(
              {},
              { v: 'throw-parallel', p2: true },
            );

            expect(updateRes3.data).toBeNull();
            expect(updateRes3.error.payload).toMatchObject({
              v: expect.objectContaining({ reason: 'validation failed' }),
            });

            expect(validatorRunCount).toMatchObject({
              'p1-v': 1,
              'p1-v-parallel': 2,
            });

            validatorRunCount = {};

            const updateRes4 = await Model.update(
              {},
              { v: 'return error-parallel', p2: true },
            );

            expect(updateRes4.data).toBeNull();
            expect(updateRes4.error.payload).toMatchObject({
              v: expect.objectContaining({ reason: 'error returned' }),
            });

            expect(validatorRunCount).toMatchObject({
              'p1-v': 1,
              'p1-v-parallel': 2,
            });
          });
        });
      });

      describe('behaviour when updating ctxOptions from within post-validators', () => {
        let ctxValue: any = {};

        beforeEach(() => {
          ctxValue = {};
        });

        const Model = new Schema(
          { p1: { default: '' }, p2: { default: '' } },
          {
            postValidate: {
              properties: ['p1', 'p2'],
              validator: ({ context: { __updateOptions__ } }) => {
                __updateOptions__({ updated: true });

                return true;
              },
            },
            onSuccess({ context: { __getOptions__ } }) {
              ctxValue = __getOptions__();
            },
          },
        ).getModel();

        it('should respect ctx updates at creation', async () => {
          expect(ctxValue).toEqual({});

          const { handleSuccess } = await Model.create();

          await handleSuccess();

          expect(ctxValue).toEqual({ updated: true });
        });

        it('should respect ctx updates during updates', async () => {
          expect(ctxValue).toEqual({});

          const { handleSuccess } = await Model.update(
            { p1: '', p2: '3' },
            { p1: true },
            { initial: true },
          );

          await handleSuccess();

          expect(ctxValue).toEqual({ updated: true, initial: true });
        });

        describe('behaviour with validator array', () => {
          let ctxValue: any = {};

          beforeEach(() => {
            ctxValue = {};
          });

          const Model = new Schema(
            { p1: { default: '' }, p2: { default: '' } },
            {
              postValidate: {
                properties: ['p1', 'p2'],
                validator: [
                  ({ context: { __updateOptions__ } }) => {
                    __updateOptions__({ updated: true });

                    return true;
                  },
                  ({ context: { __updateOptions__, __getOptions__ } }) => {
                    const { updated } = __getOptions__();

                    __updateOptions__({ v2: { updated } });

                    return true;
                  },
                ],
              },
              onSuccess({ context: { __getOptions__ } }) {
                ctxValue = __getOptions__();
              },
            },
          ).getModel();

          it('should respect ctx updates at creation', async () => {
            expect(ctxValue).toEqual({});

            const { handleSuccess } = await Model.create();

            await handleSuccess();

            expect(ctxValue).toEqual({ updated: true, v2: { updated: true } });
          });

          it('should respect ctx updates during updates', async () => {
            expect(ctxValue).toEqual({});

            const { handleSuccess } = await Model.update(
              { p1: '', p2: '3' },
              { p1: true },
              { initial: true },
            );

            await handleSuccess();

            expect(ctxValue).toEqual({
              updated: true,
              initial: true,
              v2: { updated: true },
            });
          });
        });
      });
    });
  });
};
