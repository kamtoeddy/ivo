import { belongsTo } from "../utils/functions";
import { stringPropTypes } from "../utils/interfaces";

export function isStringOk(
  str: any,
  { match, maxLength = 40, minLength = 1, enums }: stringPropTypes = {}
) {
  let valid = true,
    reason = "";

  str = String(str).trim();

  if (str.length < minLength) {
    valid = false;
    reason = "too short";
  }

  if (str.length > maxLength) {
    valid = false;
    reason = "too long";
  }

  if (match && !match.test(str)) {
    return { valid: false, reason: "Unacceptable value" };
  }

  if (enums && !belongsTo(str, enums)) {
    valid = false;
    reason = "Unacceptable value";
  }

  return { reason, valid, validated: valid ? str : undefined };
}
