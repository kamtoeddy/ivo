import { describe, it, expect } from 'vitest';

import { ERRORS } from '../../dist';

export const Test_Validators = ({ Schema }: any) => {
  describe('Validators', () => {
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
          dependent: { default: '', dependsOn: 'virtual', resolver: () => '' },
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
        prop2: {
          default: '',
          validator: () => false
        }
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
