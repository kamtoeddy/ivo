import { storeItemSchema } from '../storeItem';
import { validatePrice } from '../storeItem/validators';
import type { IOrderItem, OrderItemInput } from './types';

export { OrderItem };

const OrderItem = storeItemSchema
  .extend<OrderItemInput, IOrderItem>(
    {
      costPrice: { default: 0, readonly: true, validator: validatePrice },
      price: { default: 0, readonly: true, validator: validatePrice },
    },
    {
      timestamps: true,
      remove: [
        '_readOnlyNoInit',
        '_dependentReadOnly',
        '_virtualForDependentReadOnly',
      ],
    },
  )
  .getModel();
