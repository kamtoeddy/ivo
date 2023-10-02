import { PayloadKey } from '../../utils'

export const SCHEMA_ERRORS = {
  INVALID_SCHEMA: 'INVALID_SCHEMA'
} as const

export const VALIDATION_ERRORS = {
  INVALID_DATA: 'INVALID_DATA',
  NOTHING_TO_UPDATE: 'NOTHING_TO_UPDATE',
  VALIDATION_ERROR: 'VALIDATION_ERROR'
} as const

export const ERRORS = { ...SCHEMA_ERRORS, ...VALIDATION_ERRORS } as const

export type SchemaErrorMessage = keyof typeof SCHEMA_ERRORS
export type SchemaErrorPayload = Record<PayloadKey, string[]>
export type SchemaErrorInputPayload = Record<PayloadKey, string | string[]>

export type SchemaErrorProps = {
  payload?: SchemaErrorPayload
}

export type SchemaErrorToolProps = {
  payload?: SchemaErrorInputPayload
}

export type ValidationErrorMessage = keyof typeof VALIDATION_ERRORS
export type FieldError = {
  reasons: string[]
  metadata: Record<PayloadKey, any> | null
}
export type ErrorPayload<Keys extends PayloadKey = PayloadKey> = {
  [K in Keys]?: FieldError
}

export type InputPayload = Record<PayloadKey, string | string[] | FieldError>

export type ValidationErrorProps<Keys extends PayloadKey = PayloadKey> = {
  message: ValidationErrorMessage
  payload?: ErrorPayload<Keys>
}

export type ValidationErrorToolProps = {
  message: ValidationErrorMessage
  payload?: InputPayload
}

export interface IValidationError<OutputKeys extends PayloadKey = PayloadKey>
  extends Error {
  message: ValidationErrorMessage
  get isPayloadLoaded(): boolean
  get summary(): Record<PayloadKey, any>
  get payload(): {
    [K in OutputKeys]?: any
  }
  setMessage(message: ValidationErrorMessage): this
  throw(): never
}
