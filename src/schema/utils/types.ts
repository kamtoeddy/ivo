import { PayloadKey } from '../../utils/types'

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
export type FieldError = { errors: string[]; metadata: Record<PayloadKey, any> }
export type ErrorPayload = Record<PayloadKey, FieldError>
export type InputPayload = Record<PayloadKey, string | string[] | FieldError>

export type ValidationErrorProps = {
  message: ValidationErrorMessage
  payload?: ErrorPayload
}

export type ValidationErrorToolProps = {
  message: ValidationErrorMessage
  payload?: InputPayload
}
