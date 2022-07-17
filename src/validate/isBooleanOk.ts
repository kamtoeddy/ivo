export function isBooleanOk(value: any) {
  let valid = true,
    reasons: string[] = [];

  if (typeof value !== "boolean") {
    valid = false;
    reasons = ["Expected a boolean"];
  }

  return { reasons, valid, validated: valid ? value : undefined };
}
