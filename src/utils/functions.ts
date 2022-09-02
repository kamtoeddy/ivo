import { ObjectType } from "./interfaces";

export const belongsTo = (value: any, values: any[]) => values.includes(value);

export const toArray = (value: any) => (Array.isArray(value) ? value : [value]);

export function sortKeys<T extends ObjectType>(obj: T): T {
  const keys = Object.keys(obj).sort((a, b) => (a < b ? -1 : 1));

  return keys.reduce((prev, next: keyof T) => {
    prev[next] = obj[next];

    return prev;
  }, {} as T);
}
