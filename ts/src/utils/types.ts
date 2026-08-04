export type {
  ArrayOfMinSizeOne,
  ArrayOfMinSizeTwo,
  Buildable,
  DefinitionRule,
  InternalValidatorResponse,
  InvalidValidatorResponse,
  IvoContext,
  IvoErrorPayload,
  IvoSuccessContext,
  KeyOf,
  NotAllowedError,
  NS,
  ObjectType,
  PostValidationConfig,
  PostValidator,
  ReadonlyIvoContext,
  RealType,
  RequiredHandler,
  ResponseErrorObject,
  ReValidator,
  TypeOf,
  ValidationResponse,
  Validator,
  ValidatorResponse,
  ValidatorResponseObject,
  XOR,
  DefaultFieldErrorMetadata,
  FieldError,
  InputFieldError,
  InputPayload,
};

export {
  ALLOWED_OPTIONS,
  FIELD_CONFIG_BUILD_METHOD_NAME,
  CONSTANT_RULES,
  DEFINITION_RULES,
  LIFE_CYCLES,
  VIRTUAL_RULES,
};

type ObjectType<T = Record<string, unknown>> = T extends object
  ? T extends unknown[]
    ? never
    : T & {}
  : never;

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

type ReadonlyIvoContext<
  Input,
  Output = Input,
  CtxOptions extends ObjectType = {},
> = (
  | Readonly<
      WithReadonlyCtxOptions<
        {
          readonly changes: null;
          readonly input: Partial<RealType<Input>>;
          readonly rawInput: Partial<RealType<Input>>;
          readonly isUpdate: false;
          readonly previousValues: null;
          readonly values: Partial<RealType<Output>>;
        },
        CtxOptions
      >
    >
  | Readonly<
      WithReadonlyCtxOptions<
        {
          readonly changes: Partial<RealType<Output>>;
          readonly input: Partial<RealType<Input>>;
          readonly rawInput: Partial<RealType<Input>>;
          readonly isUpdate: true;
          readonly previousValues: RealType<Output>;
          readonly values: RealType<Output>;
        },
        CtxOptions
      >
    >
) & {};

type IvoContext<Input, Output = Input, CtxOptions extends ObjectType = {}> = (
  | Readonly<
      WithCtxOptions<
        {
          readonly changes: null;
          readonly input: Partial<RealType<Input>>;
          readonly rawInput: Partial<RealType<Input>>;
          readonly isUpdate: false;
          readonly previousValues: null;
          readonly values: Partial<RealType<Output>>;
        },
        CtxOptions
      >
    >
  | Readonly<
      WithCtxOptions<
        {
          readonly changes: Partial<RealType<Output>>;
          readonly input: Partial<RealType<Input>>;
          readonly rawInput: Partial<RealType<Input>>;
          readonly isUpdate: true;
          readonly previousValues: RealType<Output>;
          readonly values: RealType<Output>;
        },
        CtxOptions
      >
    >
) & {};

type IvoSuccessContext<
  Input,
  Output = Input,
  CtxOptions extends ObjectType = {},
> = (
  | Readonly<
      WithReadonlyCtxOptions<
        {
          readonly changes: null;
          readonly input: Partial<RealType<Input>>;
          readonly rawInput: Partial<RealType<Input>>;
          readonly isUpdate: false;
          readonly previousValues: null;
          readonly values: RealType<Output>;
        },
        CtxOptions
      >
    >
  | Readonly<
      WithReadonlyCtxOptions<
        {
          readonly changes: Partial<RealType<Output>>;
          readonly input: Partial<RealType<Input>>;
          readonly rawInput: Partial<RealType<Input>>;
          readonly isUpdate: true;
          readonly previousValues: RealType<Output>;
          readonly values: RealType<Output>;
        },
        CtxOptions
      >
    >
) & {};

export type ConstantResolverCtx<
  Input,
  Output = Input,
  CtxOptions extends ObjectType = {},
> = Readonly<
  WithCtxOptions<
    {
      readonly input: Partial<RealType<Input>>;
      readonly rawInput: Partial<RealType<Input>>;
      readonly values: Partial<Output>;
    },
    CtxOptions
  >
> & {};

export type InitResolverCtx<
  Input,
  CtxOptions extends ObjectType = {},
