import { Schema } from "../../../../../dist";
import { storeItemSchema } from "../storeItem";
import { validatePrice } from "../storeItem/validators";
import { IOrderItem } from "./interfaces";

const orderItemSchema = storeItemSchema.extend<IOrderItem>(
  {
    costPrice: { readonly: true, validator: validatePrice },
    price: { readonly: true, validator: validatePrice },
  },
  {
    errors: "throw",
    timestamps: true,

    remove: [
      "_readOnlyNoInit",
      "_dependentReadOnly",
      "_sideEffectForDependentReadOnly",
    ],
  }
);

export const OrderItem = orderItemSchema.getModel();
