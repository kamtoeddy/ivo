import { storeItemSchema } from '../storeItem'
import { validatePrice } from '../storeItem/validators'
import { IOrderItem, OrderItemType } from './types'

const orderItemSchema = storeItemSchema.extend<OrderItemType, IOrderItem>(
  {
    costPrice: { readonly: true, validator: validatePrice },
    price: { readonly: true, validator: validatePrice }
  },
  {
    errors: 'throw',
    timestamps: true,

    remove: [
      '_readOnlyNoInit',
      '_dependentReadOnly',
      '_virtualForDependentReadOnly'
    ]
  }
)

export const OrderItem = orderItemSchema.getModel()
