import { looseObject } from "./interfaces";

export function belongsTo(value: any, values: any[]): boolean {
  return values.includes(value);
}

export const isOfType = (v: any, _type: string) => typeof v === _type;

export const isFunction = (v: any) => isOfType(v, "function");

export function makeUnique({
  data = [],
  key = "id",
}: {
  data: object[];
  key: string;
}) {
  const obj: looseObject = {};

  data.forEach((dt: looseObject) => (obj[dt[key]] = dt));

  return Object.values(obj);
}
