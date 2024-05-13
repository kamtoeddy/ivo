import { describe, it, expect } from 'bun:test';

export const validateBooleanTest = ({
  validateBoolean,
}: {
  validateBoolean: Function;
}) => {
  describe('validateBoolean', () => {
    it('should tell whether or not input values are booleans', () => {
      // truthy values

      const truthyValues = [false, true];

      for (let value of truthyValues) {
        const res = validateBoolean(value);

        expect(res).toMatchObject({ valid: true, validated: value });

        expect(res.reason).toBeUndefined();
      }

      // falsy values

      const falsyValues = ['true', 'false', 1, 0, null, undefined, [], {}, NaN];

      for (let value of falsyValues) {
        const res = validateBoolean(value);

        expect(res).toMatchObject({
          reason: ['Expected a boolean'],
          valid: false,
        });

        expect(res.validated).toBeUndefined();
      }
    });
  });
};
