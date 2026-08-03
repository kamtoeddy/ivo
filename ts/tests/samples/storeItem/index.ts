import { type IvoContext, Schema } from "../../../src";
import type { StoreItem, StoreItemInput } from "./types";
import {
  sanitizeQuantities,
  validateName,
  validateOtherUnits,
  validatePrice,
  validateQuantities,
  validateQuantity,
  validateString,
} from "./validators";

export { StoreItemModel, storeItemSchema };

const storeItemSchema = new Schema<StoreItemInput, StoreItem>(
  (b, m) =>
    b
      .field(
        m
          .dependent("_dependentReadOnly", "_virtualForDependentReadOnly")
          .default(0)
          .resolve(() => 1),
      )
      .field(
        m
          .lax("_laxField")
          .default("")
          .validate(validateString("Invalid lax field")),
      )
      .field(m.lax("_readOnlyLax1").default("").readonly())
      .field(m.lax("_readOnlyLax2").default("").readonly())
      .field(m.lax("_readOnlyNoInit").default("").readonly())
      .field(m.virtual("_virtualForDependentReadOnly").validate(() => true))
      .field(m.lax("id").default("").validate(validateString("Invalid id")))
      .field(m.required("name").validate(validateName))
      .field(
        m
          .required("measureUnit")
          .validate(validateString("Invalid measure unit")),
      )
      .field(
        m.lax("otherMeasureUnits").default([]).validate(validateOtherUnits),
      )
      .field(m.required("price").validate(validatePrice))
      .field(
        m
          .virtual("quantities")
          .validate(validateQuantities)
          .sanitize(sanitizeQuantities),
      )
      .field(
        m
          .dependent("quantity", ["_quantity", "quantities"])
          .default(0)
          .resolve(resolveQuantity),
      )
      .field(
        m.virtual("_quantity").alias("__quantity").validate(validateQuantity),
      )
      .field(
        m
          .dependent("quantityChangeCounter", "quantity")
          .default(0)
          .resolve(
            ({ values: { quantityChangeCounter } }) =>
              (quantityChangeCounter ?? 0) + 1,
          ),
      ),

  {
    onSuccess,
    timestamps: { createdAt: "c_At", updatedAt: "u_At" },
  },
);

function resolveQuantity({
  input: { __quantity, quantities },
  values: { quantity },
}: IvoContext<StoreItemInput, StoreItem>) {
  const newQty = __quantity ?? quantity ?? 0;

  return quantities ? newQty + (quantities as number) : newQty;
}

function onSuccess() {}

const StoreItemModel = storeItemSchema.getModel();
