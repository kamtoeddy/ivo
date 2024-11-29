import { storeItemSchema } from '.';
import { validateString } from './validators';

const storeItemChildSchema = storeItemSchema.extend(
  {
    childID: { readonly: true, validator: validateString('Invalid child id') },
  },
  { timestamps: true },
);

const StoreItemChild = storeItemChildSchema.getModel();

export { StoreItemChild };
