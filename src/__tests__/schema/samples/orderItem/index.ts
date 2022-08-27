import { makeModel, Schema } from "../../../../../dist";
import { storeItemSchema } from "../storeItem";
import { validatePrice } from "../storeItem/validators";
import { IOrderItem } from "./interfaces";

const orderItemSchema = new Schema(
  {
    costPrice: { readonly: true, validator: validatePrice },
    price: { readonly: true, validator: validatePrice },
  },
  { timestamps: true }
).extend(storeItemSchema, {
  remove: [
    "_readOnlyNoInit",
    "_dependentReadOnly",
    "_sideEffectForDependentReadOnly",
  ],
});

export const OrderItem = makeModel<IOrderItem>(orderItemSchema);
