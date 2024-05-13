import { StringValidatorOptions, MutableSummary } from '../../../../dist';
import {
  IOtherMeasureUnit,
  IOtherQuantity,
  StoreItemInput,
  StoreItem,
} from './types';

type SummaryType = MutableSummary<StoreItemInput, StoreItem>;

import {
  makeArrayValidator,
  makeNumberValidator,
  makeStringValidator,
} from '../../../../src';
import { findBy } from '../../_utils';

export const validateName = (val: any) =>
  makeStringValidator({ trim: true })(val);

export const validateString = (
  errorMessage = '',
  options: StringValidatorOptions = {},
) => {
  return (val: any) => {
    const isValid = makeStringValidator(options)(val);

    if (!isValid.valid && errorMessage)
      isValid.reason = [errorMessage, ...isValid.reason];

    return isValid;
  };
};

export const validateOtherUnit = (value: any) => {
  const isValidCoeff = makeNumberValidator({
    min: 0.1,
  })(value?.coefficient);
  const isValidName = makeStringValidator()(value?.name);

  if (!isValidCoeff.valid || !isValidName.valid) return { valid: false };

  return {
    valid: true,
    validated: {
      coefficient: isValidCoeff.validated,
      name: isValidName.validated,
    },
  };
};

export const validateOtherUnits = (value: any) => {
  return makeArrayValidator<IOtherMeasureUnit>({
    min: 1,
    sorted: true,
    filter: (v) => validateOtherUnit(v).valid,
    modifier: (v) => validateOtherUnit(v).validated,
    sorter: (a, b) => (a.name < b.name ? -1 : 1),
    uniqueKey: 'name',
  })(value);
};

export const validatePrice = (value: any) =>
  makeNumberValidator({ min: 0 })(value);

export const validateQuantity = (value: any) =>
  makeNumberValidator({ min: 0 })(value);

export const validateOtherQuantity = (value: any, ctx: StoreItemInput) => {
  const mu = getMeasureUnit(ctx.otherMeasureUnits!, value?.name);

  const isValidQty = makeNumberValidator({
    min: 0.1,
  })(value?.quantity);

  if (!mu || !isValidQty.valid) return { valid: false };

  return {
    valid: true,
    validated: { name: value.name, quantity: isValidQty.validated },
  };
};

export const validateQuantities = async (
  value: any,
  { context }: SummaryType,
) => {
  return makeArrayValidator<IOtherQuantity>({
    min: 1,
    unique: false,
    filter: (v) => validateOtherQuantity(v, context).valid,
    modifier: (v) => validateOtherQuantity(v, context).validated,
  })(value);
};

const getMeasureUnit = (
  otherMeasureUnits: IOtherMeasureUnit[],
  name: string,
) => {
  return findBy(otherMeasureUnits, { name });
};

export const sanitizeQuantities = ({
  context: { quantities, otherMeasureUnits },
}: SummaryType) => {
  return (quantities as IOtherQuantity[]).reduce((prev, { name, quantity }) => {
    const mu = getMeasureUnit(otherMeasureUnits!, name);

    if (!mu) return prev;

    return (prev += quantity * mu.coefficient);
  }, 0);
};
