import { describe, it, expect } from 'vitest';

import { ERRORS } from '../../../../dist';

export const Test_Validators = ({ Schema }: any) => {
  describe('Validators', () => {
    describe('otherReasons', () => {
      it('should add corresponding properties and error messages passed as otherReasons', async () => {
        const messages = [
          ['Invalid Prop', ['Invalid Prop']],
          [['Invalid Prop'], ['Invalid Prop']]
        ];

        for (const [input, reasons] of messages) {
          const Model = new Schema({
            prop: { default: '', validator: () => false },
            prop2: {
              required: true,
              validator() {
                return { valid: false, otherReasons: { prop: input } };
              }
            }
          }).getModel();

          const { data, error } = await Model.create({ prop: 'invalid' });

          expect(data).toBe(null);
          expect(error).toMatchObject({
            message: ERRORS.VALIDATION_ERROR,
            payload: {
              prop: {
                reasons: expect.arrayContaining([
                  'validation failed',
                  ...reasons
                ]),
                metadata: null
              }
            }
          });
        }
      });

      it('should ignore keys of otherReasons that are not properties, aliases or are constants or dependents', async () => {
        const Model = new Schema({
          constant: { constant: true, value: 1 },
          dependent: {
            default: '',
            dependent: true,
            dependsOn: 'prop',
            resolver: () => ''
          },
          prop: {
            required: true,
            validator() {
              return {
                valid: false,
                otherReasons: {
                  constant: 'invalid constant',
                  dependent: 'invalid dependent',
                  invalidProp: 'invalid prop'
                }
              };
            }
          }
        }).getModel();

        const { data, error } = await Model.create({});

        expect(data).toBe(null);
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

      it('should ignore non-strings or array values passed to otherReasons', async () => {
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
                return { valid: false, otherReasons: { prop1: message } };
              }
            }
          }).getModel();

          const { data, error } = await Model.create({});

          expect(data).toBe(null);
          expect(error).toEqual({
            message: ERRORS.VALIDATION_ERROR,
            payload: {
              prop: {
                reasons: expect.arrayContaining(['validation failed']),
                metadata: null
              },
              prop1: {
                reasons: expect.arrayContaining(['validation failed']),
                metadata: null
              }
            }
          });
        }
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

            expect(data).toBe(null);
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
                    otherReasons: { prop: { metadata: propMetadata } }
                  };
                }
              }
            }).getModel();

            const { data, error } = await Model.create({ prop: 'invalid' });

            expect(data).toBe(null);
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

            expect(data).toBe(null);
            expect(error).toMatchObject({
              message: ERRORS.VALIDATION_ERROR,
              payload: { prop2: expect.objectContaining({ metadata }) }
            });
          }
        });
      });
    });
  });
};
