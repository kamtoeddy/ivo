import { toArray } from "../../utils/functions";
import { ResponseInput, TypeOf, ValidatorResponse } from "../interfaces";

export const makeResponse = <T = undefined>({
  reason,
  reasons,
  valid,
  validated,
}: ResponseInput<T>): ValidatorResponse<TypeOf<T>> => {
  if (valid)
    return { reasons: [], valid, validated } as ValidatorResponse<TypeOf<T>>;

  if (reasons) reasons = [...toArray(reasons)];
  else reasons = [];

  if (reason) reasons = [...reasons, ...toArray(reason)];

  return { reasons, valid, validated: undefined };
};
