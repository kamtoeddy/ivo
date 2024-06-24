import { storeItemSchema } from '../storeItem';
import { validatePrice } from '../storeItem/validators';
import type { IOrderItem, OrderItemInput } from './types';

export { OrderItem };

const OrderItem = storeItemSchema
  .extend<OrderItemInput, IOrderItem>(
    {
      costPrice: { readonly: true, validator: validatePrice },
      price: { readonly: true, validator: validatePrice },
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
