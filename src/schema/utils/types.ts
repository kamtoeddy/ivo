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

export type IValidationErrorData<ExtraData = {}> = ({
  message: ValidationErrorMessage
} & ExtraData) & {}

export interface IValidationError<ExtraData = {}> {
  message: ValidationErrorMessage
  get data(): IValidationErrorData<ExtraData>
  /** array of fields that have failed validation */
  get fields(): string[]
  /** determines if validation has failed */
  get isLoaded(): boolean
  add(field: PayloadKey, value?: InputPayload[PayloadKey]): this
  /** method to set the value of the validation error message */
  setMessage(message: ValidationErrorMessage): this
  /** throws a custom error when validation fails & Schema.option.errors == 'throws' */
  throw(): never
}
