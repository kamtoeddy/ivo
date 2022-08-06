import { storeItemSchema } from ".";
import { makeModel, Schema } from "../../../../lib";
import { IStoreItemChild } from "./interfaces";
import { validateString } from "./validators";

const storeItemChildSchema = new Schema(
  {
    childID: { readonly: true, validator: validateString("Invalid child id") },
  },
  { timestamps: true }
).extend(storeItemSchema);

const StoreItemChild = makeModel<IStoreItemChild>(storeItemChildSchema);

export { StoreItemChild };
