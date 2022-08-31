import { IStringOptions } from "../../../../../dist";
import { IOtherMeasureUnit, IOtherQuantity, IStoreItem } from "./interfaces";

import {
  isArrayOk,
  isNumberOk,
  isStringOk,
} from "../../../../../dist/validate";
import { findBy } from "../../utils";

export const validateName = (val: any) => {
  let { valid, validated } = isStringOk(val);

  if (!valid) return { valid: false };

  return { valid, validated };
};

export const validateString = (
  errorMessage = "",
  options: IStringOptions = {}
) => {
  return (val: any) => {
    let { reasons, valid, validated } = isStringOk(val, options);

    if (!valid && errorMessage) reasons = [errorMessage, ...reasons!];

    return { reasons, valid, validated };
  };
};

export const validateOtherUnit = (value: any) => {
  const { valid: validCoeff, validated: coefficient } = isNumberOk(
    value?.coefficient,
    { range: { bounds: [0], inclusiveBottom: false } }
  );
  const { valid: validName, validated: name } = isStringOk(value?.name);

  if (!validCoeff || !validName) return { valid: false };

  return { valid: true, validated: { coefficient, name } };
};

export const validateOtherUnits = (value: any) => {
  return isArrayOk<IOtherMeasureUnit>(value, {
    empty: true,
    sorted: true,
    filter: (v) => validateOtherUnit(v).valid,
    modifier: (v) => validateOtherUnit(v).validated,
    sorter: (a, b) => (a.name < b.name ? -1 : 1),
    uniqueKey: "name",
  });
};

export const validatePrice = (value: any) =>
  isNumberOk(value, { range: { bounds: [0] } });

export const validateQuantity = (value: any) =>
  isNumberOk(value, { range: { bounds: [0] } });

export const validateOtherQuantity = (value: any, ctx: IStoreItem) => {
  const mu = getMeasureUnit(ctx.otherMeasureUnits!, value?.name);

  const { valid: validQty, validated: quantity } = isNumberOk(value?.quantity, {
    range: { bounds: [0], inclusiveBottom: false },
  });

  if (!mu || !validQty) return { valid: false };

  return { valid: true, validated: { name: value.name, quantity } };
};

export const validateQuantities = async (value: any, ctx: IStoreItem) => {
  return isArrayOk<IOtherQuantity>(value, {
    empty: true,
    unique: false,
    filter: (v) => validateOtherQuantity(v, ctx).valid,
    modifier: (v) => validateOtherQuantity(v, ctx).validated,
  });
};

export const onQuantityChange = ({ quantityChangeCounter }: IStoreItem) => {
  return { quantityChangeCounter: quantityChangeCounter! + 1 };
};

const getMeasureUnit = (
  otherMeasureUnits: IOtherMeasureUnit[],
  name: string
) => {
  return findBy(otherMeasureUnits, { name });
};

export const onQuantitiesChange = ({
  quantity,
  quantities,
  otherMeasureUnits,
}: IStoreItem) => {
  const _quantity = quantities!.reduce((prev, { name, quantity }) => {
    const mu = getMeasureUnit(otherMeasureUnits!, name);

    if (!mu) return prev;

    return (prev += quantity * mu.coefficient);
  }, quantity!);

  return { quantity: _quantity };
};
