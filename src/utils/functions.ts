import { StringKey } from '../schema/types'
import { ObjectType } from './types'

export {
  belongsTo,
  getKeysAsProps,
  isFunction,
  isObject,
  isPropertyOn,
  toArray,
  sort,
  sortKeys
}

const belongsTo = (value: any, values: any[]) => values.includes(value)

const getKeysAsProps = <T>(data: T) =>
  Object.keys(data as object) as StringKey<T>[]

const isFunction = (v: any): v is Function => typeof v === 'function'

const isObject = (data: any): data is ObjectType => {
  return data && typeof data === 'object' && !Array.isArray(data)
}

const isPropertyOn = (prop: string | number, object: any) =>
  Object.hasOwnProperty.call(object, prop)

const toArray = <T>(value: T | T[]): T[] =>
  Array.isArray(value) ? value : [value]

const sort = <T>(data: T[]): T[] => data.sort((a, b) => (a < b ? -1 : 1))

function sortKeys<T extends ObjectType>(obj: T): T {
  const keys = sort(Object.keys(obj))

  return keys.reduce((prev, next: keyof T) => {
    prev[next] = obj[next]

    return prev
  }, {} as T)
}
