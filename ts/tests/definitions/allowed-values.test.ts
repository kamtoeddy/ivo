import { describe, expect, it } from 'bun:test';
import { Schema } from '../../src';
import { expectFailure, expectNoFailure, makeFx, validator } from '../_utils';

describe('allowed values', () => {
  describe('valid', () => {
    it('should not reject if allowed values provided are >= 2', () => {
      const values = [
        ['lol', 2],
        ['lol', 2, 3],
      ];

      for (const allow of values) {
        const toPass = makeFx((b) =>
          b.field(b.lax('field', allow[0]).allow(allow as never)),
        );

        expectNoFailure(toPass);

        toPass();
      }
    });

    it('should not reject if default value provided is an allowed value', () => {
      const toPass = makeFx((b) =>
        b.field(b.lax('field', null).allow([null, 'lolz', -1])),
      );

      expectNoFailure(toPass);

      toPass();
    });

    it('should allow virtuals to have allowed values', () => {
      const toPass = makeFx((b) =>
        b
          .field(
            b
              .dependent('dependent', 'virtual')
              .default(true)
              .resolve(validator as never),
          )

          .field(b.virtual('virtual').allow([null, 'lolz', -1])),
      );

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
        {},
      ];

      for (const allow of values) {
        const toFail = makeFx((b) =>
          b.field(b.lax('field', null).allow(allow as never)),
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject({
            field: ['Allowed values must be an array'],
          });
        }
      }
    });

    it('should reject if allowed values provided are less than 2', () => {
      const values = [[], ['lol']];

      for (const allow of values) {
        const toFail = makeFx((b) =>
          b.field(b.lax('field', null).allow(allow as never)),
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject({
            field: ['Allowed values must have at least 2 values'],
          });
        }
      }
    });
  });

  describe('behaviour', () => {
    const metadata = { allowed: [null, 'allowed'] };

    describe('behaviour with lax props & no validators', () => {
      const Model = new Schema<any>((b) =>
        b.field(b.lax('field', null).allow(metadata.allowed as never)),
      ).getModel();

      describe('creation', () => {
        it('should allow if value provided is allowed', async () => {
          const { data, error } = await Model.create({ field: 'allowed' }, {});

          expect(error).toBeNull();
          expect(data).toMatchObject({ field: 'allowed' });
        });

        it('should reject if value provided is not allowed', async () => {
          const { data, error } = await Model.create({ field: true }, {});

          expect(data).toBeNull();
          expect(error).toMatchObject({
            field: expect.objectContaining({
              reason: 'value not allowed',
              metadata,
            }),
          });
        });
      });

      describe('updates', () => {
        it('should allow if value provided is allowed', async () => {
          const { data, error } = await Model.update(
            { field: 'allowed' },
            { field: null },
            {},
          );

          expect(error).toBeNull();
          expect(data).toMatchObject({ field: null });
        });

        it('should reject if value provided is not allowed', async () => {
          const { data, error } = await Model.update(
            { field: null },
            { field: true },
            {},
          );

          expect(data).toBeNull();
          expect(error?.payload).toMatchObject({
            field: expect.objectContaining({
              reason: 'value not allowed',
              metadata,
            }),
          });
        });
      });
    });

    describe('behaviour with required props & no validators', () => {
      const Model = new Schema<any>((b) =>
        b.field(b.required('field').allow(metadata.allowed as never)),
      ).getModel();

      describe('creation', () => {
        it('should accept allowed values if provided', async () => {
          const { data, error } = await Model.create({ field: null }, {});

          expect(error).toBeNull();
          expect(data).toEqual({ field: null });
        });

        it('should reject non-allowed values if provided', async () => {
          const { data, error } = await Model.create({ field: 'lolz' }, {});

          expect(data).toBeNull();
          expect(error).toMatchObject({
            field: expect.objectContaining({
              reason: 'value not allowed',
              metadata,
            }),
          });
        });

        it('should reject if no value is provided', async () => {
          const { data, error } = await Model.create({}, {});

          expect(data).toBeNull();
          expect(error).toMatchObject({
            field: expect.objectContaining({
              reason: "'field' is required",
              metadata: null,
            }),
          });
        });
      });

      describe('updates', () => {
        it('should accept allowed values if provided', async () => {
          const { data, error } = await Model.update(
            { field: 'allowed' },
            { field: null },
            {},
          );

          expect(error).toBeNull();
          expect(data).toEqual({ field: null });
        });

        it('should reject non-allowed values if provided', async () => {
          const { data, error } = await Model.update(
            { field: 'allowed' },
            { field: 'whatever' },
            {},
          );

          expect(data).toBeNull();
          expect(error?.payload).toMatchObject({
            field: expect.objectContaining({
              reason: 'value not allowed',
              metadata,
            }),
          });
        });
      });
    });
  });
});
