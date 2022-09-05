export const commonTestData = {
  id: "1",
  name: "beer",
  price: 5,
  measureUnit: "bottle",
  _dependentReadOnly: 100,
  _readOnlyLax1: "lax1 set",
  _readOnlyLaxNoInit: [],
  _readOnlyNoInit: [],
  otherMeasureUnits: [
    { coefficient: 24, name: "crate24" },
    { coefficient: 5, name: "tray" },
    { coefficient: 12, name: "crate" },
  ],
  quantity: 100,
};

export const CommonInheritanceTest = (
  schemaName = "",
  Model: any,
  testData = commonTestData
) => {
  describe(`Testing schema behaviours that should be common in parent & child schemas for @${schemaName}`, () => {
    let item: any;

    beforeAll(async () => (item = await Model(testData).create()));

    describe("create", () => {
      it("should create properly with right values", () => {
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

      it("should reject missing readonly field", async () => {
        const { id, ...testData1 } = testData;

        const createWithoutReadonly = async () =>
          await Model(testData1).create();

        await expect(createWithoutReadonly()).rejects.toThrow(
          "Validation Error"
        );
      });

      it("should reject missing required field", async () => {
        const { name, ...testData1 } = testData;

        const createWithoutReadonly = async () =>
          await Model(testData1).create();

        await expect(createWithoutReadonly()).rejects.toThrow(
          "Validation Error"
        );
      });

      it("should reject dependent properties", () => {
        expect(item).toMatchObject({
          _dependentReadOnly: 0,
        });
      });

      it("should reject readonly(true) + shouldInit(false)", () => {
        expect(item).toMatchObject({ _readOnlyNoInit: "" });
      });

      it("should accept provided lax readonly properties", () => {
        expect(item).toMatchObject({
          _readOnlyLax1: "lax1 set",
          _readOnlyLax2: "",
          _readOnlyNoInit: "",
        });
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

    it("should update lax properties not initialized at creation", async () => {
      const update = await Model(item).update({
        _readOnlyLax2: "haha",
      });

      expect(update).toMatchObject({
        _readOnlyLax2: "haha",
      });

      const updateReadOnlyProperty = async () =>
        await Model({ ...item, ...update }).update({
          _readOnlyLax2: "lax1 set again",
        });

      await expect(updateReadOnlyProperty()).rejects.toThrow(
        "Nothing to update"
      );
    });

    it("should not update dependent properties", async () => {
      const updateReadOnlyProperty = async () =>
        await Model(item).update({ quantityChangeCounter: 0 });

      await expect(updateReadOnlyProperty()).rejects.toThrow(
        "Nothing to update"
      );
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

    it("should not update readonly properties that have changed", async () => {
      const updateReadOnlyProperty = async () =>
        await Model(item).update({ id: "2", _readOnlyLax1: "lax1 set again" });

      await expect(updateReadOnlyProperty()).rejects.toThrow(
        "Nothing to update"
      );
    });

    // clone
    it("should clone properly", async () => {
      const clonedItem = await Model(item).clone();

      expect(clonedItem).toMatchObject({
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

    it("should clone properly with side effects", async () => {
      const clonedItem = await Model({
        ...item,
        quantities: [
          { quantity: 1, name: "crate24" },
          { quantity: 1, name: "tray" },
        ],
      }).clone();

      expect(clonedItem).toMatchObject({
        id: "1",
        name: "beer",
        price: 5,
        measureUnit: "bottle",
        otherMeasureUnits: [
          { coefficient: 12, name: "crate" },
          { coefficient: 24, name: "crate24" },
          { coefficient: 5, name: "tray" },
        ],
        quantity: 129,
      });
    });

    it("should respect clone reset option for property with default value", async () => {
      const clone1 = await Model(item).clone({ reset: "quantity" });
      const clone2 = await Model(item).clone({ reset: ["quantity"] });
      const expectedResult = {
        id: "1",
        name: "beer",
        price: 5,
        measureUnit: "bottle",
        otherMeasureUnits: [
          { coefficient: 12, name: "crate" },
          { coefficient: 24, name: "crate24" },
          { coefficient: 5, name: "tray" },
        ],
        quantity: 0,
      };

      for (let clonedItem of [clone1, clone2])
        expect(clonedItem).toMatchObject(expectedResult);
    });

    it("should respect clone reset option for property without default value", async () => {
      const clone1 = await Model(item).clone({ reset: "measureUnit" });
      const clone2 = await Model(item).clone({ reset: ["measureUnit"] });
      const expectedResult = {
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
      };

      for (let clonedItem of [clone1, clone2])
        expect(clonedItem).toMatchObject(expectedResult);
    });
  });

  describe(`Testing schema @${schemaName} initialized with sideffect`, () => {
    let item: any;

    beforeAll(async () => {
      item = await Model({
        ...testData,
        quantities: [
          { name: "crate24", quantity: 1 },
          { name: "tray", quantity: 1 },
        ],
      }).create();
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
        quantity: 129,
      });
    });
  });

  describe(`Testing schema @${schemaName} for user defined validation errors`, () => {
    it("should respect user defined error messages at creation", () => {
      const failToCreate = async () => {
        try {
          await Model({ ...commonTestData, name: "", _laxProp: [] }).create();
        } catch (err: any) {
          expect(err.message).toBe("Validation Error");
          expect(err.payload).toMatchObject({
            _laxProp: ["Invalid lax prop", "Too short"],
            name: [],
          });
        }
      };

      failToCreate();
    });

    it("should respect user defined error messages during cloning", () => {
      const failToClone = async () => {
        try {
          await Model(testData).clone({ reset: ["name", "_laxProp"] });
        } catch (err: any) {
          expect(err.message).toBe("Validation Error");
          expect(err.payload).toMatchObject({
            _laxProp: ["Invalid lax prop", "Unacceptable value"],
            name: [],
          });
        }
      };

      failToClone();
    });

    it("should respect user defined error messages during updates", () => {
      const failToUpdate = async () => {
        try {
          await Model(commonTestData).update({ name: "", _laxProp: [] });
        } catch (err: any) {
          expect(err.message).toBe("Validation Error");
          expect(err.payload).toMatchObject({
            _laxProp: ["Invalid lax prop", "Too short"],
            name: [],
          });
        }
      };

      failToUpdate();
    });
  });
};
