import { describe, expect, it } from 'bun:test';
import { expectFailure, expectNoFailure, validator } from '../_utils';

export const Test_SchemaOptionIgnore = ({
  Schema,
  fx,
}: {
  Schema: any;
  fx: Function;
}) => {
  describe('Schema.options.ignore', () => {
    describe('signature', () => {
      it('should allow boolean, function, object, or array of config objects for ignore option', () => {
        const validValues = [
          true,
          false,
          () => false,
          { fields: ['lax', 'lax_1'], resolver: () => false },
          [{ fields: ['lax', 'lax_1'], resolver: () => false }],
        ];

        for (const ignore of validValues) {
          const toPass = fx(
            {
              lax: { default: 1234, validator },
              lax_1: { default: 5678, validator },
            },
            { ignore },
          );

          expectNoFailure(toPass);
          toPass();
        }
      });
    });

    describe('invalid', () => {
      it('should reject invalid ignore option formats', () => {
        const invalidValues = [
          123,
          'invalid_string',
          { fields: [] }, // missing resolver
          { resolver: () => false }, // missing fields
          [{ fields: 'not_an_array', resolver: () => false }],
        ];

        for (const ignore of invalidValues) {
          const toFail = fx(
            {
              lax: { default: 1234, validator },
              lax_1: { default: 5678, validator },
            },
            { ignore },
          );

          expectFailure(toFail);
        }
      });
    });
  });

  describe('Schema.options.ignoreUpdate', () => {
    describe('signature', () => {
      it('should allow boolean, function, object, or array of config objects for ignoreUpdate option', () => {
        const validValues = [
          true,
          false,
          () => false,
          { fields: ['lax', 'lax_1'], resolver: () => false },
          [{ fields: ['lax', 'lax_1'], resolver: () => false }],
        ];

        for (const ignoreUpdate of validValues) {
          const toPass = fx(
            {
              lax: { default: 1234, validator },
              lax_1: { default: 5678, validator },
            },
            { ignoreUpdate },
          );

          expectNoFailure(toPass);
          toPass();
        }
      });
    });

    describe('invalid', () => {
      it('should reject invalid ignoreUpdate option formats', () => {
        const invalidValues = [
          123,
          'invalid_string',
          { fields: [] }, // missing resolver
          { resolver: () => false }, // missing fields
          [{ fields: 'not_an_array', resolver: () => false }],
        ];

        for (const ignoreUpdate of invalidValues) {
          const toFail = fx(
            {
              lax: { default: 1234, validator },
              lax_1: { default: 5678, validator },
            },
            { ignoreUpdate },
          );

          expectFailure(toFail);
        }
      });
    });

    describe('behaviour', () => {
      it('should respect option to ignore updates', async () => {
        type Input = { lax?: string; lax_1?: string };
        // type Output = { lax: string; lax_1: string };

        const model = new Schema(
          {
            lax: { default: 'default_lax', validator },
            lax_1: { default: 'default_lax_1', validator },
          },
          { ignoreUpdate: ({ lax }: Partial<Input>) => lax === 'ignore_value' },
        ).getModel();

        const item = { lax: 'initial_lax', lax_1: 'initial_lax_1' };
        const res = await model.update(item, { lax: 'ignore_value' }, null);
        expect(res.data).toBeNull();
        expect(res.error).toBeNull();
        expect(typeof res.handleFailure).toBe('function');

        const res2 = await model.update(item, { lax: 'updated_lax' }, null);
        expect(res2.data).toEqual({ lax: 'updated_lax' });
      });
    });
  });
};
