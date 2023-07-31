import { expectFailure, expectNoFailure } from '../_utils'

export const Test_LaxProperties = ({ fx }: any) => {
  describe('lax props', () => {
    describe('valid', () => {
      it('should allow default alone', () => {
        const toPass = fx({ propertyName: { default: '' } })

        expectNoFailure(toPass)

        toPass()
      })

      it('should allow default + validator', () => {
        const toPass = fx({
          propertyName: { default: '', validator: () => ({ valid: true }) }
        })

        expectNoFailure(toPass)

        toPass()
      })
    })

    describe('invalid', () => {
      it('should reject no default', () => {
        const toFail = fx({
          propertyName: { validator: () => ({ valid: true }) }
        })

        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                'A property should at least be readonly, required, or have a default value'
              ])
            })
          )
        }
      })
    })
  })
}
