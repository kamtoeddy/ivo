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

export type ErrorPayload = Record<PayloadKey, string[]>
export type InputPayload = Record<PayloadKey, string | string[]>

export type SchemaErrorMessage =
  | 'Invalid Data'
  | 'Invalid Schema'
  | 'Nothing to update'
  | 'Validation Error'

export type SchemaErrorProps = {
  message: SchemaErrorMessage
  payload?: ErrorPayload
  statusCode?: number
}

export type ErrorToolProps = {
  message: SchemaErrorMessage
  payload?: InputPayload
  statusCode?: number
}

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
