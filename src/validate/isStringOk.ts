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
    return { valid: false, validated, reasons: ["Unacceptable value"] };

  str = String(str).trim();

  if (str.length < minLength)
    return { valid: false, validated, reasons: ["too short"] };

  if (str.length > maxLength)
    return { valid: false, validated, reasons: ["too long"] };

  if (regExp && !regExp.test(str))
    return { valid: false, validated, reasons: ["Unacceptable value"] };

  if (enums && !belongsTo(str, enums))
    return { valid: false, validated, reasons: ["Unacceptable value"] };

  return { reasons, valid, validated: str };
}
