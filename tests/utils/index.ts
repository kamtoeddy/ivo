import { describe, it, expect } from 'bun:test';

type CommonUtilsProps = { [key: string]: Function };

export const commonUtilTests = ({
  isOneOf,
  getUnique,
  getUniqueBy,
  isEqual,
  isNullOrUndefined,
}: CommonUtilsProps) => {
  describe('utils', () => {
    describe('belongsTo', () => {
      it('should return true if value passed is in array supplied else false', () => {
        const values = [1, 'hey', null, undefined, false];

        // truthy tests
        expect(isOneOf(1, values)).toBe(true);
        expect(isOneOf('hey', values)).toBe(true);
        expect(isOneOf(null, values)).toBe(true);
        expect(isOneOf(undefined, values)).toBe(true);
        expect(isOneOf(false, values)).toBe(true);

        // falsy tests
        expect(isOneOf('1', values)).toBe(false);
        expect(isOneOf('Hey', values)).toBe(false);
        expect(isOneOf('null', values)).toBe(false);
        expect(isOneOf('undefined', values)).toBe(false);
        expect(isOneOf('false', values)).toBe(false);
        expect(isOneOf(2, values)).toBe(false);
        expect(isOneOf(true, values)).toBe(false);
      });
    });

    describe('isEqual', () => {
      it('should return true if a and b are equal else false', () => {
        // truthy
        const now = new Date();
        expect(isEqual(now, now)).toBe(true);
        expect(isEqual(1, 1)).toBe(true);
        expect(isEqual({}, {})).toBe(true);
        expect(isEqual([], [])).toBe(true);
        expect(isEqual(undefined, undefined)).toBe(true);
        expect(isEqual([1, 'true', [], null], [1, 'true', [], null])).toBe(
          true,
        );
        expect(isEqual({ a: 'James' }, { a: 'James' })).toBe(true);
        expect(isEqual({ a: '' }, { a: '' })).toBe(true);
        expect(isEqual({ a: '', b: '' }, { a: '', b: '' })).toBe(true);
        expect(isEqual({ a: '', b: '' }, { b: '', a: '' })).toBe(true);
        expect(isEqual({ a: '', b: { c: '' } }, { b: { c: '' }, a: '' })).toBe(
          true,
        );

        // falsy
        expect(
          isEqual(now, new Date(new Date(now).setHours(now.getHours() + 10))),
        ).toBe(false);
        expect(isEqual(1, '1')).toBe(false);
        expect(isEqual({}, '1')).toBe(false);
        expect(isEqual([1, 'true', []], [1, 'true', '[]'])).toBe(false);
        expect(isEqual([1, 'true', [], null], [1, 'true', null, []])).toBe(
          false,
        );
        expect(isEqual({ a: 'James' }, { a: 'JameS' })).toBe(false);
        expect(isEqual({ a: 'James' }, { a: 'James', b: 17 })).toBe(false);
      });

      it('should respect the level of nesting(depth)', () => {
        // depth == undefined (defaults to 1)

        for (const depth of [undefined, 1]) {
          expect(
            isEqual({ a: '', b: { c: '' } }, { b: { c: '' }, a: '' }, depth),
          ).toEqual(true);

          expect(
            isEqual({ a: '', b: [1, 2] }, { b: [1, 2], a: '' }, depth),
          ).toEqual(true);

          expect(
            isEqual(
              { a: '', b: { c: '', d: [1, 2] } },
              { b: { d: [1, 2], c: '' }, a: '' },
              depth,
            ),
          ).toEqual(true);

          expect(
            isEqual(
              { a: '', b: { d: '', c: '' } },
              { b: { c: '', d: '' }, a: '' },
              depth,
            ),
          ).toEqual(true);
        }

        // depth == 0
        expect(
          isEqual({ a: '', b: { c: '' } }, { b: { c: '' }, a: '' }, 0),
        ).toEqual(true);

        expect(isEqual({ a: '', b: [1, 2] }, { b: [1, 2], a: '' }, 0)).toEqual(
          true,
        );

        expect(
          isEqual(
            { a: '', b: { c: '', d: [1, 2] } },
            { b: { d: [1, 2], c: '' }, a: '' },
            0,
          ),
        ).toEqual(false);

        expect(
          isEqual(
            { a: '', b: { d: '', c: '' } },
            { b: { c: '', d: '' }, a: '' },
            0,
          ),
        ).toEqual(false);

        for (const depth of [2, 3, Infinity]) {
          expect(
            isEqual({ a: '', b: { c: '' } }, { b: { c: '' }, a: '' }, depth),
          ).toEqual(true);

          expect(
            isEqual({ a: '', b: [1, 2] }, { b: [1, 2], a: '' }, depth),
          ).toEqual(true);

          expect(
            isEqual(
              { a: '', b: { c: '', d: [1, 2] } },
              { b: { d: [1, 2], c: '' }, a: '' },
              depth,
            ),
          ).toEqual(true);

          expect(
            isEqual(
              { a: '', b: { d: '', c: '' } },
              { b: { c: '', d: '' }, a: '' },
              depth,
            ),
          ).toEqual(true);
        }
      });
    });

    describe('getUnique', () => {
      it('should return an array of unique values', () => {
        const values = [
          11,
          1,
          { name: 'James' },
          { name: 'Mary' },
          2,
          { name: 'James' },
          1,
        ];

        expect(getUnique([]).length).toBe(0);
        expect(getUnique(values).length).toBe(5);
      });
    });

    describe('getUniqueBy', () => {
      it('should return an array of unique values without a key', () => {
        const values = [
          11,
          1,
          { name: 'James' },
          { name: 'Mary' },
          2,
          { name: 'James' },
          1,
        ];

        expect(getUniqueBy([]).length).toBe(0);
        expect(getUniqueBy(values).length).toBe(5);
      });

      it('should return an array of unique values with a key', () => {
        const values = [{ name: 'James' }, { name: 'Mary' }, { name: 'James' }];

        expect(getUniqueBy(values, 'name').length).toBe(2);
        expect(getUniqueBy(values, 'age').length).toBe(1);
      });
    });

    describe('isNullOrUndefined', () => {
      it('should respect null & undefined', () => {
        const values = [null, undefined];

        for (const value of values) expect(isNullOrUndefined(value)).toBe(true);
      });

      it('should reject !(null | undefined)', () => {
        const values = [-1, 0, 1, 2, true, false, [], {}];

        for (const value of values)
          expect(isNullOrUndefined(value)).toBe(false);
      });
    });
  });
};
