import { storeItemSchema } from ".";
import type { StoreItem, StoreItemInput } from "./types";
import { validateString } from "./validators";

const storeItemChildSchema = storeItemSchema.extend<
  StoreItemInput & { childID: string },
  StoreItem & { childID: string }
>(
  (b, m) =>
    b.field(
      m
        .lax("childID", "")
        .validate(validateString("Invalid child id"))
        .readonly(),
    ),
  { timestamps: true },
);

const StoreItemChild = storeItemChildSchema.getModel();

export { StoreItemChild };
