export function isBooleanOk(value: any) {
  let valid = true,
    reason = "";

  if (typeof value !== "boolean") {
    valid = false;
    reason = "Expected a boolean";
  }

  return { reason, valid, validated: valid ? value : undefined };
}
