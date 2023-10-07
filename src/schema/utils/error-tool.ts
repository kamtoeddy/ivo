import { isEqual, isPropertyOf, FieldKey, sortKeys, toArray } from '../../utils'
import {
  ErrorPayload,
  FieldError,
  InputPayload,
  ValidationErrorMessage,
  IErrorTool
} from './types'

export { DefaultErrorTool }

class DefaultErrorTool<PayloadKeys extends FieldKey = FieldKey>
  implements IErrorTool<{ payload: ErrorPayload<PayloadKeys> }>
{
  private _payload: ErrorPayload<PayloadKeys> = {}

  constructor(public message: ValidationErrorMessage) {}

  get error() {
    return {
      message: this.message,
      payload: sortKeys(this._payload)
    }
  }

  get fields() {
    return Object.keys(this._payload)
  }

  get isLoaded() {
    return Object.keys(this._payload).length > 0
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

  setMessage(message: ValidationErrorMessage) {
    this.message = message

    return this
  }

  throw = () => {
    throw new ValidationError(this.error)
  }
}

class ValidationError<OutputKeys extends FieldKey> extends Error {
  payload: ErrorPayload<OutputKeys> = {}

  constructor({
    message,
    payload = {}
  }: {
    message: ValidationErrorMessage
    payload?: ErrorPayload<OutputKeys>
  }) {
    super(message)
    this.payload = payload
  }
}

function isFieldError(data: any): data is FieldError {
  return isPropertyOf('reasons', data) && isPropertyOf('metadata', data)
}

function makeFieldError(value: InputPayload[FieldKey]): FieldError {
  return isFieldError(value)
    ? value
    : { reasons: toArray(value), metadata: null }
}
