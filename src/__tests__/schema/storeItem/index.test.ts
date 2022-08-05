import { StoreItem } from ".";
import { IStoreItem } from "./interfaces";

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
      // quantities: [{ quantity: 1, name: "crate24" }],
    }).create();

    console.log("storeItem:", storeItem);
  });

  it("should have been created properly", () => {
    expect(storeItem).toMatchObject<IStoreItem>({
      id: "1",
      name: "beer",
      price: 5,
      measureUnit: "bottle",
      quantity: 100,
      // quantity: 124,
    });
  });

  it("should update the relevant properties", async () => {
    const update = await StoreItem(storeItem).update({
      name: "Castel",
      quantity: 10,
    });

    expect(update).toMatchObject<Partial<IStoreItem>>({
      name: "Castel",
      quantityChangeCounter: 2,
      quantity: 10,
    });
  });

  it("should update on side effects", async () => {
    const update = await StoreItem(storeItem).update({
      quantities: [
        { quantity: 1, name: "crate24" },
        { name: "crate", quantity: 2 },
        { name: "tray", quantity: 5 },
      ],
    });

    expect(update).toMatchObject<Partial<IStoreItem>>({
      quantityChangeCounter: 2,
      quantity: 173,
    });
  });

  it("should update the relevant properties & on side effects", async () => {
    const update = await StoreItem(storeItem).update({
      name: "Castel",
      quantity: 10,
      quantities: [
        { quantity: 1, name: "crate24" },
        { name: "crate", quantity: 2 },
        { name: "tray", quantity: 5 },
      ],
    });

    expect(update).toMatchObject<Partial<IStoreItem>>({
      name: "Castel",
      quantityChangeCounter: 3,
      quantity: 83,
    });
  });
});