> = Readonly<
  WithCtxOptions<
    {
      readonly input: Partial<RealType<Input>>;
      readonly rawInput: Partial<RealType<Input>>;
    },
    CtxOptions
  >
> & {};

export type UpdateResolverCtx<
  Input,
  Output,
  CtxOptions extends ObjectType = {},
> = Readonly<
  WithCtxOptions<
    {
      readonly input: Partial<RealType<Input>>;
      readonly rawInput: Partial<RealType<Input>>;
      readonly previousValues: RealType<Output>;
    },
    CtxOptions
  >
> & {};

export type RequiredErrorResolver<Input, CtxOptions extends ObjectType> = (
  ctx: InitResolverCtx<Input, CtxOptions>,
) => string;

type WithReadonlyCtxOptions<T, CtxOptions extends ObjectType> = T & {
  readonly options: CtxOptions;
};

type WithCtxOptions<T, CtxOptions extends ObjectType> = WithReadonlyCtxOptions<
  T,
  CtxOptions
> & { readonly updateOptions: (updates: Partial<CtxOptions>) => void } & {};

type TypeOf<T> = Exclude<T, undefined>;

type NotAllowedError<Metadata> = string | InputFieldError<Metadata>;

type RequiredHandlerRes<Metadata> =
  | boolean
  | [boolean, string]
  | [boolean, InputFieldError<Metadata>]
  | readonly [boolean, string]
  | readonly [boolean, InputFieldError<Metadata>];

type RequiredHandler<Input, Output, CtxOptions extends ObjectType, Metadata> = (
  ctx: IvoContext<Input, Output, CtxOptions> & {},
) => RequiredHandlerRes<Metadata> | Promise<RequiredHandlerRes<Metadata>>;

type RequiredOptionHandler<
  Input,
  Output,
  CtxOptions extends ObjectType,
  Metadata,
> = (
  ctx: IvoContext<Input, Output, CtxOptions> & {},
) =>
  | ResponseErrorObject<Metadata, Input>
  | Promise<undefined | ResponseErrorObject<Metadata, Input>>;

type PostValidator<
  InputKeys extends KeyOf<Input>,
  Input,
  Output,
  CtxOptions extends ObjectType,
  Metadata,
> = (
  ctx: IvoContext<Input, Output, CtxOptions>,
  propertiesProvided: InputKeys[],
) =>
  | undefined
  | true
  | void
  | ResponseErrorObject<Metadata, Input>
  | PostValidatorSanitizedResponse<InputKeys, Input, Output>
  | Promise<
      | undefined
      | true
      // biome-ignore lint/suspicious/noConfusingVoidType: lol
      | void
      | ResponseErrorObject<Metadata, Input>
      | PostValidatorSanitizedResponse<InputKeys, Input, Output>
    >;

type PostValidatorSanitizedResponse<K extends KeyOf<Input>, Input, Output> = {
  [Key in K]?: {
    validated: TypeOf<Key extends KeyOf<Output> ? Output[Key] : Input[Key]>;
  };
};

type PostValidationConfig<
  K extends KeyOf<Input>,
  Input,
  Output,
  CtxOptions extends ObjectType,
  Metadata,
> = {
  fields: ArrayOfMinSizeTwo<K>;
  validator:
    | PostValidator<K, Input, Output, CtxOptions, Metadata>
    | ArrayOfMinSizeOne<
        | PostValidator<K, Input, Output, CtxOptions, Metadata>
        | ArrayOfMinSizeOne<
            PostValidator<K, Input, Output, CtxOptions, Metadata>
          >
      >;
};

type KeyOf<T> = Extract<keyof T, string>;

namespace NS {
  export type LifeCycle = (typeof LIFE_CYCLES)[number];

  export type DeleteHandler<Output, CtxOptions extends ObjectType> = (
    data: Readonly<Output>,
    options: Readonly<CtxOptions>,
  ) => unknown | Promise<unknown>;

  export type FailureHandler<
    Input,
    Output,
    CtxOptions extends ObjectType = {},
  > = (
    ctx: ReadonlyIvoContext<Input, Output, CtxOptions>,
    options: Readonly<CtxOptions>,
  ) => unknown | Promise<unknown>;

