import type { ObjectType } from "../../utils";
import {
  type ArrayOfMinSizeOne,
  type ArrayOfMinSizeTwo,
  BUILD,
  Buildable,
  type NotAllowedError,
  type NS,
  type RequiredHandler,
  type ReValidator,
  type Validator,
} from "../types";

export { type BlankVirtualBuilder, VirtualBuilder };

interface BlankVirtualBuilder<
  Value extends Input[keyof Input],
  Input,
  Output,
  CtxOptions extends ObjectType,
  Metadata,
> {
  alias<Alias extends keyof Input>(
    name: Alias,
  ): BlankVirtualBuilder<Value, Input, Output, CtxOptions, Metadata>;
  allow<const V extends Value>(
    values: ArrayOfMinSizeTwo<V>,
  ): BuildableVirtualConfig<V, Input, Output, CtxOptions, Metadata, "allow">;
  validate(
    validator: Validator<Value, Input, Output, CtxOptions, Metadata>,
  ): BuildableVirtualConfig<
    Value,
    Input,
    Output,
    CtxOptions,
    Metadata,
    "validate"
  >;
}

/**
 * A virtual field's `validator` is mandatory at runtime, so - mirroring
 * Rust's `VirtualFieldBuilder`, where every rule but `alias` requires
 * `HasValidator: Yes` - everything below except `[BUILD]` itself only
 * unlocks once `.allow()` or `.validate()` has been called (mutually
 * exclusive, same rule as lax/required).
 */
type BuildableVirtualConfig<
  Value extends Input[keyof Input],
  Input,
  Output,
  CtxOptions extends ObjectType,
  Metadata,
  ValidationState extends "allow" | "none" | "validate" = "none",
  HasAllowError extends boolean = false,
  HasReValidate extends boolean = false,
  HasRequired extends boolean = false,
  HasSanitizer extends boolean = false,
  HasIgnore extends boolean = false,
  HasIgnoreInit extends boolean = false,
  HasIgnoreUpdate extends boolean = false,
  HasOnFailure extends boolean = false,
  HasOnSuccess extends boolean = false,
