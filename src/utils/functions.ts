import { StringKey } from '../schema/types'
import { ObjectType } from './types'

export {
  getKeysAsProps,
  isEqual,
  isFunction,
  isKeyOf,
  isNullOrUndefined,
  isObject,
  isOneOf,
  toArray,
  sort,
  sortKeys
}

function getKeysAsProps<T>(object: T) {
  return Object.keys(object as object) as StringKey<T>[]
}

function isEqual(a: any, b: any) {
  const typeOfA = typeof a

  if (typeOfA != typeof b) return false

  if (typeOfA == 'undefined') return true

  if (['bigint', 'boolean', 'number', 'string', 'symbol'].includes(typeOfA))
    return a == b

  const refA = isNullOrUndefined(a) ? a : JSON.stringify(sortKeys(a))
  const refB = isNullOrUndefined(b) ? b : JSON.stringify(sortKeys(b))

  return refA == refB
}

function isFunction(value: any): value is Function {
  return typeof value === 'function'
}

function isKeyOf(prop: string | number, object: any): prop is keyof object {
  return Object.hasOwnProperty.call(object, prop)
}

function isNullOrUndefined(value: any): value is null | undefined {
  return isOneOf(value, [null, undefined])
}

function isObject(value: any): value is ObjectType {
  return value && typeof value === 'object' && !Array.isArray(value)
}

function isOneOf<T>(value: any, values: T[]): value is T {
  return values.includes(value)
}

function toArray<T>(value: T | T[]): T[] {
  return Array.isArray(value) ? value : [value]
}

function sort<T>(data: T[]): T[] {
  return data.sort((a, b) => (a < b ? -1 : 1))
}

function sortKeys<T extends ObjectType>(object: T): T {
  const keys = sort(Object.keys(object))

  return keys.reduce((prev, next: keyof T) => {
    prev[next] = object[next]

    return prev
  }, {} as T)
}
