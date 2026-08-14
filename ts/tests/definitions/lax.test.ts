import { describe, it } from 'bun:test';
import { expectNoFailure, makeFx } from '../_utils';

describe('lax props', () => {
  describe('valid', () => {
    it('should allow default alone', () => {
      const toPass = makeFx((b) => b.field(b.lax('fieldName', '')));

      expectNoFailure(toPass);

      toPass();
    });

    it('should allow default + validator', () => {
      const toPass = makeFx((b) =>
        b.field(b.lax('fieldName', '').validate(() => ({ valid: true }))),
      );

      expectNoFailure(toPass);

      toPass();
    });
  });
});
