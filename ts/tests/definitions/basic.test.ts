import { describe, expect, it } from "bun:test";
import { Schema } from "../../src";
import { expectFailure, makeFx } from "../_utils";

describe("Schema definitions", () => {
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
