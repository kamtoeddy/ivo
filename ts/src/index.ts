export { type IvoResultInfo, Schema } from "./schema";

export const ERRORS = {
  INVALID_SCHEMA: "INVALID_SCHEMA",
  NOTHING_TO_UPDATE: "Nothing to update",
  VALIDATION_ERROR: "Validation Error",
  NOT_ALLOWED: "value not allowed",
} as const;

export type {
  ArrayOfMinSizeOne,
  ArrayOfMinSizeTwo,
  IvoContext,
  KeyOf,
  ReadonlyIvoContext,
  RealType,
  SetterFnData,
  ValidatorResponse,
  ValidatorResponseObject,
  XOR,
} from "./schema/types";
export type { FieldError, InputFieldError, InputPayload } from "./schema/utils";
export {
  isFieldError,
  isInputFieldError,
  makeFieldError,
} from "./schema/utils";
export {
  getKeysAsProps,
  isEqual,
  isFunctionLike,
  isNullOrUndefined,
  isOneOf,
  isPropertyOf,
  isRecordLike,
  type ObjectType,
  toArray,
} from "./utils";
export * from "./validators";
