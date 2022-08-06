export const CommonInheritanceTest = (
  schemaName = "",
  Model: any,
  initialValues = {}
) => {
  describe(`Testing schema behaviours that should be common in parent & child schemas for @${schemaName}`, () => {
    let item: any;

    beforeAll(async () => {
      item = await Model(initialValues).create();
    });

    // creation
    it("should have been created properly", () => {
      expect(item).toMatchObject({
        id: "1",
        name: "beer",
        price: 5,
        measureUnit: "bottle",
        otherMeasureUnits: [
          { coefficient: 12, name: "crate" },
          { coefficient: 24, name: "crate24" },
          { coefficient: 5, name: "tray" },
        ],
        quantity: 100,
      });
    });

    it("should accept not dependent properties at creation", async () => {
      expect(item).toMatchObject({
        _dependentReadOnly: 0,
      });
    });

    it("should accept not readOnly properties with blocked initialization at creation", async () => {
      expect(item).toMatchObject({
        _readOnlyNoInit: "",
      });
    });

    // update
    it("should update the relevant properties", async () => {
      const update = await Model(item).update({
        name: "Castel",
        quantity: 10,
      });

      expect(update).toMatchObject({
        name: "Castel",
        quantityChangeCounter: 2,
        quantity: 10,
      });
    });

    it("should update on side effects", async () => {
      const update = await Model(item).update({
        quantities: [
          { quantity: 1, name: "crate24" },
          { name: "crate", quantity: 2 },
          { name: "tray", quantity: 5 },
        ],
      });

      expect(update).toMatchObject({
        quantityChangeCounter: 2,
        quantity: 173,
      });
    });

    it("should update the relevant properties & on side effects", async () => {
      const update = await Model(item).update({
        name: "Castel",
        quantity: 10,
        quantities: [
          { quantity: 1, name: "crate24" },
          { name: "crate", quantity: 2 },
          { name: "tray", quantity: 5 },
        ],
      });

      expect(update).toMatchObject({
        name: "Castel",
        quantityChangeCounter: 3,
        quantity: 83,
      });
    });

    it("should update dependent properties on side effects", async () => {
      const update = await Model(item).update({
        _sideEffectForDependentReadOnly: "haha",
      });

      expect(update).toMatchObject({
        _dependentReadOnly: 1,
      });
    });

    it("should not update readonly dependent properties that have changed", async () => {
      const update = await Model(item).update({
        _sideEffectForDependentReadOnly: "haha",
      });

      const updateToFail = async () => {
        await Model({ ...item, ...update }).update({
          _sideEffectForDependentReadOnly: "haha",
        });
      };

      await expect(updateToFail()).rejects.toThrow("Nothing to update");
    });

    it("should not update dependent properties", async () => {
      const updateReadOnlyProperty = async () =>
        await Model(item).update({ quantityChangeCounter: 0 });

      await expect(updateReadOnlyProperty()).rejects.toThrow(
        "Nothing to update"
      );
    });

    it("should not update readonly properties that have changed", async () => {
      const updateReadOnlyProperty = async () =>
        await Model(item).update({ id: "2" });

      await expect(updateReadOnlyProperty()).rejects.toThrow(
        "Nothing to update"
      );
    });

    // clone
  });
};
