import type { FieldKey, ObjectType } from '../../utils';

export { ERRORS, SCHEMA_ERRORS, VALIDATION_ERRORS };

export type {
  ErrorPayload,
  FieldError,
  IErrorTool,
  InputFieldError,
  InputPayload,
  IValidationError,
  ValidationErrorMessage,
};

const SCHEMA_ERRORS = { INVALID_SCHEMA: 'INVALID_SCHEMA' } as const;

const VALIDATION_ERRORS = {
  NOTHING_TO_UPDATE: 'NOTHING_TO_UPDATE',
  VALIDATION_ERROR: 'VALIDATION_ERROR',
} as const;

const ERRORS = { ...SCHEMA_ERRORS, ...VALIDATION_ERRORS } as const;

type ValidationErrorMessage = keyof typeof VALIDATION_ERRORS;

type FieldError = {
  reason: string;
  metadata: Record<FieldKey, unknown> | null;
};

type InputFieldError =
  | FieldError
  | { reason: FieldError['reason'] }
  | { metadata: FieldError['metadata'] };

type ErrorPayload<Keys extends FieldKey = FieldKey> = {
  [K in Keys]?: FieldError;
};

type InputPayload = Record<FieldKey, string | FieldError>;

type IValidationError<ExtraData extends ObjectType = never> = ({
  message: ValidationErrorMessage;
} & ExtraData) & {};

interface IErrorTool<ExtraData extends ObjectType = never> {
  /** return what your validation error should look like from this method */
  get data(): IValidationError<ExtraData>;

  /** array of fields that have failed validation */
  get fields(): string[];

  /** determines if validation has failed */
  get isLoaded(): boolean;

  /**
  - Appends a field to your final validation error
  - if validation payload already has provided field, only the metadata (if available) will be updated
  */
  set(field: FieldKey, error: FieldError, value?: unknown): this;

  /** method to set the value of the validation error message */
  setMessage(message: ValidationErrorMessage): this;
}
