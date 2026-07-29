import { describe, expect, it } from 'bun:test';

/**
 * Mirrors `rs/tests/extras/error_sanitizer.rs`. Rust plugs in a whole
 * `IvoErrorSanitizer<CtxOptions>` trait (custom `Metadata`/`Payload` types).
 * The TS surface is simpler: a single `sanitizeError` schema option
 * `(payload, ctxOptions) => CustomPayload` applied once the raw
 * `{ [field]: { reason, metadata } }` payload is fully built.
 */
export const Test_ErrorSanitizer = ({ Schema }: any) => {
  describe('Schema.options.sanitizeError', () => {
    type Input = { lat?: number; lon?: number };
    type Output = { lat: number; lon: number };
    type CtxOpts = { prefix: string };

    function customize(reason: string) {
      return `customized: ${reason}`;
    }

    const Place = new Schema<Input, Output, CtxOpts>(
      {
        lat: {
          default: 0,
          validator: (v: unknown) => {
            if (typeof v !== 'number' || Number.isNaN(v))
              return { valid: false, reason: 'invalid number' };

            if (v < -90 || v > 90)
              return {
                valid: false,
                reason: 'out of range',
                metadata: { extraReasons: ['must be >= -90', 'must be <= 90'] },
              };

            return { valid: true, validated: v };
          },
        },
        lon: { default: 0, validator: (v: unknown) => typeof v === 'number' },
      },
      {
        sanitizeError: (payload) => {
          const customized: Record<string, string[]> = {};

          for (const [field, err] of Object.entries(payload)) {
            const extraReasons =
              (err.metadata?.extraReasons as string[] | undefined) ?? [];

            customized[field] = [err.reason, ...extraReasons].map(customize);
          }

          return customized as never;
        },
      },
    ).getModel();

    it('sanitizes a single-error failure at creation', async () => {
      const { data, error } = await Place.create({ lat: Number.NaN, lon: 1 });

      expect(data).toBeNull();
      expect(error).toEqual({ lat: ['customized: invalid number'] });
    });

    it('sanitizes a multi-reason failure (primary reason + metadata entries), in order', async () => {
      const { data, error } = await Place.create({ lat: 200, lon: 1 });

      expect(data).toBeNull();
      expect(error).toEqual({
        lat: [
          'customized: out of range',
          'customized: must be >= -90',
          'customized: must be <= 90',
        ],
      });
    });

    it('applies the same sanitization during updates', async () => {
      const { data, error } = await Place.update(
        { lat: 10, lon: 1 },
        { lat: 200 },
      );

      expect(data).toBeNull();
      expect(error).toEqual({
        lat: [
          'customized: out of range',
          'customized: must be >= -90',
          'customized: must be <= 90',
        ],
      });
    });

    it('a genuinely-changed update still succeeds normally through the sanitizer pipeline', async () => {
      const { data, error } = await Place.update({ lat: 10, lon: 1 }, { lat: 20 });

      expect(error).toBeNull();
      expect(data).toEqual({ lat: 20 });
    });

    it('a no-op update (nothing changed) does not invoke the sanitizer', async () => {
      const { data, error } = await Place.update(
        { lat: 10, lon: 1 },
        { lat: 10 },
      );

      expect(data).toBeNull();
      expect(error).toBeNull();
    });
  });
};
