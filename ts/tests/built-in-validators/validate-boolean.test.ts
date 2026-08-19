import { describe, expect, it } from 'bun:test';
import { validateBoolean } from '../../src';

describe('validateBoolean', () => {
  it('should tell whether or not input values are booleans', () => {
    // truthy values

    const truthyValues = [false, true];

    for (const value of truthyValues) {
      const res = validateBoolean(value);

      expect(res).toEqual({ valid: true, validated: value });
    }

    // falsy values

    const falsyValues = [
      'true',
      'false',
      1,
      0,
      null,
      undefined,
      [],
      {},
      Number.NaN,
    ];

    for (const value of falsyValues) {
      const res = validateBoolean(value);

      expect(res).toEqual({
        reason: 'Expected a boolean',
        valid: false,
        metadata: null,
      });
    }
  });
});
