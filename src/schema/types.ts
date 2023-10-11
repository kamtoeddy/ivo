import { Schema } from '.';
import { FieldKey } from '../utils';
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
  NonEmptyArray,
  Summary,
  Schema as ISchema,
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

type Context<Input, Output = Input> = Readonly<Merge<Input, Output>>;

type Summary<Input, Output = Input> = (
  | Readonly<{
      changes: null;
      context: Context<Input, Output>;
      operation: 'creation';
      previousValues: null;
      values: Readonly<Output>;
    }>
  | Readonly<{
      changes: Partial<RealType<Output>>;
      context: Context<Input, Output>;
      operation: 'update';
      previousValues: Readonly<Output>;
      values: Readonly<Output>;
    }>
) & {};

type TypeOf<T> = Exclude<T, undefined>;

type AsyncSetter<T, Input, Output> = (
  context: Context<Input, Output> & {}
) => TypeOf<T> | Promise<TypeOf<T>>;

type Setter<T, Input, Output> = (
  context: Context<Input, Output> & {}
) => TypeOf<T>;

type SetterWithSummary<T, Input, Output> = (
  summary: Summary<Input, Output> & {}
) => TypeOf<T>;

type Resolver<K extends keyof Output, Input, Output> = (
  summary: Summary<Input, Output> & {}
) => TypeOf<Output[K]> | Promise<TypeOf<Output[K]>>;

type VirtualResolver<K extends keyof Input, Input, Output> = (
  summary: Summary<Input, Output> & {}
) => TypeOf<Input[K]> | Promise<TypeOf<Input[K]>>;

type KeyOf<T> = Extract<keyof T, string>;

namespace Schema {
  export type LifeCycle = (typeof LIFE_CYCLES)[number];

  export type OperationName = (typeof OPERATIONS)[number];

  export type Handler<Output> = (data: Readonly<Output>) => any | Promise<any>;

  export type FailureHandler<Input, Output> = (
    context: Context<Input, Output> & {}
  ) => any | Promise<any>;

  export type SuccessHandler<Input, Output> = (
    summary: Summary<Input, Output> & {}
  ) => any | Promise<any>;

  export type Definitions<Input, Output, Aliases = {}> = {
    [K in keyof (Input & Output)]?: K extends keyof (Input | Output)
      ? PublicProperty<K, Input, Output>
      : K extends keyof Omit<Output, keyof Input>
      ? PrivateProperty<K, Input, Output>
      : K extends keyof Omit<Input, keyof Output>
      ? Virtual<K, Output, Input, Aliases>
      : never;
  };

  type PublicProperty<K extends keyof (Output | Input), Input, Output> =
    | LaxProperty<K, Input, Output>
    | ReadOnly<K, Input, Output>
    | ReadonlyNoInit<K, Input, Output>
    | Required<K, Input, Output>
    | RequiredBy<K, Input, Output>
    | RequiredReadonly<K, Input, Output>;

  type PrivateProperty<K extends keyof Output, Input, Output> = XOR<
    Constant<K, Input, Output>,
    Dependent<K, Input, Output>
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

  type Listenable<Input, Output> = {
    onDelete?: Handler<Output> | NonEmptyArray<Handler<Output>>;
    onFailure?:
      | FailureHandler<Input, Output>
      | NonEmptyArray<FailureHandler<Input, Output>>;
    onSuccess?:
      | SuccessHandler<Input, Output>
      | NonEmptyArray<SuccessHandler<Input, Output>>;
  };

  type Constant<K extends keyof Output, Input, Output> = {
    constant: true;
    onDelete?: Handler<Output> | NonEmptyArray<Handler<Output>>;
    onSuccess?:
      | SuccessHandler<Input, Output>
      | NonEmptyArray<SuccessHandler<Input, Output>>;
    value: TypeOf<Output[K]> | AsyncSetter<Output[K], Input, Output>;
  };

  type Dependent<K extends keyof Output, Input, Output> = {
    default: TypeOf<Output[K]> | AsyncSetter<Output[K], Input, Output>;
    /** @deprecated `dependsOn` & a `resolver` function are enough to make a property dependent */
    dependent?: true;
    dependsOn:
      | Exclude<KeyOf<Context<Input, Output>>, K>
      | Exclude<KeyOf<Context<Input, Output>>, K>[];
    onDelete?: Handler<Output> | NonEmptyArray<Handler<Output>>;
    onSuccess?:
      | SuccessHandler<Input, Output>
      | NonEmptyArray<SuccessHandler<Input, Output>>;
    readonly?: true;
    resolver: Resolver<K, Input, Output>;
  };

