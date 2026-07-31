import type { ObjectType } from "../../utils";
import {
  type ArrayOfMinSizeOne,
  type ArrayOfMinSizeTwo,
  BUILD,
  Buildable,
  type NotAllowedError,
  type NS,
  type ReValidator,
  type Validator,
} from "../types";

export { type BlankRequiredBuilder, RequiredBuilder };

interface BlankRequiredBuilder<
  Value extends Output[keyof Output],
  Input,
  Output,
  CtxOptions extends ObjectType,
  Metadata,
> {
  allow<const V extends Value>(
    values: ArrayOfMinSizeTwo<V>,
  ): BuildableRequiredConfig<V, Input, Output, CtxOptions, Metadata, "allow">;
  validate(
    validator: Validator<Value, Input, Output, CtxOptions, Metadata>,
  ): BuildableRequiredConfig<
    Value,
    Input,
    Output,
    CtxOptions,
    Metadata,
    "validate"
  >;
}

/**
 * Unlike lax fields, a required field's validation is mandatory - either
 * `.allow()` or `.validate()` must be called (mutually exclusive, same as
 * lax) before `[BUILD]` is offered. `.readonly()` and `.ignoreUpdate()`
 * share a single flag - like Rust, calling either consumes both, since a
 * required property is either readonly or conditionally updatable, never
 * both.
 */
type BuildableRequiredConfig<
  Value extends Output[keyof Output],
  Input,
  Output,
  CtxOptions extends ObjectType,
  Metadata,
  ValidationState extends "allow" | "none" | "validate" = "none",
  HasAllowError extends boolean = false,
  HasReValidate extends boolean = false,
  HasReadonlyOrIgnoreUpdate extends boolean = false,
  HasOnDelete extends boolean = false,
  HasOnFailure extends boolean = false,
  HasOnSuccess extends boolean = false,
> = (ValidationState extends "none"
  ? {}
  : Buildable<NS.RequiredField<Value, Input, Output, CtxOptions, Metadata>>) &
  (ValidationState extends "allow"
    ? HasAllowError extends true
      ? {}
      : {
          allowError(
            error:
              | NotAllowedError<Metadata>
              | ((
                  value: unknown,
                  allowedValues: ArrayOfMinSizeOne<Value>,
                ) => NotAllowedError<Metadata>),
          ): BuildableRequiredConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            "allow",
            true,
            HasReValidate,
            HasReadonlyOrIgnoreUpdate,
            HasOnDelete,
            HasOnFailure,
            HasOnSuccess
          >;
        }
    : {}) &
  (ValidationState extends "none"
    ? {}
    : HasReValidate extends true
      ? {}
      : {
          reValidate(
            validator: ReValidator<Value, Input, Output, CtxOptions, Metadata>,
          ): BuildableRequiredConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            ValidationState,
            HasAllowError,
            true,
            HasReadonlyOrIgnoreUpdate,
            HasOnDelete,
            HasOnFailure,
            HasOnSuccess
          >;
        }) &
  (HasReadonlyOrIgnoreUpdate extends true
    ? {}
    : {
        readonly(): BuildableRequiredConfig<
          Value,
          Input,
          Output,
          CtxOptions,
          Metadata,
          ValidationState,
          HasAllowError,
          HasReValidate,
          true,
          HasOnDelete,
          HasOnFailure,
          HasOnSuccess
        >;
        ignoreUpdate(
          resolver?: NS.IgnoreUpdateResolver<Input, Output, CtxOptions>,
        ): BuildableRequiredConfig<
          Value,
          Input,
          Output,
          CtxOptions,
          Metadata,
          ValidationState,
          HasAllowError,
          HasReValidate,
          true,
          HasOnDelete,
          HasOnFailure,
          HasOnSuccess
        >;
      }) &
  (HasOnDelete extends true
    ? {}
    : {
        onDelete(
          handler:
            | NS.DeleteHandler<Output, CtxOptions>
            | ArrayOfMinSizeOne<NS.DeleteHandler<Output, CtxOptions>>,
        ): BuildableRequiredConfig<
          Value,
          Input,
          Output,
          CtxOptions,
          Metadata,
          ValidationState,
          HasAllowError,
          HasReValidate,
          HasReadonlyOrIgnoreUpdate,
          true,
          HasOnFailure,
          HasOnSuccess
        >;
      }) &
  (HasOnFailure extends true
    ? {}
    : {
        onFailure(
          handler:
            | NS.FailureHandler<Input, Output, CtxOptions>
            | ArrayOfMinSizeOne<NS.FailureHandler<Input, Output, CtxOptions>>,
        ): BuildableRequiredConfig<
          Value,
          Input,
          Output,
          CtxOptions,
          Metadata,
          ValidationState,
          HasAllowError,
          HasReValidate,
          HasReadonlyOrIgnoreUpdate,
          HasOnDelete,
          true,
          HasOnSuccess
        >;
      }) &
  (HasOnSuccess extends true
    ? {}
    : {
        onSuccess(
          handler:
            | NS.SuccessHandler<Input, Output, CtxOptions>
            | ArrayOfMinSizeOne<NS.SuccessHandler<Input, Output, CtxOptions>>,
        ): BuildableRequiredConfig<
          Value,
          Input,
          Output,
          CtxOptions,
          Metadata,
          ValidationState,
          HasAllowError,
          HasReValidate,
          HasReadonlyOrIgnoreUpdate,
          HasOnDelete,
          HasOnFailure,
          true
        >;
      });

