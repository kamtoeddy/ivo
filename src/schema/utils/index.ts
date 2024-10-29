import { isPropertyOf, FieldKey, isRecordLike, isEqual } from "../../utils";
import { FieldError, InputFieldError, InputPayload } from "./types";

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
    !isPropertyOf("reason", data)
  )
    return false;

  return typeof data?.reason == "string";
}

function isInputFieldError(data: unknown): data is Partial<FieldError> {
  if (isFieldError(data)) return true;

  if (!isRecordLike(data) || isEqual({}, data)) return false;

  const hasMetadata = isPropertyOf("metadata", data),
    hasReason = isPropertyOf("reason", data);

  if (!hasMetadata && !hasReason) return false;

  if (hasMetadata && !isFieldErrorMetadataOk(data?.metadata)) return false;
  if (hasReason && typeof data?.reason != "string") return false;

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
  if (isFieldError(value)) {
    if (!value.reason) value.reason = fallbackMessage;

    return value;
  }

  if (typeof value === "string") return { reason: value, metadata: null };

  return {
    reason: (value as any).reason ?? fallbackMessage,
    metadata: (value as any)?.metadata ?? null,
  };
}
