import { toArray } from "../../utils/functions";
import {
  ResponseInput_,
  ValidatorResponse,
  TypeOf,
  InternalValidatorResponse,
} from "../interfaces";

export const makeInternalResponse = <T = undefined>(
  input: ResponseInput_<any, any, T>
): InternalValidatorResponse<TypeOf<T>> => {
  if (input.valid) {
    const { valid, validated } = input;

    return { valid, validated } as InternalValidatorResponse<TypeOf<T>>;
  }

  // eslint-disable-next-line prefer-const
  let { otherReasons, reason, reasons, valid } = input as any;

  if (reasons) reasons = toArray(reasons);
  else reasons = [];

  if (reason) reasons = [...reasons, ...toArray(reason)];

  return { otherReasons, reasons, valid } as InternalValidatorResponse<
    TypeOf<T>
  >;
};

export const makeResponse = <T = undefined>(
  input: ResponseInput_<any, any, T>
) => {
  return makeInternalResponse(input) as ValidatorResponse<TypeOf<T>>;
};

export const isPropertyOn = (prop: string | number, object: any) =>
  Object.hasOwnProperty.call(object, prop);
