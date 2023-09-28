import { describe, it, expect } from 'vitest'

import { ERRORS } from '../../../..'
import { expectFailure } from '../_utils'

export const Test_BasicDefinitions = ({ fx }: any) => {
  describe('Schema definitions', () => {
    it('should reject if property definitions is not an object', () => {
      const values = [
        null,
        undefined,
        new Number(),
        new String(),
        Symbol(),
        2,
        -10,
        true,
        []
      ]

      for (const value of values) expectFailure(fx(value))
    })

    it('should reject if property definitions has no property', () => {
      const toFail = fx({})

      expectFailure(toFail)

      try {
        toFail()
      } catch (err: any) {
        expect(err.payload).toMatchObject({
          'schema properties': ['Insufficient Schema properties']
        })
      }
    })

    it("should reject if a property's definition is an empty object", () => {
      const toFail = fx({ emptyProp: {} })
      expectFailure(toFail)

      try {
        toFail()
      } catch (err: any) {
        expect(err).toEqual(
          expect.objectContaining({
            message: ERRORS.INVALID_SCHEMA,
            payload: {
              emptyProp: [
                'A property should at least be readonly, required, or have a default value'
              ]
            }
          })
        )
      }
    })

    it("should reject if a property's definition is not an object", () => {
      const invalidDefinitions = [true, false, [], 1, -1, '', 'invalid']

      for (const definition of invalidDefinitions) {
        const toFail = fx({ invalidProp0000: definition })
        expectFailure(toFail)

        try {
          toFail()
        } catch (err: any) {
          expect(err).toEqual(
            expect.objectContaining({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                invalidProp0000: [
                  `Invalid property definition. Expected an object '{}' but received '${typeof definition}'`
                ]
              }
            })
          )
        }
      }
    })

    it("should reject if a property's definition has an invalid rule", () => {
      const toFail = fx({ emptyProp: { default: '', yoo: true } })
      expectFailure(toFail)

      try {
        toFail()
      } catch (err: any) {
        expect(err).toEqual(
          expect.objectContaining({
            message: ERRORS.INVALID_SCHEMA,
            payload: {
              emptyProp: expect.arrayContaining(["'yoo' is not a valid rule"])
            }
          })
        )
      }
    })
  })
}
