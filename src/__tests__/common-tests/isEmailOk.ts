export const isEmailOkTest = ({ isEmailOk }: { isEmailOk: Function }) => {
  describe("isEmailOk", () => {
    it("should tell whether input is a valid email or not", () => {
      const truthy = [
        "example@gmail.com",
        "james71@hotmail.co.uk",
        " james71@hotmail.co.uk",
      ];

      for (const value of truthy) {
        expect(isEmailOk(value)).toMatchObject({
          reasons: [],
          valid: true,
          validated: value.trim(),
        });
      }

      const falsy = [1, null, false, "", "@gmail.com", "james71@..uk"];

      for (const value of falsy) {
        expect(isEmailOk(value)).toMatchObject({
          reasons: ["Invalid email"],
          valid: false,
          validated: undefined,
        });
      }
    });
  });
};
