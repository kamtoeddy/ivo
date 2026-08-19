import { describe, expect, it } from 'bun:test';
import { makeStringValidator } from '../../src';

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

      expect(res).toEqual({ valid: true, validated: value });
    }

    const falsy = [
      [null, 'Expected a string'],
      [undefined, 'Expected a string'],
      ['', 'too_short', { max: 255, min: 1 }],
      [Array(257).join('a'), 'too_long', { max: 255, min: 1 }],
    ];

    for (const [value, reason, metadata = null] of falsy) {
      const res = makeStringValidator()(value);

      // @ts-expect-error ikr
      expect(res).toEqual({ reason, valid: false, metadata });
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
        reason: 'too_short',
        metadata: { max: 255, min: 1 },
      },
      {
        value: '',
        reason: 'too_short',
        options: { min: 1 },
        metadata: { max: 255, min: 1 },
      },
      {
        value: '_'.repeat(256),
        reason: 'too_long',
        metadata: { max: 255, min: 1 },
      },
      {
        value: '_'.repeat(251),
        reason: 'too_long',
        options: { max: 250 },
        metadata: { max: 250, min: 1 },
      },
    ];

    for (const {
      valid = false,
      value,
      validated = value,
      reason = '',
      options = {},
      metadata = null,
    } of falsy) {
      const res = makeStringValidator(options)(value);

      if (valid) expect(res).toEqual({ valid, validated });
      else expect(res).toEqual({ reason, valid, metadata });
    }
  });

  it('should not cast numbers to strings', () => {
    const value = 1;
    const res = makeStringValidator()(value);

    expect(res).toEqual({
      valid: false,
      reason: 'Expected a string',
      metadata: null,
    });
  });

  it('should accept only enumerated values if any', () => {
    const allow = ['admin', 'moderator', 'user'] as const;

    for (const value of allow) {
      const res = makeStringValidator({ allow })(value);

      expect(res).toEqual({ valid: true, validated: value });
    }

    const falsy = ['Admin', 'ADMIN', 'superadmin', 'Moderators'];

    for (const value of falsy) {
      const res = makeStringValidator({ allow })(value);

      expect(res).toEqual({
        metadata: { allowed: allow },
        reason: 'Value not allowed',
        valid: false,
      });
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

      // @ts-expect-error ikr
      expect(res).toEqual({ valid: true, validated });
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
      // @ts-expect-error ikr
      const res = makeStringValidator(options)(num);

      if (error) {
        // @ts-expect-error ikr
        expect(res).toEqual({
          valid: false,
          reason: error,
          metadata,
        });

        return;
      }

      // @ts-expect-error ikr
      expect(res).toEqual({ valid: true, validated: num });
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

      expect(res).toEqual({ valid: true, validated });
    }

    for (const [value] of data) {
      const res = makeStringValidator()(value);

      expect(res).toEqual({ valid: true, validated: value });
    }
  });

  it('should accept values that match a regular expression', () => {
    const regExp = /^[a-zA-Z]+$/;

    const truthy = ['admin', 'Admin', 'ADMIN', 'moderator', 'user'];

    for (const value of truthy) {
      const res = makeStringValidator({
        regExp: { value: regExp, error: 'Value not allowed' },
      })(value);

      expect(res).toEqual({ valid: true, validated: value });
    }

    const falsy = ['12', '%%', '.  ', '__'];

    for (const value of falsy) {
      const res = makeStringValidator({
        regExp: { value: regExp, error: 'Value not allowed' },
      })(value);

      expect(res).toEqual({
        valid: false,
        reason: 'Value not allowed',
        metadata: null,
      });
    }
  });
});