  export type SuccessHandler<
    Input,
    Output,
    CtxOptions extends ObjectType = {},
  > = (
    ctx: IvoSuccessContext<Input, Output, CtxOptions>,
  ) => unknown | Promise<unknown>;

  export type OnSuccessConfigObject<
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = {
    fields: ArrayOfMinSizeTwo<KeyOf<Input & Output>>;
    resolver:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
  };

  export type OnSuccessConfigOption<
    Input,
    Output,
    CtxOptions extends ObjectType,
  > =
    | SuccessHandler<Input, Output, CtxOptions>
    | OnSuccessConfigObject<Input, Output, CtxOptions>
    | ArrayOfMinSizeOne<
        | SuccessHandler<Input, Output, CtxOptions>
        | OnSuccessConfigObject<Input, Output, CtxOptions>
      >;

  export type Resolver<T, Input, Output, CtxOptions extends ObjectType> = (
    ctx: IvoContext<Input, Output, CtxOptions>,
  ) => TypeOf<T> | Promise<TypeOf<T>>;

  export type ConstantResolver<
    T,
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = (
    ctx: ConstantResolverCtx<Input, Output, CtxOptions>,
  ) => TypeOf<T> | Promise<TypeOf<T>>;

  export type DefaultValueResolver<T, Input, CtxOptions extends ObjectType> = (
    ctx: InitResolverCtx<Input, CtxOptions>,
  ) => TypeOf<T> | Promise<TypeOf<T>>;

  export type IgnoreInitResolver<Input, CtxOptions extends ObjectType = {}> = (
    ctx: InitResolverCtx<Input, CtxOptions>,
  ) => boolean | Promise<boolean>;

  export type IgnoreUpdateResolver<
    Input,
    Output,
    CtxOptions extends ObjectType = {},
  > = (
    ctx: UpdateResolverCtx<Input, Output, CtxOptions>,
  ) => boolean | Promise<boolean>;

  export type VirtualResolver<
    Value,
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = (ctx: IvoContext<Input, Output, CtxOptions>) => Value | Promise<Value>;

  export type IgnoreConfigObject<
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = {
    fields: ArrayOfMinSizeTwo<KeyOf<Input> | string>;
    resolver: Resolver<boolean, Input, Output, CtxOptions>;
  };

  export type IgnoreConfigOption<
    Input,
    Output,
    CtxOptions extends ObjectType,
  > =
    | Resolver<boolean, Input, Output, CtxOptions>
    | IgnoreConfigObject<Input, Output, CtxOptions>
    | ArrayOfMinSizeOne<IgnoreConfigObject<Input, Output, CtxOptions>>;

  export type IgnoreUpdateConfigObject<
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = {
    fields: ArrayOfMinSizeTwo<KeyOf<Input> | string>;
    resolver: IgnoreUpdateResolver<Input, Output, CtxOptions>;
  };

  export type IgnoreUpdateConfigOption<
    Input,
    Output,
    CtxOptions extends ObjectType,
  > =
    | IgnoreUpdateResolver<Input, Output, CtxOptions>
    | IgnoreUpdateConfigObject<Input, Output, CtxOptions>
    | ArrayOfMinSizeOne<IgnoreUpdateConfigObject<Input, Output, CtxOptions>>;

  export type RequiredConfigObject<
    Input,
    Output,
    CtxOptions extends ObjectType,
    Metadata,
  > = {
    fields: ArrayOfMinSizeTwo<KeyOf<Input> | string>;
    handler:
      | RequiredOptionHandler<Input, Output, CtxOptions, Metadata>
      | ArrayOfMinSizeOne<
          RequiredOptionHandler<Input, Output, CtxOptions, Metadata>
        >;
  };

  export type RequiredConfigOption<
    Input,
    Output,
    CtxOptions extends ObjectType,
    Metadata,
  > =
    | RequiredConfigObject<Input, Output, CtxOptions, Metadata>
    | ArrayOfMinSizeOne<
        RequiredConfigObject<Input, Output, CtxOptions, Metadata>
      >;

