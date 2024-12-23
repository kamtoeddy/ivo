import type { ObjectType } from '../utils';
import type {
  FieldError,
  IErrorTool,
  InputFieldError,
  ValidationErrorMessage,
} from './utils';

export type {
  ArrayOfMinSizeOne,
  ArrayOfMinSizeTwo,
  DefinitionRule,
  KeyOf,
  Merge,
  NS,
  PartialContext,
  PostValidationConfig,
  PostValidator,
  RealType,
  ResponseErrorObject,
  TypeOf,
  Validator,
  ValidationResponse,
  // ctx
  Context,
  DeletionContext,
  ImmutableContext,
  MutableContext,
  // summary
  ImmutableSummary,
  MutableSummary,
  ValidatorResponse,
  ValidatorResponseObject,
  InternalValidatorResponse,
  InvalidValidatorResponse,
  XOR,
};

export {
  ALLOWED_OPTIONS,
  CONSTANT_RULES,
  DEFINITION_RULES,
  LIFE_CYCLES,
  VIRTUAL_RULES,
};

type Context<
  Input,
  Output = Input,
  CtxOptions extends ObjectType = {},
> = Readonly<
  Merge<Input, Output> & { __getOptions__: () => Readonly<CtxOptions> }
> & {};

type ImmutableContext<
  Input,
  Output = Input,
  CtxOptions extends ObjectType = {},
> = Context<Input, Output, CtxOptions>;

type MutableContext<
  Input,
  Output = Input,
  CtxOptions extends ObjectType = {},
> = Readonly<
  Merge<Input, Output> & {
    __getOptions__: () => Readonly<CtxOptions>;
    __updateOptions__: (updates: Partial<CtxOptions>) => void;
  }
> & {};

type DeletionContext<Output, CtxOptions extends ObjectType = {}> = Readonly<
  Output & { __getOptions__: () => Readonly<CtxOptions> }
> & {};

type PartialContext<Input, Output> = Readonly<Merge<Input, Output>>;

type ImmutableSummary<
  Input,
  Output = Input,
  CtxOptions extends ObjectType = {},
> = (
  | Readonly<{
      changes: null;
      context: ImmutableContext<Input, Output, CtxOptions>;
      inputValues: Partial<RealType<Input>>;
      isUpdate: false;
      previousValues: null;
      values: Readonly<Output>;
    }>
  | Readonly<{
      changes: Partial<RealType<Output>>;
      context: ImmutableContext<Input, Output, CtxOptions>;
      inputValues: Partial<RealType<Input>>;
      isUpdate: true;
      previousValues: Readonly<Output>;
      values: Readonly<Output>;
    }>
) & {};

type MutableSummary<
  Input,
  Output = Input,
  CtxOptions extends ObjectType = {},
> = (
  | Readonly<{
      changes: null;
      context: MutableContext<Input, Output, CtxOptions>;
      inputValues: Partial<RealType<Input>>;
      isUpdate: false;
      previousValues: null;
      values: Readonly<Output>;
    }>
  | Readonly<{
      changes: Partial<RealType<Output>>;
      context: MutableContext<Input, Output, CtxOptions>;
      inputValues: Partial<RealType<Input>>;
      isUpdate: true;
      previousValues: Readonly<Output>;
      values: Readonly<Output>;
    }>
) & {};

type TypeOf<T> = Exclude<T, undefined>;

type AsyncSetter<T, Input, Output, CtxOptions extends ObjectType> = (
  context: MutableContext<Input, Output, CtxOptions>,
) => TypeOf<T> | Promise<TypeOf<T>>;

type NotAllowedError = string | InputFieldError;

type Setter<T, Input, Output, CtxOptions extends ObjectType> = (
  context: MutableContext<Input, Output, CtxOptions>,
) => TypeOf<T>;

type RequiredHandlerRes =
  | boolean
  | [boolean, string]
  | [boolean, InputFieldError]
  | readonly [boolean, string]
  | readonly [boolean, InputFieldError];

type RequiredHandler<Input, Output, CtxOptions extends ObjectType> = (
  summary: MutableSummary<Input, Output, CtxOptions> & {},
) => RequiredHandlerRes | Promise<RequiredHandlerRes>;

type AsyncShouldUpdate<Input, Output, CtxOptions extends ObjectType = {}> = (
  summary: MutableSummary<Input, Output, CtxOptions> & {},
) => boolean | Promise<boolean>;

type Resolver<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType,
> = (
  summary: MutableSummary<Input, Output, CtxOptions> & {},
) => TypeOf<Output[K]> | Promise<TypeOf<Output[K]>>;

