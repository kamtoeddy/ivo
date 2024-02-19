import { describe, it, expect } from 'vitest';

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

            const message = 'Validator array may contain exactly 2 functions';

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
  });
};
