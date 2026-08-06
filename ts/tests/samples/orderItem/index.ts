import { storeItemSchema } from "../storeItem";
import { validatePrice } from "../storeItem/validators";
import type { IOrderItem, OrderItemInput } from "./types";

export { OrderItem };

const OrderItem = storeItemSchema
  .extend<OrderItemInput, IOrderItem>(
    (b, m) =>
      b
        .field(m.lax("costPrice", 0).readonly().validate(validatePrice))
        .field(m.lax("price", 0).readonly().validate(validatePrice)),
    {
      timestamps: true,
      remove: [
        "_readOnlyNoInit",
        "_dependentReadOnly",
        "_virtualForDependentReadOnly",
      ],
    },
  )
  .getModel();
