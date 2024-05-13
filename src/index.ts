import { Schema } from './schema';

export default Schema;

export { Schema } from './schema';
export * from './validators';

export { ERRORS, SCHEMA_ERRORS, VALIDATION_ERRORS } from './schema/utils';

export {
  type ObjectType,
  type FieldKey,
  getKeysAsProps,
  isEqual,
  isFunctionLike,
  isNullOrUndefined,
  isOneOf,
  isPropertyOf,
  isRecordLike,
  toArray,
} from './utils';
export type {
  // ctx
  DeletionContext,
  ImmutableContext,
  MutableContext,
  // summary
  ImmutableSummary,
  MutableSummary,
  KeyOf,
  Merge,
  ArrayOfMinSizeOne,
  ArrayOfMinSizeTwo,
  RealType,
  ValidatorResponse,
  ValidatorResponseObject,
  XOR,
} from './schema/types';
export type {
  FieldError,
  InputFieldError,
  IErrorTool,
  IValidationError,
  InputPayload,
  ValidationErrorMessage,
} from './schema/utils';
export {
  isFieldError,
  isInputFieldError,
  makeFieldError,
} from './schema/utils';
