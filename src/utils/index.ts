import {
  ISchema,
  ResponseInputObject,
  StringKey,
  TypeOf,
  ValidatorResponse
} from '../schema/types'
import {
  ErrorPayload,
  ErrorToolProps,
  InputPayload,
  ObjectType,
  PayloadKey,
  SchemaErrorMessage,
  SchemaErrorProps
} from './types'

export {
  ErrorTool,
  SchemaError,
  OptionsTool,
  makeResponse,
  getKeysAsProps,
  getUnique,
  getUniqueBy,
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

function makeResponse<T = undefined>(input: ResponseInputObject<any, any, T>) {
  if (input.valid) {
    const { valid, validated } = input

    return { valid, validated } as ValidatorResponse<TypeOf<T>>
  }

  // eslint-disable-next-line prefer-const
  let { otherReasons, reason, reasons, valid } = input as any

  if (reasons) reasons = toArray(reasons)
  else reasons = []

  if (reason) reasons = [...reasons, ...toArray(reason)]

  return { otherReasons, reasons, valid } as ValidatorResponse<TypeOf<T>>
}

type TimestampKey = StringKey<ISchema.Timestamp>

class OptionsTool {
  private _keys: TimestampKey[]
  private timestamps: ISchema.Timestamp

  constructor(config: ISchema.PrivateOptions) {
    const { timestamps } = config

    this.timestamps = timestamps
    this._keys = Object.keys(timestamps).filter(
      (key) => key.length > 0
    ) as TimestampKey[]
  }

  getKeys = () => {
    return {
      createdAt: this.timestamps.createdAt,
      updatedAt: this.timestamps.updatedAt
    }
  }

  isTimestampKey = (key: string) => this._keys.includes(key as TimestampKey)

  get withTimestamps() {
    return !!(this.timestamps.createdAt || this.timestamps.updatedAt)
  }
}

class SchemaError extends Error {
  payload: ErrorPayload = {}
  statusCode: number

  constructor({ message, payload = {}, statusCode = 400 }: SchemaErrorProps) {
    super(message)
    this.payload = payload
    this.statusCode = statusCode
  }
}

class ErrorTool extends Error {
  payload: ErrorPayload = {}
  statusCode: number
  private _initMessage: SchemaErrorMessage
  private _initStatusCode: number

  constructor({ message, payload = {}, statusCode = 400 }: ErrorToolProps) {
    super(message)
    this._initMessage = message
    this._initStatusCode = this.statusCode = statusCode
    this._setPayload(payload)
  }

  get isPayloadLoaded() {
    return Object.keys(this.payload).length > 0
  }

  get summary() {
    return {
      message: this.message as SchemaErrorMessage,
      payload: sortKeys(this.payload),
      statusCode: this.statusCode
    }
  }

  private _has = (field: PayloadKey) => isKeyOf(field, this.payload)

  private _setPayload = (payload: InputPayload) => {
    Object.entries(payload).forEach(([key, value]) => {
      this.add(key, value)
    })
  }

  add(field: PayloadKey, value?: string | string[]) {
    if (!value) value = []
    else value = toArray(value)

    if (this._has(field)) {
      const currentValues = this.payload[field]

      value.forEach((v) => {
        if (!currentValues.includes(v)) currentValues.push(v)
      })

      this.payload[field] = currentValues
    } else this.payload[field] = value

    return this
  }

  remove = (field: PayloadKey) => {
    delete this.payload?.[field]
    return this
  }

  reset = () => {
    this.message = this._initMessage
    this.payload = {}
    this.statusCode = this._initStatusCode

    return this
  }

  setMessage = (message: SchemaErrorMessage) => {
    this.message = message
    return this
  }

  throw = () => {
    const summary = this.summary
    this.reset()

    throw new SchemaError(summary)
  }
}

function getKeysAsProps<T>(object: T) {
  return Object.keys(object as object) as StringKey<T>[]
}

/**
 * tell whether `a` & `b` are equals
 * @param {any} a
 * @param {any} b
 * @param {number|undefined} depth how deep in nesting should equality checks be performed for objects
 * @returns {boolean}
 */

function isEqual(a: any, b: any, depth = 1): boolean {
  const typeOfA = typeof a

  if (typeOfA != typeof b) return false

  if (typeOfA == 'undefined') return true

  if (['bigint', 'boolean', 'number', 'string', 'symbol'].includes(typeOfA))
    return a == b

  if (isNullOrUndefined(a) || isNullOrUndefined(b)) return a == b

  let keysOfA = Object.keys(a),
    keysOfB = Object.keys(b)

  if (keysOfA.length != keysOfB.length) return false
  ;(keysOfA = sort(keysOfA)), (keysOfB = sort(keysOfB))

  if (JSON.stringify(keysOfA) != JSON.stringify(keysOfB)) return false

  if (depth > 0 && keysOfA.length)
    return keysOfA.every((key) => isEqual(a[key], b[key], depth - 1))

  return JSON.stringify(sortKeys(a)) == JSON.stringify(sortKeys(b))
}

function isFunction(value: any): value is Function {
  return typeof value === 'function'
}

function isKeyOf<T>(
  prop: string | number | symbol,
  object: T
): prop is keyof T {
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

function getUnique<T>(list: T[]) {
  let _list = list.map((dt) => _serialize(dt))

  _list = Array.from(new Set(_list))

  return _list.map((dt) => _serialize(dt, true))
}

function getUniqueBy<T>(list: T[], key?: string) {
  if (!key) return getUnique(list)

  const obj: ObjectType = {}

  list.forEach((dt) => (obj[_getDeepValue(dt as ObjectType, key)] = dt))

  return Object.values(obj) as T[]
}

function _getDeepValue(data: ObjectType, key: string): any {
  return key.split('.').reduce((prev, next) => prev?.[next], data)
}

function _serialize(dt: any, revert = false) {
  try {
    return revert ? JSON.parse(dt) : JSON.stringify(dt)
  } catch (err) {
    return dt
  }
}
