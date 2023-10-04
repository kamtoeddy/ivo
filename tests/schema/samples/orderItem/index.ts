import { storeItemSchema } from '../storeItem'
import { validatePrice } from '../storeItem/validators'
import type { OrderItem, OrderItemInput } from './types'

const orderItemSchema = storeItemSchema.extend<OrderItem, OrderItemInput>(
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

const OrderItem = orderItemSchema.getModel()

export { OrderItem }
