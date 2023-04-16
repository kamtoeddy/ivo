import { RealType } from "../../../../../dist/schema/merge-types";
import { IStoreItem, StoreItemType } from "../storeItem/interfaces";

export interface IOrderItem
  extends Omit<IStoreItem, "_readOnlyNoInit" | "_virtualForDependentReadOnly"> {
  costPrice: number;
}

export type OrderItemType = RealType<
  Omit<StoreItemType, "_readOnlyNoInit" | "_dependentReadOnly"> & {
    costPrice: number;
  }
>;
