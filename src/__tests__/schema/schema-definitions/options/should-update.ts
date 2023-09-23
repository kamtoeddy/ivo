import { ERRORS } from '../../../..'
import {
  expectFailure,
  expectNoFailure,
  getValidSchema,
  validator
} from '../_utils'

export const Test_SchemaShouldUpdateOption = ({ Schema, fx }: any) => {
  describe('Schema.options.shouldUpdate', () => {
    describe('behaviour', () => {
      const book = { name: 'book name', price: 10 }
      const update = { name: 'updated name' }

      const error = {
        message: ERRORS.NOTHING_TO_UPDATE,
        payload: {}
      }
      const options = [
        { data: update, error: null, shouldUpdate: true },
        { data: null, error, shouldUpdate: false },
        { data: null, error, shouldUpdate: () => false },
        { data: update, error: null, shouldUpdate: () => true },
        // falsy values
        { data: null, error, shouldUpdate: () => null },
        { data: null, error, shouldUpdate: () => undefined },
        { data: null, error, shouldUpdate: () => {} },
        { data: null, error, shouldUpdate: () => 0 },
        { data: null, error, shouldUpdate: () => '' },
        // truthy values
        { data: update, error: null, shouldUpdate: () => 1 },
        { data: update, error: null, shouldUpdate: () => '1' },
        { data: update, error: null, shouldUpdate: () => [] },
        { data: update, error: null, shouldUpdate: () => ({}) }
      ]

      it('should respect the "shouldUpdate" rule accordingly', async () => {
        for (const option of options) {
          const Model = new Schema(
            {
              name: { required: true, validator },
              price: { required: true, validator }
            },
            { shouldUpdate: option.shouldUpdate }
          ).getModel()

          const { data, error } = await Model.update(book, update)

          expect(data).toEqual(option.data)
          expect(error).toEqual(option.error)
        }
      })
    })

    describe('valid', () => {
      it("should allow 'shouldUpdate' as boolean | () => boolean", () => {
        const values = [true, false, () => {}]

        for (const shouldUpdate of values) {
          const toPass = fx(getValidSchema(), { shouldUpdate })

          expectNoFailure(toPass)

          toPass()
        }
      })
    })

    describe('invalid', () => {
      it("should reject 'shouldUpdate' other than boolean or function", () => {
        const invalidValues = [
          1,
          0,
          -14,
          {},
          [],
          'invalid',
          '',
          null,
          undefined
        ]

        for (const shouldUpdate of invalidValues) {
          const toFail = fx(getValidSchema(), { shouldUpdate })

          expectFailure(toFail)

          try {
            toFail()
          } catch (err: any) {
            expect(err).toMatchObject({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                shouldUpdate: expect.arrayContaining([
                  "'shouldUpdate' should either be a 'boolean' or a 'function'"
                ])
              }
            })
          }
        }
      })
    })
  })
}
