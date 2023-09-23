import { ERRORS } from '../../../..'
import {
  expectFailure,
  expectNoFailure,
  getValidSchema
  // validator
} from '../_utils'

export const Test_SchemaEqualityDepth = ({ Schema, fx }: any) => {
  describe('Schema.options.equalityDepth', () => {
    describe('behaviour', () => {
      const error = { message: ERRORS.NOTHING_TO_UPDATE }

      const user = {
        level_0_a: 'value',
        level_0_b: {
          level_1_a: { level_2_a: 'value', level_2_b: 'value' },
          level_1_b: {
            level_2_a: 'value',
            level_2_b: {
              level_3_a: 'value',
              level_3_b: 'value',
              level_3_c: { level_4_a: 'value', level_4_b: 'value' }
            }
          }
        }
      }

      describe('behaviour with previous values', () => {
        it('should respect the default equality depth(1)', async () => {
          const Model = new Schema({
            level_0_a: { default: '' },
            level_0_b: { default: {} }
          }).getModel()

          const changeToAllow = {
            level_1_a: { level_2_b: 'value', level_2_a: 'value' },
            level_1_b: {
              level_2_b: {
                level_3_b: 'value',
                level_3_a: 'value',
                level_3_c: { level_4_a: 'value', level_4_b: 'value' }
              },
              level_2_a: 'value'
            }
          }

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
                      level_3_c: { level_4_a: 'value', level_4_b: 'value' }
                    }
                  }
                },
                level_0_a: 'value'
              },
              error
            },
            {
              changes: {
                level_0_b: {
                  level_1_b: {
                    level_2_b: {
                      level_3_a: 'value',
                      level_3_b: 'value',
                      level_3_c: { level_4_a: 'value', level_4_b: 'value' }
                    },
                    level_2_a: 'value'
                  },
                  level_1_a: { level_2_b: 'value', level_2_a: 'value' }
                },
                level_0_a: 'value'
              },
              error
            },
            {
              changes: {
                level_0_b: changeToAllow,
                level_0_a: 'value'
              },
              data: { level_0_b: changeToAllow }
            }
          ] as any[]

          for (const values of values_) {
            const { data, error } = await Model.update(user, values.changes)

            if (values.data) {
              expect(error).toEqual(null)
              expect(data).toMatchObject(values.data)
            }

            if (values.error) {
              expect(data).toEqual(null)
              expect(error).toMatchObject(values.error)
            }
          }
        })

        it('should respect the equality depth(0)', async () => {
          const Model = new Schema(
            {
              level_0_a: { default: '' },
              level_0_b: { default: {} }
            },
            { equalityDepth: 0 }
          ).getModel()

          const changeToAllow = {
              level_0_b: {
                level_1_b: {
                  level_2_b: {
                    level_3_a: 'value',
                    level_3_b: 'value',
                    level_3_c: { level_4_a: 'value', level_4_b: 'value' }
                  },
                  level_2_a: 'value'
                },
                level_1_a: { level_2_b: 'value', level_2_a: 'value' }
              }
            },
            changeToAllow1 = {
              level_1_a: { level_2_b: 'value', level_2_a: 'value' },
              level_1_b: {
                level_2_b: {
                  level_3_b: 'value',
                  level_3_a: 'value',
                  level_3_c: { level_4_a: 'value', level_4_b: 'value' }
                },
                level_2_a: 'value'
              }
            }

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
                      level_3_c: { level_4_a: 'value', level_4_b: 'value' }
                    }
                  }
                },
                level_0_a: 'value'
              },
              error
            },
            {
              changes: changeToAllow,
              data: changeToAllow
            },
            {
              changes: {
                level_0_b: changeToAllow1,
                level_0_a: 'value'
              },
              data: { level_0_b: changeToAllow1 }
            }
          ] as any[]

          for (const values of values_) {
            const { data, error } = await Model.update(user, values.changes)

            if (values.data) {
              expect(error).toEqual(null)
              expect(data).toMatchObject(values.data)
            }

            if (values.error) {
              expect(data).toEqual(null)
              expect(error).toMatchObject(values.error)
            }
          }
        })

        it('should respect the equality depth(2)', async () => {
          const Model = new Schema(
            {
              level_0_a: { default: '' },
              level_0_b: { default: {} }
            },
            { equalityDepth: 2 }
          ).getModel()

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
                      level_3_c: { level_4_a: 'value', level_4_b: 'value' }
                    }
                  }
                },
                level_0_a: 'value'
              },
              error
            },
            {
              changes: {
                level_0_b: {
                  level_1_b: {
                    level_2_b: {
                      level_3_a: 'value',
                      level_3_b: 'value',
                      level_3_c: { level_4_a: 'value', level_4_b: 'value' }
                    },
                    level_2_a: 'value'
                  },
                  level_1_a: { level_2_b: 'value', level_2_a: 'value' }
                }
              },
              error
            },
            {
              changes: {
                level_0_b: {
                  level_1_a: { level_2_b: 'value', level_2_a: 'value' },
                  level_1_b: {
                    level_2_b: {
                      level_3_b: 'value',
                      level_3_a: 'value',
                      level_3_c: { level_4_a: 'value', level_4_b: 'value' }
                    },
                    level_2_a: 'value'
                  }
                },
                level_0_a: 'value'
              },
              error
            }
          ] as any[]

          for (const values of values_) {
            const { data, error } = await Model.update(user, values.changes)

            if (values.data) {
              expect(error).toEqual(null)
              expect(data).toMatchObject(values.data)
            }

            if (values.error) {
              expect(data).toEqual(null)
              expect(error).toMatchObject(values.error)
            }
          }
        })
      })
    })

    describe('valid', () => {
      it("should allow 'equalityDepth' as number", () => {
        const toPass = fx(getValidSchema(), { equalityDepth: 1 })

        expectNoFailure(toPass)

        toPass()
      })

      it('should allow numbers >= 0', () => {
        const values = [0, 1, 53, Infinity]

        for (const equalityDepth of values) {
          const toPass = fx(getValidSchema(), { equalityDepth })

          expectNoFailure(toPass)

          toPass()
        }
      })
    })

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
          null
        ]

        for (const equalityDepth of invalidValues) {
          const toFail = fx(getValidSchema(), { equalityDepth })

          expectFailure(toFail)

          try {
            toFail()
          } catch (err: any) {
            expect(err).toMatchObject({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                equalityDepth: expect.arrayContaining([
                  "'equalityDepth' must be a number between 0 and +Infinity"
                ])
              }
            })
          }
        }
      })
    })
  })
}
