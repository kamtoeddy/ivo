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

  describe('escapeHTML', () => {
    it('should escape html characters by default', () => {
      const validate = makeStringValidator();

      expect(validate('<b>Tom & "Jerry"</b> it\'s')).toEqual({
        valid: true,
        validated: '&lt;b&gt;Tom &amp; &quot;Jerry&quot;&lt;/b&gt; it&#39;s',
      });
      expect(validate('<script>alert(1)</script>')).toEqual({
        valid: true,
        validated: '&lt;script&gt;alert(1)&lt;/script&gt;',
      });
    });

    it('should leave strings without html characters as they are', () => {
      expect(makeStringValidator()('plain text / 100%')).toEqual({
        valid: true,
        validated: 'plain text / 100%',
      });
    });

    it('should not escape when escapeHTML is false', () => {
      const value = '<b>Tom & Jerry</b>';

      expect(makeStringValidator({ escapeHTML: false })(value)).toEqual({
        valid: true,
        validated: value,
      });
    });

    it('should apply length limits to the escaped text', () => {
      const validate = makeStringValidator({ max: 5 });

      expect(validate('<')).toEqual({ valid: true, validated: '&lt;' });
      expect(validate('<&')).toEqual({
        valid: false,
        reason: 'too_long',
        metadata: { max: 5, min: 1 },
      });
      expect(makeStringValidator({ escapeHTML: false, max: 5 })('<&')).toEqual({
        valid: true,
        validated: '<&',
      });
    });

    it('should escape after trimming', () => {
      expect(makeStringValidator({ trim: true })('  <a>  ')).toEqual({
        valid: true,
        validated: '&lt;a&gt;',
      });
    });

    it('should not escape allowed values', () => {
      const validate = makeStringValidator({ allow: ['<a>', '<b>'] });

      expect(validate('<a>')).toEqual({ valid: true, validated: '<a>' });
    });
  });
});