  type LaxProperty<
    K extends keyof (Output | Input),
    Input,
    Output
  > = Listenable<Input, Output> & {
    default: TypeOf<Output[K]> | AsyncSetter<Output[K], Input, Output>;
    readonly?: 'lax';
    shouldInit?: false | Setter<boolean, Input, Output>;
    shouldUpdate?: Setter<boolean, Input, Output>;
    validator?: Validator<K, Input, Output>;
  };

  type ReadOnly<K extends keyof (Output | Input), Input, Output> = Listenable<
    Input,
    Output
  > & {
    default: TypeOf<Output[K]> | AsyncSetter<Output[K], Input, Output>;
    readonly: 'lax';
    shouldUpdate?: Setter<boolean, Input, Output>;
    validator: Validator<K, Input, Output>;
  };

  type ReadonlyNoInit<
    K extends keyof (Output | Input),
    Input,
    Output
  > = Listenable<Input, Output> & {
    default: TypeOf<Output[K]> | AsyncSetter<Output[K], Input, Output>;
    readonly: true;
    shouldInit: false | Setter<boolean, Input, Output>;
    shouldUpdate?: Setter<boolean, Input, Output>;
    validator?: Validator<K, Input, Output>;
  };

  type RequiredReadonly<
    K extends keyof (Output | Input),
    Input,
    Output
  > = Listenable<Input, Output> & {
    readonly: true;
    validator: Validator<K, Input, Output>;
  };

  type Required<K extends keyof (Output | Input), Input, Output> = Listenable<
    Input,
    Output
  > & {
    required: true;
    shouldUpdate?: Setter<boolean, Input, Output>;
    validator: Validator<K, Input, Output>;
  };

  type RequiredBy<K extends keyof (Output | Input), Input, Output> = Listenable<
    Input,
    Output
  > & {
    default: TypeOf<Output[K]> | AsyncSetter<Output[K], Input, Output>;
    required: SetterWithSummary<boolean | [boolean, string], Input, Output>;
    readonly?: true;
    shouldInit?: Setter<boolean, Input, Output>;
    shouldUpdate?: Setter<boolean, Input, Output>;
    validator: Validator<K, Input, Output>;
  };

  type Virtual<K extends keyof Input, Output, Input, A> = {
    alias?: Exclude<KeyOf<A>, K> extends undefined
      ? string
      : Exclude<KeyOf<A>, K>;
    required?: SetterWithSummary<boolean | [boolean, string], Input, Output>;
    virtual: true;
    sanitizer?: VirtualResolver<K, Input, Output>;
    onFailure?:
      | FailureHandler<Input, Output>
      | NonEmptyArray<FailureHandler<Input, Output>>;
    onSuccess?:
      | SuccessHandler<Input, Output>
      | NonEmptyArray<SuccessHandler<Input, Output>>;
    shouldInit?: false | Setter<boolean, Input, Output>;
    shouldUpdate?: false | Setter<boolean, Input, Output>;
    validator: Validator<K, Input, Output>;
  };

  // options
  export type CloneOptions<T> = {
    reset?: KeyOf<T> | KeyOf<T>[];
  };

  export type InternalOptions<
    Input,
    Output,
    ErrorTool extends IErrorTool<any>
  > = {
    ErrorTool: Constructable<ErrorTool>;
    equalityDepth: number;
    errors: 'silent' | 'throw';
    onDelete?: Handler<Output> | NonEmptyArray<Handler<Output>>;
    onSuccess?:
      | SuccessHandler<Input, Output>
      | NonEmptyArray<SuccessHandler<Input, Output>>;
    setMissingDefaultsOnUpdate?: boolean;
    shouldUpdate?: boolean | SetterWithSummary<boolean, Input, Output>;
    timestamps?:
      | boolean
      | { createdAt?: boolean | string; updatedAt?: boolean | string };
  };

  export type Options<Input, Output, ErrorTool extends IErrorTool<any>> = {
    ErrorTool?: Constructable<ErrorTool>;
    equalityDepth?: number;
    errors?: 'silent' | 'throw';
    onDelete?: Handler<Output> | NonEmptyArray<Handler<Output>>;
    onSuccess?:
      | SuccessHandler<Input, Output>
      | NonEmptyArray<SuccessHandler<Input, Output>>;
    setMissingDefaultsOnUpdate?: boolean;
    shouldUpdate?: boolean | SetterWithSummary<boolean, Input, Output>;
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
    ValidationError extends IErrorTool<any>
  > = Options<Input, Output, ValidationError> & {
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

type Validator<K extends keyof Input, Input, Output> = (
  value: any,
  summary: Summary<Input, Output> & {}
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

const ALLOWED_OPTIONS: Schema.OptionsKey<any, any, any>[] = [
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

type TypeFromPromise<T> = T extends Promise<infer I> ? I : T;

interface Constructable<T> {
  new (message: ValidationErrorMessage): T;
}

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
