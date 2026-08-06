import { describe, expect, it } from "bun:test";
import { Schema } from "../../src";

/**
 * Mirrors `rs/tests/extras/error_sanitizer.rs`. Rust plugs in a whole
 * `IvoErrorSanitizer<CtxOptions>` trait (custom `Metadata`/`Payload` types).
 * The TS surface is simpler: a single `sanitizeError` schema option
 * `(payload, ctxOptions) => CustomPayload` applied once the raw
 * `{ [field]: { reason, metadata } }` payload is fully built.
 */

describe("Schema.options.sanitizeError", () => {
  type Data = { lat: number; lon: number };
  type CtxOpts = { prefix: string };
  type ErrorMetadata = Record<string, unknown>;
  type CustomErrorPayload = Record<string, string[]>;

  const ctxOptions: CtxOpts = { prefix: "" };

  function customize(reason: string) {
    return `customized: ${reason}`;
  }

  const Place = new Schema<
    Data,
    Data,
    CtxOpts,
    ErrorMetadata,
    CustomErrorPayload
  >(
    (b, m) =>
      b
        .field(
          m.lax("lat", 0).validate((v) => {
            if (typeof v !== "number" || Number.isNaN(v))
              return { valid: false, reason: "invalid number" };

            if (v < -90 || v > 90)
              return {
                valid: false,
                reason: "out of range",
                metadata: {
                  extraReasons: ["must be >= -90", "must be <= 90"],
                },
              };

            return { valid: true, validated: v };
          }),
        )
        .field(m.lax("lon", 0).validate((v) => typeof v === "number")),
    {
      sanitizeError(payload): CustomErrorPayload {
        const customized: CustomErrorPayload = {};

        for (const [field, err] of Object.entries(payload)) {
          const extraReasons =
            (err.metadata?.extraReasons as string[] | undefined) ?? [];

          customized[field] = [err.reason, ...extraReasons].map(customize);
        }

        return customized;
      },
    },
  ).getModel();

  it("sanitizes a single-error failure at creation", async () => {
    const { data, error } = await Place.create(
      { lat: Number.NaN, lon: 1 },
      ctxOptions,
    );

    expect(data).toBeNull();
    expect(error).toEqual({ lat: ["customized: invalid number"] });
  });

  it("sanitizes a multi-reason failure (primary reason + metadata entries), in order", async () => {
    const { data, error } = await Place.create(
      { lat: 200, lon: 1 },
      ctxOptions,
    );

    expect(data).toBeNull();
    expect(error).toEqual({
      lat: [
        "customized: out of range",
        "customized: must be >= -90",
        "customized: must be <= 90",
      ],
    });
  });

  it("applies the same sanitization during updates", async () => {
    const { data, error } = await Place.update(
      { lat: 10, lon: 1 },
      { lat: 200 },
      ctxOptions,
    );

    expect(data).toBeNull();
    expect(error?.payload).toEqual({
      lat: [
        "customized: out of range",
        "customized: must be >= -90",
        "customized: must be <= 90",
      ],
    });
  });

  it("a genuinely-changed update still succeeds normally through the sanitizer pipeline", async () => {
    const { data, error } = await Place.update(
      { lat: 10, lon: 1 },
      { lat: 20 },
      ctxOptions,
    );

    expect(error).toBeNull();
    expect(data).toEqual({ lat: 20 });
  });

  it("a no-op update (nothing changed) does not invoke the sanitizer", async () => {
    const { data, error } = await Place.update(
      { lat: 10, lon: 1 },
      { lat: 10 },
      ctxOptions,
    );

    expect(data).toBeNull();
    expect(error).toBeNull();
  });
});
