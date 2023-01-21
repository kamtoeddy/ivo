export const isStringOkTest = ({ isStringOk }: { isStringOk: Function }) => {
  describe("isStringOk", () => {
    it("should tell whether input is a valid string or not", () => {
      const truthy = [
        "I",
        "am",
        "very",
        "delighted",
        " valid string with spaces ",
        "valid string with at the end  ",
        "  valid string with spaces infront",
        Array(40 + 1).join("a"),
      ];

      for (const value of truthy) {
        const res = isStringOk(value);

        expect(res).toMatchObject({ valid: true, validated: value.trim() });

        expect(res.reason).toBeUndefined();
        expect(res.reasons).toBeUndefined();
      }

      const falsy = [
        [null, ["Unacceptable value"]],
        [undefined, ["Unacceptable value"]],
        ["", ["Too short"]],
        ["  ", ["Too short"]],
        [Array(41 + 1).join("a"), ["Too long"]],
      ];

      for (const [value, reasons] of falsy) {
        const res = isStringOk(value);

        expect(res).toMatchObject({ reasons, valid: false });

        expect(res.validated).toBeUndefined();
      }
    });

    it("should cast numbers to strings", () => {
      const res = isStringOk(1);

      expect(res).toMatchObject({ valid: true, validated: "1" });

      expect(res.reason).toBeUndefined();
      expect(res.reasons).toBeUndefined();
    });

    it("should accept only enumerated values if any", () => {
      const enums = ["admin", "moderator", "user"];

      for (const value of enums) {
        const res = isStringOk(value, { enums });

        expect(res).toMatchObject({ valid: true, validated: value });

        expect(res.reason).toBeUndefined();
        expect(res.reasons).toBeUndefined();
      }

      const falsy = ["Admin", "ADMIN", "superadmin", "Moderators"];

      for (const value of falsy) {
        const res = isStringOk(value, { enums });

        expect(res).toMatchObject({
          reasons: ["Unacceptable value"],
          valid: false,
        });

        expect(res.validated).toBeUndefined();
      }
    });

    it("should accept values that match a regular expression", () => {
      const regExp = /^[a-zA-Z]+$/;

      const truthy = ["admin", "Admin", "ADMIN", "moderator", "user"];

      for (const value of truthy) {
        const res = isStringOk(value, { regExp });

        expect(res).toMatchObject({ valid: true, validated: value });

        expect(res.reason).toBeUndefined();
        expect(res.reasons).toBeUndefined();
      }

      const falsy = ["12", "%%", ".  ", "__"];

      for (const value of falsy) {
        const res = isStringOk(value, { regExp });

        expect(res).toMatchObject({
          reasons: ["Unacceptable value"],
          valid: false,
        });

        expect(res.validated).toBeUndefined();
      }
    });
  });
};
