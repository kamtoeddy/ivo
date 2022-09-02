import { IStoreItem } from "../storeItem/interfaces";

export interface IOrderItem
  extends Omit<
    IStoreItem,
    "_readOnlyNoInit" | "_dependentReadOnly" | "_sideEffectForDependentReadOnly"
  > {
  costPrice: number;
}