type VirtualResolver<
  K extends keyof Input,
  Input,
  Output,
  CtxOptions extends ObjectType,
> = (
  summary: MutableSummary<Input, Output, CtxOptions>,
) => TypeOf<Input[K]> | Promise<TypeOf<Input[K]>>;

type PostValidator<Input, Output, Aliases, CtxOptions extends ObjectType> = (
  summary: MutableSummary<Input, Output, CtxOptions>,
  propertiesProvided: KeyOf<Input>[],
) =>
  | undefined
  | ResponseErrorObject<Input, Aliases>
  | Promise<undefined | ResponseErrorObject<Input, Aliases>>;

type PostValidationConfig<
  Input,
  Output,
  Aliases,
  CtxOptions extends ObjectType,
> = {
  properties: ArrayOfMinSizeTwo<KeyOf<Input>>;
  validator:
    | PostValidator<Input, Output, Aliases, CtxOptions>
    | ArrayOfMinSizeOne<
        | PostValidator<Input, Output, Aliases, CtxOptions>
        | ArrayOfMinSizeOne<PostValidator<Input, Output, Aliases, CtxOptions>>
      >;
};

type KeyOf<T> = Extract<keyof T, string>;

namespace NS {
  export type LifeCycle = (typeof LIFE_CYCLES)[number];

  export type DeleteHandler<Output, CtxOptions extends ObjectType> = (
    data: DeletionContext<Output, CtxOptions>,
  ) => unknown | Promise<unknown>;

  export type FailureHandler<
    Input,
    Output,
    CtxOptions extends ObjectType = {},
  > = (
    context: ImmutableContext<Input, Output, CtxOptions> & {},
  ) => unknown | Promise<unknown>;

  export type SuccessHandler<
    Input,
    Output,
    CtxOptions extends ObjectType = {},
  > = (
    summary: ImmutableSummary<Input, Output, CtxOptions> & {},
  ) => unknown | Promise<unknown>;

  export type OnSuccessConfigObject<
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = {
    properties: ArrayOfMinSizeTwo<KeyOf<Input & Output>>;
    handler:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
  };

  export type OnSuccessConfig<Input, Output, CtxOptions extends ObjectType> =
    | SuccessHandler<Input, Output, CtxOptions>
    | OnSuccessConfigObject<Input, Output, CtxOptions>
    | ArrayOfMinSizeOne<
        | SuccessHandler<Input, Output, CtxOptions>
        | OnSuccessConfigObject<Input, Output, CtxOptions>
      >;

  export type Definitions<
    Input,
    Output,
    Aliases = {},
    CtxOptions extends ObjectType = {},
  > = PrettyType<
    {
      [K in keyof (Input | Output)]: PublicProperty<
        K,
        Input,
        Output,
        CtxOptions
      >;
    } & {
      [K in keyof Omit<Output, keyof Input>]?: PrivateProperty<
        K,
        Input,
        Output,
        CtxOptions
      >;
    } & {
      [K in keyof Omit<Input, keyof Output>]: Virtual<
        K,
        Input,
        Output,
        Aliases,
        CtxOptions
      >;
    }
  >;

  type PublicProperty<
    K extends keyof (Output | Input),
    Input,
    Output,
    CtxOptions extends ObjectType = {},
  > = Enumerable<Output[K]> &
    (
      | LaxProperty<K, Input, Output, CtxOptions>
      | ReadOnly<K, Input, Output, CtxOptions>
      | ReadonlyNoInit<K, Input, Output, CtxOptions>
      | Required<K, Input, Output, CtxOptions>
      | RequiredBy<K, Input, Output, CtxOptions>
      | RequiredReadonly<K, Input, Output, CtxOptions>
    );

  type PrivateProperty<
    K extends keyof Output,
    Input,
    Output,
    CtxOptions extends ObjectType = {},
  > = XOR<
    Constant<K, Input, Output, CtxOptions>,
    Dependent<K, Input, Output, CtxOptions>
  >;

  export type Definitions_<Input, Output> = {
    [K in keyof Input]?: Listenable<Input, Output> & {
      allow?:
        | Readonly<ArrayOfMinSizeTwo<unknown>>
        | {
            values: Readonly<ArrayOfMinSizeTwo<unknown>>;
            error?:
              | NotAllowedError
              | ((
                  value: unknown,
                  allowedValues: ArrayOfMinSizeOne<unknown>,
                ) => NotAllowedError);
          };
      alias?: string;
      constant?: unknown;
      default?: unknown;
      dependsOn?: KeyOf<Input> | KeyOf<Input>[];
      readonly?: boolean | 'lax';
      resolver?: Function;
      required?: boolean | RequiredHandler<Input, Output, {}>;
      sanitizer?: VirtualResolver<K, Input, Output, {}>;
      shouldInit?: false | Setter<boolean, Input, Output, {}>;
      shouldUpdate?: false | Setter<boolean, Input, Output, {}>;
      validator?: Function | [Function, Function];
      value?: unknown;
      virtual?: boolean;
    };
  };

