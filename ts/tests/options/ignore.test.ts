import { describe, expect, it } from "bun:test";
import { Schema } from "../../src";
import { expectFailure, expectNoFailure, makeFx, validator } from "../_utils";

describe("Schema.options.ignore", () => {
  describe("signature", () => {
    it("should allow boolean, function, object, or array of config objects for ignore option", () => {
      const validValues = [
        true,
        false,
        () => false,
        { fields: ["lax", "lax_1"], resolver: () => false },
        [{ fields: ["lax", "lax_1"], resolver: () => false }],
      ];

      for (const ignore of validValues) {
        const toPass = makeFx(
          (b, m) =>
            b
              .field(m.lax("lax", 1234).validate(validator))
              .field(m.lax("lax_1", 5678).validate(validator)),
          { ignore },
        );

        expectNoFailure(toPass);
        toPass();
      }
    });
  });

  describe("invalid", () => {
    it("should reject invalid ignore option formats", () => {
      const invalidValues = [
        123,
        "invalid_string",
        { fields: [] }, // missing resolver
        { resolver: () => false }, // missing fields
        [{ fields: "not_an_array", resolver: () => false }],
      ];

      for (const ignore of invalidValues) {
        const toFail = makeFx(
          (b, m) =>
            b
              .field(m.lax("lax", 1234).validate(validator))
              .field(m.lax("lax_1", 5678).validate(validator)),
          { ignore },
        );

        expectFailure(toFail);
      }
    });
  });
});

describe("Schema.options.ignoreUpdate", () => {
  describe("signature", () => {
    it("should allow boolean, function, object, or array of config objects for ignoreUpdate option", () => {
      const validValues = [
        true,
        false,
        () => false,
        { fields: ["lax", "lax_1"], resolver: () => false },
        [{ fields: ["lax", "lax_1"], resolver: () => false }],
      ];

      for (const ignoreUpdate of validValues) {
        const toPass = makeFx(
          (b, m) =>
            b
              .field(m.lax("lax", 1234).validate(validator))
              .field(m.lax("lax_1", 5678).validate(validator)),
          { ignoreUpdate },
        );

        expectNoFailure(toPass);
        toPass();
      }
    });
  });

  describe("invalid", () => {
    it("should reject invalid ignoreUpdate option formats", () => {
      const invalidValues = [
        123,
        "invalid_string",
        { fields: [] }, // missing resolver
        { resolver: () => false }, // missing fields
        [{ fields: "not_an_array", resolver: () => false }],
      ];

      for (const ignoreUpdate of invalidValues) {
        const toFail = makeFx(
          (b, m) =>
            b
              .field(m.lax("lax", 1234).validate(validator))
              .field(m.lax("lax_1", 5678).validate(validator)),
          { ignoreUpdate },
        );

        expectFailure(toFail);
      }
    });
  });

  describe("behaviour", () => {
    it("should respect option to ignore updates", async () => {
      type Data = { lax?: string; lax_1?: string };

      const model = new Schema<Data>(
        (b, m) =>
          b
            .field(m.lax("lax", "default_lax").validate(validator))
            .field(m.lax("lax_1", "default_lax_1").validate(validator)),
        { ignoreUpdate: ({ lax }: Partial<Data>) => lax === "ignore_value" },
      ).getModel();

      const item = { lax: "initial_lax", lax_1: "initial_lax_1" };
      const res = await model.update(item, { lax: "ignore_value" });
      expect(res.data).toBeNull();
      expect(res.error).toBeNull();
      expect(typeof res.handleFailure).toBe("function");

      const res2 = await model.update(item, { lax: "updated_lax" });
      expect(res2.data).toEqual({ lax: "updated_lax" });
    });
  });
});
