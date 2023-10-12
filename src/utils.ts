/* eslint-disable prefer-const */

import {
  ValidatorResponseObject,
  KeyOf,
  TypeOf,
  ValidationResponse
} from './schema/types';

export {
  makeResponse,
  getKeysAsProps,
  getSetValuesAsProps,
  getUnique,
  getUniqueBy,
  hasAnyOf,
  isEqual,
  isFunction,
  isPropertyOf,
  isNullOrUndefined,
  isObject,
  isOneOf,
  toArray,
  sort,
  sortKeys
};

export type { ObjectType, FieldKey };

type FieldKey = number | string;

type ObjectType = Record<FieldKey, any> & {};

function makeResponse<T = undefined>(
  input: ValidatorResponseObject<T>
): ValidationResponse<TypeOf<T>> {
  if (input.valid) {
    const { valid, validated } = input;

    return { valid, validated } as ValidationResponse<TypeOf<T>>;
  }

  let {
    metadata = null,
    otherReasons,
    reason,
    reasons,
    valid,
    value
  } = input as any;

  if (reasons) reasons = toArray(reasons);
  else reasons = [];

  if (reason) reasons = [...reasons, ...toArray(reason)];

  return {
    metadata,
    reasons,
    valid,
    value,
    ...(otherReasons && { otherReasons })
  } as ValidationResponse<TypeOf<T>>;
}

function getKeysAsProps<T>(object: T) {
  return Object.keys(object as object) as KeyOf<T>[];
}

function getSetValuesAsProps<T>(set: Set<T>) {
  return Array.from(set.values()) as T[];
}

function hasAnyOf(object: any, props: FieldKey[]): boolean {
  return toArray(props).some((prop) => isPropertyOf(prop, object));
}

/**
 * tells whether `a` & `b` are equals
 * @param  depth how deep in nesting should equality checks be performed for objects
 */

function isEqual<T>(a: any, b: T, depth = 1): a is T {
  if (a instanceof Date && b instanceof Date)
    return a.getTime() === b.getTime();

  if (!a || !b || (typeof a !== 'object' && typeof b !== 'object'))
    return a === b;

  let keysOfA = Object.keys(a),
    keysOfB = Object.keys(b as any);

  if (keysOfA.length != keysOfB.length) return false;
  (keysOfA = sort(keysOfA)), (keysOfB = sort(keysOfB));

  if (JSON.stringify(keysOfA) != JSON.stringify(keysOfB)) return false;

  if (depth > 0 && keysOfA.length)
    return keysOfA.every((key) => isEqual(a[key], (b as any)[key], depth - 1));

  return JSON.stringify(sortKeys(a)) == JSON.stringify(sortKeys(b as any));
}

function isFunction(value: any): value is Function {
  return typeof value === 'function';
}

function isNullOrUndefined(value: any): value is null | undefined {
  return isOneOf(value, [null, undefined]);
}

function isObject(value: any): value is ObjectType {
  return value && typeof value === 'object' && !Array.isArray(value);
}

function isOneOf<T>(value: any, values: T[]): value is T {
  return values.includes(value);
}

function isPropertyOf<T>(
  prop: string | number | symbol,
  object: T
): prop is keyof T {
  return Object.hasOwnProperty.call(object, prop);
}

function toArray<T>(value: T | T[]): T[] {
  return Array.isArray(value) ? value : [value];
}

function sort<T>(data: T[]): T[] {
  return data.sort((a, b) => (a < b ? -1 : 1));
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

  list.forEach((dt) => (obj[_getDeepValue(dt as ObjectType, key)] = dt));

  return Object.values(obj) as T[];
}

function _getDeepValue(data: ObjectType, key: string): any {
  return key.split('.').reduce((prev, next) => prev?.[next], data);
}

function _serialize(dt: any, revert = false) {
  try {
    return revert ? JSON.parse(dt) : JSON.stringify(dt);
  } catch (err) {
    return dt;
  }
}
