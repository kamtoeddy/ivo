export const schemaDefinition_Tests = ({ Schema }: any) => {
  const fx =
    (definition: any = undefined) =>
    () =>
      new Schema(definition);

  describe("Invalid Schema definitions", () => {
    it("should reject if property definitions is not an object", () => {
      const values = [
        null,
        undefined,
        new Number(),
        new String(),
        Symbol(),
        2,
        -10,
        true,
        [],
      ];

      for (const value of values) expect(fx(value)).toThrow("Invalid Schema");
    });

    it("should reject if property definition has no property", () => {
      expect(fx({})).toThrow("Invalid Schema");

      try {
        fx({})();
      } catch (err: any) {
        expect(err.payload).toMatchObject({
          "schema properties": ["Insufficient Schema properties"],
        });
      }
    });
  });
};
