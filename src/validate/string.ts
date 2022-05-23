import { belongsTo } from "../utils/functions";
import { stringPropTypes } from "../utils/interfaces";

export default function isStringOk(
  value: any,
  { maxLength = 150, minLength = 1, enums }: stringPropTypes = {}
) {
  let valid = true,
    reason = "";

  if (typeof value !== "string") {
    return { valid: false, reason: "Expected a string" };
  }

  value = String(value).trim();

  if (value.length < minLength) {
    valid = false;
    reason = "too short";
  }

  if (value.length > maxLength) {
    valid = false;
    reason = "too long";
  }

  if (enums && !belongsTo(value, enums)) {
    valid = false;
    reason = "unaccepted value";
  }

  return { reason, valid, validated: value };
}