  export type AliasToVirtualMap<T> = Record<string, KeyOf<T>>;

  export type VirtualToAliasMap<T> = Record<KeyOf<T>, string>;

  export type DependencyMap<T> = { [K in KeyOf<T>]?: KeyOf<T>[] };

  type Listenable<Input, Output, CtxOptions extends ObjectType = {}> = {
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

  type Enumerable<T, V extends T | Readonly<T> = T> = {
    allow?:
      | ArrayOfMinSizeTwo<V>
      | {
          values: ArrayOfMinSizeTwo<V>;
          error?:
            | NotAllowedError
            | ((
                value: unknown,
                allowedValues: ArrayOfMinSizeOne<T>,
              ) => NotAllowedError);
        };
  };

  type Dependables<
    K extends keyof Output,
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = Exclude<
    KeyOf<MutableContext<Input, Output, CtxOptions>>,
    K | '__getOptions__' | '__updateOptions__'
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
      | Dependables<K, Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<Dependables<K, Input, Output, CtxOptions>>;
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
  > = XOR<
    {
      shouldInit?: Setter<boolean, Input, Output, CtxOptions>;
      shouldUpdate?: Setter<boolean, Input, Output, CtxOptions>;
    },
    XOR<
      {
        shouldInit?: false | Setter<boolean, Input, Output, CtxOptions>;
        shouldUpdate?: Setter<boolean, Input, Output, CtxOptions>;
      },
      {
        shouldInit?: Setter<boolean, Input, Output, CtxOptions>;
        shouldUpdate?: false | Setter<boolean, Input, Output, CtxOptions>;
      }
    >
  >;

  type LaxProperty<
    K extends keyof (Output | Input),
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = Listenable<Input, Output> &
    InitAndUpdateBlockable<Input, Output, CtxOptions> & {
      default:
        | TypeOf<Output[K]>
        | AsyncSetter<Output[K], Input, Output, CtxOptions>;
      validator?:
        | Validator<K, Input, Output, CtxOptions>
        | [
            Validator<K, Input, Output, CtxOptions>,
            SecondaryValidator<Output[K], Input, Output, CtxOptions>,
          ];
    };

  type ReadOnly<
    K extends keyof (Output | Input),
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = Listenable<Input, Output, CtxOptions> & {
    default:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>;
    readonly: 'lax';
    shouldUpdate?: Setter<boolean, Input, Output, CtxOptions>;
    validator:
      | Validator<K, Input, Output, CtxOptions>
      | [
          Validator<K, Input, Output, CtxOptions>,
          SecondaryValidator<Output[K], Input, Output, CtxOptions>,
        ];
  };

  type ReadonlyNoInit<
    K extends keyof (Output | Input),
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = Listenable<Input, Output, CtxOptions> & {
    default:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>;
    readonly: true;
    shouldInit: false | Setter<boolean, Input, Output, CtxOptions>;
    shouldUpdate?: Setter<boolean, Input, Output, CtxOptions>;
    validator?:
      | Validator<K, Input, Output, CtxOptions>
      | [
          Validator<K, Input, Output, CtxOptions>,
          SecondaryValidator<Output[K], Input, Output, CtxOptions>,
        ];
  };

  type RequiredReadonly<
    K extends keyof (Output | Input),
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = Listenable<Input, Output, CtxOptions> & {
    readonly: true;
  } & (
      | {
          validator:
            | Validator<K, Input, Output, CtxOptions>
            | [
                Validator<K, Input, Output, CtxOptions>,
                SecondaryValidator<Output[K], Input, Output, CtxOptions>,
              ];
        }
      | (Enumerable<Input[K]> & {
          validator?:
            | Validator<K, Input, Output, CtxOptions>
            | [
                Validator<K, Input, Output, CtxOptions>,
                SecondaryValidator<Output[K], Input, Output, CtxOptions>,
              ];
        })
    );

  type Required<
    K extends keyof (Output | Input),
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = Listenable<Input, Output, CtxOptions> & {
    required: true;
    shouldUpdate?: false | Setter<boolean, Input, Output, CtxOptions>;
  } & (
      | {
          validator:
            | Validator<K, Input, Output, CtxOptions>
            | [
                Validator<K, Input, Output, CtxOptions>,
                SecondaryValidator<Output[K], Input, Output, CtxOptions>,
              ];
        }
      | (Enumerable<Input[K]> & {
          validator?:
            | Validator<K, Input, Output, CtxOptions>
            | [
                Validator<K, Input, Output, CtxOptions>,
                SecondaryValidator<Output[K], Input, Output, CtxOptions>,
              ];
        })
    );

  type RequiredBy<
    K extends keyof (Output | Input),
    Input,
    Output,
    CtxOptions extends ObjectType,
  > = Listenable<Input, Output, CtxOptions> & {
    default:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>;
    required: RequiredHandler<Input, Output, CtxOptions>;
    readonly?: true;
    shouldInit?: Setter<boolean, Input, Output, CtxOptions>;
    shouldUpdate?: Setter<boolean, Input, Output, CtxOptions>;
    validator:
      | Validator<K, Input, Output, CtxOptions>
      | [
          Validator<K, Input, Output, CtxOptions>,
          SecondaryValidator<Output[K], Input, Output, CtxOptions>,
        ];
  };

  type Virtual<
    K extends keyof Input,
    Input,
    Output,
    Aliases,
    CtxOptions extends ObjectType,
  > = InitAndUpdateBlockable<Input, Output, CtxOptions> &
    Enumerable<Input[K]> & {
      alias?: Exclude<KeyOf<Aliases>, K> extends undefined
        ? string
        : Exclude<KeyOf<Aliases>, K>;
      required?: RequiredHandler<Input, Output, CtxOptions>;
      virtual: true;
      sanitizer?: VirtualResolver<K, Input, Output, CtxOptions>;
      onFailure?:
        | FailureHandler<Input, Output, CtxOptions>
        | ArrayOfMinSizeOne<FailureHandler<Input, Output, CtxOptions>>;
      onSuccess?:
        | SuccessHandler<Input, Output, CtxOptions>
        | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
      validator:
        | VirtualValidator<K, Input, Output, CtxOptions>
        | [
            VirtualValidator<K, Input, Output, CtxOptions>,
            SecondaryValidator<Input[K], Input, Output, CtxOptions>,
          ];
    };

  export type InternalOptions<
    Input,
    Output,
    CtxOptions extends ObjectType,
    ErrorTool extends IErrorTool<ObjectType> = never,
  > = {
    ErrorTool: ErrorToolClass<ErrorTool, CtxOptions>;
    equalityDepth: number;
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | ArrayOfMinSizeOne<SuccessHandler<Input, Output, CtxOptions>>;
    postValidate?:
      | PostValidationConfig<Input, Output, object, CtxOptions>
      | ArrayOfMinSizeOne<
          PostValidationConfig<Input, Output, object, CtxOptions>
        >;
    setMissingDefaultsOnUpdate?: boolean;
    shouldUpdate?: boolean | AsyncShouldUpdate<Input, Output, CtxOptions>;
    timestamps?:
      | boolean
      | { createdAt?: boolean | string; updatedAt?: boolean | string };
  };

  export type Options<
    Input,
    Output,
    Aliases,
    ErrorTool extends IErrorTool<ObjectType> = never,
    CtxOptions extends ObjectType = {},
  > = {
    ErrorTool?: ErrorToolClass<ErrorTool, CtxOptions>;
    equalityDepth?: number;
    onDelete?:
      | DeleteHandler<Output, CtxOptions>
      | ArrayOfMinSizeOne<DeleteHandler<Output, CtxOptions>>;
    onSuccess?: OnSuccessConfig<Input, Output, CtxOptions>;
    postValidate?:
      | PostValidationConfig<Input, Output, Aliases, CtxOptions>
      | ArrayOfMinSizeOne<
          PostValidationConfig<Input, Output, Aliases, CtxOptions>
        >;
    setMissingDefaultsOnUpdate?: boolean;
    shouldUpdate?: boolean | AsyncShouldUpdate<Input, Output, CtxOptions>;
    timestamps?:
      | boolean
      | {
          createdAt?: boolean | string;
          updatedAt?: boolean | string | { key?: string; nullable?: boolean };
        };
  };

  export type OptionsKey<
    Input,
    Output,
    Aliases,
    ErrorTool extends IErrorTool<never>,
  > = KeyOf<Options<Input, Output, Aliases, ErrorTool>>;

  export type PrivateOptions = { timestamps: Timestamp };

  export type Timestamp = { createdAt: string; updatedAt: string };

  export type ExtensionOptions<
    ParentInput,
    ParentOutput,
    Input,
    Output,
    Aliases,
    ErrorTool extends IErrorTool<ObjectType>,
    CtxOptions extends ObjectType = {},
  > = Options<Input, Output, Aliases, ErrorTool, CtxOptions> & {
    remove?:
      | KeyOf<Merge<ParentInput, ParentOutput>>
      | KeyOf<Merge<ParentInput, ParentOutput>>[];
    useParentOptions?: boolean;
  };
}

type ValidationResponse<T> =
  | { valid: true; validated: T }
  | { metadata: FieldError['metadata']; reason: string; valid: false };

type InvalidValidatorResponse = {
  metadata?: FieldError['metadata'];
  reason?: string;
  valid: false;
  value?: unknown;
};

type InternalValidatorResponse<T> =
  | { valid: true; validated: T }
  | InvalidValidatorResponse;

type ValidatorResponseObject<T> =
  | { valid: true; validated?: T }
  | InvalidValidatorResponse;

type ResponseErrorObject<Input = object, Aliases = object> = {
  [K in KeyOf<Input & Aliases>]?: string | InputFieldError;
};

type ValidatorResponse<T> = boolean | (ValidatorResponseObject<T> & {});

type Validator<
  K extends keyof (Output | Input),
  Input,
  Output,
  CtxOptions extends ObjectType = {},
> = (
  value: unknown,
  summary: MutableSummary<Input, Output, CtxOptions> & {},
) =>
  | ValidatorResponse<TypeOf<Output[K]>>
  | Promise<ValidatorResponse<TypeOf<Output[K]>>>;

type SecondaryValidator<
  T,
  Input,
  Output,
  CtxOptions extends ObjectType = {},
> = (
  value: T,
  summary: MutableSummary<Input, Output, CtxOptions> & {},
) => ValidatorResponse<T> | Promise<ValidatorResponse<T>>;

type VirtualValidator<
  K extends keyof Input,
  Input,
  Output,
  CtxOptions extends ObjectType = {},
> = (
  value: unknown,
  summary: MutableSummary<Input, Output, CtxOptions> & {},
) =>
  | ValidatorResponse<TypeOf<Input[K]>>
  | Promise<ValidatorResponse<TypeOf<Input[K]>>>;

type ArrayOfMinSizeOne<T> = [T, ...T[]];
type ArrayOfMinSizeTwo<T> = [T, T, ...T[]];

const DEFINITION_RULES = [
  'alias',
  'allow',
  'constant',
  'default',
  'dependsOn',
  'onDelete',
  'onFailure',
  'onSuccess',
  'readonly',
  'resolver',
  'required',
  'sanitizer',
  'shouldInit',
  'shouldUpdate',
  'validator',
  'value',
  'virtual',
] as const;

type DefinitionRule = (typeof DEFINITION_RULES)[number];

const ALLOWED_OPTIONS: NS.OptionsKey<unknown, unknown, unknown, never>[] = [
  'ErrorTool',
  'equalityDepth',
  'onDelete',
  'onSuccess',
  'postValidate',
  'setMissingDefaultsOnUpdate',
  'shouldUpdate',
  'timestamps',
];
const CONSTANT_RULES = ['constant', 'onDelete', 'onSuccess', 'value'];
const VIRTUAL_RULES = [
  'alias',
  'allow',
  'sanitizer',
  'onFailure',
  'onSuccess',
  'required',
  'shouldInit',
  'shouldUpdate',
  'validator',
  'virtual',
];

const LIFE_CYCLES = ['onDelete', 'onFailure', 'onSuccess'] as const;

interface ErrorToolClass<ErrorTool, CtxOptions extends ObjectType> {
  new (message: ValidationErrorMessage, ctxOptions: CtxOptions): ErrorTool;
}

type TypeFromPromise<T> = T extends Promise<infer I> ? I : T;

type RealType_<T> = T extends (...args: never) => infer I ? I : T;

type PrettyType<T> = {
  [K in keyof T]: T[K];
} & {};

type RealType<T> = {
  [K in keyof T]: TypeFromPromise<Exclude<T[K], Function> | RealType_<T[K]>>;
} & {};

type Merge<A, B> = {
  [K in keyof A | keyof B]: Exclude<
    K extends keyof A
      ? K extends keyof B
        ? A[K] | B[K]
        : A[K]
      : K extends keyof B
        ? B[K]
        : never,
    undefined
  >;
};

type Without<T, U> = { [P in Exclude<keyof T, keyof U>]?: never };

type XOR<T, U> = (T | U extends object
  ? (Without<T, U> & U) | ((Without<U, T> & T) & {})
  : T | U) & {};
