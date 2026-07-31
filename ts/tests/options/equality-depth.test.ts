import { describe, expect, it } from 'bun:test';
import { Schema } from '../../src';
import {
  ERRORS,
  expectFailure,
  expectNoFailure,
  getValidSchema,
  makeFx,
} from '../_utils';

describe('Schema.options.equalityDepth', () => {
  describe('behaviour', () => {
    const error = null;

    const user = {
      level_0_a: 'value',
      level_0_b: {
        level_1_a: { level_2_a: 'value', level_2_b: 'value' },
        level_1_b: {
          level_2_a: 'value',
          level_2_b: {
            level_3_a: 'value',
            level_3_b: 'value',
            level_3_c: { level_4_a: 'value', level_4_b: 'value' },
          },
        },
      },
    };

    describe('behaviour with previous values', () => {
      it('should respect the default equality depth(1)', async () => {
        const Model = new Schema<any>((b, m) =>
          b
            .field(m.lax('level_0_a').default(''))
            .field(m.lax('level_0_b').default({})),
        ).getModel();

        const changeToAllow = {
          level_1_a: { level_2_b: 'value', level_2_a: 'value' },
          level_1_b: {
            level_2_b: {
              level_3_b: 'value',
              level_3_a: 'value',
              level_3_c: { level_4_a: 'value', level_4_b: 'value' },
            },
            level_2_a: 'value',
          },
        };

        const values_ = [
          { changes: user, error },
          {
            changes: {
              level_0_b: {
                level_1_a: { level_2_a: 'value', level_2_b: 'value' },
                level_1_b: {
                  level_2_a: 'value',
                  level_2_b: {
                    level_3_a: 'value',
                    level_3_b: 'value',
                    level_3_c: { level_4_a: 'value', level_4_b: 'value' },
                  },
                },
              },
              level_0_a: 'value',
            },
            error,
          },
          {
            changes: {
              level_0_b: {
                level_1_b: {
                  level_2_b: {
                    level_3_a: 'value',
                    level_3_b: 'value',
                    level_3_c: { level_4_a: 'value', level_4_b: 'value' },
                  },
                  level_2_a: 'value',
                },
                level_1_a: { level_2_b: 'value', level_2_a: 'value' },
              },
              level_0_a: 'value',
            },
            error,
          },
          {
            changes: {
              level_0_b: changeToAllow,
              level_0_a: 'value',
            },
            data: { level_0_b: changeToAllow },
          },
        ];

        for (const values of values_) {
          const { data, error, handleFailure } = await Model.update(
            user,
            values.changes,
          );

          if (values.data) {
            expect(error).toEqual(null);
            expect(data).toMatchObject(values.data);
          }

          if ((values as any).error) {
            expect(data).toEqual(null);
            expect(error).toBeNull();
            expect(typeof handleFailure).toBe('function');
          }
        }
      });

      it('should respect the equality depth(0)', async () => {
        const Model = new Schema<any>(
          (b, m) =>
            b
              .field(m.lax('level_0_a').default(''))
              .field(m.lax('level_0_b').default({})),
          { equalityDepth: 0 },
        ).getModel();

        const changeToAllow = {
            level_0_b: {
              level_1_b: {
                level_2_b: {
                  level_3_a: 'value',
                  level_3_b: 'value',
                  level_3_c: { level_4_a: 'value', level_4_b: 'value' },
                },
                level_2_a: 'value',
              },
              level_1_a: { level_2_b: 'value', level_2_a: 'value' },
            },
          },
          changeToAllow1 = {
            level_1_a: { level_2_b: 'value', level_2_a: 'value' },
            level_1_b: {
              level_2_b: {
                level_3_b: 'value',
                level_3_a: 'value',
                level_3_c: { level_4_a: 'value', level_4_b: 'value' },
              },
              level_2_a: 'value',
            },
          };

        const values_ = [
          { changes: user, error },
          {
            changes: {
              level_0_b: {
                level_1_a: { level_2_a: 'value', level_2_b: 'value' },
                level_1_b: {
                  level_2_a: 'value',
                  level_2_b: {
                    level_3_a: 'value',
                    level_3_b: 'value',
                    level_3_c: { level_4_a: 'value', level_4_b: 'value' },
                  },
                },
              },
              level_0_a: 'value',
            },
            error,
          },
          {
            changes: changeToAllow,
            data: changeToAllow,
          },
          {
            changes: {
              level_0_b: changeToAllow1,
              level_0_a: 'value',
            },
            data: { level_0_b: changeToAllow1 },
          },
        ];

        for (const values of values_) {
          const { data, error, handleFailure } = await Model.update(
            user,
            values.changes,
          );

          if (values.data) {
            expect(error).toEqual(null);
            expect(data).toMatchObject(values.data);
          }

          if ((values as any).error) {
            expect(data).toEqual(null);
            expect(error).toBeNull();
            expect(typeof handleFailure).toBe('function');
          }
        }
      });

      it('should respect the equality depth(2)', async () => {
        const Model = new Schema<any>(
          (b, m) =>
            b
              .field(m.lax('level_0_a').default(''))
              .field(m.lax('level_0_b').default({})),
          { equalityDepth: 2 },
        ).getModel();

        const values_ = [
          { changes: user, error },
          {
            changes: {
              level_0_b: {
                level_1_a: { level_2_a: 'value', level_2_b: 'value' },
                level_1_b: {
                  level_2_a: 'value',
                  level_2_b: {
                    level_3_a: 'value',
                    level_3_b: 'value',
                    level_3_c: { level_4_a: 'value', level_4_b: 'value' },
                  },
                },
              },
              level_0_a: 'value',
            },
            error,
          },
          {
            changes: {
              level_0_b: {
                level_1_b: {
                  level_2_b: {
                    level_3_a: 'value',
                    level_3_b: 'value',
                    level_3_c: { level_4_a: 'value', level_4_b: 'value' },
                  },
                  level_2_a: 'value',
                },
                level_1_a: { level_2_b: 'value', level_2_a: 'value' },
              },
            },
            error,
          },
          {
            changes: {
              level_0_b: {
                level_1_a: { level_2_b: 'value', level_2_a: 'value' },
                level_1_b: {
                  level_2_b: {
                    level_3_b: 'value',
                    level_3_a: 'value',
                    level_3_c: { level_4_a: 'value', level_4_b: 'value' },
                  },
                  level_2_a: 'value',
                },
              },
              level_0_a: 'value',
            },
            error,
          },
        ];

        for (const values of values_) {
          const { data, error, handleFailure } = await Model.update(
            user,
            values.changes,
          );

          if (values.error) {
            expect(data).toEqual(null);
            expect(error).toBeNull();
            expect(typeof handleFailure).toBe('function');
          }
        }
      });
    });
  });

  describe('valid', () => {
    it("should allow 'equalityDepth' as number", () => {
      const toPass = makeFx(getValidSchema(), { equalityDepth: 1 });

      expectNoFailure(toPass);

      toPass();
    });

    it('should allow numbers >= 0', () => {
      const values = [0, 1, 53, Number.POSITIVE_INFINITY];

      for (const equalityDepth of values) {
        const toPass = makeFx(getValidSchema(), { equalityDepth });

        expectNoFailure(toPass);

        toPass();
      }
    });
  });

  describe('invalid', () => {
    it("should reject 'equalityDepth' if not a number >= 0", () => {
      const invalidValues = [
        -1,
        [],
        {},
        () => '',
        () => 23,
        true,
        false,
        'invalid',
        '',
        null,
      ];

      for (const equalityDepth of invalidValues) {
        const toFail = makeFx(getValidSchema(), { equalityDepth });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err).toMatchObject({
            message: ERRORS.INVALID_SCHEMA,
            payload: {
              equalityDepth: expect.arrayContaining([
                "'equalityDepth' must be a number between 0 and +Infinity",
              ]),
            },
          });
        }
      }
    });
  });
});
