import { looseObject } from "../interfaces";

import * as f_number from "./number";
import * as f_string from "./string";

type modeType = "number" | "string";

const datatypes: looseObject = { number: f_number, string: f_string };

export default function format(
  value: any,
  mode: modeType,
  options: looseObject
) {
  const _type = datatypes[mode];

  if (_type) {
    Object.keys(options).forEach((method) => {
      value = _type[method](value, options[method]);
    });
  }

  return value;
}
