export type {
  DefaultFieldErrorMetadata,
  FieldError,
  InputFieldError,
  InputPayload,
};
export { INVALID_SCHEMA_ERROR };

const INVALID_SCHEMA_ERROR = "INVALID_SCHEMA" as const;

type DefaultFieldErrorMetadata = Record<string, unknown>;

type FieldError<Metadata = DefaultFieldErrorMetadata> = {
  reason: string;
  metadata: Metadata | null;
};

type InputFieldError<Metadata> =
  | FieldError<Metadata>
  | { reason: FieldError["reason"] }
  | { metadata: FieldError<Metadata>["metadata"] };

type InputPayload = Record<string, string | FieldError>;
