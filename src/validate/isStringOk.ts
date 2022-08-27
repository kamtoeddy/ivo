import { belongsTo } from "../utils/functions";
import { IStringOptions } from "../utils/interfaces";

export function isStringOk(
  str: any,
  { enums, maxLength = 40, minLength = 1, regExp }: IStringOptions = {}
) {
  let valid = true,
    reasons: string[] = [],
    validated = undefined;

  if (belongsTo(str, [null, undefined]))
    return { reasons: ["Unacceptable value"], valid: false, validated };

  str = String(str).trim();

  if (str.length < minLength)
    return { reasons: ["Too short"], valid: false, validated };

  if (str.length > maxLength)
    return { reasons: ["Too long"], valid: false, validated };

  if (regExp && !regExp.test(str))
    return { reasons: ["Unacceptable value"], valid: false, validated };

  if (enums && !belongsTo(str, enums))
    return { reasons: ["Unacceptable value"], valid: false, validated };

  return { reasons, valid, validated: str };
}
