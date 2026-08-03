import type { ObjectType } from "../../utils";
import {
  type ArrayOfMinSizeOne,
  type ArrayOfMinSizeTwo,
  FIELD_CONFIG_BUILD_METHOD_NAME,
  type Buildable,
  type NotAllowedError,
  type NS,
  type ReValidator,
  type Validator,
  InitResolverCtx,
} from "../types";
import { extractAllowedValues } from "./_utils";

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
  requiredError(
    error: string | ((ctx: InitResolverCtx<Input, CtxOptions>) => string),
  ): HasRequiredError<Value, Input, Output, CtxOptions, Metadata>;
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

interface HasRequiredError<
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

type BuildableRequiredConfig<
  Value extends Output[keyof Output],
  Input,
  Output,
  CtxOptions extends ObjectType,
  Metadata,
  ValidationState extends "allow" | "none" | "validate" = "none",
  HasAllowError extends boolean = false,
  HasReValidate extends boolean = false,
  HasReadonly extends boolean = false,
  HasIgnoreUpdate extends "yes" | "yes-computed" | "no" = "no",
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
            HasReadonly,
            HasIgnoreUpdate,
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
            HasReadonly,
            HasIgnoreUpdate,
            HasOnDelete,
            HasOnFailure,
            HasOnSuccess
          >;
        }) &
  (HasReadonly extends false
    ? HasIgnoreUpdate extends "no"
      ? {
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
            HasIgnoreUpdate,
            HasOnDelete,
            HasOnFailure,
            HasOnSuccess
          >;
          ignoreUpdate(): BuildableRequiredConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasReadonly,
            "yes",
            HasOnDelete,
            HasOnFailure,
            HasOnSuccess
          >;
          ignoreUpdate(
            resolver: NS.IgnoreUpdateResolver<Input, Output, CtxOptions>,
          ): BuildableRequiredConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasReadonly,
            "yes-computed",
            HasOnDelete,
            HasOnFailure,
            HasOnSuccess
          >;
        }
      : HasIgnoreUpdate extends "yes-computed"
        ? {
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
              HasIgnoreUpdate,
              HasOnDelete,
              HasOnFailure,
              HasOnSuccess
            >;
          }
        : {}
    : HasIgnoreUpdate extends "no"
      ? {
          ignoreUpdate(
            resolver: NS.IgnoreUpdateResolver<Input, Output, CtxOptions>,
          ): BuildableRequiredConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasReadonly,
            "yes-computed",
            HasOnDelete,
            HasOnFailure,
            HasOnSuccess
          >;
        }
      : {}) &
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
          HasReadonly,
          HasIgnoreUpdate,
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
          HasReadonly,
          HasIgnoreUpdate,
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
          HasReadonly,
          HasIgnoreUpdate,
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
  private config: Partial<
    NS.RequiredField<Value, Input, Output, CtxOptions, Metadata>
  > = { type: "required" };

  constructor(name: string) {
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

  requiredError(
    error: string | ((ctx: InitResolverCtx<Input, CtxOptions>) => string),
  ) {
    this.config.requiredError = error;
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

  [FIELD_CONFIG_BUILD_METHOD_NAME]() {
    return this.config as NS.RequiredField<
      Value,
      Input,
      Output,
      CtxOptions,
      Metadata
    >;
  }
}