class RequiredBuilder<
  Value extends Output[keyof Output],
  Input,
  Output,
  CtxOptions extends ObjectType,
  Metadata,
>
  implements
    BlankRequiredBuilder<Value, Input, Output, CtxOptions, Metadata>,
    BuildableRequiredConfig<Value, Input, Output, CtxOptions, Metadata, "allow">
{
  name: string;
  private config: Partial<
    NS.RequiredField<Value, Input, Output, CtxOptions, Metadata>
  > = { type: "required" };

  constructor(name: string) {
    this.name = name;
    this.config.name = name;
  }

  allow(values: ArrayOfMinSizeTwo<Value>) {
    this.config.allow = values;
    return this as never;
  }

  allowError(
    error:
      | NotAllowedError<Metadata>
      | ((
          value: unknown,
          allowedValues: ArrayOfMinSizeOne<Value>,
        ) => NotAllowedError<Metadata>),
  ) {
    const values = extractAllowedValues(this.config.allow);

    this.config.allow = { values, error } as never;
    return this as never;
  }

  validate(validator: Validator<Value, Input, Output, CtxOptions, Metadata>) {
    this.config.validator = validator;
    return this as never;
  }

  reValidate(
    validator: ReValidator<Value, Input, Output, CtxOptions, Metadata>,
  ) {
    this.config.reValidator = validator;
    return this as never;
  }

  readonly() {
    this.config.readonly = true;
    return this as never;
  }

  ignoreUpdate(resolver?: NS.IgnoreUpdateResolver<Input, Output, CtxOptions>) {
    this.config.ignoreUpdate = resolver ?? true;
    return this as never;
  }

  onDelete(
    handler:
      | NS.DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<NS.DeleteHandler<Output, CtxOptions>>,
  ) {
    this.config.onDelete = handler;
    return this as never;
  }

  onFailure(
    handler:
      | NS.FailureHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<NS.FailureHandler<Input, Output, CtxOptions>>,
  ) {
    this.config.onFailure = handler;
    return this as never;
  }

  onSuccess(
    handler:
      | NS.SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<NS.SuccessHandler<Input, Output, CtxOptions>>,
  ) {
    this.config.onSuccess = handler;
    return this as never;
  }

  [BUILD]() {
    return this.config as NS.RequiredField<
      Value,
      Input,
      Output,
      CtxOptions,
      Metadata
    >;
  }
}

/**
 * `Array.isArray` doesn't narrow a union containing a readonly tuple (like
 * `ArrayOfMinSizeTwo`'s `readonly [T, T, ...T[]]` branch) cleanly, so this
 * narrows from `unknown` instead of the generic union directly.
 */
function extractAllowedValues(allow: unknown) {
  if (allow == null) return undefined;

  return Array.isArray(allow) ? allow : (allow as { values: unknown }).values;
}
