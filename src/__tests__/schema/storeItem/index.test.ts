import { StoreItem } from ".";
import { IOtherQuantity, IStoreItem } from "./interfaces";

describe("Testing schema of StoreItem", () => {
  let storeItem: IStoreItem;

  beforeAll(async () => {
    storeItem = await StoreItem({
      id: "1",
      name: "beer",
      price: 5,
      measureUnit: "bottle",
      otherMeasureUnits: [
        { coefficient: 24, name: "crate24" },
        { coefficient: 5, name: "tray" },
        { coefficient: 12, name: "crate" },
      ],
      quantity: 100,
      quantities: [{ quantity: 1, name: "crate24" }],
    }).create();

    console.log("storeItem:", storeItem);
  });

  it("should have been created properly", () => {
    expect(storeItem).toMatchObject<IStoreItem>({
      id: "1",
      name: "beer",
      price: 5,
      measureUnit: "bottle",
      quantity: 124,
    });
  });

  it("should update the relevant properties", async () => {
    const update = await StoreItem(storeItem).update({
      name: "Castel",
      quantities: [
        { name: "crate", quantity: 2 },
        { name: "tray", quantity: 5 },
      ],
    });

    expect(update).toMatchObject<Partial<IStoreItem>>({
      name: "Castel",
      quantityChangeCounter: 3,
      quantity: 173,
    });
  });
});
