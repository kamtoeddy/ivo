import { StoreItem } from ".";
import { CommonInheritanceTest } from "./common-tests";
import { IStoreItem } from "./interfaces";

CommonInheritanceTest("StoreItem", StoreItem, {
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
  // quantities: [{ quantity: 1, name: "crate24" }],
});
