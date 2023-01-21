import { toArray } from "../../utils/functions";
import { ResponseInput, ValidatorResponse, TypeOf } from "../interfaces";

export const makeResponse = <T = undefined>(
  input: ResponseInput<T>
): ValidatorResponse<TypeOf<T>> => {
  if (input.valid) {
    const { valid, validated } = input;

    return { valid, validated } as ValidatorResponse<TypeOf<T>>;
  }

  let { reason, reasons, valid } = input as any;

  if (reasons) reasons = toArray(reasons);
  else reasons = [];

  if (reason) reasons = [...reasons, ...toArray(reason)];

  return { reasons, valid, validated: undefined } as ValidatorResponse<
    TypeOf<T>
  >;
};
