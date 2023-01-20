import { ObjectType } from "./interfaces";

export const belongsTo = (value: any, values: any[]) => values.includes(value);

export const toArray = <T>(value: T | T[]): T[] =>
  Array.isArray(value) ? value : [value];

export const sort = <T>(data: T[]): T[] =>
  data.sort((a, b) => (a < b ? -1 : 1));

export function sortKeys<T extends ObjectType>(obj: T): T {
  const keys = sort(Object.keys(obj));

  return keys.reduce((prev, next: keyof T) => {
    prev[next] = obj[next];

    return prev;
  }, {} as T);
}
