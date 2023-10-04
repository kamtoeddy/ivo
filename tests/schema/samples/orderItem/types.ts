import { StoreItem, StoreItemInput } from '../storeItem/types'

export interface OrderItemInput
  extends Omit<
    StoreItemInput,
    '_readOnlyNoInit' | '_virtualForDependentReadOnly'
  > {
  costPrice: number
}

export type OrderItem = Omit<
  StoreItem,
  '_readOnlyNoInit' | '_dependentReadOnly'
> & { costPrice: number }
