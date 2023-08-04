import { IStoreItem, StoreItemType } from '../storeItem/types'

export interface IOrderItem
  extends Omit<IStoreItem, '_readOnlyNoInit' | '_virtualForDependentReadOnly'> {
  costPrice: number
}

export type OrderItemType = Omit<
  StoreItemType,
  '_readOnlyNoInit' | '_dependentReadOnly'
> & { costPrice: number }
