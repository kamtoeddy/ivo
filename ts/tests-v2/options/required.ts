import { describe, expect, it } from 'bun:test';
import { expectNoFailure, validator } from '../_utils';

/**
 * Rust mirrors this with a schema-level grouped `required` option
 * (`rs/tests/options/required.rs`) that validates: min 2 properties,
 * no duplicates, only lax/virtual fields allowed, alias-must-use-real-name,
 * and timestamp fields excluded.
 *
 * The TS type surface declares an equivalent `required` schema option
 * (`NS.Options.required` / `RequiredConfigOption`, listed in `ALLOWED_OPTIONS`),
 * but as of this writing `SchemaCore._checkOptions` has no validation branch for
 * it and `ModelTool` never reads `this._options.required` at runtime — so the
 * option is currently a type-level stub with no construction-time validation
 * and no create/update effect. These tests document that current (likely
 * unfinished) state rather than asserting the Rust-parity behavior, so this
 * file should be revisited once the option is wired up.
 */
export const Test_SchemaOptionRequired = ({ Schema, fx }: any) => {
  describe('Schema.options.required (currently unimplemented at runtime)', () => {
    it('should accept any value for the "required" option without validation', () => {
      const values = [
        'not-a-valid-config',
        123,
        {},
        { properties: ['a'], handler: () => true },
        null,
      ];

      for (const required of values) {
        const toPass = fx(
          { a: { default: 1, validator }, b: { default: 2, validator } },
          { required },
        );

        expectNoFailure(toPass);
        toPass();
      }
    });

    it('should not enforce grouped required rules at create/update time', async () => {
      const Model = new Schema(
        { a: { default: 1 }, b: { default: 2 } },
        {
          required: {
            properties: ['a', 'b'],
            handler: () => true,
          },
        },
      ).getModel();

      const { data, error } = await Model.create({});

      expect(error).toBeNull();
      expect(data).toEqual({ a: 1, b: 2 });
    });
  });
};
