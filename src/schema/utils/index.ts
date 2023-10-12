import { isPropertyOf, FieldKey, toArray, isObject } from '../../utils';
import { FieldError, InputPayload } from './types';

export * from './types';
export * from './error-tool';
export * from './schema-error';
export * from './timestamp-tool';

export { makeFieldError };

function isFieldError(data: any): data is FieldError {
  return (
    isObject(data) &&
    isPropertyOf('reasons', data) &&
    isPropertyOf('metadata', data)
  );
}

function makeFieldError(value: InputPayload[FieldKey]): FieldError {
  if (isFieldError(value)) return value;

  return isObject(value)
    ? {
        reasons: toArray((value as any)?.reasons ?? ['validation failed']),
        metadata: (value as any)?.metadata ?? null
      }
    : { reasons: toArray(value), metadata: null };
}
