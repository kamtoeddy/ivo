import { belongsTo } from "../utils/functions";
import { stringPropTypes } from "../utils/interfaces";

export default function isStringOk(
  str: any,
  { match, maxLength = 30, minLength = 1, enums }: stringPropTypes = {}
) {
  let valid = true,
    reason = "";

  if (typeof str !== "string") {
    return { valid: false, reason: "Expected a string" };
  }

  if (match && !match.test(str)) {
    return { valid: false, reason: "Unacceptable value" };
  }

  str = String(str).trim();

  if (str.length < minLength) {
    valid = false;
    reason = "too short";
  }

  if (str.length > maxLength) {
    valid = false;
    reason = "too long";
  }

  if (enums && !belongsTo(str, enums)) {
    valid = false;
    reason = "Unacceptable value";
  }

  return { reason, valid, validated: str };
}
