export const isBooleanOkTest = ({ isBooleanOk }: { isBooleanOk: Function }) => {
  describe("Testing isBooleanOk", () => {
    it("should tell whether or not input values are booleans", () => {
      // truthy values

      const truthyValues = [false, true];

      for (let value of truthyValues) {
        const res = isBooleanOk(value);

        expect(res).toMatchObject({ valid: true, validated: value });

        expect(res.reason).toBeUndefined();
        expect(res.reasons).toBeUndefined();
      }

      // falsy values

      const falsyValues = ["true", "false", 1, 0, null, undefined, [], {}, NaN];

      for (let value of falsyValues) {
        expect(isBooleanOk(value)).toMatchObject({
          reasons: ["Expected a boolean"],
          valid: false,
          validated: undefined,
        });
      }
    });
  });
};
