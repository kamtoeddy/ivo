import { describe, expect, it } from 'bun:test';
import { makeNumberValidator } from '../../src';

describe('makeNumberValidator', () => {
  it('should tell whether input is a number or not', () => {
    // truthy values
    const truthyValues = [-45, 0, 0.23, 1, 100];

    for (const value of truthyValues) {
      const res = makeNumberValidator()(value);

      expect(res).toEqual({ valid: true, validated: value });
    }

    // falsy values
    const falsyValues = [
      false,
      true,
      'hey',
      Number.NaN,
      null,
      undefined,
      [],
      {},
    ];

    for (const value of falsyValues) {
      const res = makeNumberValidator()(value);

      expect(res).toEqual({
        reason: 'Expected a number',
        valid: false,
        metadata: null,
      });
    }
  });

  it('should reject values < min or values > max', () => {
    // truthy values
    const valid = [0, 1, 9, 10];

    const options = { min: 0, max: 10 };

    valid.forEach((num) => {
      const res = makeNumberValidator({ min: 0, max: 10 })(num);

      expect(res).toEqual({ valid: true, validated: num });
    });

    // falsy values
    const invalid = [
      [-0.000001, 'too_small'],
      [-1, 'too_small'],
      [10.00001, 'too_big'],
    ];

    invalid.forEach(([num, error]) => {
      const res = makeNumberValidator({ min: 0, max: 10 })(num);

      // @ts-expect-error ikr
      expect(res).toEqual({
        valid: false,
        reason: error,
        metadata: options,
      });
    });
  });

  it('should reject excluded values', () => {
    const valueToBeExcluded = { exclude: 0 };
    const valuesToBeExcluded = { exclude: [0, 1, 2] };
    const valueToBeExcludedWithError = {
      exclude: { values: 0, error: '0 (zero) is not allowed here' },
    };
    const valuesToBeExcludedWithError = {
      exclude: { values: [0, 1, 2], error: '0, 1 & 2 are not allowed' },
    };

    const data = [
      [0, valueToBeExcluded, 'Value not allowed', { excluded: [0] }],
      [
        0,
        valuesToBeExcluded,
        'Value not allowed',
        { excluded: valuesToBeExcluded.exclude },
      ],
      [
        1,
        valuesToBeExcluded,
        'Value not allowed',
        { excluded: valuesToBeExcluded.exclude },
      ],
      [
        2,
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
        0,
        valuesToBeExcludedWithError,
        valuesToBeExcludedWithError.exclude.error,
        { excluded: valuesToBeExcludedWithError.exclude.values },
      ],
      [
        1,
        valuesToBeExcludedWithError,
        valuesToBeExcludedWithError.exclude.error,
        { excluded: valuesToBeExcludedWithError.exclude.values },
      ],
      [
        2,
        valuesToBeExcludedWithError,
        valuesToBeExcludedWithError.exclude.error,
        { excluded: valuesToBeExcludedWithError.exclude.values },
      ],
    ];

    data.forEach(([num, options, error, metadata]) => {
      // @ts-expect-error ikr
      const res = makeNumberValidator(options)(num);

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

  it('should accept only enumerated values if any', () => {
    const allow = [1, 55, 3, 17, 0, -15] as const;

    for (const value of allow) {
      const res = makeNumberValidator({ allow })(value);

      expect(res).toEqual({ valid: true, validated: value });
    }

    const falsy = [2, 10, 11, undefined];

    for (const value of falsy) {
      const res = makeNumberValidator({ allow })(value);

      expect(res).toEqual({
        metadata: { allowed: allow },
        reason: 'Value not allowed',
        valid: false,
      });
    }
  });
});

it('should respect nullable if provided', () => {
  const data = [
    [1, 1],
    [50, 50],
    [null, null],
    [undefined, null],
    [-4, -4],
  ];

  for (const [value, validated] of data) {
    const res = makeNumberValidator({ nullable: true })(value);

    // @ts-expect-error ikr
    expect(res).toEqual({ valid: true, validated });
  }
});
