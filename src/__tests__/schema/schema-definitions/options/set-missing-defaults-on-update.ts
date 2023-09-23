import { ERRORS } from '../../../..'
import {
  expectFailure,
  expectNoFailure,
  getValidSchema,
  validator
} from '../_utils'

export const Test_SchemaSetMissingDefaultsOnUpdateOption = ({
  Schema,
  fx
}: any) => {
  describe('Schema.options.setMissingDefaultsOnUpdate', () => {
    describe('behaviour', () => {
      it('should not generate default values if "setMissingDefaultsOnUpdate" is false | undefined', async () => {
        for (const setMissingDefaultsOnUpdate of [false, undefined]) {
          let summary: any

          const Model = new Schema(
            {
              id: { constant: true, value: 1 },
              dependent: {
                default: '',
                dependent: true,
                dependsOn: 'lax',
                resolver: ({ context }: any) => context.lax
              },
              lax: { default: 'lax', validator },
              prop: { default: '', validator }
            },
            { setMissingDefaultsOnUpdate, onSuccess: (s: any) => (summary = s) }
          ).getModel()

          const previousValues = { prop: '' }
          const updates = { prop: 'Prop updated' }

          const { data, handleSuccess } = await Model.update(
            previousValues,
            updates
          )

          await handleSuccess()

          expect(data).toEqual(updates)
          expect(summary).toEqual({
            changes: updates,
            context: updates,
            operation: 'update',
            previousValues,
            values: updates
          })
        }
      })

      describe('setMissingDefaultsOnUpdate: true', () => {
        let summary: any

        const Model = new Schema(
          {
            id: { constant: true, value: 1 },
            dependent: {
              default: '',
              dependent: true,
              dependsOn: 'lax',
              resolver: ({ context }: any) => context.lax
            },
            dependent_1: {
              default: '',
              dependent: true,
              dependsOn: 'dependent',
              resolver: ({ context }: any) => context.dependent
            },
            lax: { default: 'lax', validator },
            prop: { default: '', validator }
          },
          {
            setMissingDefaultsOnUpdate: true,
            onSuccess: (s: any) => (summary = s)
          }
        ).getModel()

        it('should generate default values if "setMissingDefaultsOnUpdate" is true', async () => {
          const previousValues = { prop: '' }
          const updates = { prop: 'Prop updated' }
          const changes = {
            dependent: '',
            dependent_1: '',
            lax: 'lax',
            ...updates
          }

          const { data, handleSuccess } = await Model.update(
            previousValues,
            updates
          )

          await handleSuccess()

          expect(data).toEqual(changes)
          expect(summary).toEqual({
            changes,
            context: changes,
            operation: 'update',
            previousValues,
            values: changes
          })
        })

        it('generated values should not overwrite updates provided', async () => {
          const previousValues = {}
          const updates = { lax: 'lax updated' }
          const changes = {
            dependent: updates.lax,
            dependent_1: updates.lax,
            prop: '',
            ...updates
          }

          const { data, handleSuccess } = await Model.update(
            previousValues,
            updates
          )

          await handleSuccess()

          expect(data).toEqual(changes)
          expect(summary).toEqual({
            changes,
            context: changes,
            operation: 'update',
            previousValues,
            values: changes
          })
        })
      })
    })

    describe('valid', () => {
      it("should allow 'setMissingDefaultsOnUpdate' as boolean", () => {
        const values = [true, false]

        for (const setMissingDefaultsOnUpdate of values) {
          const toPass = fx(getValidSchema(), { setMissingDefaultsOnUpdate })

          expectNoFailure(toPass)

          toPass()
        }
      })
    })

    describe('invalid', () => {
      it("should reject 'setMissingDefaultsOnUpdate' other than boolean or function", () => {
        const invalidValues = [1, 0, -14, {}, [], 'invalid', '', null]

        for (const setMissingDefaultsOnUpdate of invalidValues) {
          const toFail = fx(getValidSchema(), { setMissingDefaultsOnUpdate })

          expectFailure(toFail)

          try {
            toFail()
          } catch (err: any) {
            expect(err).toMatchObject({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                setMissingDefaultsOnUpdate: expect.arrayContaining([
                  "'setMissingDefaultsOnUpdate' should be a 'boolean'"
                ])
              }
            })
          }
        }
      })
    })
  })
}
