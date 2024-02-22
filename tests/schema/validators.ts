import { describe, it, expect, afterEach } from 'vitest';

import { ERRORS } from '../../dist';
import { expectFailure, expectNoFailure, validator } from './_utils';

export const Test_Validators = ({ Schema, fx }: any) => {
  describe('Validators', () => {
    describe('signature', () => {
      describe('valid', () => {
        it('should allow functions', async () => {
          const toPass = fx({
            prop: { default: '', validator },
            prop2: { required: true, validator },
            dependent: {
              default: '',
              dependsOn: 'virtual',
              resolver: () => ''
            },
            virtual: { virtual: true, validator }
          });

          expectNoFailure(toPass);

          toPass();
        });

        it('should allow 2 functions in array', async () => {
          const validatorArr = [validator, validator];

          const toPass = fx({
            prop: { default: '', validator: validatorArr },
            prop2: { required: true, validator: validatorArr },
            dependent: {
              default: '',
              dependsOn: 'virtual',
              resolver: () => ''
            },
            virtual: { virtual: true, validator: validatorArr }
          });

          expectNoFailure(toPass);

          toPass();
        });
      });

      describe('invalid', () => {
        const invalidValidators = [-1, 0, 1, true, false, null, undefined, {}];

        it('should reject non-functions', async () => {
          for (const validator of invalidValidators) {
            const toFail = fx({
              prop: { default: '', validator },
              prop2: { required: true, validator },
              dependent: {
                default: '',
                dependsOn: 'virtual',
                resolver: () => ''
              },
              virtual: { virtual: true, validator }
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err) {
              expect(err.payload).toMatchObject({
                prop: expect.arrayContaining(['Invalid validator']),
                prop2: expect.arrayContaining(['Invalid validator']),
                virtual: expect.arrayContaining(['Invalid validator'])
              });
            }
          }
        });

        describe('should reject invalid array of functions', async () => {
          const invalidValidators = [
            -1,
            0,
            1,
            true,
            false,
            null,
            undefined,
            {},
            []
          ];

          it('should reject if array provided is not of length 2', () => {
            const invalidValidators = [
              [],
              [validator],
              [validator, validator, validator]
            ];

            const message = 'Validator array must contain exactly 2 functions';

            for (const validatorArr of invalidValidators) {
              const toFail = fx({
                prop: { default: '', validator: validatorArr },
                prop2: {
                  required: true,
                  validator: validatorArr
                },
                dependent: {
                  default: '',
                  dependsOn: 'virtual',
                  resolver: () => ''
                },
                virtual: {
                  virtual: true,
                  validator: validatorArr
                }
              });

              expectFailure(toFail);

              try {
                toFail();
              } catch (err) {
                expect(err.payload).toMatchObject({
                  prop: expect.arrayContaining([message]),
                  prop2: expect.arrayContaining([message]),
                  virtual: expect.arrayContaining([message])
                });
              }
            }
          });

          it('should reject if first function is invalid', () => {
            for (const invalidValidator of invalidValidators) {
              const toFail = fx({
                prop: { default: '', validator: [invalidValidator, validator] },
                prop2: {
                  required: true,
                  validator: [invalidValidator, validator]
                },
                dependent: {
                  default: '',
                  dependsOn: 'virtual',
                  resolver: () => ''
                },
                virtual: {
                  virtual: true,
                  validator: [invalidValidator, validator]
                }
              });

              expectFailure(toFail);

              try {
                toFail();
              } catch (err) {
                expect(err.payload).toMatchObject({
                  prop: expect.arrayContaining([
                    'Validator at index 0 is invalid'
                  ]),
                  prop2: expect.arrayContaining([
                    'Validator at index 0 is invalid'
                  ]),
                  virtual: expect.arrayContaining([
                    'Validator at index 0 is invalid'
                  ])
                });
              }
            }
          });

          it('should reject if second function is invalid', () => {
            for (const invalidValidator of invalidValidators) {
              const toFail = fx({
                prop: { default: '', validator: [validator, invalidValidator] },
                prop2: {
                  required: true,
                  validator: [validator, invalidValidator]
                },
                dependent: {
                  default: '',
                  dependsOn: 'virtual',
                  resolver: () => ''
                },
                virtual: {
                  virtual: true,
                  validator: [validator, invalidValidator]
                }
              });

              expectFailure(toFail);

              try {
                toFail();
              } catch (err) {
                expect(err.payload).toMatchObject({
                  prop: expect.arrayContaining([
                    'Validator at index 1 is invalid'
                  ]),
                  prop2: expect.arrayContaining([
                    'Validator at index 1 is invalid'
                  ]),
                  virtual: expect.arrayContaining([
                    'Validator at index 1 is invalid'
                  ])
                });
              }
            }
          });

          it('should reject if both functions are invalid', () => {
            for (const invalidValidator of invalidValidators) {
              const toFail = fx({
                prop: {
                  default: '',
                  validator: [invalidValidator, invalidValidator]
                },
                prop2: {
                  required: true,
                  validator: [invalidValidator, invalidValidator]
                },
                dependent: {
                  default: '',
                  dependsOn: 'virtual',
                  resolver: () => ''
                },
                virtual: {
                  virtual: true,
                  validator: [invalidValidator, invalidValidator]
                }
              });

              expectFailure(toFail);

              try {
                toFail();
              } catch (err) {
                expect(err.payload).toMatchObject({
                  prop: expect.arrayContaining(['Invalid validators']),
                  prop2: expect.arrayContaining(['Invalid validators']),
                  virtual: expect.arrayContaining(['Invalid validators'])
                });
              }
            }
          });
        });
      });
    });

    describe('behaviour', () => {
      describe('reason as object', () => {
        it('should add corresponding properties and error messages passed with reason as an object', async () => {
          const messages = [
            ['Invalid Prop', ['Invalid Prop']],
            [['Invalid Prop'], ['Invalid Prop']]
          ];

          for (const [input, reason] of messages) {
            const Model = new Schema({
              prop: { default: '', validator: () => false },
              prop2: {
                required: true,
                validator() {
                  return { valid: false, reason: { prop: input } };
                }
              }
            }).getModel();

            const { data, error } = await Model.create({ prop: 'invalid' });

            expect(data).toBeNull();
            expect(error).toMatchObject({
              message: ERRORS.VALIDATION_ERROR,
              payload: {
                prop: {
                  reasons: expect.arrayContaining([
                    'validation failed',
                    ...reason
                  ]),
                  metadata: null
                }
              }
            });
          }
        });

        it('should ignore keys of "reason object" that are not properties, aliases or are constants or dependents', async () => {
          const Model = new Schema({
            constant: { constant: true, value: 1 },
            dependent: {
              default: '',
              dependsOn: 'prop',
              resolver: () => ''
            },
            prop: {
              required: true,
              validator() {
                return {
                  valid: false,
                  'reason object': {
                    constant: 'invalid constant',
                    dependent: 'invalid dependent',
                    invalidProp: 'invalid prop'
                  }
                };
              }
            }
          }).getModel();

          const { data, error } = await Model.create({});

          expect(data).toBeNull();
          expect(error).toMatchObject({
            message: ERRORS.VALIDATION_ERROR,
            payload: {
              prop: {
                reasons: expect.arrayContaining(['validation failed']),
                metadata: null
              }
            }
          });
        });

        it('should ignore non-strings or array values passed to "reason object"', async () => {
          const invalidMessages = [
            null,
            true,
            false,
            undefined,
            -100,
            0,
            14,
            {},
            () => {},
            [() => {}]
          ];

          for (const message of invalidMessages) {
            const Model = new Schema({
              prop1: { default: '' },
              prop: {
                required: true,
                validator() {
                  return { valid: false, reason: { prop1: message } };
                }
              }
            }).getModel();

            const { data, error } = await Model.create({});

            expect(data).toBeNull();
            expect(error).toEqual({
              message: ERRORS.VALIDATION_ERROR,
              payload: {
                prop1: {
                  reasons: expect.arrayContaining(['validation failed']),
                  metadata: null
                }
              }
            });
          }
        });

        describe('should respect nested props derived from props/aliased passed to "reason object"', () => {
          const Model = new Schema({
            dependent: {
              default: '',
              dependsOn: 'virtual',
              resolver: () => ''
            },
            aliasedDependent: {
              default: '',
              dependsOn: 'virtualWithAlias',
              resolver: () => ''
            },
            name: {
              required: true,
              validator() {
                return {
                  valid: false,
                  reason: {
                    'name.first': 'Invalid first name',
                    'name.last': 'Invalid last name',
                    'lol.shouldReject': 'lol',
                    shouldReject: 'lol'
                  }
                };
              }
            },
            virtual: {
              virtual: true,
              validator() {
                return {
                  valid: false,
                  reason: {
                    virtual: 'Invalid value'
                  }
                };
              }
            },
            virtualWithAlias: {
              virtual: true,
              alias: 'aliasedDependent',
              validator() {
                return {
                  valid: false,
                  reason: {
                    aliasedDependent: 'Invalid value provided'
                  }
                };
              }
            }
          }).getModel();

          describe('creation', () => {
            it('should reject properly at creation', async () => {
              const { data, error } = await Model.create({
                virtual: '',
                virtualWithAlias: ''
              });

              expect(data).toBeNull();
              expect(error).toEqual({
                message: ERRORS.VALIDATION_ERROR,
                payload: {
                  'name.first': {
                    reasons: expect.arrayContaining(['Invalid first name']),
                    metadata: null
                  },
                  'name.last': {
                    reasons: expect.arrayContaining(['Invalid last name']),
                    metadata: null
                  },
                  virtual: {
                    reasons: expect.arrayContaining(['Invalid value']),
                    metadata: null
                  },
                  aliasedDependent: {
                    reasons: expect.arrayContaining(['Invalid value provided']),
                    metadata: null
                  }
                }
              });
            });

            it('should reject properly at creation when virtual alias is provided', async () => {
              const { data, error } = await Model.create({
                aliasedDependent: ''
              });

              expect(data).toBeNull();
              expect(error).toEqual({
                message: ERRORS.VALIDATION_ERROR,
                payload: {
                  'name.first': {
                    reasons: expect.arrayContaining(['Invalid first name']),
                    metadata: null
                  },
                  'name.last': {
                    reasons: expect.arrayContaining(['Invalid last name']),
                    metadata: null
                  },
                  aliasedDependent: {
                    reasons: expect.arrayContaining(['Invalid value provided']),
                    metadata: null
                  }
                }
              });
            });
          });

          describe('updates', () => {
            it('should reject properly during updates', async () => {
              const { data, error } = await Model.update(
                { aliasedDependent: '', dependent: '', name: undefined },
                { virtualWithAlias: '', virtual: '', name: 'lol' }
              );

              expect(data).toBeNull();
              expect(error).toEqual({
                message: ERRORS.VALIDATION_ERROR,
                payload: {
                  'name.first': {
                    reasons: expect.arrayContaining(['Invalid first name']),
                    metadata: null
                  },
                  'name.last': {
                    reasons: expect.arrayContaining(['Invalid last name']),
                    metadata: null
                  },
                  virtual: {
                    reasons: expect.arrayContaining(['Invalid value']),
                    metadata: null
                  },
                  aliasedDependent: {
                    reasons: expect.arrayContaining(['Invalid value provided']),
                    metadata: null
                  }
                }
              });
            });

            it('should reject properly during updates when virtual alias is provided', async () => {
              const { data, error } = await Model.update(
                { aliasedDependent: '', dependent: '', name: undefined },
                { aliasedDependent: '', name: 'lol' }
              );

              expect(data).toBeNull();
              expect(error).toEqual({
                message: ERRORS.VALIDATION_ERROR,
                payload: {
                  'name.first': {
                    reasons: expect.arrayContaining(['Invalid first name']),
                    metadata: null
                  },
                  'name.last': {
                    reasons: expect.arrayContaining(['Invalid last name']),
                    metadata: null
                  },
                  aliasedDependent: {
                    reasons: expect.arrayContaining(['Invalid value provided']),
                    metadata: null
                  }
                }
              });
            });
          });
        });
      });

      describe('metadata', () => {
        describe('Model.create', () => {
          it('should respect valid metadata provided by custom validators', async () => {
            const info = [{ prop2: 'Invalid Prop' }];

            for (const metadata of info) {
              const Model = new Schema({
                prop: { default: '' },
                prop2: {
                  required: true,
                  validator() {
                    return { valid: false, metadata };
                  }
                }
              }).getModel();

              const { data, error } = await Model.create();

              expect(data).toBeNull();
              expect(error).toMatchObject({
                message: ERRORS.VALIDATION_ERROR,
                payload: { prop2: expect.objectContaining({ metadata }) }
              });
            }
          });

          it('should respect valid metadata provided by custom validators', async () => {
            const info = [{ prop2: 'Invalid Prop' }];
            const propMetadata = { message1: 'try again' };

            for (const metadata of info) {
              const Model = new Schema({
                prop: {
                  default: '',
                  validator: () => ({
                    valid: false,
                    metadata: { message: 'too bad' }
                  })
                },
                prop2: {
                  required: true,
                  validator() {
                    return {
                      valid: false,
                      metadata,
                      reason: { prop: { metadata: propMetadata } }
                    };
                  }
                }
              }).getModel();

              const { data, error } = await Model.create({ prop: 'invalid' });

              expect(data).toBeNull();
              expect(error).toMatchObject({
                message: ERRORS.VALIDATION_ERROR,
                payload: {
                  prop: expect.objectContaining({
                    metadata: { message: 'too bad', ...propMetadata },
                    reasons: expect.arrayContaining(['validation failed'])
                  }),
                  prop2: expect.objectContaining({ metadata })
                }
              });
            }
          });
        });

        describe('Model.update', () => {
          it('should respect valid metadata provided by custom validators', async () => {
            const info = [{ prop2: 'Invalid Prop' }];

            for (const metadata of info) {
              const Model = new Schema({
                prop: { default: '' },
                prop2: {
                  required: true,
                  validator() {
                    return { valid: false, metadata };
                  }
                }
              }).getModel();

              const { data, error } = await Model.update({}, { prop2: '' });

              expect(data).toBeNull();
              expect(error).toMatchObject({
                message: ERRORS.VALIDATION_ERROR,
                payload: { prop2: expect.objectContaining({ metadata }) }
              });
            }
          });
        });
      });

      describe('behaviour with errors thrown in the validator', () => {
        const Model = new Schema({
          prop1: {
            default: '',
            validator() {
              throw new Error('lolol');
            }
          },
          prop2: { default: '', validator: () => false }
        }).getModel();

        it("should return 'validation failed' at creation", async () => {
          const { data, error } = await Model.create({ prop1: '', prop2: '' });

          expect(data).toBeNull();
          expect(error).toMatchObject({
            message: ERRORS.VALIDATION_ERROR,
            payload: expect.objectContaining({
              prop1: expect.objectContaining({
                reasons: expect.arrayContaining(['validation failed'])
              }),
              prop2: expect.objectContaining({
                reasons: expect.arrayContaining(['validation failed'])
              })
            })
          });
        });

        it("should return 'validation failed' during updates", async () => {
          const { data, error } = await Model.update(
            { prop1: '', prop2: '' },
            { prop1: 'updated', prop2: 'updated' }
          );

          expect(data).toBeNull();
          expect(error).toMatchObject({
            message: ERRORS.VALIDATION_ERROR,
            payload: expect.objectContaining({
              prop1: expect.objectContaining({
                reasons: expect.arrayContaining(['validation failed'])
              }),
              prop2: expect.objectContaining({
                reasons: expect.arrayContaining(['validation failed'])
              })
            })
          });
        });
      });

      describe('behaviour with secondary validators (validation array)', () => {
        describe('should properly trigger secondary validators', () => {
          let valuesProvided = {} as any;
          let summaryStats = {} as any;

          function makeSecondaryValidator(prop: string) {
            return (value: any, summary: any) => {
              summaryStats[prop] = summary;

              valuesProvided[prop] = value;

              return true;
            };
          }

          afterEach(() => {
            summaryStats = {};
            valuesProvided = {};
          });

          const Model = new Schema({
            dependent: {
              default: '',
              dependsOn: ['virtual', 'virtual2'],
              resolver: validator
            },
            lax: { default: '' },
            readonly: {
              readonly: true,
              validator: [validator, makeSecondaryValidator('readonly')]
            },
            readonlyLax: {
              default: '',
              readonly: 'lax',
              validator: [validator, makeSecondaryValidator('readonlyLax')]
            },
            required: {
              required: true,
              validator: [validator, makeSecondaryValidator('required')]
            },
            virtual: {
              virtual: true,
              validator: [validator, makeSecondaryValidator('virtual')]
            },
            virtual2: {
              virtual: true,
              validator: [validator, makeSecondaryValidator('virtual2')]
            }
          }).getModel();

          afterEach(() => {
            summaryStats = {};
            valuesProvided = {};
          });

          it('should trigger all secondary validators at creation', async () => {
            const { error } = await Model.create();

            expect(error).toBeNull();
            expect(valuesProvided).toEqual({
              readonly: undefined,
              readonlyLax: '',
              required: undefined
            });
          });

          it('should not trigger secondary validators of virtuals not provided at creation', async () => {
            const { error } = await Model.create({ virtual2: true });

            expect(error).toBeNull();
            expect(valuesProvided).toEqual({
              readonly: undefined,
              readonlyLax: '',
              required: undefined,
              virtual2: true
            });
          });

          it('should only trigger secondary validators of props that change during updates', async () => {
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
            expect(valuesProvided).toEqual({ virtual: true, virtual2: true });
          });

          it('should only trigger secondary validators of readonly props that have not changed during updates', async () => {
            const { error } = await Model.update(
              { lax: 2, required: 1, readonly: 1, readonlyLax: '' },
              { readonly: true, readonlyLax: true }
            );

            expect(error).toBeNull();
            expect(valuesProvided).toEqual({ readonlyLax: true });
          });

          describe('should respect "shouldInit" & "shouldUpdate" rules', async () => {
            let stats: any = {};

            function makeSecondaryValidator(prop: string) {
              return [
                () => {
                  if (!stats[prop]) stats[prop] = {};

                  stats[prop].primary = true;

                  return true;
                },
                () => {
                  if (!stats[prop]) stats[prop] = {};

                  stats[prop].secondary = true;

                  return true;
                }
              ];
            }

            function resetStats() {
              stats = {};
            }

            afterEach(resetStats);

            it('should respect "shouldInit" rule', async () => {
              const Model = new Schema({
                lax: {
                  default: '',
                  shouldInit: false,
                  validator: makeSecondaryValidator('lax')
                },
                dependent: {
                  default: '',
                  dependsOn: ['v1', 'v2'],
                  resolver: () => ''
                },
                v1: {
                  virtual: true,
                  validator: makeSecondaryValidator('v1')
                },
                v2: {
                  virtual: true,
                  shouldInit: false,
                  validator: makeSecondaryValidator('v2')
                }
              }).getModel();

              await Model.create();
              expect(stats).toEqual({});

              await Model.create({ lax: true, v2: true });
              expect(stats).toEqual({});

              await Model.create({ lax: true, v1: true, v2: true });
              expect(stats).toEqual({ v1: { primary: true, secondary: true } });
            });

            it('should respect "shouldUpdate" rule', async () => {
              const Model = new Schema({
                lax: {
                  default: '',
                  shouldUpdate: false,
                  validator: makeSecondaryValidator('lax')
                },
                dependent: {
                  default: '',
                  dependsOn: ['v1', 'v2'],
                  resolver: () => ''
                },
                v1: {
                  virtual: true,
                  validator: makeSecondaryValidator('v1')
                },
                v2: {
                  virtual: true,
                  shouldUpdate: false,
                  validator: makeSecondaryValidator('v2')
                }
              }).getModel();

              const data = { dependent: true, lax: true };

              await Model.update(data, {});
              expect(stats).toEqual({});

              await Model.update(data, { lax: 1, v2: true });
              expect(stats).toEqual({});

              await Model.update(data, { v1: true });
              expect(stats).toEqual({ v1: { primary: true, secondary: true } });

              resetStats();

              expect(stats).toEqual({});
              await Model.update(data, { lax: true, v1: true, v2: true });
              expect(stats).toEqual({ v1: { primary: true, secondary: true } });
            });
          });
        });

        describe('values returned from secondary validators should be handled accordingly', () => {
          it('should treat invalid values returned from secondary validators validation failed errors', async () => {
            const values = [-1, 0, 1, '', 'lol', undefined, null, () => {}, []];

            for (const value of values) {
              const Model = new Schema({
                p1: { default: '' },
                p2: { default: '', validator: [validator, () => value] }
              }).getModel();

              const { data, error } = await Model.create();

              expect(data).toBeNull();
              expect(error).toMatchObject({
                message: ERRORS.VALIDATION_ERROR,
                payload: {
                  p2: expect.objectContaining({
                    reasons: expect.arrayContaining(['validation failed'])
                  })
                }
              });

              const { data: updates, error: error2 } = await Model.update(
                { p1: 'p1', p2: '' },
                { p1: 'updated', p2: 'updated' }
              );

              expect(updates).toBeNull();
              expect(error2).toMatchObject({
                message: ERRORS.VALIDATION_ERROR,
                payload: {
                  p2: expect.objectContaining({
                    reasons: expect.arrayContaining(['validation failed'])
                  })
                }
              });
            }
          });

          it('should respect errors returned in secondary validators(sync & async)', async () => {
            const resolver = ({ context }: any) => context.v;

            const Model = new Schema({
              p1: {
                default: '',
                validator: [
                  validator,
                  (_, { isUpdate }) => ({
                    reason: isUpdate ? 'failed to validate' : 'p1'
                  })
                ]
              },
              p2: {
                default: '',
                validator: [validator, () => ({ reason: ['p2'] })]
              },
              p3: {
                default: '',
                validator: [validator, () => ({ reason: ['error1', 'error2'] })]
              },
              p4: {
                default: '',
                validator: [validator, () => null]
              },
              d1: { default: '', dependsOn: 'v', resolver },
              d2: { default: '', dependsOn: 'v', resolver },
              v: {
                alias: 'd1',
                virtual: true,
                validator: [
                  validator,
                  function (v: any, { isUpdate }: any) {
                    if (v == 'throw') throw new Error('lol');

                    return isUpdate
                      ? { reason: ['lolz'] }
                      : { reason: 'error', metadata: { lol: true } };
                  }
                ]
              }
            }).getModel();

            const res = await Model.create({ v: true });

            expect(res.data).toBeNull();
            expect(res.error.payload).toMatchObject({
              p1: expect.objectContaining({
                reasons: expect.arrayContaining(['p1'])
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

            const res2 = await Model.update(
              {},
              { p1: true, p2: 'updated', d1: 'updated' }
            );
            expect(res2.data).toBeNull();
            expect(res2.error.payload).toMatchObject({
              p1: expect.objectContaining({
                reasons: expect.arrayContaining(['failed to validate'])
              }),
              p2: expect.objectContaining({
                reasons: expect.arrayContaining(['p2'])
              }),
              d1: expect.objectContaining({
                reasons: expect.arrayContaining(['lolz'])
              })
            });

            const res3 = await Model.create({ v: 'throw' });
            expect(res3.data).toBeNull();
            expect(res3.error.payload).toMatchObject({
              v: expect.objectContaining({ reasons: ['validation failed'] })
            });
          });
        });
      });
    });
  });
};
