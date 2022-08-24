export const isCreditCardOkTest = ({
  isCreditCardOk,
}: {
  isCreditCardOk: Function;
}) => {
  describe("Testing isCreditCardOk", () => {
    it("should tell whether a character is a valid credit card number", () => {
      const truthyvalues = [5420596721435293, "5420596721435293"];

      truthyvalues.forEach((value) => {
        expect(isCreditCardOk(value)).toMatchObject({
          reasons: [],
          valid: true,
          validated: value,
        });
      });

      // falsy tests
      const falsyValues = [
        -1000,
        0,
        1,
        "",
        "123-2342-25-6750",
        4187622910505690,
        "4187622910505690",
      ];

      falsyValues.forEach((value) => {
        expect(isCreditCardOk(value)).toMatchObject({
          reasons: ["Invalid card number"],
          valid: false,
          validated: undefined,
        });
      });
    });
  });
};