  export type FieldDefinition<
    _K extends keyof Input | keyof Output,
    Input,
    Output,
    CtxOptions extends ObjectType,
    Metadata,
  > =
    | Buildable<ConstantField<any, Input, Output, CtxOptions>>
    | Buildable<DependentField<any, Input, Output, CtxOptions>>
    | Buildable<LaxField<any, Input, Output, CtxOptions, Metadata>>
    | Buildable<RequiredField<any, Input, Output, CtxOptions, Metadata>>
    | Buildable<VirtualField<any, any, Input, Output, CtxOptions, Metadata>>;

  export type Definitions<
    Input,
    Output,
    CtxOptions extends ObjectType,
    Metadata,
  > = PrettyType<
    {
      [K in keyof Input | keyof Output]?: FieldDefinition<
        K,
        Input,
        Output,
        CtxOptions,
        Metadata
      >;
    } & {
      [K: string]: FieldDefinition<
        keyof Input | keyof Output,
        Input,
        Output,
        CtxOptions,
        Metadata
      >;
    }
  >;

  export type Definitions_<
    Input,
    Output,
    CtxOptions extends ObjectType,
    Metadata,
  > = Record<
    string,
    | ConstantField<any, Input, Output, CtxOptions>
    | DependentField<any, Input, Output, CtxOptions>
    | LaxField<any, Input, Output, CtxOptions, Metadata>
    | RequiredField<any, Input, Output, CtxOptions, Metadata>
    | VirtualField<any, any, Input, Output, CtxOptions, Metadata>
  >;

  export type AliasToVirtualMap<T> = Record<string, KeyOf<T>>;

  export type VirtualToAliasMap<T> = Record<KeyOf<T>, string>;

  export type DependencyMap<T> = { [K in KeyOf<T>]?: KeyOf<T>[] };

  export type ConstantField<
    Value extends Output[keyof Output],
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = {
    name: string;
    type: "constant";
    value: Value | ConstantResolver<Value, Input, Output, CtxOptions>;
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
  };

  export type Dependables<K extends keyof Output, Input, Output> = Exclude<
    (KeyOf<Input> | KeyOf<Output>) | (string & {}),
    K
  >;

  export type DependentField<
    K extends keyof Output,
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = {
    name: string;
    type: "dependent";
    default: TypeOf<Output[K]> | Resolver<Output[K], Input, Output, CtxOptions>;
    dependsOn: ArrayOfMinSizeOne<Dependables<K, Input, Output>>;
    resolver: Resolver<Output[K], Input, Output, CtxOptions>;
    readonly?: true;
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
  };

  export type LaxField<
    Value extends Output[keyof Output],
    Input,
    Output,
    CtxOptions extends ObjectType,
    Metadata,
  > = {
    name: string;
    type: "lax";
    default: Value | Resolver<Value, Input, Output, CtxOptions>;
    allow?:
      | ArrayOfMinSizeTwo<Value>
      | {
          values: ArrayOfMinSizeTwo<Value>;
          error?:
            | NotAllowedError<Metadata>
            | ((
                value: unknown,
                allowedValues: ArrayOfMinSizeOne<Value>,
              ) => NotAllowedError<Metadata>);
        };
    readonly?: true;
    ignore?: Resolver<boolean, Input, Output, CtxOptions>;
    ignoreInit?: true | IgnoreInitResolver<Input, CtxOptions>;
    ignoreUpdate?: true | IgnoreUpdateResolver<Input, Output, CtxOptions>;
    validator?: Validator<Value, Input, Output, CtxOptions, Metadata>;
    reValidator?: ReValidator<Value, Input, Output, CtxOptions, Metadata>;
    required?: RequiredHandler<Input, Output, CtxOptions, Metadata>;
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>
      | undefined;
    onFailure?:
      | FailureHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<FailureHandler<Input, Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
  };

  export type RequiredField<
    Value extends Output[keyof Output],
    Input,
    Output,
    CtxOptions extends ObjectType,
    Metadata,
  > = {
    name: string;
    type: "required";
    allow?:
      | ArrayOfMinSizeTwo<Value>
      | {
          values: ArrayOfMinSizeTwo<Value>;
          error?:
            | NotAllowedError<Metadata>
            | ((
                value: unknown,
                allowedValues: ArrayOfMinSizeOne<Value>,
              ) => NotAllowedError<Metadata>);
        };
    requiredError?:
      | string
      | ((ctx: InitResolverCtx<Input, CtxOptions>) => string);
    readonly?: true;
    ignoreUpdate?: true | IgnoreUpdateResolver<Input, Output, CtxOptions>;
    validator?: Validator<Value, Input, Output, CtxOptions, Metadata>;
    reValidator?: ReValidator<Value, Input, Output, CtxOptions, Metadata>;
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>
      | undefined;
    onFailure?:
      | FailureHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<FailureHandler<Input, Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
  };

