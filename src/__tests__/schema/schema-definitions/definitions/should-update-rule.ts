import { ERRORS } from '../../../..'
import { expectFailure, expectNoFailure, validator } from '../_utils'

export const Test_ShouldUpdateRule = ({ Schema, fx }: any) => {
  describe('shouldUpdate', () => {
    describe('valid', () => {
      it('should accept shouldUpdate(() => boolean)', () => {
        const validValues = [() => false, () => true]

        for (const shouldUpdate of validValues) {
          const toPass = fx({ propertyName: { default: '', shouldUpdate } })

          expectNoFailure(toPass)

          toPass()
        }
      })

      it('should accept shouldUpdate(() => boolean) & shouldInit(false)', () => {
        // [shouldInit, shouldUpdate]
        const values = [
          [false, () => false],
          [() => false, () => false]
        ]

        for (const [shouldInit, shouldUpdate] of values) {
          const toPass = fx({
            propertyName: { default: '', shouldInit, shouldUpdate }
          })

          expectNoFailure(toPass)

          toPass()
        }
      })

      it('should accept shouldInit(() => boolean) & shouldUpdate(false) for virtuals', () => {
        const values = [() => true, () => false]

        for (const shouldInit of values) {
          const toPass = fx({
            dependentProp: {
              default: '',
              dependent: true,
              dependsOn: 'virtual',
              resolver: () => ''
            },
            virtual: {
              virtual: true,
              shouldInit,
              shouldUpdate: false,
              validator
            }
          })

          expectNoFailure(toPass)

          toPass()
        }
      })

      it("should accept shouldInit(() => boolean) + shouldUpdate(false | () => boolean) + readonly(true | 'lax')", () => {
        // [readonly, shouldInit, shouldUpdate]
        const readonlyTrue = [
          [true, false, () => {}],
          [true, () => {}, () => {}]
        ]

        for (const [readonly, shouldInit, shouldUpdate] of readonlyTrue) {
          const toPass = fx({
            dependentProp: {
              default: '',
              readonly,
              shouldInit,
              shouldUpdate,
              validator
            }
          })

          expectNoFailure(toPass)

          toPass()
        }

        const toPass = fx({
          dependentProp: {
            default: '',
            readonly: 'lax',
            shouldUpdate: () => {},
            validator
          }
        })

        expectNoFailure(toPass)

        toPass()
      })

      describe('behaviour', () => {
        let onSuccessValues: any = {}
        let onSuccessStats: any = {}

        function incrementOnSuccessCountOf(prop: string) {
          return ({ context }: any) => {
            const previousCount = onSuccessStats[prop] ?? 0

            onSuccessStats[prop] = previousCount + 1
            onSuccessValues[prop] = context[prop]
          }
        }

        const Model = new Schema({
          dependentProp: {
            default: false,
            dependent: true,
            dependsOn: 'virtual',
            resolver: ({ context }: any) => context.virtual,
            onSuccess: incrementOnSuccessCountOf('dependentProp')
          },
          dependentProp_1: {
            default: false,
            dependent: true,
            dependsOn: 'virtual_1',
            resolver: ({ context }: any) => context.virtual_1,
            onSuccess: incrementOnSuccessCountOf('dependentProp_1')
          },
          laxProp: {
            default: '',
            readonly: 'lax',
            shouldUpdate: (ctx: any) => ctx.laxProp_1 == 'test',
            onSuccess: incrementOnSuccessCountOf('laxProp')
          },
          laxProp_1: { default: 'dev' },
          virtual: {
            virtual: true,
            shouldUpdate: false,
            validator: () => ({ valid: true }),
            onSuccess: incrementOnSuccessCountOf('virtual')
          },
          virtual_1: {
            virtual: true,
            shouldUpdate: (ctx: any) => ctx.laxProp_1 == 'test',
            validator: () => ({ valid: true }),
            onSuccess: incrementOnSuccessCountOf('virtual_1')
          }
        }).getModel()

        afterEach(() => {
          onSuccessValues = {}
          onSuccessStats = {}
        })

        it("should not update properties when 'shouldUpdate' resolved to 'false'", async () => {
          const { data, error } = await Model.update(
            {
              dependentProp: 'dev',
              dependentProp_1: 'dev',
              laxProp: '',
              laxProp_1: ''
            },
            { laxProp: 'yoyo', virtual: true, virtual_1: true }
          )

          expect(data).toBe(null)
          expect(error.message).toBe(ERRORS.NOTHING_TO_UPDATE)
        })

        it("should update properties when 'shouldUpdate' resolved to 'true'", async () => {
          const { data, error, handleSuccess } = await Model.update(
            {
              dependentProp: 'dev',
              dependentProp_1: 'dev',
              laxProp: '',
              laxProp_1: 'test'
            },
            { laxProp: 'yoyo', virtual: true, virtual_1: true }
          )

          await handleSuccess()

          expect(error).toBe(null)
          expect(data).toEqual({ dependentProp_1: true, laxProp: 'yoyo' })

          expect(onSuccessStats).toEqual({
            dependentProp_1: 1,
            laxProp: 1,
            virtual_1: 1
          })

          expect(onSuccessValues).toEqual({
            dependentProp_1: true,
            laxProp: 'yoyo',
            virtual_1: true
          })
        })

        it("should not update readonly properties that have changed even when 'shouldUpdate' resolved to 'true'", async () => {
          const { data, error } = await Model.update(
            {
              dependentProp: 'dev',
              dependentProp_1: 'dev',
              laxProp: 'changed',
              laxProp_1: 'test'
            },
            { laxProp: 'yoyo' }
          )

          expect(data).toBe(null)
          expect(error.message).toBe(ERRORS.NOTHING_TO_UPDATE)
        })

        describe('behaviour when shouldUpdate method returns nothing', () => {
          const Model = new Schema({
            isBlocked: { default: false, shouldUpdate: () => {} },
            laxProp: { default: 0 }
          }).getModel()

          it('should assume updatability of a property as falsy if shouldInit method returns nothing', async () => {
            const { data, error } = await Model.update(
              { isBlocked: false, laxProp: 0 },
              { isBlocked: true }
            )

            expect(data).toBe(null)
            expect(error).toMatchObject({
              message: ERRORS.NOTHING_TO_UPDATE,
              payload: {}
            })
          })
        })
      })
    })

    describe('invalid', () => {
      it('should reject shouldUpdate !(false | () => boolean)', () => {
        const invalidValues = [
          true,
          1,
          0,
          -1,
          'true',
          'false',
          [],
          null,
          undefined,
          {}
        ]

        for (const shouldUpdate of invalidValues) {
          const toFail = fx({ propertyName: { default: '', shouldUpdate } })

          expectFailure(toFail)

          try {
            toFail()
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "'shouldUpdate' only accepts false or a function that returns a boolean"
                ])
              })
            )
          }
        }
      })

      it('should reject shouldUpdate(false) & shouldInit(false)', () => {
        const toFail = fx({
          propertyName: {
            default: '',
            shouldInit: false,
            shouldUpdate: false
          }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                "Both 'shouldInit' & 'shouldUpdate' cannot be 'false'"
              ])
            })
          )
        }
      })

      it('should reject shouldUpdate(false) for non-virtuals', () => {
        const toFail = fx({
          propertyName: { default: '', shouldUpdate: false }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                "Only 'Virtuals' are allowed to have 'shouldUpdate' as 'false'"
              ])
            })
          )
        }
      })

      it('should reject shouldUpdate & readonly(true) & no shouldInit', () => {
        const toFail = fx({
          propertyName: {
            default: '',
            readonly: true,
            shouldUpdate: () => {}
          }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                "Cannot block the update of 'readonly' properties that do not have initialization('shouldInit') blocked. Either add 'shouldInit' or use readonly: 'lax'"
              ])
            })
          )
        }
      })
    })
  })
}
