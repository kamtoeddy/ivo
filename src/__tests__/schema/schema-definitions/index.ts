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

    it("should reject if property definitions has no property", () => {
      expect(fx({})).toThrow("Invalid Schema");

      try {
        fx({})();
      } catch (err: any) {
        expect(err.payload).toMatchObject({
          "schema properties": ["Insufficient Schema properties"],
        });
      }
    });

    it("should reject if property definition is not an object", () => {
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

      for (const value of values)
        expect(fx({ name: value })).toThrow("Invalid Schema");
    });

    it("should reject if sideEffect property has no validator or onChange listeners", () => {
      try {
        fx({ age: { sideEffect: true } })();
      } catch (err: any) {
        expect(err.payload).toMatchObject({
          age: [
            "Invalid validator",
            "SideEffects must have at least one onChange listener",
            "A property should at least be readonly, required, or have a default value",
          ],
        });
      }
    });
  });
};
