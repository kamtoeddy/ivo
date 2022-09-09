import { Schema } from "../../../../../dist";
import { IStoreItem } from "./interfaces";
import {
  onQuantitiesChange,
  onQuantityChange,
  validateName,
  validateOtherUnits,
  validatePrice,
  validateQuantities,
  validateQuantity,
  validateString,
} from "./validators";

const storeItemSchema = new Schema<IStoreItem>(
  {
    _dependentReadOnly: {
      default: setDependentReadOnly,
      readonly: true,
      dependent: true,
    },
    _laxProp: { default: "", validator: validateString("Invalid lax prop") },
    _readOnlyLax1: { default: "", readonly: "lax" },
    _readOnlyLax2: { default: "", readonly: "lax" },
    _readOnlyNoInit: { default: "", readonly: true, shouldInit: false },
    _sideEffectForDependentReadOnly: {
      sideEffect: true,
      onChange: [badHandler, () => ({ _dependentReadOnly: 1 })],
      validator: () => ({ valid: true }),
    },
    id: {
      readonly: true,
      onChange: (ctx) => ({}),
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
      sideEffect: true,
      onChange: [onQuantitiesChange],
      validator: validateQuantities,
    },
    quantity: {
      default: 0,
      onChange: onQuantityChange,
      validator: validateQuantity,
    },
    quantityChangeCounter: { default: 0, dependent: true },
  },
  { timestamps: { createdAt: "c_At", updatedAt: "u_At" } }
);

// this type of handler should not affect the next
// operation context
function badHandler(ctx: any) {
  ctx._dependentReadOnly = 1;
  ctx.quantity = 10000;
}

function setDependentReadOnly(ctx: any) {
  ctx._laxProp = 25; // to not affect operation ctx

  return 0;
}

const StoreItem = storeItemSchema.getModel();

export { StoreItem, storeItemSchema };
