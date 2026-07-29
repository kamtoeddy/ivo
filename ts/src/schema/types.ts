import type { ObjectType } from '../utils';
import type {
  DefaultFieldErrorMetadata,
  FieldError,
  InputFieldError,
} from './utils';

export type {
  ArrayOfMinSizeOne,
  ArrayOfMinSizeTwo,
  DefinitionRule,
  InternalValidatorResponse,
  InvalidValidatorResponse,
  IvoContext,
  IvoErrorPayload,
  KeyOf,
  NS,
  PostValidationConfig,
  PostValidator,
  ReadonlyIvoContext,
  RealType,
  ResponseErrorObject,
  SetterFnData,
  TypeOf,
  ValidationResponse,
  Validator,
  ValidatorResponse,
  ValidatorResponseObject,
  XOR,
};

export {
  ALLOWED_OPTIONS,
  CONSTANT_RULES,
  DEFINITION_RULES,
  LIFE_CYCLES,
  VIRTUAL_RULES,
};

type SetterFnData<
  Input,
  Output = Input,
  CtxOptions extends ObjectType = {},
> = IvoContext<Input, Output, CtxOptions>;

type ReadonlyIvoContext<Input, Output, CtxOptions extends ObjectType = {}> = (
  | Readonly<
      WithReadonlyCtxOptions<
        {
          changes: null;
          rawInput: Partial<RealType<Input>>;
          input: Partial<RealType<Input>>;
          isUpdate: false;
          previousValues: null;
          values: Readonly<Output>;
        },
        CtxOptions
      >
    >
  | Readonly<
      WithReadonlyCtxOptions<
        {
          changes: Partial<RealType<Output>>;
          rawInput: Partial<RealType<Input>>;
          input: Partial<RealType<Input>>;
          isUpdate: true;
          previousValues: Readonly<Output>;
          values: Readonly<Output>;
        },
        CtxOptions
      >
    >
) & {};

type IvoContext<Input, Output = Input, CtxOptions extends ObjectType = {}> = (
  | Readonly<
      WithCtxOptions<
        {
          changes: null;
          rawInput: Partial<RealType<Input>>;
          input: Partial<RealType<Input>>;
          isUpdate: false;
          previousValues: null;
          values: Readonly<Output>;
        },
        CtxOptions
      >
    >
  | Readonly<
      WithCtxOptions<
        {
          changes: Partial<RealType<Output>>;
          rawInput: Partial<RealType<Input>>;
          input: Partial<RealType<Input>>;
          isUpdate: true;
          previousValues: Readonly<Output>;
          values: Readonly<Output>;
        },
        CtxOptions
      >
    >
) & {};

type WithReadonlyCtxOptions<T, CtxOptions extends ObjectType> = T & {
  options: Readonly<CtxOptions>;
};

type WithCtxOptions<T, CtxOptions extends ObjectType> = WithReadonlyCtxOptions<
  T,
  CtxOptions
> & { updateOptions: (updates: Partial<CtxOptions>) => void } & {};

type TypeOf<T> = Exclude<T, undefined>;

type AsyncSetter<T, Input, Output, CtxOptions extends ObjectType> = (
  data: SetterFnData<Input, Output, CtxOptions>,
) => TypeOf<T> | Promise<TypeOf<T>>;

type NotAllowedError<Metadata> = string | InputFieldError<Metadata>;

type SetterWithSummary<T, Input, Output, CtxOptions extends ObjectType> = (
  ctx: IvoContext<Input, Output, CtxOptions> & {},
) => TypeOf<T>;

type Setter<T, Input, Output, CtxOptions extends ObjectType> = (
  data: SetterFnData<Input, Output, CtxOptions>,
) => TypeOf<T>;

type RequiredHandlerRes<Metadata> =
  | boolean
  | [boolean, string]
  | [boolean, InputFieldError<Metadata>]
  | readonly [boolean, string]
  | readonly [boolean, InputFieldError<Metadata>];