  export type VirtualField<
    Alias extends keyof Input | never,
    Value extends Input[keyof Input],
    Input,
    Output,
    CtxOptions extends ObjectType,
    Metadata,
  > = {
    name: string;
    type: "virtual";
    alias?: Alias;
    required?: RequiredHandler<Input, Output, CtxOptions, Metadata>;
    validator?: Validator<Value, Input, Output, CtxOptions, Metadata>;
    reValidator?: ReValidator<Value, Input, Output, CtxOptions, Metadata>;
    sanitizer?: VirtualResolver<Value, Input, Output, CtxOptions>;
    allow?:
      | ArrayOfMinSizeTwo<Value>
      | {
          values: ArrayOfMinSizeTwo<Value>;
          error?:
            | NotAllowedError<Metadata>
            | ((
                value: unknown,
                allowedValues: ArrayOfMinSizeOne<Value>,
              ) => NotAllowedError<Metadata>);
        };
    ignore?: Resolver<boolean, Input, Output, CtxOptions>;
    ignoreInit?: true | IgnoreInitResolver<Input, CtxOptions>;
    ignoreUpdate?: true | IgnoreUpdateResolver<Input, Output, CtxOptions>;
    onFailure?:
      | FailureHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<FailureHandler<Input, Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
  };

  export type InternalOptions<
    Input,
    Output,
    CtxOptions extends ObjectType,
    ErrorMetadata = DefaultFieldErrorMetadata,
    ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<Input>>,
  > = Omit<
    Options<Input, Output, CtxOptions, ErrorMetadata, ErrorPayload>,
    "sanitizeError" | "timestamps"
  > & {
    sanitizeError: (
      payload: IvoErrorPayload<ErrorMetadata, KeyOf<Input>>,
      ctxOptions: CtxOptions,
    ) => ErrorPayload;
  };

  export type Options<
    Input,
    Output,
    CtxOptions extends ObjectType = {},
    ErrorMetadata = DefaultFieldErrorMetadata,
    ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<Input>>,
  > = {
    equalityDepth?: number;
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onSuccess?: OnSuccessConfigOption<Input, Output, CtxOptions>;
    postValidate?:
      | PostValidationConfig<
          KeyOf<Input>,
          Input,
          Output,
          CtxOptions,
          ErrorMetadata
        >
      | ArrayOfMinSizeOne<
          PostValidationConfig<
            KeyOf<Input>,
            Input,
            Output,
            CtxOptions,
            ErrorMetadata
          >
        >;
    ignore?: IgnoreConfigOption<Input, Output, CtxOptions>;
    ignoreUpdate?: IgnoreUpdateConfigOption<Input, Output, CtxOptions>;
    required?: RequiredConfigOption<Input, Output, CtxOptions, ErrorMetadata>;
    sanitizeError?: (
      payload: IvoErrorPayload<ErrorMetadata, KeyOf<Input>>,
      ctxOptions: CtxOptions,
    ) => ErrorPayload;
    timestamps?:
      | boolean
      | {
          createdAt?: boolean | string;
          updatedAt?: boolean | string | { key?: string; nullable?: boolean };
        };
  };

  export type OptionsKey<Input, Output> = KeyOf<Options<Input, Output>>;

  export type PrivateOptions = { timestamps: Timestamp };

  export type Timestamp = { createdAt: string; updatedAt: string };

  export type ExtensionOptions<
    ParentInput,
    ParentOutput,
    Input,
    Output,
    CtxOptions extends ObjectType = {},
    ErrorMetadata = DefaultFieldErrorMetadata,
    ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<Input>>,
  > = Options<Input, Output, CtxOptions, ErrorMetadata, ErrorPayload> & {
    remove?:
      | (KeyOf<ParentInput> | KeyOf<ParentOutput>)
      | (KeyOf<ParentInput> | KeyOf<ParentOutput>)[];
    useParentOptions?: boolean;
  };
}

