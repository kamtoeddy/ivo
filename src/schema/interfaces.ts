import { Merge, RealType } from "./merge-types";

export type {
  Context,
  DefinitionRule,
  Summary,
  Schema as ISchema,
  NonEmptyArray,
  RealType,
  ResponseInput,
  ResponseInput_,
  StringKey,
  TypeOf,
  Validator,
  ValidatorResponse,
};

type Context<I, O = I> = Readonly<Merge<I, O>>;

type Summary<I, O = I> = (
  | Readonly<{
      context: Context<I, O>;
      operation: "creation";
      previousValues: undefined;
      values: Readonly<O>;
    }>
  | Readonly<{
      context: Context<I, O>;
      operation: "update";
      previousValues: Readonly<O>;
      values: Readonly<O>;
    }>
) & {};

type TypeOf<T> = Exclude<T, undefined>;

type AsyncSetter<K extends keyof (I & O), I, O> = (
  context: Context<I, O>,
  operation: Schema.OperationName
) => TypeOf<(I & O)[K]> | Promise<TypeOf<(I & O)[K]>>;

type BooleanSetter<I, O> = (context: Context<I, O>) => boolean;

type ConditionalRequiredSetter<I, O> = (
  summary: Summary<I, O> & {}
) => boolean | [boolean, string];

type StringKey<T> = Extract<keyof T, string>;

namespace Schema {
  export type LifeCycles = "onDelete" | "onFailure" | "onSuccess";

  export type OperationName = "creation" | "update";

  export type Listener<I, O> = (context: Context<I, O>) => any | Promise<any>;

  export type SuccessListener<I, O> = (
    summary: Summary<I, O> & {}
  ) => any | Promise<any>;

  export type Definitions<I, O = I, A = {}> = {
    [K in keyof (I & O)]?: Property<K, I, O, A>;
  };

  export type Definitions_<I, O> = {
    [K in keyof I]?: Listenable<I, O> & {
      alias?: string;
      constant?: any;
      default?: any;
      dependent?: boolean;
      dependsOn?: StringKey<I> | StringKey<I>[];
      readonly?: boolean | "lax";
      resolver?: Function;
      required?: boolean | ConditionalRequiredSetter<I, O>;
      sanitizer?: AsyncSetter<K, I, O>;
      shouldInit?: false | BooleanSetter<I, O>;
      shouldUpdate?: false | BooleanSetter<I, O>;
      validator?: Function;
      value?: any;
      virtual?: boolean;
    };
  };

  export type AliasToVirtualMap<T> = Record<string, StringKey<T>>;
  export type VirtualToAliasMap<T> = Record<StringKey<T>, string>;

  export type DependencyMap<T> = {
    [K in StringKey<T>]?: StringKey<T>[];
  };

  type Property<K extends keyof (I & O), I, O, A> =
    | Constant<K, I, O>
    | Dependent<K, I, O>
    | LaxProperty<K, I, O>
    | Readonly_<K, I, O>
    | ReadonlyNoInit<K, I, O>
    | Required<K, I, O>
    | RequiredBy<K, I, O>
    | ReadonlyRequired<K, I, O>
    | Virtual<K, I, O, A>;

  type Listenable<I, O> = {
    onDelete?: Listener<I, O> | NonEmptyArray<Listener<I, O>>;
    onFailure?: Listener<I, O> | NonEmptyArray<Listener<I, O>>;
    onSuccess?: SuccessListener<I, O> | NonEmptyArray<SuccessListener<I, O>>;
  };

  type Constant<K extends keyof (I & O), I, O = I> = {
    constant: true;
    onDelete?: Listener<I, O> | NonEmptyArray<Listener<I, O>>;
    onSuccess?: SuccessListener<I, O> | NonEmptyArray<SuccessListener<I, O>>;
    value: TypeOf<(I & O)[K]> | AsyncSetter<K, I, O>;
  };

  type Dependent<K extends keyof (I & O), I, O = I> = Listenable<I, O> & {
    default: TypeOf<(I & O)[K]> | AsyncSetter<K, I, O>;
    dependent: true;
    dependsOn: Exclude<StringKey<I>, K> | Exclude<StringKey<I>, K>[];
    readonly?: true;
    resolver: AsyncSetter<K, I, O>;
  };

  type LaxProperty<K extends keyof (I & O), I, O = I> = Listenable<I, O> & {
    default: TypeOf<(I & O)[K]> | AsyncSetter<K, I, O>;
    readonly?: "lax";
    shouldInit?: false | BooleanSetter<I, O>;
    shouldUpdate?: BooleanSetter<I, O>;
    validator?: Validator<K, I, O>;
  };

