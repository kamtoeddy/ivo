import { beforeAll, describe, expect, it } from "bun:test";
import { Schema } from "../../src";
import { expectNoFailure, makeFx, validator } from "../_utils";

describe("readonly", () => {
  describe("valid", () => {
    it("should allow readonly(true) + dependent + default", () => {
      const toPass = makeFx((b, m) =>
        b
          .field(
            m
              .dependent("dependentField", "field")
              .default("value")
              .resolve(() => 1)
              .readonly(),
          )
          .field(m.lax("field", "")),
      );

      expectNoFailure(toPass);

      toPass();
    });

    it("should allow readonly(true) + requiredBy", () => {
      const toPass = makeFx((b, m) =>
        b.field(
          m
            .lax("fieldName", "")
            .validate(validator)
            .readonly()
            .required(() => true),
        ),
      );

      expectNoFailure(toPass);

      toPass();
    });

    it("should allow readonly(true) + strictly required", () => {
      const toPass = makeFx((b, m) =>
        b.field(m.required("fieldName").validate(validator).readonly()),
      );

      expectNoFailure(toPass);

      toPass();
    });

    describe("behaviour", () => {
      let Model: any;

      beforeAll(() => {
        Model = new Schema<any>((b, m) =>
          b
            .field(m.lax("age", null).readonly())
            .field(m.lax("name", "Default Name")),
        ).getModel();
      });

      it("should not modify readonly props that have changed via life cycle listeners at creation", async () => {
        const { data } = await Model.create({ age: 25 });

        expect(data).toMatchObject({ age: 25, name: "Default Name" });
      });

      it("should not modify readonly props that have changed via life cycle listeners during updates", async () => {
        const { data } = await Model.update(
          { age: null, name: "Default Name" },
          { age: 25, name: "YoYo" },
        );

        expect(data).toMatchObject({ age: 25, name: "YoYo" });
      });

      it("should still accept updates while the readonly value still equals its default", async () => {
        const { data, error } = await Model.update(
          { age: null, name: "Default Name" },
          { age: 30 },
        );

        expect(error).toBeNull();
        expect(data).toEqual({ age: 30 });
      });

      it("should permanently lock a readonly lax field once its value has diverged from the default", async () => {
        const { data, error } = await Model.update(
          { age: 30, name: "Default Name" },
          { age: 40 },
        );

        // nothing-to-update sentinel: no error, but no data either
        expect(error).toBeNull();
        expect(data).toBeNull();
      });
    });

    describe("behaviour with readonly + strictly required", () => {
      let Book: any;
      const book = { title: "A Book" };

      beforeAll(() => {
        Book = new Schema<any>((b, m) =>
          b.field(m.required("title").validate(validator).readonly()),
        ).getModel();
      });

      it("should create normally, requiring the field once", async () => {
        const { data, error } = await Book.create({ title: "A Book" });

        expect(error).toBeNull();
        expect(data).toEqual(book);
      });

      it("should permanently reject every subsequent update, regardless of value", async () => {
        const { data, error } = await Book.update(book, {
          title: "A different title",
        });

        expect(error).toBeNull();
        expect(data).toBeNull();
      });
    });
  });

  // "invalid" discarded entirely: "should reject readonly !== true"
  // (`.readonly()` takes no arguments and can only ever set `readonly:
  // true`) and "should reject readonly(true) + dependent & no default"
  // (`.resolve()`/`.readonly()` aren't available on `DependentBuilder`
  // until `.default()` has been called) are both structurally
  // unrepresentable through the builder by design.
});
