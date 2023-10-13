import { FieldKey, ObjectType } from '../utils';
import {
  FieldError,
  IErrorTool,
  InputPayload,
  ValidationErrorMessage
} from './utils';

export type {
  Context,
  DefinitionRule,
  KeyOf,
  Merge,
  NS,
  NonEmptyArray,
  PartialContext,
  Summary,
  RealType,
  TypeOf,
  Validator,
  ValidationResponse,
  ValidatorResponse,
  ValidatorResponseObject,
  InternalValidatorResponse,
  InvalidValidatorResponse,
  XOR
};

export {
  ALLOWED_OPTIONS,
  CONSTANT_RULES,
  DEFINITION_RULES,
  LIFE_CYCLES,
  OPERATIONS,
  VIRTUAL_RULES
};

type Context<
  Input,
  Output = Input,
  CtxOptions extends ObjectType = {}
> = Readonly<
  Merge<Input, Output> & { __getOptions__: () => Readonly<CtxOptions> }
> & {};

type DeleteContext<Output, CtxOptions extends ObjectType = {}> = Readonly<
  Output & { __getOptions__: () => Readonly<CtxOptions> }
> & {};

type PartialContext<Input, Output> = Readonly<Merge<Input, Output>>;

type Summary<Input, Output = Input, CtxOptions extends ObjectType = {}> = (
  | Readonly<{
      changes: null;
      context: Context<Input, Output, CtxOptions>;
      operation: 'creation';
      previousValues: null;
      values: Readonly<Output>;
    }>
  | Readonly<{
      changes: Partial<RealType<Output>>;
      context: Context<Input, Output, CtxOptions>;
      operation: 'update';
      previousValues: Readonly<Output>;
      values: Readonly<Output>;
    }>
) & {};

type TypeOf<T> = Exclude<T, undefined>;

type AsyncSetter<T, Input, Output, CtxOptions extends ObjectType = {}> = (
  context: Context<Input, Output, CtxOptions>
) => TypeOf<T> | Promise<TypeOf<T>>;

type Setter<T, Input, Output, CtxOptions extends ObjectType = {}> = (
  context: Context<Input, Output, CtxOptions>
) => TypeOf<T>;

type SetterWithSummary<T, Input, Output, CtxOptions extends ObjectType = {}> = (
  summary: Summary<Input, Output, CtxOptions> & {}
) => TypeOf<T>;

type AsyncShouldUpdateResponse<CtxOptions extends ObjectType = {}> = {
  update: boolean;
  ctxOptionsUpdate?: Partial<CtxOptions>;
};

type AsyncShouldUpdate<Input, Output, CtxOptions extends ObjectType = {}> = (
  summary: Summary<Input, Output, CtxOptions> & {}
) =>
  | boolean
  | AsyncShouldUpdateResponse
  | Promise<boolean>
  | Promise<AsyncShouldUpdateResponse>;

type Resolver<
  K extends keyof Output,
  Input,
  Output,
  CtxOptions extends ObjectType = {}
> = (
  summary: Summary<Input, Output, CtxOptions> & {}
) => TypeOf<Output[K]> | Promise<TypeOf<Output[K]>>;

type VirtualResolver<
  K extends keyof Input,
  Input,
  Output,
  CtxOptions extends ObjectType = {}
> = (
  summary: Summary<Input, Output, CtxOptions> & {}
) => TypeOf<Input[K]> | Promise<TypeOf<Input[K]>>;

type KeyOf<T> = Extract<keyof T, string>;

namespace NS {
  export type LifeCycle = (typeof LIFE_CYCLES)[number];

  export type Operation = (typeof OPERATIONS)[number];

  export type Handler<Output, CtxOptions extends ObjectType> = (
    data: DeleteContext<Output, CtxOptions>
  ) => any | Promise<any>;

  export type FailureHandler<
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = (context: Context<Input, Output, CtxOptions> & {}) => any | Promise<any>;

  export type SuccessHandler<
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = (summary: Summary<Input, Output, CtxOptions> & {}) => any | Promise<any>;

