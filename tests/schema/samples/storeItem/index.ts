import { Summary, Schema, isStringOk } from '../../../../dist';
import type { StoreItem, StoreItemInput } from './types';
import {
  sanitizeQuantities,
  validateName,
  validateOtherUnits,
  validatePrice,
  validateQuantities,
  validateQuantity,
  validateString
} from './validators';

const storeItemSchema = new Schema<StoreItemInput, StoreItem>(
  {
    _dependentReadOnly: {
      default: 0,
      dependsOn: '_virtualForDependentReadOnly',
      readonly: true,
      resolver: () => 1
    },
    _laxProp: { default: '', validator: validateString('Invalid lax prop') },
    _readOnlyLax1: { default: () => '', readonly: 'lax' },
    _readOnlyLax2: { default: '', readonly: 'lax' },
    _readOnlyNoInit: { default: '', readonly: true, shouldInit: false },
    _virtualForDependentReadOnly: { virtual: true, validator: () => true },
    id: { readonly: true, validator: validateString('Invalid id') },
    name: { required: true, validator: validateName },
    measureUnit: {
      required: true,
      validator: validateString('Invalid measure unit')
    },
    otherMeasureUnits: { default: [], validator: validateOtherUnits },
    price: { required: true, validator: validatePrice },
    quantities: {
      virtual: true,
      sanitizer: sanitizeQuantities,
      validator: validateQuantities
    },
    quantity: {
      default: 0,
      dependsOn: ['_quantity', 'quantities'],
      resolver: resolveQuantity
    },
    _quantity: {
      alias: '__quantity',
      virtual: true,
      validator: validateQuantity
    },
    quantityChangeCounter: {
      default: 0,
      dependsOn: 'quantity',
      resolver({ context: { quantityChangeCounter } }) {
        return quantityChangeCounter! + 1;
      }
    }
  },
  {
    errors: 'throw',
    onSuccess,
    timestamps: { createdAt: 'c_At', updatedAt: 'u_At' }
  }
);

function resolveQuantity({
  context: { quantity, _quantity, quantities }
}: Summary<StoreItemInput, StoreItem>) {
  const newQty = _quantity ?? quantity;

  return quantities ? newQty + (quantities as number) : newQty;
}

function onSuccess({
  context: { quantity, _quantity, quantities }
}: Summary<StoreItemInput, StoreItem>) {
  const newQty = _quantity ?? quantity;

  return quantities ? newQty + (quantities as number) : newQty;
}

const StoreItem = storeItemSchema.getModel();

export { StoreItem, storeItemSchema };
