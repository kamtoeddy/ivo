import { isPropertyOf, FieldKey, sortKeys, toArray } from '../../utils'
import { SCHEMA_ERRORS } from './types'

export { SchemaErrorTool, SchemaError }

class SchemaError extends Error {
  constructor(public payload: ErrorPayload) {
    super(SCHEMA_ERRORS.INVALID_SCHEMA)
  }
}

class SchemaErrorTool {
  private _payload: ErrorPayload = {}

  constructor(payload: ErrorPayload = {}) {
    this._setPayload(payload)
  }

  get isPayloadLoaded() {
    return Object.keys(this._payload).length > 0
  }

  get data() {
    return {
      message: SCHEMA_ERRORS.INVALID_SCHEMA,
      payload: sortKeys(this._payload)
    }
  }

  private _has = (field: FieldKey) => isPropertyOf(field, this._payload)

  private _setPayload(payload: InputPayload) {
    Object.entries(payload).forEach(([key, value]) => {
      this.add(key, value)
    })
  }

  add(field: FieldKey, value?: string | string[]) {
    if (!value) value = []
    else value = toArray(value)

    if (this._has(field)) {
      const currentValues = this._payload[field]

      value.forEach((v) => {
        if (!currentValues.includes(v)) currentValues.push(v)
      })

      this._payload[field] = currentValues
    } else this._payload[field] = value

    return this
  }

  throw() {
    throw new SchemaError(this._payload)
  }
}

type ErrorPayload = Record<FieldKey, string[]>
type InputPayload = Record<FieldKey, string | string[]>