  export type Definitions<
    Input,
    Output,
    Aliases = {},
    CtxOptions extends ObjectType = {}
  > = {
    [K in keyof (Input & Output)]?: K extends keyof (Input | Output)
      ? PublicProperty<K, Input, Output, CtxOptions>
      : K extends keyof Omit<Output, keyof Input>
      ? PrivateProperty<K, Input, Output, CtxOptions>
      : K extends keyof Omit<Input, keyof Output>
      ? Virtual<K, Output, Input, Aliases, CtxOptions>
      : never;
  };

  type PublicProperty<
    K extends keyof (Output | Input),
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > =
    | LaxProperty<K, Input, Output, CtxOptions>
    | ReadOnly<K, Input, Output, CtxOptions>
    | ReadonlyNoInit<K, Input, Output, CtxOptions>
    | Required<K, Input, Output, CtxOptions>
    | RequiredBy<K, Input, Output, CtxOptions>
    | RequiredReadonly<K, Input, Output, CtxOptions>;

  type PrivateProperty<
    K extends keyof Output,
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = XOR<
    Constant<K, Input, Output, CtxOptions>,
    Dependent<K, Input, Output, CtxOptions>
  >;

  export type Definitions_<Input, Output> = {
    [K in keyof Input]?: Listenable<Input, Output> & {
      alias?: string;
      constant?: any;
      default?: any;
      dependent?: boolean;
      dependsOn?: KeyOf<Input> | KeyOf<Input>[];
      readonly?: boolean | 'lax';
      resolver?: Function;
      required?:
        | boolean
        | SetterWithSummary<boolean | [boolean, string], Input, Output>;
      sanitizer?: VirtualResolver<K, Input, Output>;
      shouldInit?: false | Setter<boolean, Input, Output>;
      shouldUpdate?: false | Setter<boolean, Input, Output>;
      validator?: Function;
      value?: any;
      virtual?: boolean;
    };
  };

  export type AliasToVirtualMap<T> = Record<string, KeyOf<T>>;

  export type VirtualToAliasMap<T> = Record<KeyOf<T>, string>;

  export type DependencyMap<T> = {
    [K in KeyOf<T>]?: KeyOf<T>[];
  };

  type Listenable<Input, Output, CtxOptions extends ObjectType = {}> = {
    onDelete?:
      | Handler<Output, CtxOptions>
      | NonEmptyArray<Handler<Output, CtxOptions>>;
    onFailure?:
      | FailureHandler<Input, Output, CtxOptions>
      | NonEmptyArray<FailureHandler<Input, Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | NonEmptyArray<SuccessHandler<Input, Output, CtxOptions>>;
  };

  type Constant<
    K extends keyof Output,
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = {
    constant: true;
    onDelete?:
      | Handler<Output, CtxOptions>
      | NonEmptyArray<Handler<Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | NonEmptyArray<SuccessHandler<Input, Output, CtxOptions>>;
    value:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>;
  };

  type Dependent<
    K extends keyof Output,
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = {
    default: TypeOf<Output[K]> | AsyncSetter<Output[K], Input, Output>;
    /** @deprecated `dependsOn` & a `resolver` function are enough to make a property dependent */
    dependent?: true;
    dependsOn:
      | Exclude<KeyOf<Context<Input, Output, CtxOptions>>, K>
      | Exclude<KeyOf<Context<Input, Output, CtxOptions>>, K>[];
    onDelete?:
      | Handler<Output, CtxOptions>
      | NonEmptyArray<Handler<Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | NonEmptyArray<SuccessHandler<Input, Output, CtxOptions>>;
    readonly?: true;
    resolver: Resolver<K, Input, Output, CtxOptions>;
  };

  type LaxProperty<
    K extends keyof (Output | Input),
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = Listenable<Input, Output> & {
    default:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>;
    readonly?: 'lax';
    shouldInit?: false | Setter<boolean, Input, Output, CtxOptions>;
    shouldUpdate?: Setter<boolean, Input, Output, CtxOptions>;
    validator?: Validator<K, Input, Output, CtxOptions>;
  };

