import {
  type ArrayOfMinSizeOne,
  type ArrayOfMinSizeTwo,
  FIELD_CONFIG_BUILD_METHOD_NAME,
  type Buildable,
  type NotAllowedError,
  type NS,
  type RequiredHandler,
  type ReValidator,
  type Validator,
  type ObjectType,
} from "../../utils/types";
import { extractAllowedValues } from "./_utils";

export { type BuildableLaxConfig, LaxBuilder };

type BuildableLaxConfig<
  Value extends Output[keyof Output],
  Input,
  Output,
  CtxOptions extends ObjectType,
  Metadata,
  DefaultState extends "value" | "resolver" = "resolver",
  ValidationState extends "allow" | "none" | "validate" = "none",
  HasAllowError extends boolean = false,
  HasReValidate extends boolean = false,
  HasRequired extends boolean = false,
  HasIgnore extends "init" | "update" | "ignore" | "none" = "none",
  HasReadonly extends boolean = false,
  HasOnDelete extends boolean = false,
  HasOnFailure extends boolean = false,
  HasOnSuccess extends boolean = false,
> = Buildable<NS.LaxField<Value, Input, Output, CtxOptions, Metadata>> &
  (ValidationState extends "none"
    ? {
        allow<const V extends Value>(
          values: ArrayOfMinSizeTwo<V>,
        ): BuildableLaxConfig<
          V,
          Input,
          Output,
          CtxOptions,
          Metadata,
          DefaultState,
          "allow",
          HasAllowError,
          HasReValidate,
          HasRequired,
          HasIgnore,
          HasReadonly,
          HasOnDelete,
          HasOnFailure,
          HasOnSuccess
        >;
        validate(
          validator: Validator<Value, Input, Output, CtxOptions, Metadata>,
        ): BuildableLaxConfig<
          Value,
          Input,
          Output,
          CtxOptions,
          Metadata,
          DefaultState,
          "validate",
          HasAllowError,
          HasReValidate,
          HasRequired,
          HasIgnore,
          HasReadonly,
          HasOnDelete,
          HasOnFailure,
          HasOnSuccess
        >;
      }
    : {}) &
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
          ): BuildableLaxConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            DefaultState,
            "allow",
            true,
            HasReValidate,
            HasRequired,
            HasIgnore,
            HasReadonly,
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
          ): BuildableLaxConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            DefaultState,
            ValidationState,
            HasAllowError,
            true,
            HasRequired,
            HasIgnore,
            HasReadonly,
            HasOnDelete,
            HasOnFailure,
            HasOnSuccess
          >;
        }) &
  (HasRequired extends true
    ? {}
    : {
        required(
          handler: RequiredHandler<Input, Output, CtxOptions, Metadata>,
        ): BuildableLaxConfig<
          Value,
          Input,
          Output,
          CtxOptions,
          Metadata,
          DefaultState,
          ValidationState,
          HasAllowError,
          HasReValidate,
          true,
          HasIgnore,
          HasReadonly,
          HasOnDelete,
          HasOnFailure,
          HasOnSuccess
        >;
      }) &
  (HasIgnore extends "none"
    ? HasReadonly extends true
      ? {}
      : {
          ignore(
            resolver: NS.Resolver<boolean, Input, Output, CtxOptions>,
          ): BuildableLaxConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            DefaultState,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            "ignore",
            HasReadonly,
            HasOnDelete,
            HasOnFailure,
            HasOnSuccess
          >;
          ignoreInit(): BuildableLaxConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            DefaultState,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            "init",
            HasReadonly,
            HasOnDelete,
            HasOnFailure,
            HasOnSuccess
          >;
          ignoreUpdate(): BuildableLaxConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            DefaultState,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            "update",
            HasReadonly,
            HasOnDelete,
            HasOnFailure,
            HasOnSuccess
          >;
          readonly(): BuildableLaxConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            DefaultState,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            HasIgnore,
            true,
            HasOnDelete,
            HasOnFailure,
            HasOnSuccess
          >;
        }
    : {}) &
  (DefaultState extends "resolver"
    ? {}
    : HasReadonly extends true
      ? {
          ignore(
            resolver: NS.Resolver<boolean, Input, Output, CtxOptions>,
          ): BuildableLaxConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            DefaultState,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            "ignore",
            HasReadonly,
            HasOnDelete,
            HasOnFailure,
            HasOnSuccess
          >;
        }
      : HasIgnore extends "init" | "update"
        ? {}
        : {
            readonly(): BuildableLaxConfig<
              Value,
              Input,
              Output,
              CtxOptions,
              Metadata,
              DefaultState,
              ValidationState,
              HasAllowError,
              HasReValidate,
              HasRequired,
              HasIgnore,
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
        ): BuildableLaxConfig<
          Value,
          Input,
          Output,
          CtxOptions,
          Metadata,
          DefaultState,
          ValidationState,
          HasAllowError,
          HasReValidate,
          HasRequired,
          HasIgnore,
          HasReadonly,
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
        ): BuildableLaxConfig<
          Value,
          Input,
          Output,
          CtxOptions,
          Metadata,
          DefaultState,
          ValidationState,
          HasAllowError,
          HasReValidate,
          HasRequired,
          HasIgnore,
          HasReadonly,
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
        ): BuildableLaxConfig<
          Value,
          Input,
          Output,
          CtxOptions,
          Metadata,
          DefaultState,
          ValidationState,
          HasAllowError,
          HasReValidate,
          HasRequired,
          HasIgnore,
          HasReadonly,
          HasOnDelete,
          HasOnFailure,
          true
        >;
      });

class LaxBuilder<
  Value extends Output[keyof Output],
  Input,
  Output,
  CtxOptions extends ObjectType,
  Metadata,
> implements BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata> {
  private config: Partial<
    NS.LaxField<Value, Input, Output, CtxOptions, Metadata>
  > = { type: "lax" };

  constructor(
    name: string,
    defaultValue: Value | NS.Resolver<Value, Input, Output, CtxOptions>,
  ) {
    this.config.name = name;
    this.config.default = defaultValue;
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

  required(handler: RequiredHandler<Input, Output, CtxOptions, Metadata>) {
    this.config.required = handler;
    return this as never;
  }

  ignore(resolver: NS.Resolver<boolean, Input, Output, CtxOptions>) {
    this.config.ignore = resolver;
    this.config.ignoreInit = undefined;
    this.config.ignoreUpdate = undefined;
    return this as never;
  }

  ignoreInit() {
    this.config.ignoreInit = true;
    this.config.ignore = undefined;
    this.config.ignoreUpdate = undefined;
    this.config.readonly = undefined;
    return this as never;
  }

  ignoreUpdate() {
    this.config.ignoreUpdate = true;
    this.config.ignore = undefined;
    this.config.ignoreInit = undefined;
    this.config.readonly = undefined;
    return this as never;
  }

  readonly() {
    this.config.readonly = true;
    this.config.ignoreInit = undefined;
    this.config.ignoreUpdate = undefined;
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
    return this.config as NS.LaxField<
      Value,
      Input,
      Output,
      CtxOptions,
      Metadata
    >;
  }
}
