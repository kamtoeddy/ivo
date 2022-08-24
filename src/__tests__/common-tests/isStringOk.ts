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
        expect(isStringOk(value)).toMatchObject({
          reasons: [],
          valid: true,
          validated: value.trim(),
        });
      }

      const falsy = [
        [null, ["Unacceptable value"]],
        [undefined, ["Unacceptable value"]],
        ["", ["too short"]],
        [Array(41 + 1).join("a"), ["too long"]],
      ];

      for (const [value, reasons] of falsy) {
        expect(isStringOk(value)).toMatchObject({
          reasons,
          valid: false,
          validated: undefined,
        });
      }
    });

    it("should cast numbers to strings", () => {
      expect(isStringOk(1)).toMatchObject({
        reasons: [],
        valid: true,
        validated: "1",
      });
    });

    it("should accept only enumerated values if any", () => {
      const enums = ["admin", "moderator", "user"];

      for (const value of enums) {
        expect(isStringOk(value, { enums })).toMatchObject({
          reasons: [],
          valid: true,
          validated: value,
        });
      }

      const falsy = ["Admin", "ADMIN", "superadmin", "Moderators"];

      for (const value of falsy) {
        expect(isStringOk(value, { enums })).toMatchObject({
          reasons: ["Unacceptable value"],
          valid: false,
          validated: undefined,
        });
      }
    });

    it("should accept values that match a regular expression", () => {
      const regExp = /^[a-zA-Z]+$/;

      const truthy = ["admin", "Admin", "ADMIN", "moderator", "user"];

      for (const value of truthy) {
        expect(isStringOk(value, { regExp })).toMatchObject({
          reasons: [],
          valid: true,
          validated: value,
        });
      }

      const falsy = ["12", "%%", ".  ", "__"];

      for (const value of falsy) {
        expect(isStringOk(value, { regExp })).toMatchObject({
          reasons: ["Unacceptable value"],
          valid: false,
          validated: undefined,
        });
      }
    });
  });
};
