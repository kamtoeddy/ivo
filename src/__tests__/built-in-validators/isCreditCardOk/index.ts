export const isCreditCardOkTest = ({
  isCreditCardOk,
}: {
  isCreditCardOk: Function;
}) => {
  describe("Testing isCreditCardOk", () => {
    const truthyValues = [
      [5420596721435293, 5420596721435293],
      ["5420596721435293", "5420596721435293"],
      ["5420596721435293 ", "5420596721435293"],
      ["5420596721435293 ", "5420596721435293"],
    ];

    it("should tell whether a character is a valid credit card number", () => {
      for (const [value, validated] of truthyValues) {
        expect(isCreditCardOk(value)).toEqual({
          reasons: [],
          valid: true,
          validated,
        });
      }

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
        expect(isCreditCardOk(value)).toEqual({
          reasons: ["Invalid card number"],
          valid: false,
          validated: undefined,
        });
      });
    });

    it("should validated value should be of same type as input value", () => {
      truthyValues.forEach(([value]) => {
        expect(typeof isCreditCardOk(value).validated).toBe(typeof value);
      });
    });
  });
};
