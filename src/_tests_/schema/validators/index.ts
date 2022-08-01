import { stringPropTypes } from "../../../utils/interfaces";
import { isStringOk } from "../../../validate";
import { IUser } from "../interface";

export const validate_String =
  (message?: string, options: stringPropTypes = {}) =>
  (value: any) => {
    const { reasons, valid, validated } = isStringOk(value, options);

    if (!valid && message) reasons.unshift(message);

    return { reasons, valid, validated };
  };

export function onNameChange({ firstName, lastName }: IUser) {
  return { name: `${firstName} ${lastName}` };
}
