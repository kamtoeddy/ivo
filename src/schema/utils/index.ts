import { toArray } from "../../utils/functions";
import { ResponseInput, ValidatorResponse } from "../interfaces";

export const makeResponse = <T = undefined>({
  reason,
  reasons,
  valid,
  validated,
}: ResponseInput): ValidatorResponse<T> => {
  if (valid) return { reasons: [], valid, validated };

  if (reasons) reasons = [...toArray(reasons)];
  else reasons = [];

  if (reason) reasons = [...reasons, ...toArray(reason)];

  return { reasons, valid, validated: undefined };
};
