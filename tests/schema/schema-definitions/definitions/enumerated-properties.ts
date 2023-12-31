import { describe, it, expect } from 'vitest';

import { expectFailure, expectNoFailure, validator } from '../_utils';

export const Test_EnumeratedProperties = ({ fx, Schema }: any) => {
  describe('allow rule', () => {
    describe('valid', () => {
      it('should not reject if allowed values provided are >= 2', () => {
        const values = [
          ['lol', 2],
          ['lol', 2, 3]
        ];

        for (const allow of values) {
          const toPass = fx({ prop: { default: allow[0], allow } });

          expectNoFailure(toPass);

          toPass();
        }
      });

      it('should not reject if default value provided is an allowed value', () => {
        const toPass = fx({
          prop: { default: null, allow: [null, 'lolz', -1] }
        });

        expectNoFailure(toPass);

        toPass();
      });

      it('should allow virtuals to have allowed values', () => {
        const toPass = fx({
          dependent: {
            default: true,
            dependsOn: 'virtual',
            resolver: validator
          },
          virtual: { virtual: true, allow: [null, 'lolz', -1], validator }
        });

        expectNoFailure(toPass);

        toPass();
      });

      // describe('allow as an object', () => {
      //   it('should not reject if "allow" rule is an object with values and error as only keys', () => {
      //     const toPass = fx({
      //       dependent: {
      //         default: true,
      //         dependsOn: 'virtual',
      //         resolver: validator
      //       },
      //       virtual: {
      //         virtual: true,
      //         allow: { values: [null, 'lolz', -1], error: 'value not allowed' },
      //         validator
      //       }
      //     });

      //     expectNoFailure(toPass);

      //     toPass();
      //   });
      // });
    });

    describe('invalid', () => {
      it('should reject if non-array value is provided', () => {
        const values = [
          null,
          undefined,
          new Number(),
          new String(),
          Symbol(),
          2,
          -10,
          true,
          () => {},
          {}
        ];

        for (const allow of values) {
          const toFail = fx({ prop: { allow } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              prop: ['Allowed values must be an array']
            });
          }
        }
      });

      it('should reject if allowed values provided are not unique', () => {
        const values = [
          [1, 2, 2, 4, 5],
          ['lol', 59, 'lol', null],
          [true, false, true],
          [{}, {}],
          [{ id: 'lol' }, { id: 'lol' }]
        ];

        for (const allow of values) {
          const toFail = fx({ prop: { allow } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              prop: ['Allowed values must be an array of unique values']
            });
          }
        }
      });

      it('should reject if allowed values provided are less than 2', () => {
        const values = [[], ['lol']];

        for (const allow of values) {
          const toFail = fx({ prop: { allow } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              prop: ['Allowed values must have at least 2 values']
            });
          }
        }
      });

      it('should reject if default value provided is not an allowed value', () => {
        const values = [
          ['lol', [null, 'lolz', -1]],
          [null, [1, 4, 'lol', undefined]]
        ];

        for (const [_default, allow] of values) {
          const toFail = fx({ prop: { default: _default, allow } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              prop: ['The default value must be an allowed value']
            });
          }
        }
      });

      describe('allow as an object', () => {
        it('should reject if values array is not provided', () => {
          const toFail = fx({ prop: { allow: {} } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              prop: ['Allowed values must be an array']
            });
          }
        });

        it('should reject if non-array value is provided', () => {
          const invalidValues = [
            null,
            undefined,
            new Number(),
            new String(),
            Symbol(),
            2,
            -10,
            true,
            () => {},
            {}
          ];

          for (const values of invalidValues) {
            const toFail = fx({ prop: { allow: { values } } });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toMatchObject({
                prop: ['Allowed values must be an array']
              });
            }
          }
        });

        it('should reject if allowed values provided are not unique', () => {
          const invalidValues = [
            [1, 2, 2, 4, 5],
            ['lol', 59, 'lol', null],
            [true, false, true],
            [{}, {}],
            [{ id: 'lol' }, { id: 'lol' }]
          ];

          for (const values of invalidValues) {
            const toFail = fx({ prop: { allow: { values } } });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toMatchObject({
                prop: ['Allowed values must be an array of unique values']
              });
            }
          }
        });

        it('should reject if allowed values provided are less than 2', () => {
          const invalidValues = [[], ['lol']];

          for (const values of invalidValues) {
            const toFail = fx({ prop: { allow: { values } } });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toMatchObject({
                prop: ['Allowed values must have at least 2 values']
              });
            }
          }
        });

        it('should reject if default value provided is not an allowed value', () => {
          const data = [
            ['lol', [null, 'lolz', -1]],
            [null, [1, 4, 'lol', undefined]]
          ];

          for (const [_default, values] of data) {
            const toFail = fx({
              prop: { default: _default, allow: { values } }
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toMatchObject({
                prop: ['The default value must be an allowed value']
              });
            }
          }
        });
      });
    });

    describe('behaviour', () => {
      const metadata = { allowed: [null, 'allowed'] };

      describe('behaviour with lax props & no validators', () => {
        const Model = new Schema({
          prop: { default: null, allow: metadata.allowed }
        }).getModel();

        describe('creation', () => {
          it('should allow if value provided is allowed', async () => {
            const { data, error } = await Model.create({ prop: 'allowed' });

            expect(error).toBe(null);
            expect(data).toMatchObject({ prop: 'allowed' });
          });

          it('should reject if value provided is not allowed', async () => {
            const { data, error } = await Model.create({ prop: true });

            expect(data).toBe(null);

            expect(error.payload).toMatchObject({
              prop: expect.objectContaining({
                reasons: ['value not allowed'],
                metadata
              })
            });
          });
        });

        describe('updates', () => {
          it('should allow if value provided is allowed', async () => {
            const { data, error } = await Model.update(
              { prop: 'allowed' },
              { prop: null }
            );

            expect(error).toBe(null);
            expect(data).toMatchObject({ prop: null });
          });

          it('should reject if value provided is not allowed', async () => {
            const { data, error } = await Model.update(
              { prop: null },
              { prop: true }
            );

            expect(data).toBe(null);

            expect(error.payload).toMatchObject({
              prop: expect.objectContaining({
                reasons: ['value not allowed'],
                metadata
              })
            });
          });
        });
      });

      describe('behaviour with lax props & validators', () => {
        const Model = new Schema({
          prop: {
            default: null,
            allow: metadata.allowed,
            validator(v: any) {
              if (v) return { valid: true, validated: 'validated' };

              return false;
            }
          }
        }).getModel();

        describe('creation', () => {
          it('should respect validators even if value provided is allowed', async () => {
            const { data, error } = await Model.create({ prop: null });

            expect(data).toBe(null);
            expect(error.payload).toMatchObject({
              prop: expect.objectContaining({
                reasons: ['validation failed']
              })
            });
          });

          it('should ignore validated value from validator if value is not allowed', async () => {
            const { data, error } = await Model.create({ prop: 'allowed' });

            expect(error).toBe(null);
            expect(data).toMatchObject({ prop: 'allowed' });
          });
        });

        describe('updates', () => {
          it('should respect validators even if value provided is allowed', async () => {
            const { data, error } = await Model.update(
              { prop: 'allowed' },
              { prop: null }
            );

            expect(data).toBe(null);
            expect(error.payload).toMatchObject({
              prop: expect.objectContaining({
                reasons: ['validation failed']
              })
            });
          });

          it('should ignore validated value from validator if value is not allowed', async () => {
            const { data, error } = await Model.update(
              { prop: null },
              { prop: 'allowed' }
            );

            expect(error).toBe(null);
            expect(data).toMatchObject({ prop: 'allowed' });
          });
        });
      });

      describe('behaviour with virtuals', () => {
        const Model = new Schema({
          dependent: {
            default: null,
            dependsOn: 'virtual',
            resolver: ({ context: { virtual } }) => virtual
          },
          virtual: {
            virtual: true,
            allow: metadata.allowed,
            validator(v: any) {
              if (v) return { valid: true, validated: 'validated' };

              return false;
            }
          }
        }).getModel();

        describe('creation', () => {
          it('should respect validators even if value provided is allowed', async () => {
            const { data, error } = await Model.create({ virtual: null });

            expect(data).toBe(null);
            expect(error.payload).toMatchObject({
              virtual: expect.objectContaining({
                reasons: ['validation failed']
              })
            });
          });

          it('should ignore validated value from validator if value is not allowed', async () => {
            const { data, error } = await Model.create({ virtual: 'allowed' });

            expect(error).toBe(null);
            expect(data).toMatchObject({ dependent: 'allowed' });
          });
        });

        describe('updates', () => {
          it('should respect validators even if value provided is allowed', async () => {
            const { data, error } = await Model.update(
              { dependent: 'allowed' },
              { virtual: null }
            );

            expect(data).toBe(null);
            expect(error.payload).toMatchObject({
              virtual: expect.objectContaining({
                reasons: ['validation failed']
              })
            });
          });

          it('should ignore validated value from validator if value is not allowed', async () => {
            const { data, error } = await Model.update(
              { dependent: null },
              { virtual: 'allowed' }
            );

            expect(error).toBe(null);
            expect(data).toMatchObject({ dependent: 'allowed' });
          });
        });
      });

      describe('behaviour with virtuals & alias', () => {
        const Model = new Schema({
          dependent: {
            default: null,
            dependsOn: 'virtual',
            resolver: ({ context: { virtual } }) => virtual
          },
          virtual: {
            alias: 'dependent',
            virtual: true,
            allow: metadata.allowed,
            validator(v: any) {
              if (v) return { valid: true, validated: 'validated' };

              return false;
            }
          }
        }).getModel();

        describe('creation', () => {
          it('should respect validators even if value provided is allowed', async () => {
            const { data, error } = await Model.create({ dependent: null });

            expect(data).toBe(null);
            expect(error.payload).toMatchObject({
              dependent: expect.objectContaining({
                reasons: ['validation failed']
              })
            });
          });

          it('should ignore validated value from validator if value is not allowed', async () => {
            const { data, error } = await Model.create({
              dependent: 'allowed'
            });

            expect(error).toBe(null);
            expect(data).toMatchObject({ dependent: 'allowed' });
          });
        });

        describe('updates', () => {
          it('should respect validators even if value provided is allowed', async () => {
            const { data, error } = await Model.update(
              { dependent: 'allowed' },
              { dependent: null }
            );

            expect(data).toBe(null);
            expect(error.payload).toMatchObject({
              dependent: expect.objectContaining({
                reasons: ['validation failed']
              })
            });
          });

          it('should ignore validated value from validator if value is not allowed', async () => {
            const { data, error } = await Model.update(
              { dependent: null },
              { dependent: 'allowed' }
            );

            expect(error).toBe(null);
            expect(data).toMatchObject({ dependent: 'allowed' });
          });
        });
      });
    });
  });
};
