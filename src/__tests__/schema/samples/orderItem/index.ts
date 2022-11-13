import { Schema } from "../../../../../dist";
import { storeItemSchema } from "../storeItem";
import { validatePrice } from "../storeItem/validators";
import { IOrderItem } from "./interfaces";

const orderItemSchema = new Schema<IOrderItem>(
  {
    costPrice: { readonly: true, validator: validatePrice },
    price: { readonly: true, validator: validatePrice },
  },
  { errors: "throw", timestamps: true }
).extend(storeItemSchema, {
  remove: [
    "_dependentReadOnly",
    "_readOnlyNoInit",
    "_sideEffectForDependentReadOnly",
  ],
});

export const OrderItem = orderItemSchema.getModel();
