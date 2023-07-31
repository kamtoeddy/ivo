// import { any } from "../../validate/isNumberOk";

export const isNumberOkTest = ({ isNumberOk }: { isNumberOk: Function }) => {
  describe('isNumberOk', () => {
    it('should tell whether input is a number or not', () => {
      // truthy values
      const truthyValues = [-45, 0, 0.23, 1, 100]

      for (let value of truthyValues) {
        const res = isNumberOk(value)

        expect(res).toMatchObject({ valid: true, validated: value })

        expect(res.reason).toBeUndefined()
        expect(res.reasons).toBeUndefined()
      }

      // fasly values that will be parsed
      const falsyButParsed = ['-1', '', '1']

      const parsedByValueMap: Record<string, number> = {
        '-1': -1,
        '': 0,
        '1': 1
      }

      for (let value of falsyButParsed) {
        const res = isNumberOk(value)

        expect(res).toMatchObject({
          valid: true,
          validated: parsedByValueMap[value]
        })

        expect(res.reason).toBeUndefined()
        expect(res.reasons).toBeUndefined()
      }

      // falsy values
      const falsyValues = [false, true, 'hey', NaN, null, undefined, [], {}]

      for (let value of falsyValues) {
        const res = isNumberOk(value)

        expect(res).toMatchObject({
          reasons: ['Expected a number'],
          valid: false
        })

        expect(res.validated).toBeUndefined()
      }
    })

    it('should tell whether input is in range specified', () => {
      // truthy values
      const zero_Twelve: any = { bounds: [0, 12] }
      const zero_Twelve_Ex: any = {
        bounds: [0, 12],
        inclusiveBottom: false,
        inclusiveTop: false
      }
      const zero_Twelve_ExBottom: any = {
        bounds: [0, 12],
        inclusiveBottom: false
      }
      const zero_Twelve_ExTop: any = {
        bounds: [0, 12],
        inclusiveTop: false
      }

      const pairsToPass: [number, any][] = [
        [0, zero_Twelve],
        [5, zero_Twelve],
        [10.75, zero_Twelve],
        [12, zero_Twelve],

        // exclusive top & bottom
        [0.0001, zero_Twelve_Ex],
        [5, zero_Twelve_Ex],
        [10.75, zero_Twelve_Ex],
        [11.99, zero_Twelve_Ex],

        // exclusive bottom
        [0.0001, zero_Twelve_ExBottom],
        [5, zero_Twelve_ExBottom],
        [10.75, zero_Twelve_ExBottom],
        [12, zero_Twelve_ExBottom],

        // exclusive top
        [0, zero_Twelve_ExTop],
        [5, zero_Twelve_ExTop],
        [10.75, zero_Twelve_ExTop],
        [11.99, zero_Twelve_ExTop]
      ]

      pairsToPass.forEach(([num, range]) => {
        const res = isNumberOk(num, { range })

        expect(res).toMatchObject({ valid: true, validated: num })

        expect(res.reason).toBeUndefined()
        expect(res.reasons).toBeUndefined()
      })

      // falsy values
      const pairsToFail: [number, any, string][] = [
        [-1, zero_Twelve, 'Too small'],
        [12.01, zero_Twelve, 'Too large'],

        // exclusive top & bottom
        [0, zero_Twelve_Ex, 'Too small'],
        [12, zero_Twelve_Ex, 'Too large'],

        // exclusive bottom
        [-1, zero_Twelve_ExBottom, 'Too small'],
        [0, zero_Twelve_ExBottom, 'Too small'],

        // exclusive top
        [12, zero_Twelve_ExTop, 'Too large'],
        [12.01, zero_Twelve_ExTop, 'Too large']
      ]

      pairsToFail.forEach(([num, range, error]) => {
        const res = isNumberOk(num, { range })

        expect(res).toMatchObject({
          reasons: [error],
          valid: false
        })

        expect(res.validated).toBeUndefined()
      })
    })
  })
}
