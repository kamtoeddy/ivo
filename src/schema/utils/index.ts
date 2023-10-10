import { isPropertyOf, FieldKey, toArray } from '../../utils';
import { FieldError, InputPayload } from './types';

export * from './types';
export * from './error-tool';
export * from './schema-error';
export * from './timestamp-tool';

export { makeFieldError };

function isFieldError(data: any): data is FieldError {
  return isPropertyOf('reasons', data) && isPropertyOf('metadata', data);
}

function makeFieldError(value: InputPayload[FieldKey]): FieldError {
  return isFieldError(value)
    ? value
    : { reasons: toArray(value), metadata: null };
}
