import { storeItemSchema } from ".";
import { Schema } from "../../../../../dist";
import { validateString } from "./validators";

const storeItemChildSchema = new Schema(
  {
    childID: { readonly: true, validator: validateString("Invalid child id") },
  },
  { timestamps: true }
).extend(storeItemSchema);

const StoreItemChild = storeItemChildSchema.getModel();

export { StoreItemChild };
