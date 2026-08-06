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
});

describe("behaviour of schema when errors thrown in setter of default values", () => {
  const Model = new Schema<any>((b, m) =>
    b
      .field(
        m.lax("field", () => {
          throw new Error("lolol");
        }),
      )
      .field(m.lax("prop1", "")),
  ).getModel();

  it("should set value as null on error generating default value at creation", async () => {
    const { data, error } = await Model.create({}, {});

    expect(error).toBeNull();
    expect(data).toMatchObject({ field: null, prop1: "" });
  });
});
