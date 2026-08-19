import {
  type ArrayOfMinSizeOne,
  type ArrayOfMinSizeTwo,
  type Buildable,
  FIELD_CONFIG_BUILD_METHOD_NAME,
  type NotAllowedError,
  type NS,
  type ObjectType,
  type RequiredHandler,
  type ReValidator,
  type Validator,
} from '../../utils/types';
import { extractAllowedValues } from './_utils';

export { type BlankVirtualBuilder, VirtualBuilder };

type BlankVirtualBuilder<
  Value extends Input[keyof Input],
  Input,
  Output,
  CtxOptions extends ObjectType,
  Metadata,
  HasAlias extends boolean = false,
> = (HasAlias extends true
  ? {}
  : {
      alias<Alias extends keyof Input>(
        name: Alias,
      ): BlankVirtualBuilder<
        Input[Alias],
        Input,
        Output,
        CtxOptions,
        Metadata,
        true
      >;
    }) & {
  allow<const V extends Value>(
    values: ArrayOfMinSizeTwo<V>,
  ): BuildableVirtualConfig<
    V,
    Input,
    Output,
    CtxOptions,
    Metadata,
    HasAlias,
    'allow'
  >;
  validate(
    validator: Validator<Value, Input, Output, CtxOptions, Metadata>,
  ): BuildableVirtualConfig<
    Value,
    Input,
    Output,
    CtxOptions,
    Metadata,
    HasAlias,
    'validate'
  >;
};

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
  HasAlias extends boolean = false,
  ValidationState extends 'allow' | 'none' | 'validate' = 'none',
  HasAllowError extends boolean = false,
  HasReValidate extends boolean = false,
  HasRequired extends boolean = false,
  HasSanitizer extends boolean = false,
  HasIgnore extends 'init' | 'update' | 'ignore' | 'none' = 'none',
  HasOnFailure extends boolean = false,
  HasOnSuccess extends boolean = false,
> = (HasAlias extends true
  ? {}
  : {
      alias<Alias extends keyof Input>(
        name: Alias,
      ): BuildableVirtualConfig<
        Value,
        Input,
        Output,
        CtxOptions,
        Metadata,
        true,
        ValidationState,
        HasAllowError,
        HasSanitizer,
        HasRequired,
        HasSanitizer,
        HasIgnore,
        HasOnFailure,
        HasOnSuccess
      >;
    }) &
  (ValidationState extends 'none'
    ? {}
    : Buildable<
        NS.VirtualField<never, Value, Input, Output, CtxOptions, Metadata>
      >) &
  (ValidationState extends 'allow'
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
            HasAlias,
            'allow',
            true,
            HasReValidate,
            HasRequired,
            HasSanitizer,
            HasIgnore,
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
          ): BuildableVirtualConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            HasAlias,
            ValidationState,
            HasAllowError,
            true,
            HasRequired,
            HasSanitizer,
            HasIgnore,
            HasOnFailure,
            HasOnSuccess
          >;
        }) &
  (ValidationState extends 'none'
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
            HasAlias,
            ValidationState,
            HasAllowError,
            HasReValidate,
            true,
            HasSanitizer,
            HasIgnore,
            HasOnFailure,
            HasOnSuccess
          >;
        }) &
  (ValidationState extends 'none'
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
            HasAlias,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            true,
            HasIgnore,
            HasOnFailure,
            HasOnSuccess
          >;
        }) &
  (ValidationState extends 'none'
    ? {}
    : HasIgnore extends 'none'
      ? {
          ignore(
            resolver: NS.Resolver<boolean, Input, Output, CtxOptions>,
          ): BuildableVirtualConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            HasAlias,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            HasSanitizer,
            'ignore',
            HasOnFailure,
            HasOnSuccess
          >;
          ignoreInit(): BuildableVirtualConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            HasAlias,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            HasSanitizer,
            'init',
            HasOnFailure,
            HasOnSuccess
          >;
          ignoreUpdate(): BuildableVirtualConfig<
            Value,
            Input,
            Output,
            CtxOptions,
            Metadata,
            HasAlias,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            HasSanitizer,
            'update',
            HasOnFailure,
            HasOnSuccess
          >;
        }
      : {}) &
  (ValidationState extends 'none'
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
            HasAlias,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            HasSanitizer,
            HasIgnore,
            true,
            HasOnSuccess
          >;
        }) &
  (ValidationState extends 'none'
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
            HasAlias,
            ValidationState,
            HasAllowError,
            HasReValidate,
            HasRequired,
            HasSanitizer,
            HasIgnore,
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
> implements
    BlankVirtualBuilder<Value, Input, Output, CtxOptions, Metadata>,
    BuildableVirtualConfig<Value, Input, Output, CtxOptions, Metadata>
{
  private config: Partial<
    NS.VirtualField<never, Value, Input, Output, CtxOptions, Metadata>
  > = { type: 'virtual' };

  constructor(name: string) {
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
    delete this.config.ignoreInit;
    delete this.config.ignoreUpdate;

    return this as never;
  }

  ignoreInit() {
    this.config.ignoreInit = true;
    delete this.config.ignore;
    delete this.config.ignoreUpdate;

    return this as never;
  }

  ignoreUpdate() {
    this.config.ignoreUpdate = true;
    delete this.config.ignore;
    delete this.config.ignoreInit;

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
