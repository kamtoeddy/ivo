import { describe, expect, it } from 'bun:test';
import { validateEmail } from '../../src';

describe('validateEmail', () => {
  it('should tell whether input is a valid email or not', () => {
    const truthy = [
      'example@gmail.com',
      'james71@hotmail.co.uk',
      ' james71@hotmail.co.uk',
    ];

    for (const value of truthy) {
      const res = validateEmail(value);

      expect(res).toEqual({ valid: true, validated: value.trim() });
    }

    const falsy = [1, null, false, '', '@gmail.com', 'james71@..uk'];

    for (const value of falsy) {
      const res = validateEmail(value);

      expect(res).toEqual({
        reason: 'Invalid email',
        valid: false,
        metadata: null,
      });
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

      // @ts-expect-error ikr
      expect(res.reason).toBeUndefined();
    }

    const falsy = [1, null, false, '', '@gmail.com', 'james71@..uk'];

    for (const value of falsy) {
      const res = validateEmail(value);

      expect(res).toMatchObject({ reason: 'Invalid email', valid: false });

      // @ts-expect-error ikr
      expect(res.validated).toBeUndefined();
    }
  });
});
