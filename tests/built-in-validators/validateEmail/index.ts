import { describe, expect, it } from 'bun:test';

export const validateEmailTest = ({
  validateEmail,
}: {
  validateEmail: Function;
}) => {
  describe('validateEmail', () => {
    it('should tell whether input is a valid email or not', () => {
      const truthy = [
        'example@gmail.com',
        'james71@hotmail.co.uk',
        ' james71@hotmail.co.uk',
      ];

      for (const value of truthy) {
        const res = validateEmail(value);

        expect(res).toMatchObject({ valid: true, validated: value.trim() });

        expect(res.reason).toBeUndefined();
      }

      const falsy = [1, null, false, '', '@gmail.com', 'james71@..uk'];

      for (const value of falsy) {
        const res = validateEmail(value);

        expect(res).toMatchObject({ reason: 'Invalid email', valid: false });

        expect(res.validated).toBeUndefined();
      }
    });

    it('should respect custom regular expression', () => {
      const regExp = /\w+@\w.\w/;

      const truthy = [
        'example@gmail.com',
        'james71@hotmail.co.uk',
        ' james71@hotmail.co.uk',
      ];

      for (const value of truthy) {
        const res = validateEmail(value, regExp);

        expect(res).toMatchObject({ valid: true, validated: value.trim() });

        expect(res.reason).toBeUndefined();
      }

      const falsy = [1, null, false, '', '@gmail.com', 'james71@..uk'];

      for (const value of falsy) {
        const res = validateEmail(value);

        expect(res).toMatchObject({ reason: 'Invalid email', valid: false });

        expect(res.validated).toBeUndefined();
      }
    });
  });
};
