import { describe, expect, it } from "bun:test";
import { Schema } from "../src";
import { expectNoFailure } from "./_utils";

describe("Values Parsing", () => {
  const validData = { age: 15, name: "Frank" };
  const invalidData = [1, -10, 0, false, true, "", "true", null];

  const User = new Schema<{ age: number; id: number; name: string }>((b, m) =>
    b
      .field(m.lax("age").default(0))
      .field(m.constant("id", 1))
      .field(m.lax("name").default("")),
  ).getModel();

  describe("valid data", () => {
    it("should allow for create method of model to be empty", async () => {
      const { data, error } = await User.create({}, {});

      expect(error).toBeNull();

      expect(data).toEqual({ age: 0, id: 1, name: "" });
    });

    it("should set values properly at creation", async () => {
      const { data, error } = await User.create(validData, {});

      expect(error).toBeNull();

      expect(data).toEqual({ ...validData, id: 1 });
    });

    it("should set values properly during deletion", async () => {
      expectNoFailure(
        async () => await User.delete({ ...validData, id: 1 }, {}),
      );
    });

    it("should set values properly during updates", async () => {
      const user = { ...validData, id: 1 };
      const name = "Mike";

      const { data, error } = await User.update(user, { name }, {});

      expect(error).toBeNull();

      expect(data).toEqual({ name });
    });
  });

  describe("invalid data", () => {
    it("should ignore invalid data at creation", async () => {
      for (const val of invalidData) {
        // @ts-expect-error ikr
        const { data, error } = await User.create(val, {});

        expect(error).toBeNull();
        expect(data).toEqual({ age: 0, id: 1, name: "" });
      }
    });

    it("should reject invalid data during updates", async () => {
      for (const val of invalidData) {
        // @ts-expect-error ikr
        const { data, error } = await User.update(val, { name: "yoo" }, {});

        expect(data).toBeNull();
        expect(error).toBeNull();
      }
    });
  });
});
