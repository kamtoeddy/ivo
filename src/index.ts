import { Schema } from './schema';

export default Schema;

export { Schema } from './schema';
export * as validate from './validate';
export * from './validate';

export { ERRORS, SCHEMA_ERRORS, VALIDATION_ERRORS } from './schema/utils';

export {
  ObjectType,
  FieldKey,
  getKeysAsProps,
  isEqual,
  isFunctionLike,
  isNullOrUndefined,
  isPropertyOf,
  isRecordLike,
  toArray
} from './utils';
export {
  Context,
  DeleteContext,
  OPERATIONS,
  KeyOf,
  Merge,
  ArrayOfMinSizeOne as NonEmptyArray,
  RealType,
  Summary,
  ValidatorResponse,
  ValidatorResponseObject,
  XOR
} from './schema/types';
export {
  FieldError,
  IErrorTool,
  IValidationError,
  InputPayload,
  ValidationErrorMessage
} from './schema/utils';
