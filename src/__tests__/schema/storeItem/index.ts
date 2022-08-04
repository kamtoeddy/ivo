import { Schema } from "../../../schema";
import { makeModel } from "../../../schema/model";
import { IStoreItem } from "./interfaces";
import {
  onQuantitiesChange,
  validateOtherUnits,
  validatePrice,
  validateQuantities,
  validateQuantity,
  validateString,
} from "./validators";

const storeItemSchema = new Schema(
  {
    id: { readonly: true, validator: validateString("Invalid id") },
    name: { required: true, validator: validateString("Invalid name") },
    price: { required: true, validator: validatePrice },
    quantities: {
      sideEffect: true,
      onUpdate: [onQuantitiesChange],
      validator: validateQuantities,
    },
    quantity: { default: 0, validator: validateQuantity },
    measureUnit: {
      required: true,
      validator: validateString("Invalid measure unit"),
    },
    otherMeasureUnits: { default: [], validator: validateOtherUnits },
  },
  { timestamps: true }
);

const StoreItem = makeModel<IStoreItem>(storeItemSchema);

export { StoreItem, storeItemSchema };
