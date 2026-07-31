import type { ObjectType } from '../../utils';
import {
  type ArrayOfMinSizeOne,
  type ArrayOfMinSizeTwo,
  BUILD,
  type NotAllowedError,
  type NS,
  type RequiredHandler,
  type ReValidator,
  type Validator,
} from '../types';

export { type BlankLaxBuilder, LaxBuilder };

interface BlankLaxBuilder<
  Value extends Output[keyof Output],
  Input,
  Output,
  CtxOptions extends ObjectType,
  Metadata,
> {
  default<const V extends Value>(
    value: V | NS.Resolver<V, Input, Output, CtxOptions>,
  ): BuildableLaxConfig<V, Input, Output, CtxOptions, Metadata>;
}

/**
 * `.allow()` and `.validate()` are mutually exclusive - whichever is called
 * first becomes unavailable through the other, since `allow` serves as the
 * field's primary validator when provided. `.allowError()` only becomes
 * available once `.allow()` has been called, and `.reValidate()` only once
 * either `.allow()` or `.validate()` has (matching Rust's `re_validate`,
 * which requires `HasValidator: Yes`, extended here to also accept `allow`
 * as the primary).
 */
type BuildableLaxConfig<
  Value extends Output[keyof Output],
  Input,
  Output,
  CtxOptions extends ObjectType,
  Metadata,
  ValidationState extends 'allow' | 'none' | 'validate' = 'none',
  HasAllowError extends boolean = false,
  HasReValidate extends boolean = false,
  HasRequired extends boolean = false,
  HasIgnore extends boolean = false,
  HasIgnoreInit extends boolean = false,
  HasIgnoreUpdate extends boolean = false,
  HasReadonly extends boolean = false,
  HasOnDelete extends boolean = false,
  HasOnFailure extends boolean = false,
  HasOnSuccess extends boolean = false,
> = {
  [BUILD](): NS.LaxField<Value, Input, Output, CtxOptions, Metadata>;
} & (ValidationState extends 'none'
  ? {
      allow(
        values: ArrayOfMinSizeTwo<Value>,
      ): BuildableLaxConfig<
        Value,
        Input,
        Output,
        CtxOptions,
        Metadata,
        'allow',
        HasAllowError,
        HasReValidate,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
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
        'validate',
        HasAllowError,
        HasReValidate,
        HasRequired,
        HasIgnore,
        HasIgnoreInit,
        HasIgnoreUpdate,
        HasReadonly,
        HasOnDelete,
        HasOnFailure,
        HasOnSuccess
      >;
    }
  : {}) &
  (ValidationState extends 'allow'
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
            'allow',
            true,
            HasReValidate,
            HasRequired,
            HasIgnore,
            HasIgnoreInit,
            HasIgnoreUpdate,
            HasReadonly,
            HasOnDelete,
            HasOnFailure,
            HasOnSuccess
          >;
        }
    : {}) &
  (ValidationState extends 'none'
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
            ValidationState,
            HasAllowError,
            true,
            HasRequired,
            HasIgnore,
            HasIgnoreInit,
            HasIgnoreUpdate,
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
          ValidationState,
          HasAllowError,
          HasReValidate,
          true,
          HasIgnore,
          HasIgnoreInit,
          HasIgnoreUpdate,
          HasReadonly,
          HasOnDelete,
          HasOnFailure,
          HasOnSuccess
        >;
      }) &
  (HasIgnore extends true
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
          ValidationState,
          HasAllowError,
          HasReValidate,
          HasRequired,
          true,
          HasIgnoreInit,
          HasIgnoreUpdate,
          HasReadonly,
          HasOnDelete,
          HasOnFailure,
          HasOnSuccess
        >;
      }) &
  (HasIgnoreInit extends true
    ? {}
    : {
        ignoreInit(
          resolver?: NS.IgnoreInitResolver<Input, CtxOptions>,
        ): BuildableLaxConfig<
          Value,
          Input,
          Output,
          CtxOptions,
          Metadata,
          ValidationState,
          HasAllowError,
          HasReValidate,
          HasRequired,
          HasIgnore,
          true,
          HasIgnoreUpdate,
          HasReadonly,
          HasOnDelete,
          HasOnFailure,
          HasOnSuccess
        >;
      }) &
  (HasIgnoreUpdate extends true
    ? {}
    : {
        ignoreUpdate(
          resolver?: NS.IgnoreUpdateResolver<Input, Output, CtxOptions>,
        ): BuildableLaxConfig<
          Value,
          Input,
          Output,
          CtxOptions,
          Metadata,
          ValidationState,
          HasAllowError,
          HasReValidate,
          HasRequired,
          HasIgnore,
          HasIgnoreInit,
          true,
          HasReadonly,
          HasOnDelete,
          HasOnFailure,
          HasOnSuccess
        >;
      }) &
  (HasReadonly extends true
    ? {}
    : {
        readonly(): BuildableLaxConfig<
          Value,
          Input,
          Output,
          CtxOptions,
          Metadata,
          ValidationState,
          HasAllowError,
          HasReValidate,
          HasRequired,
          HasIgnore,
          HasIgnoreInit,
          HasIgnoreUpdate,
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
          ValidationState,
          HasAllowError,
          HasReValidate,
          HasRequired,
          HasIgnore,
          HasIgnoreInit,
          HasIgnoreUpdate,
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
          ValidationState,
          HasAllowError,
          HasReValidate,
          HasRequired,
          HasIgnore,
          HasIgnoreInit,
          HasIgnoreUpdate,
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
          ValidationState,
          HasAllowError,
          HasReValidate,
          HasRequired,
          HasIgnore,
          HasIgnoreInit,
          HasIgnoreUpdate,
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
> implements
    BlankLaxBuilder<Value, Input, Output, CtxOptions, Metadata>,
    BuildableLaxConfig<Value, Input, Output, CtxOptions, Metadata>
{
  private config: Partial<
    NS.LaxField<Value, Input, Output, CtxOptions, Metadata>
  > = { type: 'lax' };

  default(value: Value | NS.Resolver<Value, Input, Output, CtxOptions>) {
    this.config.default = value;
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

  readonly() {
    this.config.readonly = true;
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
    return this.config as NS.LaxField<
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