  type ReadOnly<
    K extends keyof (Output | Input),
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = Listenable<Input, Output, CtxOptions> & {
    default:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>;
    readonly: 'lax';
    shouldUpdate?: Setter<boolean, Input, Output, CtxOptions>;
    validator: Validator<K, Input, Output, CtxOptions>;
  };

  type ReadonlyNoInit<
    K extends keyof (Output | Input),
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = Listenable<Input, Output, CtxOptions> & {
    default:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>;
    readonly: true;
    shouldInit: false | Setter<boolean, Input, Output, CtxOptions>;
    shouldUpdate?: Setter<boolean, Input, Output, CtxOptions>;
    validator?: Validator<K, Input, Output, CtxOptions>;
  };

  type RequiredReadonly<
    K extends keyof (Output | Input),
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = Listenable<Input, Output, CtxOptions> & {
    readonly: true;
    validator: Validator<K, Input, Output, CtxOptions>;
  };

  type Required<
    K extends keyof (Output | Input),
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = Listenable<Input, Output, CtxOptions> & {
    required: true;
    shouldUpdate?: Setter<boolean, Input, Output, CtxOptions>;
    validator: Validator<K, Input, Output, CtxOptions>;
  };

  type RequiredBy<
    K extends keyof (Output | Input),
    Input,
    Output,
    CtxOptions extends ObjectType = {}
  > = Listenable<Input, Output, CtxOptions> & {
    default:
      | TypeOf<Output[K]>
      | AsyncSetter<Output[K], Input, Output, CtxOptions>;
    required: SetterWithSummary<
      boolean | [boolean, string],
      Input,
      Output,
      CtxOptions
    >;
    readonly?: true;
    shouldInit?: Setter<boolean, Input, Output, CtxOptions>;
    shouldUpdate?: Setter<boolean, Input, Output, CtxOptions>;
    validator: Validator<K, Input, Output, CtxOptions>;
  };

  type Virtual<
    K extends keyof Input,
    Output,
    Input,
    Aliases,
    CtxOptions extends ObjectType = {}
  > = {
    alias?: Exclude<KeyOf<Aliases>, K> extends undefined
      ? string
      : Exclude<KeyOf<Aliases>, K>;
    required?: SetterWithSummary<
      boolean | [boolean, string],
      Input,
      Output,
      CtxOptions
    >;
    virtual: true;
    sanitizer?: VirtualResolver<K, Input, Output, CtxOptions>;
    onFailure?:
      | FailureHandler<Input, Output, CtxOptions>
      | NonEmptyArray<FailureHandler<Input, Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | NonEmptyArray<SuccessHandler<Input, Output, CtxOptions>>;
    shouldInit?: false | Setter<boolean, Input, Output, CtxOptions>;
    shouldUpdate?: false | Setter<boolean, Input, Output, CtxOptions>;
    validator: Validator<K, Input, Output, CtxOptions>;
  };

  // options
  export type CloneOptions<T, CtxOptions> = {
    reset?: KeyOf<T> | KeyOf<T>[];
    contextOptions?: Partial<CtxOptions>;
  };

  export type InternalOptions<
    Input,
    Output,
    ErrorTool extends IErrorTool<any>,
    CtxOptions extends ObjectType = {}
  > = {
    ErrorTool: ErrorToolClass<ErrorTool, CtxOptions>;
    equalityDepth: number;
    errors: 'silent' | 'throw';
    onDelete?:
      | Handler<Output, CtxOptions>
      | NonEmptyArray<Handler<Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | NonEmptyArray<SuccessHandler<Input, Output, CtxOptions>>;
    setMissingDefaultsOnUpdate?: boolean;
    shouldUpdate?: boolean | AsyncShouldUpdate<Input, Output, CtxOptions>;
    timestamps?:
      | boolean
      | { createdAt?: boolean | string; updatedAt?: boolean | string };
  };

