import { IStoreItem } from "../storeItem/interfaces";

export interface IOrderItem extends IStoreItem {
  costPrice: number;
  price: number;
}
