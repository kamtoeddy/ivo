import { IStoreItem } from "../storeItem/interfaces";

export interface IOrderItem
  extends Omit<
    IStoreItem,
    | "c_At"
    | "u_At"
    | "_readOnlyNoInit"
    | "_dependentReadOnly"
    | "_sideEffectForDependentReadOnly"
  > {
  costPrice: number;
  createdAt?: string;
  updatedAt?: string;
}
