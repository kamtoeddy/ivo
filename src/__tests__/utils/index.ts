type CommonUtilsProps = { [key: string]: Function }

export const commonUtilTests = ({
  isOneOf,
  getUnique,
  getUniqueBy,
  isEqual
}: CommonUtilsProps) => {
  describe('belongsTo', () => {
    it('should return true if value passed is in array supplied else false', () => {
      const values = [1, 'hey', null, undefined, false]

      // truthy tests
      expect(isOneOf(1, values)).toBe(true)
      expect(isOneOf('hey', values)).toBe(true)
      expect(isOneOf(null, values)).toBe(true)
      expect(isOneOf(undefined, values)).toBe(true)
      expect(isOneOf(false, values)).toBe(true)

      // falsy tests
      expect(isOneOf('1', values)).toBe(false)
      expect(isOneOf('Hey', values)).toBe(false)
      expect(isOneOf('null', values)).toBe(false)
      expect(isOneOf('undefined', values)).toBe(false)
      expect(isOneOf('false', values)).toBe(false)
      expect(isOneOf(2, values)).toBe(false)
      expect(isOneOf(true, values)).toBe(false)
    })
  })

  describe('isEqual', () => {
    it('should return true if a and b are equal else false', () => {
      // truthy
      expect(isEqual(1, 1)).toEqual(true)
      expect(isEqual({}, {})).toEqual(true)
      expect(isEqual([], [])).toEqual(true)
      expect(isEqual(undefined, undefined)).toEqual(true)
      expect(isEqual([1, 'true', [], null], [1, 'true', [], null])).toEqual(
        true
      )
      expect(isEqual({ name: 'James' }, { name: 'James' })).toEqual(true)

      // falsy
      expect(isEqual(1, '1')).toEqual(false)
      expect(isEqual({}, '1')).toEqual(false)
      expect(isEqual([1, 'true', []], [1, 'true', '[]'])).toEqual(false)
      expect(isEqual([1, 'true', [], null], [1, 'true', null, []])).toEqual(
        false
      )
      expect(isEqual({ name: 'James' }, { name: 'JameS' })).toEqual(false)
      expect(isEqual({ name: 'James' }, { name: 'James', age: 17 })).toEqual(
        false
      )
    })
  })

  describe('getUnique', () => {
    it('should return an array of unique values', () => {
      const values = [
        11,
        1,
        { name: 'James' },
        { name: 'Mary' },
        2,
        { name: 'James' },
        1
      ]

      expect(getUnique([]).length).toBe(0)
      expect(getUnique(values).length).toBe(5)
    })
  })

  describe('getUniqueBy', () => {
    it('should return an array of unique values without a key', () => {
      const values = [
        11,
        1,
        { name: 'James' },
        { name: 'Mary' },
        2,
        { name: 'James' },
        1
      ]

      expect(getUniqueBy([]).length).toBe(0)
      expect(getUniqueBy(values).length).toBe(5)
    })

    it('should return an array of unique values with a key', () => {
      const values = [{ name: 'James' }, { name: 'Mary' }, { name: 'James' }]

      expect(getUniqueBy(values, 'name').length).toBe(2)
      expect(getUniqueBy(values, 'age').length).toBe(1)
    })
  })
}