  export type Options<
    Input,
    Output,
    ErrorTool extends IErrorTool<any>,
    CtxOptions extends ObjectType = {}
  > = {
    ErrorTool?: ErrorToolClass<ErrorTool, CtxOptions>;
    equalityDepth?: number;
    errors?: 'silent' | 'throw';
    onDelete?:
      | Handler<Output, CtxOptions>
      | NonEmptyArray<Handler<Output, CtxOptions>>;
    onSuccess?:
      | SuccessHandler<Input, Output, CtxOptions>
      | NonEmptyArray<SuccessHandler<Input, Output, CtxOptions>>;
    setMissingDefaultsOnUpdate?: boolean;
    shouldUpdate?: boolean | AsyncShouldUpdate<Input, Output, CtxOptions>;
    timestamps?:
      | boolean
      | { createdAt?: boolean | string; updatedAt?: boolean | string };
  };

  export type OptionsKey<
    Input,
    Output,
    ErrorTool extends IErrorTool<any>
  > = KeyOf<Options<Input, Output, ErrorTool>>;

  export type PrivateOptions = { timestamps: Timestamp };

  export type Timestamp = { createdAt: string; updatedAt: string };

  export type ExtensionOptions<
    ParentInput,
    ParentOutput,
    Input,
    Output,
    ErrorTool extends IErrorTool<any>,
    CtxOptions extends ObjectType = {}
  > = Options<Input, Output, ErrorTool, CtxOptions> & {
    remove?:
      | KeyOf<Merge<ParentInput, ParentOutput>>
      | KeyOf<Merge<ParentInput, ParentOutput>>[];
    useParentOptions?: boolean;
  };
}

type ValidationResponse<T> =
  | { valid: true; validated: T }
  | { metadata: FieldError['metadata']; reasons: string[]; valid: false };

type InternalValidatorResponse<T> =
  | { valid: true; validated: T }
  | InvalidValidatorResponse;

type InvalidValidatorResponse = {
  metadata?: FieldError['metadata'];
  otherReasons?: InputPayload;
  reasons: FieldError['reasons'];
  valid: false;
  value: any;
};

type ValidatorResponseObject<T> =
  | { valid: true; validated?: T }
  | {
      otherReasons?: Record<FieldKey, string | string[] | FieldError>;
      metadata?: FieldError['metadata'];
      reason?: FieldError['reasons'][number];
      reasons?: FieldError['reasons'];
      valid: false;
      value?: any;
    };

type ValidatorResponse<T> = boolean | (ValidatorResponseObject<T> & {});

type Validator<
  K extends keyof Input,
  Input,
  Output,
  CtxOptions extends ObjectType = {}
> = (
  value: any,
  summary: Summary<Input, Output, CtxOptions> & {}
) =>
  | ValidatorResponse<TypeOf<Input[K]>>
  | Promise<ValidatorResponse<TypeOf<Input[K]>>>;

type NonEmptyArray<T> = [T, ...T[]];

const DEFINITION_RULES = [
  'alias',
  'constant',
  'default',
  'dependent',
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
  'virtual'
] as const;

type DefinitionRule = (typeof DEFINITION_RULES)[number];

const ALLOWED_OPTIONS: NS.OptionsKey<any, any, any>[] = [
  'ErrorTool',
  'equalityDepth',
  'errors',
  'onDelete',
  'onSuccess',
  'setMissingDefaultsOnUpdate',
  'shouldUpdate',
  'timestamps'
];
const CONSTANT_RULES = ['constant', 'onDelete', 'onSuccess', 'value'];
const VIRTUAL_RULES = [
  'alias',
  'sanitizer',
  'onFailure',
  'onSuccess',
  'required',
  'shouldInit',
  'shouldUpdate',
  'validator',
  'virtual'
];

const LIFE_CYCLES = ['onDelete', 'onFailure', 'onSuccess'] as const;
const OPERATIONS = ['creation', 'update'] as const;

interface ErrorToolClass<ErrorTool, CtxOptions extends ObjectType> {
  new (message: ValidationErrorMessage, ctxOptions: CtxOptions): ErrorTool;
}

type TypeFromPromise<T> = T extends Promise<infer I> ? I : T;

type RealType_<T> = T extends (...args: any) => infer I ? I : T;

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
