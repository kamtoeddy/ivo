export const Test_Validators = ({ Schema }: any) => {
  describe('Model.validate', () => {
    describe('should respect the validator provided', () => {
      const Model = new Schema({
        prop: {
          default: '',
          validator(value: any) {
            return value == 'valid'
              ? { valid: true }
              : { valid: false, reason: 'Invalid prop' }
          }
        }
      }).getModel()

      it('should return the correct invalid message on validation failure', async () => {
        const res = await Model.validate('prop', 'yoo')

        expect(res).toEqual({ valid: false, reasons: ['Invalid prop'] })
      })

      it('should respect the validator provided', async () => {
        const res = await Model.validate('prop', 'valid')

        expect(res).toEqual({ valid: true, validated: 'valid' })
      })
    })

    describe('should respect the validators that return booleans', () => {
      const Model = new Schema({
        prop: {
          default: '',
          validator: (value: any) => value == 'valid'
        }
      }).getModel()

      it('should return the correct invalid message on validation failure', async () => {
        const res = await Model.validate('prop', 'yoo')

        expect(res).toEqual({ valid: false, reasons: ['validation failed'] })
      })

      it('should respect the validator provided', async () => {
        const res = await Model.validate('prop', 'valid')

        expect(res).toEqual({ valid: true, validated: 'valid' })
      })
    })

    describe('Behaviour with invalid user validation response', () => {
      const invalidResponses = [
        null,
        undefined,
        -100,
        0,
        14,
        '',
        'Invalid response',
        []
      ]

      for (const response of invalidResponses) {
        const Model = new Schema({
          prop: { default: '', validator: () => response }
        }).getModel()

        it("should fail validation with 'validation failed' message when value returned from validator is invalid instead of crashing", async () => {
          const res = await Model.validate('prop', 'yoo')

          expect(res).toEqual({ valid: false, reasons: ['validation failed'] })
        })
      }
    })

    describe('OtherReasons', () => {
      it('should add corresponding properties and error messages passed as otherReasons', async () => {
        const messages = [
          ['Invalid Prop', ['Invalid Prop']],
          [['Invalid Prop'], ['Invalid Prop']]
        ]

        for (const [input, output] of messages) {
          const Model = new Schema({
            prop: { default: '' },
            prop2: {
              required: true,
              validator() {
                return { valid: false, otherReasons: { prop: input } }
              }
            }
          }).getModel()

          const { data, error } = await Model.create({})

          expect(data).toBe(null)
          expect(error).toMatchObject({
            message: 'Validation Error',
            payload: { prop: output }
          })
        }
      })

      it('should ignore keys of otherReasons that are not properties, aliases or are constants or dependents', async () => {
        const Model = new Schema({
          constant: { constant: true, value: 1 },
          dependent: {
            default: '',
            dependent: true,
            dependsOn: 'prop',
            resolver: () => ''
          },
          prop: {
            required: true,
            validator() {
              return {
                valid: false,
                otherReasons: {
                  constant: 'invalid constant',
                  dependent: 'invalid dependent',
                  invalidProp: 'invalid prop'
                }
              }
            }
          }
        }).getModel()

        const { data, error } = await Model.create({})

        expect(data).toBe(null)
        expect(error).toMatchObject({
          message: 'Validation Error',
          payload: { prop: ['validation failed'] }
        })
      })

      it('should ignore non-strings or array values passed to otherReasons', async () => {
        const invalidMessages = [
          null,
          true,
          false,
          undefined,
          -100,
          0,
          14,
          {},
          () => {},
          [() => {}]
        ]

        for (const message of invalidMessages) {
          const Model = new Schema({
            prop1: { default: '' },
            prop: {
              required: true,
              validator() {
                return { valid: false, otherReasons: { prop1: message } }
              }
            }
          }).getModel()

          const { data, error } = await Model.create({})

          expect(data).toBe(null)
          expect(error).toEqual({
            message: 'Validation Error',
            payload: {
              prop: ['validation failed'],
              prop1: ['validation failed']
            },
            statusCode: 400
          })
        }
      })
    })
  })
}