type RequiredHandler<Input, Output, CtxOptions extends ObjectType, Metadata> = (
  ctx: IvoContext<Input, Output, CtxOptions> & {},
) => RequiredHandlerRes<Metadata> | Promise<RequiredHandlerRes<Metadata>>;

type IgnoreResolver<Input, Output, CtxOptions extends ObjectType = {}> = (
  ctx: IvoContext<Input, Output, CtxOptions> & {},
) => boolean | Promise<boolean>;

type IgnoreUpdateResolver<Input, Output, CtxOptions extends ObjectType = {}> = (
  input: Partial<Input>,
  previousValues: Output,
  o: {
    options: CtxOptions;
    updateOptions: (updates: Partial<CtxOptions>) => void;
  },
) => boolean | Promise<boolean>;

type Resolver<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> = (
  ctx: IvoContext<Input, Output, CtxOptions> & {},
) => TypeOf<Output[K]> | Promise<TypeOf<Output[K]>>;

type VirtualResolver<Value, Input, Output, CtxOptions extends ObjectType> = (
  ctx: IvoContext<Input, Output, CtxOptions>,
) => Value | Promise<Value>;

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
  | ResponseErrorObject<Metadata, Input>
  | PostValidatorSanitizedResponse<InputKeys, Input, Output>
  | Promise<
      | undefined
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
  properties: ArrayOfMinSizeTwo<K>;
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
    ctx: ReadonlyIvoContext<Input, Output, CtxOptions>,
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

  export type IgnoreConfigObject<
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = {
    fields: ArrayOfMinSizeTwo<KeyOf<Input> | string>;
    resolver: IgnoreResolver<Input, Output, CtxOptions>;
  };

  export type IgnoreConfigOption<Input, Output, CtxOptions extends ObjectType> =
    | boolean
    | IgnoreResolver<Input, Output, CtxOptions>
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
    | boolean
    | IgnoreUpdateResolver<Input, Output, CtxOptions>
    | IgnoreUpdateConfigObject<Input, Output, CtxOptions>
    | ArrayOfMinSizeOne<IgnoreUpdateConfigObject<Input, Output, CtxOptions>>;

  export type RequiredConfigObject<
    Input,
    Output,
    CtxOptions extends ObjectType,
    Metadata,
  > = {
    properties: ArrayOfMinSizeTwo<KeyOf<Input> | string>;
    handler:
      | RequiredHandler<Input, Output, CtxOptions, Metadata>
      | ArrayOfMinSizeOne<RequiredHandler<Input, Output, CtxOptions, Metadata>>;
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

  type FieldDefinition<
    K extends keyof Input | keyof Output,
    Input,
    Output,
    CtxOptions extends ObjectType,
    Metadata,
  > =
    | (K extends keyof Output & keyof Input
        ? PublicField<K, Input, Output, CtxOptions, Metadata>
        : never)
    | (K extends keyof Output
        ? PrivateField<K, Input, Output, CtxOptions>
        : never)
    | (K extends keyof Input
        ? VirtualField<K, Input, Output, CtxOptions, Metadata>
        : never);

  export type Definitions<
    Input,
    Output,
    CtxOptions extends ObjectType,
    Metadata,
  > = PrettyType<{
    [K in keyof Input | keyof Output]?: FieldDefinition<
      K,
      Input,
      Output,
      CtxOptions,
      Metadata
    >;
  }>;

  type PublicField<
    K extends keyof Output & keyof Input,
    Input,
    Output,
    CtxOptions extends ObjectType,
    Metadata,
  > = Enumerable<Metadata, TypeOf<Output[K]>> &
    (
      | LaxField<K, Input, Output, CtxOptions, Metadata>
      | Required<K, Input, Output, CtxOptions, Metadata>
    );

  type PrivateField<
    K extends keyof Output,
    Input,
    Output,
    CtxOptions extends ObjectType = {},
  > = XOR<
    Constant<K, Input, Output, CtxOptions>,
    Dependent<K, Input, Output, CtxOptions>
  >;

  export type Definitions_<Input, Output, Metadata> = {
    [K in keyof Input]?: Listenable<Input, Output, {}> & {
      allow?:
        | Readonly<ArrayOfMinSizeTwo<unknown>>
        | {
            values: Readonly<ArrayOfMinSizeTwo<unknown>>;
            error?:
              | NotAllowedError<Metadata>
              | ((
                  value: unknown,
                  allowedValues: ArrayOfMinSizeOne<unknown>,
                ) => NotAllowedError<Metadata>);
          };
      alias?: string;
      constant?: unknown;
      default?: unknown;
      dependsOn?: KeyOf<Input> | KeyOf<Input>[];
      readonly?: boolean | 'lax';
      resolver?: Function;
      required?: boolean | RequiredHandler<Input, Output, {}, Metadata>;
      sanitizer?: VirtualResolver<K, Input, Output, {}>;
      ignore?: SetterWithSummary<boolean, Input, Output, {}>;
      ignoreInit?: true | Setter<boolean, Input, Output, {}>;
      ignoreUpdate?: true | Setter<boolean, Input, Output, {}>;
      validator?: Function | [Function, Function];
      value?: unknown;
      virtual?: boolean;
    };
  };

  export type AliasToVirtualMap<T> = Record<string, KeyOf<T>>;

  export type VirtualToAliasMap<T> = Record<KeyOf<T>, string>;

  export type DependencyMap<T> = { [K in KeyOf<T>]?: KeyOf<T>[] };

  type Listenable<Input, Output, CtxOptions extends ObjectType> = {
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onFailure?:
      | FailureHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<FailureHandler<Input, Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
  };

  type Constant<
    K extends keyof Output,
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = {
    constant: true;
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
    value:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>;
  };

  type Enumerable<Metadata, T, V extends T | Readonly<T> = T> = {
    allow?:
      | ArrayOfMinSizeTwo<V>
      | {
          values: ArrayOfMinSizeTwo<V>;
          error?:
            | NotAllowedError<Metadata>
            | ((
                value: unknown,
                allowedValues: ArrayOfMinSizeOne<T>,
              ) => NotAllowedError<Metadata>);
        };
  };

  type Dependables<K extends keyof Output, Input, Output> = Exclude<
    KeyOf<Input> | KeyOf<Output>,
    K
  >;

  type Dependent<
    K extends keyof Output,
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = {
    default:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>;
    dependsOn:
      | Dependables<K, Input, Output>
      | ArrayOfMinSizeOne<Dependables<K, Input, Output>>;
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
    readonly?: true;
    resolver: Resolver<K, Input, Output, CtxOptions>;
  };

  type InitAndUpdateBlockable<
    Input,
    Output,
    CtxOptions extends ObjectType,
    T,
  > = XOR<
    { ignore?: SetterWithSummary<boolean, Input, Output, CtxOptions> },
    XOR<
      { default: T; readonly?: boolean | 'lax' },
      XOR<
        {
          ignoreInit?: Setter<boolean, Input, Output, CtxOptions>;
          ignoreUpdate?: Setter<boolean, Input, Output, CtxOptions>;
        },
        XOR<
          {
            ignoreInit?: true | Setter<boolean, Input, Output, CtxOptions>;
            ignoreUpdate?: Setter<boolean, Input, Output, CtxOptions>;
          },
          {
            ignoreInit?: Setter<boolean, Input, Output, CtxOptions>;
            ignoreUpdate?: true | Setter<boolean, Input, Output, CtxOptions>;
          }
        >
      >
    >
  >;

  type LaxField<
    K extends keyof Output & keyof Input,
    Input,
    Output,
    CtxOptions extends ObjectType,
    Metadata,
  > = Listenable<Input, Output, CtxOptions> &
    InitAndUpdateBlockable<Input, Output, CtxOptions, TypeOf<Output[K]>> & {
      default:
        | TypeOf<Output[K]>
        | AsyncSetter<Output[K], Input, Output, CtxOptions>;
      validator?:
        | Validator<K, Input, Output, CtxOptions>
        | [
            Validator<K, Input, Output, CtxOptions>,
            SecondaryValidator<Output[K], Input, Output, CtxOptions>,
          ];
      required?: RequiredHandler<Input, Output, CtxOptions, Metadata>;
    };

  type Required<
    K extends keyof Output & keyof Input,
    Input,
    Output,
    CtxOptions extends ObjectType,
    Metadata,
  > = Listenable<Input, Output, CtxOptions> & {
    required: true;
    ignoreUpdate?: true | Setter<boolean, Input, Output, CtxOptions>;
  } & (
      | {
          validator:
            | Validator<K, Input, Output, CtxOptions>
            | [
                Validator<K, Input, Output, CtxOptions>,
                SecondaryValidator<Output[K], Input, Output, CtxOptions>,
              ];
        }
      | (Enumerable<Metadata, Input[K]> & {
          validator?:
            | Validator<K, Input, Output, CtxOptions>
            | [
                Validator<K, Input, Output, CtxOptions>,
                SecondaryValidator<Output[K], Input, Output, CtxOptions>,
              ];
        })
    );

  type VirtualField<
    K extends keyof Input | string,
    Input,
    Output,
    CtxOptions extends ObjectType,
    Metadata,
  > =
    // @ts-expect-error too_bad_alias_type_is_not_inferred
    Enumerable<Metadata, TypeOf<Input[K]>> & {
      alias?: string;
      required?: RequiredHandler<Input, Output, CtxOptions, Metadata>;
      virtual: true;
      // @ts-expect-error too_bad_alias_type_is_not_inferred
      sanitizer?: VirtualResolver<Input[K], Input, Output, CtxOptions>;
      onFailure?:
        | FailureHandler<Input, Output, CtxOptions>
        | ArrayOfMinSizeOne<FailureHandler<Input, Output, CtxOptions>>;
      onSuccess?:
        | SuccessHandler<Input, Output, CtxOptions>
        | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
      validator: // @ts-expect-error too_bad_alias_type_is_not_inferred
        | VirtualValidator<Input[K], Input, Output, CtxOptions>
        | [
            // @ts-expect-error too_bad_alias_type_is_not_inferred
            VirtualValidator<Input[K], Input, Output, CtxOptions>,
            // @ts-expect-error too_bad_alias_type_is_not_inferred
            SecondaryValidator<Input[K], Input, Output, CtxOptions>,
          ];
    } & XOR<
        {},
        XOR<
          {
            ignoreInit?: Setter<boolean, Input, Output, CtxOptions>;
            ignoreUpdate?: Setter<boolean, Input, Output, CtxOptions>;
          },
          XOR<
            {
              ignoreInit?: true | Setter<boolean, Input, Output, CtxOptions>;
              ignoreUpdate?: Setter<boolean, Input, Output, CtxOptions>;
            },
            {
              ignoreInit?: Setter<boolean, Input, Output, CtxOptions>;
              ignoreUpdate?: true | Setter<boolean, Input, Output, CtxOptions>;
            }
          >
        >
      >;

  export type InternalOptions<
    Input,
    Output,
    CtxOptions extends ObjectType,
    ErrorMetadata = DefaultFieldErrorMetadata,
    ErrorPayload = IvoErrorPayload<ErrorMetadata, KeyOf<Input>>,
  > = {
    equalityDepth: number;
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
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
    setMissingDefaultsOnUpdate?: boolean;
    ignore?: IgnoreConfigOption<Input, Output, CtxOptions>;
    ignoreUpdate?: IgnoreUpdateConfigOption<Input, Output, CtxOptions>;
    required?: RequiredConfigOption<Input, Output, CtxOptions, ErrorMetadata>;
    sanitizeError: (
      payload: IvoErrorPayload<ErrorMetadata, KeyOf<Input>>,
      ctxOptions: CtxOptions,
    ) => ErrorPayload;
    timestamps?:
      | boolean
      | { createdAt?: boolean | string; updatedAt?: boolean | string };
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
    setMissingDefaultsOnUpdate?: boolean;
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
      metadata: FieldError<Metadata>['metadata'];
      reason: string;
      valid: false;
    };

type InvalidValidatorResponse<Metadata> = {
  metadata?: FieldError<Metadata>['metadata'];
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
  K extends keyof (Output | Input),
  Input,
  Output,
  CtxOptions extends ObjectType = {},
  Metadata = DefaultFieldErrorMetadata,
> = (
  value: unknown,
  ctx: IvoContext<Input, Output, CtxOptions> & {},
) =>
  | ValidatorResponse<TypeOf<Output[K]>, Metadata>
  | Promise<ValidatorResponse<TypeOf<Output[K]>, Metadata>>;

type SecondaryValidator<
  T,
  Input,
  Output,
  CtxOptions extends ObjectType = {},
  Metadata = DefaultFieldErrorMetadata,
> = (
  value: T,
  ctx: IvoContext<Input, Output, CtxOptions> & {},
) => ValidatorResponse<T, Metadata> | Promise<ValidatorResponse<T, Metadata>>;

type VirtualValidator<
  Value,
  Input,
  Output,
  CtxOptions extends ObjectType = {},
  Metadata = DefaultFieldErrorMetadata,
> = (
  value: unknown,
  ctx: IvoContext<Input, Output, CtxOptions> & {},
) =>
  | ValidatorResponse<Value, Metadata>
  | Promise<ValidatorResponse<Value, Metadata>>;

type ArrayOfMinSizeOne<T> = [T, ...T[]] | readonly [T, ...T[]];
type ArrayOfMinSizeTwo<T> = [T, T, ...T[]] | readonly [T, T, ...T[]];

const DEFINITION_RULES = [
  'alias',
  'allow',
  'constant',
  'default',
  'dependsOn',
  'ignore',
  'onDelete',
  'onFailure',
  'onSuccess',
  'readonly',
  'resolver',
  'required',
  'sanitizer',
  'ignoreInit',
  'ignoreUpdate',
  'validator',
  'value',
  'virtual',
] as const;

type DefinitionRule = (typeof DEFINITION_RULES)[number];

const ALLOWED_OPTIONS: NS.OptionsKey<unknown, unknown>[] = [
  'equalityDepth',
  'ignore',
  'ignoreUpdate',
  'onDelete',
  'onSuccess',
  'postValidate',
  'required',
  'sanitizeError',
  'setMissingDefaultsOnUpdate',
  'timestamps',
];
const CONSTANT_RULES = ['constant', 'onDelete', 'onSuccess', 'value'];
const VIRTUAL_RULES = [
  'alias',
  'allow',
  'ignore',
  'sanitizer',
  'onFailure',
  'onSuccess',
  'required',
  'ignoreInit',
  'ignoreUpdate',
  'validator',
  'virtual',
];

const LIFE_CYCLES = ['onDelete', 'onFailure', 'onSuccess'] as const;

type IvoErrorPayload<Metadata, Keys extends string> = Record<
  Keys,
  FieldError<Metadata>
>;

type TypeFromPromise<T> = T extends Promise<infer I> ? I : T;

type RealType_<T> = T extends (...args: never) => infer I ? I : T;

type PrettyType<T> = {
  [K in keyof T]: T[K];
} & {};

type RealType<T> = {
  [K in keyof T]: TypeFromPromise<Exclude<T[K], Function> | RealType_<T[K]>>;
} & {};

type Without<T, U> = { [P in Exclude<keyof T, keyof U>]?: never };

type XOR<T, U> = (T | U extends object
  ? (Without<T, U> & U) | ((Without<U, T> & T) & {})
  : T | U) & {};
