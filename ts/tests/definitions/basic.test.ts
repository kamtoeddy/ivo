import { describe, expect, it } from "bun:test";
import { Schema } from "../../src";
import { expectFailure, makeFx } from "../_utils";

describe("Schema definitions", () => {
  // "should reject if property definitions is not an object" discarded:
  // the builder closure always resolves to a real `SchemaFieldBuilder`
  // whose FIELD_BUILDER_DEFINITIONS getter always returns an object, so
  // SchemaCore's `!isRecordLike(definitions)` branch is unreachable through
  // the public API - passing a non-function first argument to `Schema`
  // fails earlier (and differently) as `builder is not a function`.

  it("should reject if property definitions has no property", () => {
    const toFail = makeFx((b: any) => b);

    expectFailure(toFail);

    try {
      toFail();
    } catch (err: any) {
      expect(err.payload).toMatchObject({
        "schema fields": ["Insufficient Schema fields"],
      });
    }
  });

  // "should reject if a property's definition is an empty object",
  // "should reject if a property's definition is not an object", and
  // "should reject if a property's definition has an invalid rule"
  // discarded: `SchemaFieldBuilder.field()` only accepts `Buildable`s -
  // any non-Buildable value (empty object, primitive, malformed rule set)
  // is silently dropped rather than reaching SchemaCore's per-property
  // validation, so these malformed-shape scenarios are structurally
  // unrepresentable through the builder by design.

  it("should allow access to reservedKeys of valid schemas", () => {
    const schema = new Schema<any>(
      (b, m) =>
        b
          .field(m.constant("id", 1))
          .field(
            m
              .dependent("dependent", "virtual")
              .default("")
              .resolve(() => ""),
          )
          .field(m.lax("lax").default(true))
          .field(m.virtual("virtual").validate(() => true)),
      { timestamps: { createdAt: "c_At" } },
    );

    expect(schema.reservedKeys).toEqual(
      expect.arrayContaining([
        "c_At",
        "dependent",
        "id",
        "lax",
        "updatedAt",
        "virtual",
      ]),
    );
  });
});

describe("behaviour of schema when errors thrown in setter of default values", () => {
  const Model = new Schema<any>((b, m) =>
    b
      .field(
        m.lax("field").default(() => {
          throw new Error("lolol");
        }),
      )
      .field(m.lax("prop1").default("")),
  ).getModel();

  it("should set value as null on error generating default value at creation", async () => {
    const { data, error } = await Model.create({}, {});

    expect(error).toBeNull();
    expect(data).toMatchObject({ field: null, prop1: "" });
  });
});
