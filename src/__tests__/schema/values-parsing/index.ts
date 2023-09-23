import { ERRORS } from '../../..'

export const valuesParsing_Tests = ({ Schema }: any) => {
  const expectPromiseFailure = (
    fx: Function,
    message: string = ERRORS.INVALID_SCHEMA
  ) => {
    expect(fx).rejects.toThrow(message)
  }

  const expectNoFailure = (fx: Function) => expect(fx).not.toThrow()

  describe('Values Parsing', () => {
    const INVALID_DATA_ERROR = { message: ERRORS.INVALID_DATA, payload: {} }
    const validData = { age: 15, name: 'Frank' }
    const invalidData = [1, -10, 0, false, true, '', 'true', null]

    describe('with silent errors', () => {
      let User: any

      beforeAll(async () => {
        User = new Schema({
          age: { default: 10 },
          id: { constant: true, value: 1 },
          name: { default: '' }
        }).getModel()
      })

      describe('valid data', () => {
        it('should set values properly during cloning', async () => {
          const cloned = { ...validData, id: 1 }

          const { data, error } = await User.clone(cloned)

          expect(error).toBe(null)

          expect(data).toEqual(cloned)
        })

        it('should allow for create method of model to be empty', async () => {
          const { data, error } = await User.create()

          expect(error).toBe(null)

          expect(data).toEqual({ age: 10, id: 1, name: '' })
        })

        it('should set values properly at creation', async () => {
          const { data, error } = await User.create(validData)

          expect(error).toBe(null)

          expect(data).toEqual({ ...validData, id: 1 })
        })

        it('should set values properly during deletion', async () => {
          expectNoFailure(
            async () => await User.delete({ ...validData, id: 1 })
          )
        })

        it('should set values properly during updates', async () => {
          const user = { ...validData, id: 1 }
          const name = 'Mike'

          const { data, error } = await User.update(user, { name })

          expect(error).toBe(null)

          expect(data).toEqual({ name })
        })
      })

      describe('invalid data', () => {
        it('should reject invalid data during cloning', async () => {
          for (const val of invalidData) {
            const operation = async () => await User.clone(val)

            expectNoFailure(operation)

            const { data, error } = await operation()

            expect(data).toBe(null)

            expect(error).toEqual(INVALID_DATA_ERROR)
          }
        })

        it('should reject invalid data at creation', async () => {
          for (const val of invalidData) {
            const operation = async () => await User.create(val)

            expectNoFailure(operation)

            const { data, error } = await operation()

            expect(data).toBe(null)

            expect(error).toEqual(INVALID_DATA_ERROR)
          }
        })

        it('should reject invalid data during deletion', async () => {
          for (const val of invalidData) {
            const operation = async () => await User.delete(val)

            expectPromiseFailure(operation, ERRORS.INVALID_DATA)

            try {
              await operation()
            } catch (err: any) {
              expect(err).toMatchObject(INVALID_DATA_ERROR)
            }
          }
        })

        it('should reject invalid data during updates', async () => {
          for (const val of invalidData) {
            const operation = async () =>
              await User.update(val, { name: 'yoo' })

            expectNoFailure(operation)

            const { data, error } = await operation()

            expect(data).toBe(null)

            expect(error).toEqual(INVALID_DATA_ERROR)
          }
        })
      })
    })

    describe('with thrown errors', () => {
      let User: any

      beforeAll(async () => {
        User = new Schema(
          {
            age: { default: 10 },
            id: { constant: true, value: 1 },
            name: { default: '' }
          },
          { errors: 'throw' }
        ).getModel()
      })

      describe('valid data', () => {
        it('should set values properly during cloning', async () => {
          const cloned = { ...validData, id: 1 }

          const { data, error } = await User.clone(cloned)

          expect(error).toBe(null)

          expect(data).toEqual(cloned)
        })

        it('should set values properly at creation', async () => {
          const { data, error } = await User.create(validData)

          expect(error).toBe(null)

          expect(data).toEqual({ ...validData, id: 1 })
        })

        it('should set values properly during deletion', async () => {
          expectNoFailure(
            async () => await User.delete({ ...validData, id: 1 })
          )
        })

        it('should set values properly during updates', async () => {
          const user = { ...validData, id: 1 }
          const name = 'Mike'

          const { data, error } = await User.update(user, { name })

          expect(error).toBe(null)

          expect(data).toEqual({ name })
        })
      })

      describe('invalid data', () => {
        it('should reject invalid data during cloning', async () => {
          for (const val of invalidData) {
            const operation = async () => await User.clone(val)

            expectPromiseFailure(operation, ERRORS.INVALID_DATA)

            try {
              await operation()
            } catch (err: any) {
              expect(err).toMatchObject(INVALID_DATA_ERROR)
            }
          }
        })

        it('should reject invalid data at creation', async () => {
          for (const val of invalidData) {
            const operation = async () => await User.create(val)

            expectPromiseFailure(operation, ERRORS.INVALID_DATA)

            try {
              await operation()
            } catch (err: any) {
              expect(err).toMatchObject(INVALID_DATA_ERROR)
            }
          }
        })

        it('should reject invalid data during deletion', async () => {
          for (const val of invalidData) {
            const operation = async () => await User.delete(val)

            expectPromiseFailure(operation, ERRORS.INVALID_DATA)

            try {
              await operation()
            } catch (err: any) {
              expect(err).toMatchObject(INVALID_DATA_ERROR)
            }
          }
        })

        it('should reject invalid data during updates', async () => {
          for (const val of invalidData) {
            const operation = async () =>
              await User.update(val, { name: 'yoo' })

            expectPromiseFailure(operation, ERRORS.INVALID_DATA)

            try {
              await operation()
            } catch (err: any) {
              expect(err).toMatchObject(INVALID_DATA_ERROR)
            }
          }
        })
      })
    })
  })
}
