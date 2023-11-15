import { beforeAll, describe, test, it, expect } from 'vitest';

import { ERRORS } from '../../../../dist';
import {
  expectFailure,
  expectNoFailure,
  expectPromiseFailure,
  getValidSchema
} from '../_utils';

export const Test_SchemaErrors = ({ Schema, fx }: any) => {
  describe('Schema.options.errors', () => {
    it("should allow 'silent' | 'throw'", () => {
      const values = ['silent', 'throw'];

      for (const errors of values) {
        const toPass = fx(getValidSchema(), { errors });

        expectNoFailure(toPass);

        toPass();
      }
    });

    describe('valid', () => {
      let silentModel: any,
        modelToThrow: any,
        models: any[] = [];

      beforeAll(() => {
        const validator = (value: any) => {
          return value
            ? { valid: true }
            : { reason: 'Invalid value', valid: false };
        };

        const definition = {
          lax: { default: 'lax-default', validator },
          readonly: {
            readonly: 'lax',
            default: 'readonly-default',
            validator
          },
          required: { required: true, validator }
        };

        silentModel = new Schema(definition).getModel();
        modelToThrow = new Schema(definition, { errors: 'throw' }).getModel();

        models = [silentModel, modelToThrow];
      });

      test('silent & throw with valid data', () => {
        // create
        for (const model of models) {
          it('should create normally', async () => {
            const { data } = await model.create({
              readonly: 'lax',
              required: true
            });

            expect(data).toEqual({
              lax: 'lax-default',
              readonly: 'lax',
              required: true
            });
          });

          // clone
          it('should clone normally', async () => {
            const { data } = await model.clone({
              readonly: 'lax',
              required: true
            });

            expect(data).toEqual({
              lax: 'lax-default',
              readonly: 'lax',
              required: true
            });
          });

          // update
          it('should update normally', async () => {
            const { data } = await model.update(
              {
                lax: 'lax-default',
                readonly: 'lax',
                required: true
              },
              { required: 'required' }
            );

            expect(data).toEqual({ required: 'required' });
          });
        }
      });

      describe('silent', () => {
        // create
        it('should reject invalid props on create', async () => {
          const { error } = await silentModel.create({
            lax: false,
            readonly: 'lax',
            required: ''
          });

          expect(error).toEqual(
            expect.objectContaining({
              message: ERRORS.VALIDATION_ERROR,
              payload: {
                lax: {
                  reasons: expect.arrayContaining(['Invalid value']),
                  metadata: null
                },
                required: {
                  reasons: expect.arrayContaining(['Invalid value']),
                  metadata: null
                }
              }
            })
          );
        });

        // update
        it('should reject invalid props on update', async () => {
          const { error } = await silentModel.update(
            {
              lax: 'lax-default',
              readonly: 'lax',
              required: true
            },
            { lax: false, required: '' }
          );

          expect(error).toEqual(
            expect.objectContaining({
              message: ERRORS.VALIDATION_ERROR,
              payload: {
                lax: {
                  reasons: expect.arrayContaining(['Invalid value']),
                  metadata: null
                },
                required: {
                  reasons: expect.arrayContaining(['Invalid value']),
                  metadata: null
                }
              }
            })
          );
        });

        it('should reject on nothing to update', async () => {
          const { error } = await silentModel.update(
            {
              lax: 'lax-default',
              readonly: 'lax',
              required: true
            },
            { readonly: 'New val' }
          );

          expect(error).toEqual(
            expect.objectContaining({
              message: ERRORS.NOTHING_TO_UPDATE,
              payload: {}
            })
          );
        });
      });

      describe('throw', () => {
        // create
        it('should reject invalid props on create', async () => {
          const toFail = () =>
            modelToThrow.create({
              lax: false,
              readonly: 'lax',
              required: ''
            });

          expectPromiseFailure(toFail, ERRORS.VALIDATION_ERROR);

          try {
            await toFail();
          } catch (err: any) {
            expect(err).toEqual(
              expect.objectContaining({
                message: ERRORS.VALIDATION_ERROR,
                payload: {
                  lax: {
                    reasons: expect.arrayContaining(['Invalid value']),
                    metadata: null
                  },
                  required: {
                    reasons: expect.arrayContaining(['Invalid value']),
                    metadata: null
                  }
                }
              })
            );
          }
        });

        // update
        it('should reject invalid props on update', async () => {
          const toFail = () =>
            modelToThrow.update(
              {
                lax: 'lax-default',
                readonly: 'lax',
                required: true
              },
              { lax: false, required: '' }
            );

          expectPromiseFailure(toFail, ERRORS.VALIDATION_ERROR);

          try {
            await toFail();
          } catch (err: any) {
            expect(err).toMatchObject(
              expect.objectContaining({
                message: ERRORS.VALIDATION_ERROR,
                payload: expect.objectContaining({
                  lax: {
                    reasons: expect.arrayContaining(['Invalid value']),
                    metadata: null
                  },
                  required: {
                    reasons: expect.arrayContaining(['Invalid value']),
                    metadata: null
                  }
                })
              })
            );
          }
        });

        it('should reject on nothing to update', async () => {
          const toFail = () =>
            modelToThrow.update(
              {
                lax: 'lax-default',
                readonly: 'lax',
                required: true
              },
              { readonly: 'New val' }
            );

          expectPromiseFailure(toFail, ERRORS.NOTHING_TO_UPDATE);

          try {
            await toFail();
          } catch (err: any) {
            expect(err).toEqual(
              expect.objectContaining({
                message: ERRORS.NOTHING_TO_UPDATE,
                payload: {}
              })
            );
          }
        });
      });
    });

    describe('invalid', () => {
      it("should reject anything other than ('silent' | 'throw')", () => {
        const values = ['silence', 1, null, false, true, 'throws', [], {}];

        for (const errors of values) {
          const toFail = fx(getValidSchema(), { errors });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                errors: expect.arrayContaining([
                  "should be 'silent' or 'throw'"
                ])
              })
            );
          }
        }
      });
    });
  });
};
