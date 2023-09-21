import { expectFailure, expectNoFailure, getValidSchema } from '../_utils'

export const Test_SchemaOptionFormat = ({ fx }: any) => {
  describe('Schema.options', () => {
    it('should allow empty objects', () => {
      const toPass = fx(getValidSchema(), {})

      expectNoFailure(toPass)

      toPass()
    })

    it('should reject non-object values', () => {
      const values = [null, false, true, 1, 'abc', []]

      for (const options of values) {
        const toFail = fx(getValidSchema(), options)

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              'schema options': expect.arrayContaining(['Must be an object'])
            })
          )
        }
      }
    })

    it('should reject invalid option name', () => {
      const toFail = fx(getValidSchema(), { propertyName: true })

      expectFailure(toFail)

      try {
        toFail()
      } catch (err: any) {
        expect(err.payload).toEqual(
          expect.objectContaining({
            propertyName: expect.arrayContaining(['Invalid option'])
          })
        )
      }
    })
  })
}
