import { describe, it, expect } from 'vitest';

import { expectFailure, expectNoFailure, validator } from '../_utils';

export const Test_EnumeratedProperties = ({ fx }: any) => {
  describe('allow rule', () => {
    describe('valid', () => {
      it('should not reject if allowed values provided are >= 2', () => {
        const values = [
          ['lol', 2],
          ['lol', 2, 3]
        ];

        for (const allow of values) {
          const toPass = fx({ prop: { default: allow[0], allow } });

          expectNoFailure(toPass);

          toPass();
        }
      });

      it('should not reject if default value provided is an allowed value', () => {
        const toPass = fx({
          prop: { default: null, allow: [null, 'lolz', -1] }
        });

        expectNoFailure(toPass);

        toPass();
      });

      it('should allow virtuals to have allowed values', () => {
        const toPass = fx({
          dependent: {
            default: true,
            dependsOn: 'virtual',
            resolver: validator
          },
          virtual: { virtual: true, allow: [null, 'lolz', -1], validator }
        });

        expectNoFailure(toPass);

        toPass();
      });
    });

    describe('invalid', () => {
      it('should reject if non-array value is provided', () => {
        const values = [
          null,
          undefined,
          new Number(),
          new String(),
          Symbol(),
          2,
          -10,
          true,
          () => {},
          {}
        ];

        for (const allow of values) {
          const toFail = fx({ prop: { allow } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              prop: ['Allowed values must be an array']
            });
          }
        }
      });

      it('should reject if allowed values provided are not unique', () => {
        const values = [
          [1, 2, 2, 4, 5],
          ['lol', 59, 'lol', null],
          [true, false, true],
          [{}, {}],
          [{ id: 'lol' }, { id: 'lol' }]
        ];

        for (const allow of values) {
          const toFail = fx({ prop: { allow } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              prop: ['Allowed values must be an array of unique values']
            });
          }
        }
      });

      it('should reject if allowed values provided are less than 2', () => {
        const values = [[], ['lol']];

        for (const allow of values) {
          const toFail = fx({ prop: { allow } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              prop: ['Allowed values must have at least 2 values']
            });
          }
        }
      });

      it('should reject if default value provided is not an allowed value', () => {
        const values = [
          ['lol', [null, 'lolz', -1]],
          [null, [1, 4, 'lol', undefined]]
        ];

        for (const [_default, allow] of values) {
          const toFail = fx({ prop: { default: _default, allow } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              prop: ['The default value must be an allowed value']
            });
          }
        }
      });
    });
  });
};
