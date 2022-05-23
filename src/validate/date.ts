import { datePropTypes } from "../utils/interfaces";

export default function isDateOk(
  value: any,
  { max = 150, min = 1, enums }: datePropTypes = {}
) {
  let valid = true,
    reason = "";

  // if () {
  //   return { valid: false, reason: "Expected a date" };
  // }

  // Date.parse()

  // value = Date(value)

  // if (value.length < minLength) {
  //   valid = false;
  //   reason = "too short";
  // }

  // if (value.length > maxLength) {
  //   valid = false;
  //   reason = "too long";
  // }

  if (enums && !enums.includes(value)) {
    valid = false;
    reason = "unaccepted value";
  }

  return { reason, valid, validated: value };
}
