import { CommonInheritanceTest } from "./common-tests";
import { IStoreItemChild } from "./interfaces";
import { StoreItemChild } from "./storeItemChild";

CommonInheritanceTest("StoreItemChild", StoreItemChild, {
  childID: "1",
  id: "1",
  name: "beer",
  price: 5,
  measureUnit: "bottle",
  _dependentReadOnly: 100,
  _readOnlyNoInit: [],
  otherMeasureUnits: [
    { coefficient: 24, name: "crate24" },
    { coefficient: 5, name: "tray" },
    { coefficient: 12, name: "crate" },
  ],
  quantity: 100,
});

describe("Testing non-inherited properties for StoreItemChild", () => {
  let item: IStoreItemChild;

  beforeAll(async () => {
    item = await StoreItemChild({
      childID: "1",
      id: "1",
      name: "beer",
      price: 5,
      measureUnit: "bottle",
      _dependentReadOnly: 100,
      _readOnlyNoInit: [],
      otherMeasureUnits: [
        { coefficient: 24, name: "crate24" },
        { coefficient: 5, name: "tray" },
        { coefficient: 12, name: "crate" },
      ],
      quantity: 100,
    }).create();
  });

  // creation
  it("should have the correct properties at creation", () => {
    expect(item).toMatchObject({ childID: "1" });
  });

  // updates
  it("should have the correct properties after updates", async () => {
    const update = await StoreItemChild(item).update({
      childID: "12",
      name: "Guiness ",
    });

    expect(update.childID).toBe(undefined);
    expect(update).toMatchObject({ name: "Guiness" });
  });
});
