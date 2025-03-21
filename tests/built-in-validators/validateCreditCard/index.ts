import { describe, expect, it } from 'bun:test';

export const validateCreditCardTest = ({
  validateCreditCard,
}: {
  validateCreditCard: Function;
}) => {
  describe('validateCreditCard', () => {
    const truthyValues = [
      [5420596721435293, 5420596721435293],
      ['5420596721435293', '5420596721435293'],
      ['5420596721435293 ', '5420596721435293'],
      ['5420596721435293 ', '5420596721435293'],
    ];

    it('should tell whether a character is a valid credit card number', () => {
      for (const [value, validated] of truthyValues) {
        const res = validateCreditCard(value);

        expect(res).toEqual({ valid: true, validated });

        expect(res.reason).toBeUndefined();
      }

      // falsy tests
      const falsyValues = [
        -1000,
        0,
        1,
        '',
        '123-2342-25-6750',
        4187622910505690,
        '4187622910505690',
      ];

      falsyValues.forEach((value) => {
        const res = validateCreditCard(value);

        expect(res).toEqual({
          reason: 'Invalid card number',
          valid: false,
          metadata: null,
        });

        expect(res.validated).toBeUndefined();
      });
    });

    it('should validated value should be of same type as input value', () => {
      truthyValues.forEach(([value]) => {
        expect(typeof validateCreditCard(value).validated).toBe(typeof value);
      });
    });
  });
};
