import { describe, it, expect, afterEach } from 'vitest';

import { ERRORS } from '../../../dist';
import {
  getInvalidPostValidateConfigMessage,
  getInvalidPostValidateConfigMessageForRepeatedProperties,
  getInvalidPostValidateConfigMessageForSubsetProperties
} from '../../../src/schema/schema-core';
import {
  expectFailure,
  expectNoFailure,
  getValidSchema,
  validator
} from '../_utils';

export const Test_SchemaOptionPostValidate = ({ Schema, fx }: any) => {
  describe('Schema.options.postValidate', () => {
    describe('signature', () => {
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
          it("should allow 'postValidate' as an array of non-subset valid configs", () => {
            const configs = [
              ['propertyName1', 'p1', 'p3'],
              ['propertyName1', 'p1', 'p2'],
              ['p1', 'p2', 'p3'],
              ['v1', 'v2'],
              ['p1', 'v2'],
              ['v1', 'p2']
            ];

            const toPass = fx(
              getValidSchema(
                {},
                {
                  dependent: {
                    default: '',
                    dependsOn: ['v1', 'v2'],
                    resolver() {}
                  },
                  p1: { default: true },
                  p2: { default: true },
                  p3: { default: true },
                  v1: { virtual: true, validator() {} },
                  v2: { virtual: true, validator() {} }
                }
              ),
              {
                postValidate: configs.map((properties) => ({
                  properties,
                  handler() {}
                }))
              }
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

          it('should reject if any config is the subset of another', () => {
            const configs = [
              [
                ['p1', 'p2'],
                [3, 4, 6]
              ],
              [
                ['p1', 'v1'],
                [3, 5, 6]
              ],
              [
                ['p1', 'v2'],
                [4, 5, 6]
              ],
              [['p1', 'p2', 'v1'], [6]],
              [['p1', 'p2', 'v2'], [6]],
              [['p1', 'v1', 'v2'], [6]],
              [['p1', 'p2', 'v1', 'v2'], []],
              [
                ['p2', 'v1'],
                [3, 6, 9]
              ],
              [
                ['p2', 'v2'],
                [6, 9]
              ],
              [['p2', 'v1', 'v2'], [6]]
            ] as [string[], number[]][];

            const reasons = configs.reduce((prev, c, i) => {
              return [
                ...prev,
                ...c[1].map((i2) =>
                  getInvalidPostValidateConfigMessageForSubsetProperties(i, i2)
                )
              ];
            }, [] as string[]);

            const toFail = fx(
              getValidSchema(
                {},
                {
                  dependent: {
                    default: '',
                    dependsOn: ['v1', 'v2'],
                    resolver() {}
                  },
                  p1: { default: '' },
                  p2: { default: '' },
                  v1: { virtual: true, validator() {} },
                  v2: { virtual: true, validator() {} }
                }
              ),
              {
                postValidate: configs.map((c) => ({
                  properties: c[0],
                  handler() {}
                }))
              }
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

    describe('behaviour', () => {
      let providedPropertiesStats = {} as any;
      let summaryStats = {} as any;

      function handlePostValidate(
        prop: string,
        summary: any,
        propsProvided: string[]
      ) {
        summaryStats[prop] = summary;

        if (propsProvided.includes(prop))
          providedPropertiesStats[prop] =
            (providedPropertiesStats[prop] ?? 0) + 1;
      }

      function makePostValidator(properties: string[]) {
        return {
          properties,
          handler(summary: any, propsProvided: string[]) {
            for (const prop of properties)
              handlePostValidate(prop, summary, propsProvided);
          }
        };
      }

      describe('should properly trigger post-validators', () => {
        describe('behaviour with single post-validators', () => {
          const Model = new Schema(
            {
              dependent: {
                default: '',
                dependsOn: ['virtual', 'virtual2'],
                resolver: validator
              },
              lax: { default: '' },
              readonly: { readonly: true, validator },
              readonlyLax: { default: '', readonly: 'lax', validator },
              required: { required: true, validator },
              virtual: { virtual: true, validator },
              virtual2: { virtual: true, validator }
            },
            {
              postValidate: makePostValidator([
                'lax',
                'required',
                'readonly',
                'readonlyLax',
                'virtual',
                'virtual2'
              ])
            }
          ).getModel();

          afterEach(() => {
            summaryStats = {};
            providedPropertiesStats = {};
          });

          it('should trigger all post-validators at creation', async () => {
            const { error } = await Model.create();

            expect(error).toBeNull();
            expect(providedPropertiesStats).toEqual({
              lax: 1,
              required: 1,
              readonly: 1,
              readonlyLax: 1
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
              virtual2: 1
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
                virtual2: true
              }
            );

            expect(error).toBeNull();
            expect(providedPropertiesStats).toEqual({
              lax: 1,
              virtual: 1,
              virtual2: 1
            });
          });

          it('should only trigger post-validators of readonly props that have not changed during updates', async () => {
            const { error } = await Model.update(
              { lax: 2, required: 1, readonly: 1, readonlyLax: '' },
              {
                readonly: true,
                readonlyLax: true
              }
            );

            expect(error).toBeNull();
            expect(providedPropertiesStats).toEqual({ readonlyLax: 1 });
          });
        });

        describe('behaviour with multiple post-validators', () => {
          const Model = new Schema(
            {
              dependent: {
                default: '',
                dependsOn: ['virtual', 'virtual2'],
                resolver: validator
              },
              lax: { default: '' },
              readonly: { readonly: true, validator },
              readonlyLax: { default: '', readonly: 'lax', validator },
              required: { required: true, validator },
              virtual: { virtual: true, validator },
              virtual2: { virtual: true, validator }
            },
            {
              postValidate: [
                makePostValidator([
                  'lax',
                  'required',
                  'readonly',
                  'readonlyLax'
                ]),
                makePostValidator(['lax', 'virtual']),
                makePostValidator(['virtual', 'virtual2'])
              ]
            }
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
              readonlyLax: 1
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
              virtual2: 1
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
                virtual2: true
              }
            );

            expect(error).toBeNull();
            expect(providedPropertiesStats).toEqual({
              lax: 2,
              required: 1,
              virtual: 2,
              virtual2: 1
            });
          });

          it('should only trigger post-validators of readonly props that have not changed during updates', async () => {
            const { error } = await Model.update(
              { lax: 2, required: 1, readonly: 1, readonlyLax: '' },
              {
                readonly: true,
                readonlyLax: true
              }
            );

            expect(error).toBeNull();
            expect(providedPropertiesStats).toEqual({ readonlyLax: 1 });
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
                p2: { default: '' }
              },
              {
                postValidate: { properties: ['p1', 'p2'], handler: () => value }
              }
            ).getModel();

            const { data, error } = await Model.create();

            expect(error).toBeNull();
            expect(data).toEqual({ p1: '', p2: '' });

            const updates = { p1: 'updated', p2: 'updated' };

            const { data: updated, error: error2 } = await Model.update(
              data,
              updates
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
              v: { alias: 'd1', virtual: true, validator }
            },
            {
              postValidate: [
                {
                  properties: ['p1', 'v'],
                  handler({ context: { v }, isUpdate }: any) {
                    if (v == 'allow') return;

                    if (v == 'throw') throw new Error('lol');

                    return isUpdate
                      ? { d1: 'lolz' }
                      : {
                          p1: 'p1',
                          p2: ['p2'],
                          p3: ['error1', 'error2'],
                          p4: null,
                          v: { reason: 'error', metadata: { lol: true } }
                        };
                  }
                },
                {
                  properties: ['p1', 'p2'],
                  handler: ({ context: { v } }: any) => {
                    if (v == 'throw') throw new Error('lol');

                    return Promise.resolve(
                      v == 'allow' ? false : { p1: 'failed to validate' }
                    );
                  }
                }
              ]
            }
          ).getModel();

          const res = await Model.create();

          expect(res.data).toBeNull();
          expect(res.error.payload).toMatchObject({
            p1: expect.objectContaining({
              reasons: expect.arrayContaining(['p1', 'failed to validate'])
            }),
            p2: expect.objectContaining({
              reasons: expect.arrayContaining(['p2'])
            }),
            p3: expect.objectContaining({
              reasons: expect.arrayContaining(['error1', 'error2'])
            }),
            p4: expect.objectContaining({
              reasons: expect.arrayContaining(['validation failed'])
            }),
            v: expect.objectContaining({
              reasons: expect.arrayContaining(['error']),
              metadata: { lol: true }
            })
          });

          const res2 = await Model.update({}, { p2: 'updated', v: 'updated' });
          expect(res2.data).toBeNull();
          expect(res2.error.payload).toMatchObject({
            p1: expect.objectContaining({
              reasons: expect.arrayContaining(['failed to validate'])
            }),
            d1: expect.objectContaining({
              reasons: expect.arrayContaining(['lolz'])
            })
          });

          const res3 = await Model.create({ v: 'allow' });
          expect(res3.error).toBeNull();
          expect(res3.data).toEqual({
            p1: '',
            p2: '',
            p3: '',
            p4: '',
            d1: 'allow',
            d2: 'allow'
          });

          const res4 = await Model.update(res3.data, {
            p1: 'data',
            v: 'allow'
          });
          expect(res4.error).toBeNull();
          expect(res4.data).toEqual({ p1: 'data' });

          const res5 = await Model.create({
            p1: 'provided',
            p2: 'provided',
            p4: 'provided',
            v: 'throw'
          });
          expect(res5.data).toBeNull();
          expect(Object.keys(res5.error.payload).length).toBe(3);
          expect(res5.error.payload).toMatchObject({
            p1: expect.objectContaining({
              reasons: ['validation failed']
            }),
            p2: expect.objectContaining({
              reasons: ['validation failed']
            }),
            v: expect.objectContaining({
              reasons: ['validation failed']
            })
          });

          const res6 = await Model.update(
            {},
            { p1: 'updated', p2: 'updated', v: 'throw' }
          );
          expect(res6.data).toBeNull();
          expect(Object.keys(res6.error.payload).length).toBe(3);
          expect(res6.error.payload).toEqual({
            p1: expect.objectContaining({
              reasons: ['validation failed']
            }),
            p2: expect.objectContaining({
              reasons: ['validation failed']
            }),
            v: expect.objectContaining({
              reasons: ['validation failed']
            })
          });

          const res7 = await Model.create({ d1: 'throw' });
          expect(res7.data).toBeNull();
          expect(Object.keys(res7.error.payload).length).toBe(1);
          expect(res7.error.payload).toMatchObject({
            d1: expect.objectContaining({
              reasons: ['validation failed']
            })
          });

          const res8 = await Model.update({}, { d1: 'throw' });
          expect(res8.data).toBeNull();
          expect(Object.keys(res8.error.payload).length).toBe(1);
          expect(res8.error.payload).toEqual({
            d1: expect.objectContaining({
              reasons: ['validation failed']
            })
          });
        });
      });
    });
  });
};
