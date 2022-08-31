import { makeResponse } from "../schema/SchemaUtils";

export function isBooleanOk(value: any) {
  if (typeof value !== "boolean")
    return makeResponse({ reason: "Expected a boolean", valid: false });

  return makeResponse<boolean>({ valid: true, validated: value });
}
