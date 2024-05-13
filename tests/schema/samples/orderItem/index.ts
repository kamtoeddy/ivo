import { storeItemSchema } from '../storeItem';
import { validatePrice } from '../storeItem/validators';
import type { IOrderItem, OrderItemInput } from './types';

const orderItemSchema = storeItemSchema.extend<OrderItemInput, IOrderItem>(
  {
    costPrice: { readonly: true, validator: validatePrice },
    price: { readonly: true, validator: validatePrice },
  },
  {
    errors: 'throw',
    timestamps: true,
    remove: [
      '_readOnlyNoInit',
      '_dependentReadOnly',
      '_virtualForDependentReadOnly',
    ],
  },
);

const OrderItem = orderItemSchema.getModel();

export { OrderItem };
