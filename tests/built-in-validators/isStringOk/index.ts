import { describe, it, expect } from 'vitest';

function getStringOfLength(length: number) {
  return Array(length).fill('a').join('');
}

export const isStringOkTest = ({ isStringOk }: { isStringOk: Function }) => {
  describe('isStringOk', () => {
    it('should tell whether input is a valid string or not', () => {
      const truthy = [
        'I',
        'am',
        'very',
        'delighted',
        ' valid string with spaces ',
        'valid string with at the end  ',
        '  valid string with spaces infront',
        Array(40 + 1).join('a')
      ];

      for (const value of truthy) {
        const res = isStringOk(value);

        expect(res).toMatchObject({ valid: true, validated: value });

        expect(res.reason).toBeUndefined();
        expect(res.reasons).toBeUndefined();
      }

      const falsy = [
        [null, ['Unacceptable value']],
        [undefined, ['Unacceptable value']],
        ['', ['Too short'], { maxLength: 255, minLength: 1 }],
        [Array(257).join('a'), ['Too long'], { maxLength: 255, minLength: 1 }]
      ];

      for (const [value, reasons, metadata = null] of falsy) {
        const res = isStringOk(value);

        expect(res).toMatchObject({ reasons, valid: false, metadata });

        expect(res.validated).toBeUndefined();
      }
    });

    it('should respect minLength & maxLength options', () => {
      const falsy = [
        {
          valid: true,
          validated: 'valid',
          value: 'valid'
        },
        {
          valid: true,
          value: getStringOfLength(20),
          options: { maxLength: 21, minLength: 20 }
        },
        {
          valid: true,
          value: getStringOfLength(1),
          options: { maxLength: 1, minLength: 2 },
          metadata: { maxLength: 1, minLength: 1 }
        },
        {
          valid: true,
          value: getStringOfLength(1),
          options: { maxLength: 0, minLength: 1 }
        },
        {
          value: '',
          reasons: ['Too short'],
          metadata: { maxLength: 255, minLength: 1 }
        },
        {
          value: '',
          reasons: ['Too short'],
          options: { minLength: 1 },
          metadata: { maxLength: 255, minLength: 1 }
        },
        {
          value: getStringOfLength(256),
          reasons: ['Too long'],
          metadata: { maxLength: 255, minLength: 1 }
        },
        {
          value: getStringOfLength(251),
          reasons: ['Too long'],
          options: { maxLength: 250 },
          metadata: { maxLength: 250, minLength: 1 }
        }
      ];

      for (const {
        valid = false,
        value,
        validated = value,
        reasons = [],
        options = {},
        metadata = null
      } of falsy) {
        const res = isStringOk(value, options);

        if (valid) expect(res).toMatchObject({ valid, validated });
        else expect(res).toMatchObject({ reasons, valid, metadata });
      }
    });

    it('should cast numbers to strings', () => {
      const res = isStringOk(1);

      expect(res).toMatchObject({ valid: true, validated: '1' });

      expect(res.reason).toBeUndefined();
      expect(res.reasons).toBeUndefined();
    });

    it('should accept only enumerated values if any', () => {
      const allow = ['admin', 'moderator', 'user'];

      for (const value of allow) {
        const res = isStringOk(value, { allow });

        expect(res).toMatchObject({ valid: true, validated: value });

        expect(res.reason).toBeUndefined();
        expect(res.reasons).toBeUndefined();
      }

      const falsy = ['Admin', 'ADMIN', 'superadmin', 'Moderators'];

      for (const value of falsy) {
        const res = isStringOk(value, { allow });

        expect(res).toMatchObject({
          metadata: { allowed: allow },
          reasons: ['Unacceptable value'],
          valid: false
        });

        expect(res.validated).toBeUndefined();
      }
    });

    it('should trim strings only when trim option is passed', () => {
      const data = [
        [' admin', 'admin'],
        ['moderator ', 'moderator'],
        [' user ', 'user']
      ];

      for (const [value, validated] of data) {
        const res = isStringOk(value, { trim: true });

        expect(res).toMatchObject({ valid: true, validated });

        expect(res.reason).toBeUndefined();
        expect(res.reasons).toBeUndefined();
      }

      for (const [value] of data) {
        const res = isStringOk(value);

        expect(res).toMatchObject({ valid: true, validated: value });

        expect(res.reason).toBeUndefined();
        expect(res.reasons).toBeUndefined();
      }
    });

    it('should accept values that match a regular expression', () => {
      const regExp = /^[a-zA-Z]+$/;

      const truthy = ['admin', 'Admin', 'ADMIN', 'moderator', 'user'];

      for (const value of truthy) {
        const res = isStringOk(value, { regExp });

        expect(res).toMatchObject({ valid: true, validated: value });

        expect(res.reason).toBeUndefined();
        expect(res.reasons).toBeUndefined();
      }

      const falsy = ['12', '%%', '.  ', '__'];

      for (const value of falsy) {
        const res = isStringOk(value, { regExp });

        expect(res).toMatchObject({
          reasons: ['Unacceptable value'],
          valid: false
        });

        expect(res.validated).toBeUndefined();
      }
    });
  });
};
