import { describe, it } from "bun:test";
import { expectNoFailure, makeFx } from "../_utils";

describe("lax props", () => {
  describe("valid", () => {
    it("should allow default alone", () => {
      const toPass = makeFx((b, m) => b.field(m.lax("fieldName", "")));

      expectNoFailure(toPass);

      toPass();
    });

    it("should allow default + validator", () => {
      const toPass = makeFx((b, m) =>
        b.field(m.lax("fieldName", "").validate(() => ({ valid: true }))),
      );

      expectNoFailure(toPass);

      toPass();
    });
  });

  // "invalid > should reject no default" discarded: `.validate()` isn't
  // available on `LaxBuilder` until `.default()` has been called, so a
  // field with a validator but no default is structurally unrepresentable.
});
