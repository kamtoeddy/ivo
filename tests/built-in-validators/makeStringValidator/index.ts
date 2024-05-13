import { describe, it, expect } from 'bun:test';

export const makeStringValidatorTest = ({
  makeStringValidator,
}: {
  makeStringValidator: Function;
}) => {
  describe('makeStringValidator', () => {
    it('should tell whether input is a valid string or not', () => {
      const truthy = [
        'I',
        'am',
        'very',
        'delighted',
        ' valid string with spaces ',
        'valid string with at the end  ',
        '  valid string with spaces infront',
        Array(40 + 1).join('a'),
      ];

      for (const value of truthy) {
        const res = makeStringValidator()(value);

        expect(res).toMatchObject({ valid: true, validated: value });

        expect(res.reason).toBeUndefined();
      }

      const falsy = [
        [null, ['Expected a string']],
        [undefined, ['Expected a string']],
        ['', ['too_short'], { max: 255, min: 1 }],
        [Array(257).join('a'), ['too_long'], { max: 255, min: 1 }],
      ];

      for (const [value, reason, metadata = null] of falsy) {
        const res = makeStringValidator()(value);

        expect(res).toMatchObject({ reason, valid: false, metadata });

        expect(res.validated).toBeUndefined();
      }
    });

    it('should respect min & max options', () => {
      const falsy = [
        {
          valid: true,
          validated: 'valid',
          value: 'valid',
        },
        {
          valid: true,
          value: '_'.repeat(20),
          options: { max: 21, min: 20 },
        },
        {
          valid: true,
          value: '_'.repeat(1),
          options: { min: 1 },
        },
        {
          value: '',
          reason: ['too_short'],
          metadata: { max: 255, min: 1 },
        },
        {
          value: '',
          reason: ['too_short'],
          options: { min: 1 },
          metadata: { max: 255, min: 1 },
        },
        {
          value: '_'.repeat(256),
          reason: ['too_long'],
          metadata: { max: 255, min: 1 },
        },
        {
          value: '_'.repeat(251),
          reason: ['too_long'],
          options: { max: 250 },
          metadata: { max: 250, min: 1 },
        },
      ];

      for (const {
        valid = false,
        value,
        validated = value,
        reason = [],
        options = {},
        metadata = null,
      } of falsy) {
        const res = makeStringValidator(options)(value);

        if (valid) expect(res).toMatchObject({ valid, validated });
        else expect(res).toMatchObject({ reason, valid, metadata });
      }
    });

    it('should not cast numbers to strings', () => {
      const res = makeStringValidator()(1);

      expect(res).toMatchObject({
        valid: false,
        reason: expect.arrayContaining(['Expected a string']),
      });

      expect(res.validated).toBeUndefined();
    });

    it('should accept only enumerated values if any', () => {
      const allow = ['admin', 'moderator', 'user'];

      for (const value of allow) {
        const res = makeStringValidator({ allow })(value);

        expect(res).toMatchObject({ valid: true, validated: value });

        expect(res.reason).toBeUndefined();
      }

      const falsy = ['Admin', 'ADMIN', 'superadmin', 'Moderators'];

      for (const value of falsy) {
        const res = makeStringValidator({ allow })(value);

        expect(res).toMatchObject({
          metadata: { allowed: allow },
          reason: ['Value not allowed'],
          valid: false,
        });

        expect(res.validated).toBeUndefined();
      }
    });

    it('should respect nullable if provided', () => {
      const data = [
        [' admin', ' admin'],
        ['admin', 'admin'],
        ['', null],
        [null, null],
        [undefined, null],
      ];

      for (const [value, validated] of data) {
        const res = makeStringValidator({ nullable: true })(value);

        expect(res).toMatchObject({ valid: true, validated });

        expect(res.reason).toBeUndefined();
      }
    });

    it('should reject excluded values', () => {
      const valueToBeExcluded = { exclude: '0' };
      const valuesToBeExcluded = { exclude: ['0', '1', '2'] };
      const valueToBeExcludedWithError = {
        exclude: { values: '0', error: '"0" (zero) is not allowed here' },
      };
      const valuesToBeExcludedWithError = {
        exclude: {
          values: ['0', '1', '2'],
          error: '"0", "1" & "2" are not allowed',
        },
      };

      const data = [
        ['0', valueToBeExcluded, 'Value not allowed', { excluded: ['0'] }],
        [
          '0',
          valuesToBeExcluded,
          'Value not allowed',
          { excluded: valuesToBeExcluded.exclude },
        ],
        [
          '1',
          valuesToBeExcluded,
          'Value not allowed',
          { excluded: valuesToBeExcluded.exclude },
        ],
        [
          '2',
          valuesToBeExcluded,
          'Value not allowed',
          { excluded: valuesToBeExcluded.exclude },
        ],
        [
          valueToBeExcludedWithError.exclude.values,
          valueToBeExcludedWithError,
          valueToBeExcludedWithError.exclude.error,
          { excluded: [valueToBeExcludedWithError.exclude.values] },
        ],
        [
          '0',
          valuesToBeExcludedWithError,
          valuesToBeExcludedWithError.exclude.error,
          { excluded: valuesToBeExcludedWithError.exclude.values },
        ],
        [
          '1',
          valuesToBeExcludedWithError,
          valuesToBeExcludedWithError.exclude.error,
          { excluded: valuesToBeExcludedWithError.exclude.values },
        ],
        [
          '2',
          valuesToBeExcludedWithError,
          valuesToBeExcludedWithError.exclude.error,
          { excluded: valuesToBeExcludedWithError.exclude.values },
        ],
      ];

      data.forEach(([num, options, error, metadata]) => {
        const res = makeStringValidator(options)(num);

        if (error) {
          expect(res).toMatchObject({
            valid: false,
            reason: [error],
            metadata,
          });

          expect(res.validated).toBeUndefined();

          return;
        }

        expect(res).toMatchObject({ valid: true, validated: num });

        expect(res.reason).toBeUndefined();
      });
    });

    it('should trim strings only when trim option is passed', () => {
      const data = [
        [' admin', 'admin'],
        ['moderator ', 'moderator'],
        [' user ', 'user'],
      ];

      for (const [value, validated] of data) {
        const res = makeStringValidator({ trim: true })(value);

        expect(res).toMatchObject({ valid: true, validated });

        expect(res.reason).toBeUndefined();
      }

      for (const [value] of data) {
        const res = makeStringValidator()(value);

        expect(res).toMatchObject({ valid: true, validated: value });

        expect(res.reason).toBeUndefined();
      }
    });

    it('should accept values that match a regular expression', () => {
      const regExp = /^[a-zA-Z]+$/;

      const truthy = ['admin', 'Admin', 'ADMIN', 'moderator', 'user'];

      for (const value of truthy) {
        const res = makeStringValidator({
          regExp: { value: regExp, error: 'Value not allowed' },
        })(value);

        expect(res).toMatchObject({ valid: true, validated: value });

        expect(res.reason).toBeUndefined();
      }

      const falsy = ['12', '%%', '.  ', '__'];

      for (const value of falsy) {
        const res = makeStringValidator({
          regExp: { value: regExp, error: 'Value not allowed' },
        })(value);

        expect(res).toMatchObject({
          reason: ['Value not allowed'],
          valid: false,
        });

        expect(res.validated).toBeUndefined();
      }
    });
  });
};
