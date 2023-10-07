import { isEqual, isPropertyOf, sortKeys, toArray } from '../../utils'
import { PayloadKey } from '../../utils'
import { ISchema, KeyOf } from '../types'
import {
  ErrorPayload,
  FieldError,
  InputPayload,
  SchemaErrorMessage,
  SchemaErrorProps,
  SchemaErrorPayload,
  SchemaErrorInputPayload,
  ValidationErrorProps,
  ValidationErrorMessage,
  ValidationErrorToolProps,
  SCHEMA_ERRORS,
  IValidationError
} from './types'

export * from './types'

export { ErrorTool, SchemaErrorTool, SchemaError, TimeStampTool }

type TimestampKey = KeyOf<ISchema.Timestamp>

class TimeStampTool {
  private _keys: TimestampKey[]
  private timestamps: ISchema.Timestamp

  constructor(timestamps: ISchema.Options<any, any>['timestamps']) {
    this.timestamps = this._makeTimestamps(timestamps)

    this._keys = Object.keys(this.timestamps).filter(
      (key) => key.length > 0
    ) as TimestampKey[]
  }

  private _makeTimestamps(timestamps: ISchema.Options<any, any>['timestamps']) {
    if (isEqual(timestamps, undefined)) return { createdAt: '', updatedAt: '' }

    let createdAt = 'createdAt',
      updatedAt = 'updatedAt'

    if (!timestamps || timestamps === true)
      return timestamps
        ? { createdAt, updatedAt }
        : { createdAt: '', updatedAt: '' }

    const custom_createdAt = timestamps?.createdAt
    const custom_updatedAt = timestamps?.updatedAt

    if (custom_createdAt && typeof custom_createdAt == 'string')
      createdAt = custom_createdAt.trim()

    if (custom_createdAt === false) createdAt = ''

    if (custom_updatedAt && typeof custom_updatedAt == 'string')
      updatedAt = custom_updatedAt.trim()

    if (custom_updatedAt === false) updatedAt = ''

    return { createdAt, updatedAt }
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
  payload: SchemaErrorPayload = {}

  constructor({ payload = {} }: SchemaErrorProps) {
    super(SCHEMA_ERRORS.INVALID_SCHEMA)
    this.payload = payload
  }
}

class SchemaErrorTool extends Error {
  payload: SchemaErrorPayload = {}

  constructor({ payload = {} }: SchemaErrorProps = {}) {
    super(SCHEMA_ERRORS.INVALID_SCHEMA)
    this._setPayload(payload)
  }

  get isPayloadLoaded() {
    return Object.keys(this.payload).length > 0
  }

  get data() {
    return {
      message: this.message as SchemaErrorMessage,
      payload: sortKeys(this.payload)
    }
  }

  private _has = (field: PayloadKey) => isPropertyOf(field, this.payload)

  private _setPayload = (payload: SchemaErrorInputPayload) => {
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
    this.payload = {}

    return this
  }

  setMessage = (message: SchemaErrorMessage) => {
    this.message = message

    return this
  }

  throw = () => {
    const summary = this.data
    this.reset()

    throw new SchemaError(summary)
  }
}

class ValidationError<
  OutputKeys extends PayloadKey = PayloadKey
> extends Error {
  payload: ErrorPayload<OutputKeys> = {}

  constructor({ message, payload = {} }: ValidationErrorProps<OutputKeys>) {
    super(message)
    this.payload = payload
  }
}

class ErrorTool<PayloadKeys extends PayloadKey = PayloadKey>
  extends Error
  implements IValidationError<PayloadKeys>
{
  private _payload: ErrorPayload<PayloadKeys> = {}

  constructor(public message: ValidationErrorToolProps['message']) {
    super(message)
  }

  get isLoaded() {
    return Object.keys(this._payload).length > 0
  }

  get data() {
    return {
      message: this.message as ValidationErrorMessage,
      payload: sortKeys(this._payload)
    }
  }

  get payload() {
    return this._payload
  }

  private _has = (field: PayloadKeys) => isPropertyOf(field, this._payload)

  add(field: PayloadKeys, value?: InputPayload[PayloadKeys]) {
    const _value = makeFieldError(value ?? [])

    if (this._has(field)) {
      const currentValues = this._payload[field]!

      const { reasons = [], metadata } = _value

      reasons.forEach((reason) => {
        if (!currentValues.reasons.includes(reason))
          currentValues.reasons.push(reason)
      })

      if (metadata && !isEqual(currentValues.metadata, metadata))
        currentValues.metadata = {
          ...(currentValues?.metadata ?? {}),
          ...metadata
        }

      this._payload[field] = currentValues
    } else this._payload[field] = _value

    return this
  }

  setMessage = (message: ValidationErrorMessage) => {
    this.message = message

    return this
  }

  throw = () => {
    throw new ValidationError(this.data)
  }
}

function isFieldError(data: any): data is FieldError {
  return isPropertyOf('reasons', data) && isPropertyOf('metadata', data)
}

function makeFieldError(value: InputPayload[PayloadKey]): FieldError {
  return isFieldError(value)
    ? value
    : { reasons: toArray(value), metadata: null }
}
