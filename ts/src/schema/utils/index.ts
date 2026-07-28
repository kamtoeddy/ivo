import { isEqual, isPropertyOf, isRecordLike } from '../../utils';
import type { FieldError, InputFieldError, InputPayload } from './types';

export * from './constants';
export * from './error-tool';
export * from './schema-error';
export * from './timestamp-tool';
export * from './types';

export { cloneValue, isFieldError, isInputFieldError, makeFieldError };

function cloneValue<T>(value: T): T {
  try {
    return structuredClone(value);
  } catch {
    return JSON.parse(JSON.stringify(value));
  }
}

function isFieldError(data: unknown): data is FieldError {
  if (!isRecordLike(data) || isEqual({}, data)) return false;

  if (
    !isPropertyOf('metadata', data) ||
    !isFieldErrorMetadataOk(data) ||
    !isPropertyOf('reason', data)
  )
    return false;

  return typeof data?.reason === 'string';
}

function isInputFieldError(data: unknown): data is Partial<FieldError> {
  if (isFieldError(data)) return true;

  if (!isRecordLike(data) || isEqual({}, data)) return false;

  const hasMetadata = isPropertyOf('metadata', data),
    hasReason = isPropertyOf('reason', data);

  if (!hasMetadata && !hasReason) return false;

  if (hasMetadata && !isFieldErrorMetadataOk(data?.metadata)) return false;
  if (hasReason && typeof data?.reason !== 'string') return false;

  return true;
}

function isFieldErrorMetadataOk(data: unknown): data is FieldError['metadata'] {
  const metadata = (data as FieldError)?.metadata;

  return metadata == null || isRecordLike(metadata);
}

function makeFieldError<Metadata>(
  value: InputPayload[string] | InputFieldError<Metadata>,
  fallbackMessage = 'validation failed',
): FieldError<Metadata> {
  if (isFieldError(value)) {
    if (!value.reason) value.reason = fallbackMessage;

    return value as never;
  }

  if (typeof value === 'string') return { reason: value, metadata: null };

  return {
    reason: (value as any).reason ?? fallbackMessage,
    metadata: (value as any)?.metadata ?? null,
  };
}
