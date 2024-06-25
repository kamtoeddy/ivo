import { ObjectType, FieldKey } from '../../utils';

export { ERRORS, SCHEMA_ERRORS, VALIDATION_ERRORS };

export type {
  ErrorPayload,
  FieldError,
  FullInputFieldError,
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
  reasons: string[];
  metadata: Record<FieldKey, any> | null;
};

type FullInputFieldError = {
  reason: FieldError['reasons'][number] | FieldError['reasons'];
  metadata: FieldError['metadata'];
};

type InputFieldError =
  | FullInputFieldError
  | { reason: FullInputFieldError['reason'] }
  | { metadata: FullInputFieldError['metadata'] };

type ErrorPayload<Keys extends FieldKey = FieldKey> = {
  [K in Keys]?: FieldError;
};

type InputPayload = Record<FieldKey, string | string[] | FieldError>;

type IValidationError<ExtraData extends ObjectType = {}> = ({
  message: ValidationErrorMessage;
} & ExtraData) & {};

interface IErrorTool<ExtraData extends ObjectType = {}> {
  /** return what your validation error should look like from this method */
  get data(): IValidationError<ExtraData>;

  /** array of fields that have failed validation */
  get fields(): string[];

  /** determines if validation has failed */
  get isLoaded(): boolean;

  /** used to append a field to your final validation error */
  add(field: FieldKey, error: FieldError, value?: any): this;

  /** method to set the value of the validation error message */
  setMessage(message: ValidationErrorMessage): this;
}
