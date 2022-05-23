import { looseObject } from "./interfaces";

type isAcceptableOptionsType = {
  toAccept?: any[];
  toReject?: any[];
};

export function isAcceptable(
  value: any,
  {
    toAccept = [],
    toReject = [NaN, null, undefined],
  }: isAcceptableOptionsType = {
    toReject: [NaN, null, undefined],
  }
): boolean {
  if (!toAccept?.includes(value) || toReject?.includes(value)) return false;

  return true;
}

export function makeUnique({
  data = [],
  key = "id",
}: {
  data: object[];
  key: string;
}) {
  const obj: looseObject = {};

  data.forEach((dt: looseObject) => setProperty(obj, getProperty(dt, key), dt));

  return Object.values(obj);
}

export function getProperty<O, K extends keyof O>(obj: O, key: K) {
  return obj[key];
}

export function setProperty<O, K extends keyof O>(obj: O, key: K, value: O[K]) {
  obj[key] = value;
}