type ValidationResponse<T, Metadata = DefaultFieldErrorMetadata> =
  | { valid: true; validated: T }
  | {
      metadata: FieldError<Metadata>["metadata"];
      reason: string;
      valid: false;
    };

type InvalidValidatorResponse<Metadata> = {
  metadata?: FieldError<Metadata>["metadata"];
  reason?: string;
  valid: false;
  value?: unknown;
};

type InternalValidatorResponse<T, Metadata> =
  | { valid: true; validated: T }
  | InvalidValidatorResponse<Metadata>;

type ValidatorResponseObject<T, Metadata> =
  | { valid: true; validated?: T }
  | InvalidValidatorResponse<Metadata>;

type ResponseErrorObject<Metadata, Input = object> = {
  [K in KeyOf<Input>]?: string | InputFieldError<Metadata>;
};

type ValidatorResponse<T, Metadata> =
  | boolean
  | (ValidatorResponseObject<T, Metadata> & {});

type Validator<
  T,
  Input,
  Output,
  CtxOptions extends ObjectType = {},
  Metadata = DefaultFieldErrorMetadata,
> = (
  value: unknown,
  ctx: IvoContext<Input, Output, CtxOptions> & {},
) => ValidatorResponse<T, Metadata> | Promise<ValidatorResponse<T, Metadata>>;

type ReValidator<
  T,
  Input,
  Output,
  CtxOptions extends ObjectType = {},
  Metadata = DefaultFieldErrorMetadata,
> = (
  value: T,
  ctx: IvoContext<Input, Output, CtxOptions> & {},
) => ValidatorResponse<T, Metadata> | Promise<ValidatorResponse<T, Metadata>>;

type ArrayOfMinSizeOne<T> = [T, ...T[]] | readonly [T, ...T[]];
type ArrayOfMinSizeTwo<T> = [T, T, ...T[]] | readonly [T, T, ...T[]];

const DEFINITION_RULES = [
  "name",
  "type",
  "alias",
  "allow",
  "constant",
  "default",
  "dependsOn",
  "ignore",
  "onDelete",
  "onFailure",
  "onSuccess",
  "readonly",
  "resolver",
  "required",
  "reValidator",
  "sanitizer",
  "ignoreInit",
  "ignoreUpdate",
  "validator",
  "value",
  "virtual",
] as const;

type DefinitionRule = (typeof DEFINITION_RULES)[number];

const ALLOWED_OPTIONS: NS.OptionsKey<unknown, unknown>[] = [
  "equalityDepth",
  "ignore",
  "ignoreUpdate",
  "onDelete",
  "onSuccess",
  "postValidate",
  "required",
  "sanitizeError",
  "timestamps",
];
const CONSTANT_RULES = [
  "name",
  "type",
  "constant",
  "onDelete",
  "onSuccess",
  "value",
];
const VIRTUAL_RULES = [
  "name",
  "type",
  "alias",
  "allow",
  "ignore",
  "sanitizer",
  "onFailure",
  "onSuccess",
  "required",
  "reValidator",
  "ignoreInit",
  "ignoreUpdate",
  "validator",
  "virtual",
];

const LIFE_CYCLES = ["onDelete", "onFailure", "onSuccess"] as const;

type IvoErrorPayload<Metadata, Keys extends string> = {
  [K in Keys]?: FieldError<Metadata>;
};

type TypeFromPromise<T> = T extends Promise<infer I> ? I : T;

type RealType_<T> = T extends (...args: never) => infer I ? I : T;

type PrettyType<T> = {
  [K in keyof T]: T[K];
} & {};

type RealType<T> = {
  [K in keyof T]: TypeFromPromise<Exclude<T[K], Function> | RealType_<T[K]>>;
} & {};

const FIELD_CONFIG_BUILD_METHOD_NAME: unique symbol = Symbol(
  "ivo.field-config-build-method-name",
);

type Buildable<T> = { [FIELD_CONFIG_BUILD_METHOD_NAME]: () => T };

type Without<T, U> = { [P in Exclude<keyof T, keyof U>]?: never };

type XOR<T, U> = (T | U extends object
  ? (Without<T, U> & U) | ((Without<U, T> & T) & {})
  : T | U) & {};
