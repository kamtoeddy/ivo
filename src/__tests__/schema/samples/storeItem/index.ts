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
      default: () => 0,
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
  { errors: "throw", timestamps: { createdAt: "c_At", updatedAt: "u_At" } }
);

// this type of handler should not affect the next
// operation context
function badHandler(ctx: any) {
  try {
    ctx._dependentReadOnly = 1;
    ctx.otherMeasureUnits = null;
    ctx.quantity = 10000;
  } catch (err: any) {}
}

const StoreItem = storeItemSchema.getModel();

// const {data} = await StoreItem.create({})

// data

export { StoreItem, storeItemSchema };
