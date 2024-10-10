/* eslint-disable prefer-const */

import {
  ValidatorResponseObject,
  KeyOf,
  TypeOf,
  ValidationResponse,
  ArrayOfMinSizeTwo,
} from "./schema/types";

export {
  makeResponse,
  getKeysAsProps,
  getSetValuesAsProps,
  getUnique,
  getUniqueBy,
  hasAnyOf,
  isEqual,
  isFunctionLike,
  isNullOrUndefined,
  isOneOf,
  isPropertyOf,
  isRecordLike,
  toArray,
  sort,
  sortKeys,
};

export type { ObjectType, FieldKey };

type FieldKey = number | string;

type ObjectType<T = Record<FieldKey, unknown>> = T extends object
  ? T extends unknown[]
    ? never
    : T & {}
  : never;

function makeResponse<T = undefined>(
  input: ValidatorResponseObject<T>,
): ValidationResponse<TypeOf<T>> {
  if (input.valid) {
    const { valid, validated } = input;

    return { valid, validated } as ValidationResponse<TypeOf<T>>;
  }

  const { metadata = null, reason: inputReason, valid, value } = input;

  const reason = inputReason
    ? isRecordLike(inputReason)
      ? inputReason
      : toArray(inputReason)
    : [];

  return {
    metadata,
    reason,
    valid,
    value,
  } as ValidationResponse<TypeOf<T>>;
}

function getKeysAsProps<T>(object: T) {
  return Object.keys(object as object) as KeyOf<T>[];
}

function getSetValuesAsProps<T>(set: Set<T>) {
  return Array.from(set.values()) as T[];
}

function hasAnyOf(object: unknown, props: FieldKey[]): boolean {
  return toArray(props).some((prop) => isPropertyOf(prop, object));
}

/**
 * tells whether `a` & `b` are equals
 * @param  depth how deep in nesting should equality checks be performed for objects
 */

function isEqual<T>(a: unknown, b: T, depth = 1): a is T {
  if (a instanceof Date && b instanceof Date)
    return a.getTime() === b.getTime();

  if (!a || !b || (typeof a !== "object" && typeof b !== "object"))
    return a === b;

  let keysOfA = Object.keys(a),
    keysOfB = Object.keys(b as never);

  if (keysOfA.length != keysOfB.length) return false;
  (keysOfA = sort(keysOfA)), (keysOfB = sort(keysOfB));

  if (JSON.stringify(keysOfA) != JSON.stringify(keysOfB)) return false;

  if (depth > 0 && keysOfA.length)
    return keysOfA.every((key) =>
      isEqual(a[key as never], (b as never)[key], depth - 1),
    );

  return (
    JSON.stringify(sortKeys(a as never)) == JSON.stringify(sortKeys(b as never))
  );
}

function isFunctionLike<T extends Function>(value: unknown): value is T {
  return typeof value === "function";
}

function isNullOrUndefined(value: unknown): value is null | undefined {
  return isOneOf(value, [null, undefined]);
}

function isOneOf<T>(value: unknown, values: ArrayOfMinSizeTwo<T>): value is T {
  return values.includes(value as never);
}

function isRecordLike<T extends ObjectType>(
  value: unknown,
): value is ObjectType<T> {
  return !!value && typeof value === "object" && !Array.isArray(value);
}

function isPropertyOf<T>(
  prop: string | number | symbol,
  object: T,
): prop is keyof T {
  return Object.hasOwnProperty.call(object, prop);
}

function toArray<T>(value: T | T[]): T[] {
  return Array.isArray(value) ? value : [value];
}

function sort<T>(data: T[]): T[] {
  return [...data].sort((a, b) => (a < b ? -1 : 1));
}

function sortKeys<T extends ObjectType>(object: T): T {
  const keys = sort(Object.keys(object));

  return keys.reduce((prev, next: keyof T) => {
    prev[next] = object[next];

    return prev;
  }, {} as T);
}

function getUnique<T>(list: T[]) {
  let _list = list.map((dt) => _serialize(dt));

  _list = Array.from(new Set(_list));

  return _list.map((dt) => _serialize(dt, true));
}

function getUniqueBy<T>(list: T[], key?: string) {
  if (!key) return getUnique(list);

  const obj: ObjectType = {};

  list.forEach(
    (dt) => (obj[_getDeepValue(dt as ObjectType, key) as never] = dt),
  );

  return Object.values(obj) as T[];
}

function _getDeepValue(data: ObjectType, key: string): unknown {
  return key.split(".").reduce((prev, next) => prev?.[next] as never, data);
}

function _serialize(dt: unknown, revert = false) {
  try {
    return revert ? JSON.parse(dt as never) : JSON.stringify(dt);
  } catch {
    return dt;
  }
}
