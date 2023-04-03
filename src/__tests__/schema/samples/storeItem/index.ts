import { Schema, Context, Summary } from "../../../../../dist";
import { IStoreItem, StoreItemType } from "./interfaces";
import {
  sanitizeQuantities,
  validateName,
  validateOtherUnits,
  validatePrice,
  validateQuantities,
  validateQuantity,
  validateString,
} from "./validators";

const storeItemSchema = new Schema<IStoreItem, StoreItemType>(
  {
    _dependentReadOnly: {
      default: () => 0,
      dependsOn: "_virtualForDependentReadOnly",
      readonly: true,
      dependent: true,
      resolver: () => 1,
    },
    _laxProp: { default: "", validator: validateString("Invalid lax prop") },
    _readOnlyLax1: { default: "", readonly: "lax" },
    _readOnlyLax2: { default: "", readonly: "lax" },
    _readOnlyNoInit: { default: "", readonly: true, shouldInit: false },
    _virtualForDependentReadOnly: {
      virtual: true,
      validator: () => ({ valid: true }),
    },
    id: {
      readonly: true,
      validator: validateString("Invalid id"),
    },
    name: { required: true, validator: validateName },
    measureUnit: {
      required: true,
      validator: validateString("Invalid measure unit"),
    },
    otherMeasureUnits: { default: [], validator: validateOtherUnits },
    price: { required: true, validator: validatePrice },
    quantities: {
      virtual: true,
      sanitizer: sanitizeQuantities,
      validator: validateQuantities,
    },
    quantity: {
      default: 0,
      dependent: true,
      dependsOn: ["_quantity", "quantities"],
      resolver: resolveQuantity,
    },
    _quantity: { virtual: true, validator: validateQuantity },
    quantityChangeCounter: {
      default: 0,
      dependent: true,
      dependsOn: "quantity",
      resolver({ quantityChangeCounter }) {
        return quantityChangeCounter! + 1;
      },
    },
  },
  { errors: "throw", timestamps: { createdAt: "c_At", updatedAt: "u_At" } }
);

function resolveQuantity({ quantity, _quantity, quantities }: IStoreItem) {
  const newQty = _quantity ?? (quantity as number);

  return quantities ? newQty + (quantities as number) : newQty;
}

const StoreItem = storeItemSchema.getModel();

export { StoreItem, storeItemSchema };
