import { IStringOptions } from "../../../../utils/interfaces";
import { isArrayOk, isNumberOk, isStringOk } from "../../../../validate";

export const validateString = (
  errorMessage = "",
  options: IStringOptions = {}
) => {
  return (val: any) => {
    let { reasons, valid, validated } = isStringOk(val, options);

    if (!valid && errorMessage) reasons = [errorMessage];

    return { reasons, valid, validated };
  };
};

export const validateOtherUnits = (value: any) => {
  return isArrayOk(value, { empty: true, uniqueKey: "name" });
};

export const validatePrice = (value: any) =>
  isNumberOk(value, { range: { bounds: [0] } });

export const validateQuantity = (value: any) =>
  isNumberOk(value, { range: { bounds: [0] } });

export const validateQuantities = (value: any) =>
  isNumberOk(value, { range: { bounds: [0] } });
