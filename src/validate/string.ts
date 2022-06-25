import { belongsTo } from "../utils/functions";
import { stringPropTypes } from "../utils/interfaces";

export function isStringOk(
  str: any,
  { match, maxLength = 40, minLength = 1, enums }: stringPropTypes = {}
) {
  if (belongsTo(str, [null, undefined]))
    return { valid: false, reason: "Unacceptable value" };

  let valid = true,
    reason = "";

  str = String(str).trim();

  if (str.length < minLength) return { valid: false, reason: "too short" };

  if (str.length > maxLength) return { valid: false, reason: "too long" };

  if (match && !match.test(str))
    return { valid: false, reason: "Unacceptable value" };

  if (enums && !belongsTo(str, enums))
    return { valid: false, reason: "Unacceptable value" };

  return { reason, valid, validated: valid ? str : undefined };
}
