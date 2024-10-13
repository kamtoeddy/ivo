import {
  isPropertyOf,
  FieldKey,
  toArray,
  isRecordLike,
  isEqual,
} from "../../utils";
import {
  FieldError,
  FullInputFieldError,
  InputFieldError,
  InputPayload,
} from "./types";

export * from "./types";
export * from "./error-tool";
export * from "./schema-error";
export * from "./timestamp-tool";

export { isFieldError, isInputFieldError, makeFieldError };

function isFieldError(data: unknown): data is FieldError {
  if (!isRecordLike(data) || isEqual({}, data)) return false;

  if (
    !isPropertyOf("metadata", data) ||
    !isFieldErrorMetadataOk(data) ||
    !isPropertyOf("reasons", data)
  )
    return false;

  return Array.isArray(data?.reasons);
}

function isInputFieldError(
  data: unknown,
): data is Partial<FullInputFieldError> {
  if (isFieldError(data)) return true;

  if (!isRecordLike(data) || isEqual({}, data)) return false;

  if (isPropertyOf("reasons", data)) return false;

  const hasMetadata = isPropertyOf("metadata", data),
    hasReason = isPropertyOf("reason", data);

  if (!hasMetadata && !hasReason) return false;

  if (hasMetadata && !isFieldErrorMetadataOk(data?.metadata)) return false;

  if (
    hasReason &&
    typeof data?.reason != "string" &&
    !Array.isArray(data?.reason)
  )
    return false;

  return true;
}

function isFieldErrorMetadataOk(data: unknown): data is FieldError["metadata"] {
  const metadata = (data as FieldError)?.metadata;

  return metadata == null || isRecordLike(metadata);
}

function makeFieldError(
  value: InputPayload[FieldKey] | InputFieldError,
  fallbackMessage = "validation failed",
): FieldError {
  if (isFieldError(value)) return value;

  if (!isRecordLike(value)) return { reasons: toArray(value), metadata: null };

  if (isInputFieldError(value))
    return {
      reasons: toArray(value?.reason || fallbackMessage),
      metadata: value?.metadata ?? null,
    };

  return {
    reasons: toArray(
      (value as unknown as { reasons: string[] })?.reasons ?? [fallbackMessage],
    ),
    metadata: (value as Partial<FullInputFieldError>)?.metadata ?? null,
  };
}
