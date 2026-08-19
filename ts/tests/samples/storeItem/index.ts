import { type IvoContext, Schema } from '../../../src';
import type { StoreItem, StoreItemInput } from './types';
import {
  sanitizeQuantities,
  validateName,
  validateOtherUnits,
  validatePrice,
  validateQuantities,
  validateQuantity,
  validateString,
} from './validators';

export { StoreItemModel, storeItemSchema };

const storeItemSchema = new Schema<StoreItemInput, StoreItem>(
  (b) =>
    b
      .field(
        b
          .dependent('_dependentReadOnly', '_virtualForDependentReadOnly')
          .default(0)
          .resolve(() => 1),
      )
      .field(
        b.lax('_laxField', '').validate(validateString('Invalid lax field')),
      )
      .field(b.lax('_readOnlyLax1', '').readonly())
      .field(b.lax('_readOnlyLax2', '').readonly())
      .field(b.lax('_readOnlyNoInit', '').readonly())
      .field(b.virtual('_virtualForDependentReadOnly').validate(() => true))
      .field(b.lax('id', '').validate(validateString('Invalid id')))
      .field(b.required('name').validate(validateName))
      .field(
        b
          .required('measureUnit')
          .validate(validateString('Invalid measure unit')),
      )
      .field(b.lax('otherMeasureUnits', []).validate(validateOtherUnits))
      .field(b.required('price').validate(validatePrice))
      .field(
        b
          .virtual('quantities')
          .validate(validateQuantities)
          .sanitize(sanitizeQuantities),
      )
      .field(
        b
          .dependent('quantity', ['_quantity', 'quantities'])
          .default(0)
          .resolve(resolveQuantity),
      )
      .field(
        b.virtual('_quantity').alias('__quantity').validate(validateQuantity),
      )
      .field(
        b
          .dependent('quantityChangeCounter', 'quantity')
          .default(0)
          .resolve(
            ({ values: { quantityChangeCounter } }) =>
              (quantityChangeCounter ?? 0) + 1,
          ),
      ),

  {
    onSuccess,
    timestamps: { createdAt: 'c_At', updatedAt: 'u_At' },
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
