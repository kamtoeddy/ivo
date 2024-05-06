import {
  isPropertyOf,
  FieldKey,
  toArray,
  isRecordLike,
  isEqual,
} from '../../utils';
import {
  FieldError,
  FullInputFieldError,
  InputFieldError,
  InputPayload,
} from './types';

export * from './types';
export * from './error-tool';
export * from './schema-error';
export * from './timestamp-tool';

export { isFieldError, isInputFieldError, makeFieldError };

function isFieldError(data: any): data is FieldError {
  if (!isRecordLike(data) || isEqual({}, data)) return false;

  if (
    !isPropertyOf('metadata', data) ||
    !isFieldErrorMetadata(data?.metadata) ||
    !isPropertyOf('reasons', data)
  )
    return false;

  return typeof data?.reasons != 'string' && !Array.isArray(data?.reasons);
}

function isInputFieldError(data: any): data is Partial<FullInputFieldError> {
  if (!isRecordLike(data) || isEqual({}, data)) return false;

  if (isPropertyOf('reasons', data)) return false;

  const hasMetadata = isPropertyOf('metadata', data),
    hasReason = isPropertyOf('reason', data);

  if (!hasMetadata && !hasReason) return false;

  if (hasMetadata && !isFieldErrorMetadata(data?.metadata)) return false;

  if (
    hasReason &&
    typeof data?.reason != 'string' &&
    !Array.isArray(data?.reason)
  )
    return false;

  return true;
}

function isFieldErrorMetadata(data: any): data is FieldError['metadata'] {
  return data?.metadata == null || isRecordLike(data?.metadata);
}

function makeFieldError(
  value: InputPayload[FieldKey] | InputFieldError,
  fallbackMessage = 'validation failed',
): FieldError {
  if (isFieldError(value)) return value;

  if (!isRecordLike(value)) return { reasons: toArray(value), metadata: null };

  if (isInputFieldError(value))
    return {
      reasons: toArray(value?.reason || fallbackMessage),
      metadata: value?.metadata ?? null,
    };

  return {
    reasons: toArray((value as any)?.reasons ?? [fallbackMessage]),
    metadata: (value as any)?.metadata ?? null,
  };
}
