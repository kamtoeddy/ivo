import { describe, expect, it } from 'vitest'

import { VALIDATION_ERRORS } from '../../../../dist'

import { expectPromiseFailure } from '../_utils'

export const Test_ExtendedSchemas = ({ Schema }: any) => {
  describe('Extended Schema', () => {
    describe('Options', () => {
      describe('errors', () => {
        it('should respect "errors" option from baseSchema if enabled', async () => {
          const Model = new Schema(
            { id: { constant: true, value: 1 } },
            { errors: 'throw' }
          )
            .extend({ name: { default: '', validator: () => false } })
            .getModel()

          const toFail = () => Model.create({ name: 'lol' })

          expectPromiseFailure(toFail, VALIDATION_ERRORS.VALIDATION_ERROR)
        })

        it('should respect overwritten "errors" option in child schema', async () => {
          const Model = new Schema(
            { id: { constant: true, value: 1 } },
            { errors: 'silent' }
          )
            .extend({ name: { default: '', validator: () => false } })
            .getModel()

          const { data, error } = await Model.create({ name: 'lol' })

          expect(data).toBeNull()
          expect(error).not.toBeNull()
        })
      })

      describe('setMissingDefaultsOnUpdate', () => {
        it('should respect "setMissingDefaultsOnUpdate" option if enabled in base schema', async () => {
          const Model = new Schema(
            { id: { constant: true, value: 1 } },
            { setMissingDefaultsOnUpdate: true }
          )
            .extend({ age: { default: 12 }, name: { default: '' } })
            .getModel()

          const { data, error } = await Model.update(
            { id: 1, name: '' },
            { name: 'updated' }
          )

          expect(error).toBeNull()
          expect(data).toMatchObject({ age: 12, name: 'updated' })
        })

        it('should respect "setMissingDefaultsOnUpdate" option if not enabled in base schema', async () => {
          const Model = new Schema({ id: { constant: true, value: 1 } }, {})
            .extend({ age: { default: 12 }, name: { default: '' } })
            .getModel()

          const { data, error } = await Model.update(
            { id: 1, name: '' },
            { name: 'updated' }
          )

          expect(error).toBeNull()
          expect(data.age).toBeUndefined()
          expect(data).toMatchObject({ name: 'updated' })
        })

        it('should respect "setMissingDefaultsOnUpdate" option if overwritten in child schema', async () => {
          const Model = new Schema(
            { id: { constant: true, value: 1 } },
            { setMissingDefaultsOnUpdate: true }
          )
            .extend(
              { age: { default: 12 }, name: { default: '' } },
              { setMissingDefaultsOnUpdate: false }
            )
            .getModel()

          const { data, error } = await Model.update(
            { id: 1, name: '' },
            { name: 'updated' }
          )

          expect(error).toBeNull()
          expect(data.age).toBeUndefined()
          expect(data).toMatchObject({ name: 'updated' })
        })
      })

      describe('shouldUpdate', () => {
        it('should ignore "shouldUpdate" option even if provided in base schema', async () => {
          const Model = new Schema(
            { id: { constant: true, value: 1 } },
            { shouldUpdate: false }
          )
            .extend({ name: { default: '' } })
            .getModel()

          const { data, error } = await Model.update(
            { id: 1, name: '' },
            { name: 'updated' }
          )

          expect(error).toBeNull()
          expect(data).not.toBeNull()
        })
      })

      describe('timestamps', () => {
        it('should respect "timestamps" option from baseSchema if enabled', async () => {
          const Model = new Schema(
            { id: { constant: true, value: 1 } },
            { timestamps: { updatedAt: 'u_At' } }
          )
            .extend({ name: { default: '' } })
            .getModel()

          const { data, error } = await Model.create()

          expect(error).toBeNull()
          expect(data).toMatchObject({ id: 1, name: '' })
          expect(data.createdAt).toBeDefined()
          expect(data.u_At).toBeDefined()
        })

        it('should respect "timestamps" option from baseSchema if not enabled', async () => {
          const Model = new Schema(
            { id: { constant: true, value: 1 } },
            { timestamps: false }
          )
            .extend({ name: { default: '' } })
            .getModel()

          const { data, error } = await Model.create()

          expect(error).toBeNull()
          expect(data).toMatchObject({ id: 1, name: '' })
          expect(data.createdAt).toBeUndefined()
          expect(data.updatedAt).toBeUndefined()
        })

        it('should respect overwritten "timestamps" option from baseSchema', async () => {
          const Model = new Schema(
            { id: { constant: true, value: 1 } },
            { timestamps: { createdAt: 'c_at', updatedAt: 'uAt' } }
          )
            .extend({ name: { default: '' } })
            .getModel()

          const { data, error } = await Model.create()

          expect(error).toBeNull()
          expect(data).toMatchObject({ id: 1, name: '' })
          expect(data.c_at).toBeDefined()
          expect(data.uAt).toBeDefined()
        })
      })
    })
  })
}
