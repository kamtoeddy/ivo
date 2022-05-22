export default function isBooleanOk(value: any) {
  let valid = true,
    reason = "";

  if (![false, true].includes(value)) {
    valid = false;
    reason = "Expected a boolean";
  }

  return { valid, reason };
}
