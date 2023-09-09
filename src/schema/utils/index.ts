import type { ISchema, StringKey } from '../types'
import { ResponseInputObject, ValidatorResponse, TypeOf } from '../types'
import { isKeyOf, sortKeys, toArray } from '../../utils'
import {
  ErrorPayload,
  ErrorToolProps,
  InputPayload,
  PayloadKey,
  SchemaErrorMessage,
  SchemaErrorProps
} from '../../utils/types'

export { ErrorTool, SchemaError, OptionsTool, makeResponse }

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
