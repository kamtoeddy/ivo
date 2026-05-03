import type { IvoSummary, StringValidatorOptions } from '../../../../dist';
import type {
  IOtherMeasureUnit,
  IOtherQuantity,
  StoreItem,
  StoreItemInput,
} from './types';

type SummaryType = IvoSummary<StoreItemInput, StoreItem>;

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

    if (!isValid.valid && errorMessage) isValid.reason = errorMessage;

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
  return makeArrayValidator<
    any,
    | {
        valid: boolean;
        validated?: undefined;
      }
    | {
        valid: boolean;
        validated: {
          coefficient: number;
          name: string;
        };
      },
    IOtherMeasureUnit
  >({
    min: 1,
    filter: (v) => !!v,
    modifier: (v) => validateOtherUnit(v),
    postModFilter: (v) => v.valid,
    map: (v) => v.validated!,
    sort: (a, b) => (a.name < b.name ? -1 : 1),
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

export const validateQuantities = async (value: any, { ctx }: SummaryType) => {
  //   IOtherMeasureUnit
  // >({
  //   min: 1,
  //   filter: (v) => !!v,
  //   modifier: (v) => validateOtherUnit(v),
  //   postModFilter: (v) => v.valid,
  //   map: (v) => v.validated!,
  //   sort: (a, b) => (a.name < b.name ? -1 : 1),
  return makeArrayValidator<
    any,
    | {
        valid: boolean;
        validated?: undefined;
      }
    | {
        valid: boolean;
        validated: {
          name: string;
          quantity: number;
        };
      },
    IOtherQuantity
  >({
    min: 1,
    unique: false,
    filter: (v) => !!v,
    modifier: (v) => validateOtherQuantity(v, ctx),
    postModFilter: (v) => v.valid,
    map: (v) => v.validated!,
  })(value);
};

const getMeasureUnit = (
  otherMeasureUnits: IOtherMeasureUnit[],
  name: string,
) => {
  return findBy(otherMeasureUnits, { name });
};

export const sanitizeQuantities = ({
  ctx: { quantities, otherMeasureUnits },
}: SummaryType) => {
  return (quantities as IOtherQuantity[]).reduce((prev, { name, quantity }) => {
    const mu = getMeasureUnit(otherMeasureUnits!, name);

    if (!mu) return prev;

    // biome-ignore lint/suspicious/noAssignInExpressions: lol
    return (prev += quantity * mu.coefficient);
  }, 0);
};
