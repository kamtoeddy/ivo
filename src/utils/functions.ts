import { looseObject } from "./interfaces";

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
