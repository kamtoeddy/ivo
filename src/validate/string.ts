import { stringPropTypes } from "../utils/interfaces";

export default function isStringOk(
  value: any,
  { maxLength = 150, minLength = 1, enums }: stringPropTypes = {}
) {
  let valid = true,
    reason = "";

  if (!["bigint", "boolean", "number", "string"].includes(typeof value)) {
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

  if (enums && !enums.includes(value)) {
    valid = false;
    reason = "unaccepted value";
  }

  return { valid, reason };
}
