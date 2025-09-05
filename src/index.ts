import { Schema } from './schema';

export default Schema;

export { Schema } from './schema';
export type {
  ArrayOfMinSizeOne,
  ArrayOfMinSizeTwo,
  // ctx
  DeletionContext,
  ImmutableContext,
  // summary
  ImmutableSummary,
  KeyOf,
  Merge,
  MutableContext,
  MutableSummary,
  RealType,
  ValidatorResponse,
  ValidatorResponseObject,
  XOR,
} from './schema/types';
export type {
  FieldError,
  IErrorTool,
  InputFieldError,
  InputPayload,
  IValidationError,
  ValidationErrorMessage,
} from './schema/utils';
export {
  ERRORS,
  isFieldError,
  isInputFieldError,
  makeFieldError,
  SCHEMA_ERRORS,
  VALIDATION_ERRORS,
} from './schema/utils';
export {
  type FieldKey,
  getKeysAsProps,
  isEqual,
  isFunctionLike,
  isNullOrUndefined,
  isOneOf,
  isPropertyOf,
  isRecordLike,
  type ObjectType,
  toArray,
} from './utils';
export * from './validators';
