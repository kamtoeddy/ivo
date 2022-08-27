import { storeItemSchema } from ".";
import { Schema } from "../../../../../dist";
import { IStoreItemChild } from "./interfaces";
import { validateString } from "./validators";

const storeItemChildSchema = new Schema(
  {
    childID: { readonly: true, validator: validateString("Invalid child id") },
  },
  { timestamps: true }
).extend(storeItemSchema);

const StoreItemChild = storeItemChildSchema.getModel<IStoreItemChild>();

export { StoreItemChild };
