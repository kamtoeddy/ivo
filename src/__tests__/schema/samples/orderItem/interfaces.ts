import { IStoreItem } from "../storeItem/interfaces";

export interface IOrderItem extends IStoreItem {
  // Omit<
  //   IStoreItem,
  //   "_readOnlyNoInit" | "_dependentReadOnly" | "_sideEffectForDependentReadOnly"
  // >

  costPrice: number;
}