  type Readonly_<K extends keyof (I & O), I, O = I> = Listenable<I, O> & {
    default: TypeOf<(I & O)[K]> | AsyncSetter<K, I, O>;
    readonly: "lax";
    shouldUpdate?: BooleanSetter<I, O>;
    validator: Validator<K, I, O>;
  };

  type ReadonlyNoInit<K extends keyof (I & O), I, O = I> = Listenable<I, O> & {
    default: TypeOf<(I & O)[K]> | AsyncSetter<K, I, O>;
    readonly: true;
    shouldInit: false | BooleanSetter<I, O>;
    shouldUpdate?: BooleanSetter<I, O>;
    validator?: Validator<K, I, O>;
  };

  type ReadonlyRequired<K extends keyof (I & O), I, O = I> = Listenable<
    I,
    O
  > & {
    readonly: true;
    validator: Validator<K, I, O>;
  };

  type Required<K extends keyof (I & O), I, O = I> = Listenable<I, O> & {
    required: true;
    shouldUpdate?: BooleanSetter<I, O>;
    validator: Validator<K, I, O>;
  };

  type RequiredBy<K extends keyof (I & O), I, O = I> = Listenable<I, O> & {
    default: TypeOf<(I & O)[K]> | AsyncSetter<K, I, O>;
    required: ConditionalRequiredSetter<I, O>;
    readonly?: true;
    shouldUpdate?: BooleanSetter<I, O>;
    validator: Validator<K, I, O>;
  };

  type Virtual<K extends keyof (I & O), I, O, A> = {
    alias?: Exclude<StringKey<A>, K> extends undefined
      ? string
      : Exclude<StringKey<A>, K>;
    required?: ConditionalRequiredSetter<I, O>;
    virtual: true;
    sanitizer?: AsyncSetter<K, I, O>;
    onFailure?: Listener<I, O> | NonEmptyArray<Listener<I, O>>;
    onSuccess?: SuccessListener<I, O> | NonEmptyArray<SuccessListener<I, O>>;
    shouldInit?: false | BooleanSetter<I, O>;
    shouldUpdate?: false | BooleanSetter<I, O>;
    validator: Validator<K, I, O>;
  };

  // options
  export interface CloneOptions<T> {
    reset?: StringKey<T> | StringKey<T>[];
  }

  export type Options<I, O> = {
    errors?: "silent" | "throw";
    onSuccess?: SuccessListener<I, O> | NonEmptyArray<SuccessListener<I, O>>;
    timestamps?:
      | boolean
      | {
          createdAt?: boolean | string;
          updatedAt?: boolean | string;
        };
  };

  export type OptionsKey<I, O> = StringKey<Options<I, O>>;

  export interface PrivateOptions {
    timestamps: Timestamp;
  }

  export interface Timestamp {
    createdAt: string;
    updatedAt: string;
  }

  export type ExtensionOptions<ParentInput, I, O> = Options<I, O> & {
    remove?: StringKey<ParentInput> | StringKey<ParentInput>[];
  };
}

type ValidatorResponse<T> =
  | { valid: true; validated: T }
  | { reasons: string[]; valid: false };

type ResponseInput_<T> =
  | { valid: true; validated?: TypeOf<T> }
  | { reason?: string; reasons?: string[]; valid: false };

type ResponseInput<T> = boolean | (ResponseInput_<T> & {});

type Validator<K extends keyof (I & O), I, O> = (
  value: any,
  summary: Summary<I, O> & {}
) => ResponseInput<(I & O)[K]> | Promise<ResponseInput<(I & O)[K]>>;

type NonEmptyArray<T> = [T, ...T[]];

const DEFINITION_RULES = [
  "alias",
  "constant",
  "default",
  "dependent",
  "dependsOn",
  "onDelete",
  "onFailure",
  "onSuccess",
  "readonly",
  "resolver",
  "required",
  "sanitizer",
  "shouldInit",
  "shouldUpdate",
  "validator",
  "value",
  "virtual",
] as const;

type DefinitionRule = typeof DEFINITION_RULES[number];

const ALLOWED_OPTIONS: Schema.OptionsKey<any, any>[] = [
  "errors",
  "onSuccess",
  "timestamps",
];
const CONSTANT_RULES = ["constant", "onDelete", "onSuccess", "value"];
const VIRTUAL_RULES = [
  "alias",
  "sanitizer",
  "onFailure",
  "onSuccess",
  "required",
  "shouldInit",
  "shouldUpdate",
  "validator",
  "virtual",
];

export { ALLOWED_OPTIONS, CONSTANT_RULES, DEFINITION_RULES, VIRTUAL_RULES };