> = (ValidationState extends "none"
  ? {}
  : Buildable<
      NS.VirtualField<never, Value, Input, Output, CtxOptions, Metadata>
    >) &
  (ValidationState extends "allow"
    ? HasAllowError extends true
      ? {}
      : {
          allowError(
            error:
              | NotAllowedError<Metadata>
              | ((
                  value: unknown,
                  allowedValues: ArrayOfMinSizeOne<unknown>,
                ) => NotAllowedError<Metadata>),
          ): BuildableVirtualConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            "allow",
            true,
            HasReValidate,
            HasRequired,
            HasSanitizer,
            HasIgnore,
            HasIgnoreInit,
            HasIgnoreUpdate,
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
          ): BuildableVirtualConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            ValidationState,
            HasAllowError,
            true,
            HasRequired,
            HasSanitizer,
            HasIgnore,
            HasIgnoreInit,
            HasIgnoreUpdate,
            HasOnFailure,
            HasOnSuccess
          >;
        }) &
  (ValidationState extends "none"
    ? {}
    : HasRequired extends true
      ? {}
      : {
          required(
            handler: RequiredHandler<Input, Output, CtxOptions, Metadata>,
          ): BuildableVirtualConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            ValidationState,
            HasAllowError,
            HasReValidate,
            true,
            HasSanitizer,
            HasIgnore,
            HasIgnoreInit,
            HasIgnoreUpdate,
            HasOnFailure,
            HasOnSuccess
          >;
        }) &
  (ValidationState extends "none"
    ? {}
    : HasSanitizer extends true
      ? {}
      : {
          sanitize(
            sanitizer: NS.VirtualResolver<unknown, Input, Output, CtxOptions>,
          ): BuildableVirtualConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            true,
            HasIgnore,
            HasIgnoreInit,
            HasIgnoreUpdate,
            HasOnFailure,
            HasOnSuccess
          >;
        }) &
  (ValidationState extends "none"
    ? {}
    : HasIgnore extends true
      ? {}
      : {
          ignore(
            resolver: NS.Resolver<boolean, Input, Output, CtxOptions>,
          ): BuildableVirtualConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            HasSanitizer,
            true,
            HasIgnoreInit,
            HasIgnoreUpdate,
            HasOnFailure,
            HasOnSuccess
          >;
        }) &
  (ValidationState extends "none"
    ? {}
    : HasIgnoreInit extends true
      ? {}
      : {
          ignoreInit(
            resolver?: NS.IgnoreInitResolver<Input, CtxOptions>,
          ): BuildableVirtualConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            HasSanitizer,
            HasIgnore,
            true,
            HasIgnoreUpdate,
            HasOnFailure,
            HasOnSuccess
          >;
        }) &
  (ValidationState extends "none"
    ? {}
    : HasIgnoreUpdate extends true
      ? {}
      : {
          ignoreUpdate(
            resolver?: NS.IgnoreUpdateResolver<Input, Output, CtxOptions>,
          ): BuildableVirtualConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            HasSanitizer,
            HasIgnore,
            HasIgnoreInit,
            true,
            HasOnFailure,
            HasOnSuccess
          >;
        }) &
  (ValidationState extends "none"
    ? {}
    : HasOnFailure extends true
      ? {}
      : {
          onFailure(
            handler:
              | NS.FailureHandler<Input, Output, CtxOptions>
              | ArrayOfMinSizeOne<NS.FailureHandler<Input, Output, CtxOptions>>,
          ): BuildableVirtualConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            HasSanitizer,
            HasIgnore,
            HasIgnoreInit,
            HasIgnoreUpdate,
            true,
            HasOnSuccess
          >;
        }) &
  (ValidationState extends "none"
    ? {}
    : HasOnSuccess extends true
      ? {}
      : {
          onSuccess(
            handler:
              | NS.SuccessHandler<Input, Output, CtxOptions>
              | ArrayOfMinSizeOne<NS.SuccessHandler<Input, Output, CtxOptions>>,
          ): BuildableVirtualConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            HasSanitizer,
            HasIgnore,
            HasIgnoreInit,
            HasIgnoreUpdate,
            HasOnFailure,
            true
          >;
        });

class VirtualBuilder<
  Value extends Input[keyof Input],
  Input,
  Output,
  CtxOptions extends ObjectType,
  Metadata,
>
  implements
    BlankVirtualBuilder<Value, Input, Output, CtxOptions, Metadata>,
    BuildableVirtualConfig<Value, Input, Output, CtxOptions, Metadata, "allow">
{
  name: string;
  private config: Partial<
    NS.VirtualField<never, Value, Input, Output, CtxOptions, Metadata>
  > = { type: "virtual" };

  constructor(name: string) {
    this.name = name;
    this.config.name = name;
  }

  alias<Alias extends keyof Input | never>(name: Alias) {
    // @ts-expect-error ikr
    this.config.alias = name;
    return this as never;
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

  sanitize(sanitizer: NS.VirtualResolver<Value, Input, Output, CtxOptions>) {
    this.config.sanitizer = sanitizer;
    return this as never;
  }

  ignore(resolver: NS.Resolver<boolean, Input, Output, CtxOptions>) {
    this.config.ignore = resolver;
    return this as never;
  }

  ignoreInit(resolver?: NS.IgnoreInitResolver<Input, CtxOptions>) {
    this.config.ignoreInit = resolver ?? true;
    return this as never;
  }

  ignoreUpdate(resolver?: NS.IgnoreUpdateResolver<Input, Output, CtxOptions>) {
    this.config.ignoreUpdate = resolver ?? true;
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
    return this.config as never as NS.VirtualField<
      never,
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
