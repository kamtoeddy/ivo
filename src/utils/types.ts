import { XOR } from '../schema/types'

export type ArrayOptions<T> = {
  empty?: boolean
  filter?: (data: T) => boolean | Promise<boolean>
  modifier?: (data: T) => any | Promise<any>
  sorted?: boolean
  sorter?: (a: T, b: T) => number
  sortOrder?: 'asc' | 'desc'
  unique?: boolean
  uniqueKey?: string
}

export type PayloadKey = number | string

export type ObjectType = Record<PayloadKey, any> & {}

export type NumberRangeType = {
  bounds: number[]
  inclusiveBottom?: boolean
  inclusiveTop?: boolean
}

export type StringOptions<T extends string = string> = XOR<
  { enums: T[] | readonly T[] },
  { maxLength?: number; minLength?: number; regExp?: RegExp; trim?: boolean }
>
