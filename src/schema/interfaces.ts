import { InputPayload } from "../utils/interfaces";
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
  InternalValidatorResponse,
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

type BooleanSetter<I, O> = (context: Context<I, O>) => boolean;

type DefaultSetter<K extends keyof (I & O), I, O> = (
  context: Context<I, O>
) => TypeOf<(I & O)[K]> | Promise<TypeOf<(I & O)[K]>>;

type RequiredSetter<I, O> = (
  summary: Summary<I, O> & {}
) => boolean | [boolean, string];

type Resolver<K extends keyof (I & O), I, O> = (
  summary: Summary<I, O> & {}
) => TypeOf<(I & O)[K]> | Promise<TypeOf<(I & O)[K]>>;

type StringKey<T> = Extract<keyof T, string>;

namespace Schema {
  export type LifeCycles = "onDelete" | "onFailure" | "onSuccess";

  export type OperationName = "creation" | "update";

  export type Handler<O> = (data: Readonly<O>) => any | Promise<any>;

  export type FailureHandler<I, O> = (
    context: Context<I, O>
  ) => any | Promise<any>;

  export type SuccessHandler<I, O> = (
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
      required?: boolean | RequiredSetter<I, O>;
      sanitizer?: DefaultSetter<K, I, O>;
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
    onDelete?: Handler<O> | NonEmptyArray<Handler<O>>;
    onFailure?: FailureHandler<I, O> | NonEmptyArray<FailureHandler<I, O>>;
    onSuccess?: SuccessHandler<I, O> | NonEmptyArray<SuccessHandler<I, O>>;
  };

  type Constant<K extends keyof (I & O), I, O = I> = {
    constant: true;
    onDelete?: Handler<O> | NonEmptyArray<Handler<O>>;
    onSuccess?: SuccessHandler<I, O> | NonEmptyArray<SuccessHandler<I, O>>;
    value: TypeOf<(I & O)[K]> | DefaultSetter<K, I, O>;
  };

  type Dependent<K extends keyof (I & O), I, O = I> = {
    default: TypeOf<(I & O)[K]> | DefaultSetter<K, I, O>;
    dependent: true;
    dependsOn:
      | Exclude<StringKey<Context<I, O>>, K>
      | Exclude<StringKey<Context<I, O>>, K>[];
    onDelete?: Handler<O> | NonEmptyArray<Handler<O>>;
    onSuccess?: SuccessHandler<I, O> | NonEmptyArray<SuccessHandler<I, O>>;
    readonly?: true;
    resolver: Resolver<K, I, O>;
  };

  type LaxProperty<K extends keyof (I & O), I, O = I> = Listenable<I, O> & {
    default: TypeOf<(I & O)[K]> | DefaultSetter<K, I, O>;
    readonly?: "lax";
    shouldInit?: false | BooleanSetter<I, O>;
    shouldUpdate?: BooleanSetter<I, O>;
    validator?: Validator<K, I, O>;
  };

  type Readonly_<K extends keyof (I & O), I, O = I> = Listenable<I, O> & {
    default: TypeOf<(I & O)[K]> | DefaultSetter<K, I, O>;
    readonly: "lax";
    shouldUpdate?: BooleanSetter<I, O>;
    validator: Validator<K, I, O>;
  };

  type ReadonlyNoInit<K extends keyof (I & O), I, O = I> = Listenable<I, O> & {
    default: TypeOf<(I & O)[K]> | DefaultSetter<K, I, O>;
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
    default: TypeOf<(I & O)[K]> | DefaultSetter<K, I, O>;
    required: RequiredSetter<I, O>;
    readonly?: true;
    shouldUpdate?: BooleanSetter<I, O>;
    validator: Validator<K, I, O>;
  };

  type Virtual<K extends keyof (I & O), I, O, A> = {
    alias?: Exclude<StringKey<A>, K> extends undefined
      ? string
      : Exclude<StringKey<A>, K>;
    required?: RequiredSetter<I, O>;
    virtual: true;
    sanitizer?: Resolver<K, I, O>;
    onFailure?: FailureHandler<I, O> | NonEmptyArray<FailureHandler<I, O>>;
    onSuccess?: SuccessHandler<I, O> | NonEmptyArray<SuccessHandler<I, O>>;
    shouldInit?: false | BooleanSetter<I, O>;
    shouldUpdate?: false | BooleanSetter<I, O>;
    validator: Validator<K, I, O>;
  };

  // options
  export interface CloneOptions<T> {
    reset?: StringKey<T> | StringKey<T>[];
  }

  export type ArchivedOptions<O> = {
    createdAt?: string;
    onDelete?: Handler<O> | NonEmptyArray<Handler<O>>;
    onSuccess?: Handler<O> | NonEmptyArray<Handler<O>>;
  };

  export type Options<I, O> = {
    errors?: "silent" | "throw";
    onDelete?: Handler<O> | NonEmptyArray<Handler<O>>;
    onSuccess?: SuccessHandler<I, O> | NonEmptyArray<SuccessHandler<I, O>>;
    timestamps?:
      | boolean
      | { createdAt?: boolean | string; updatedAt?: boolean | string };
  };

  export type ArchivedOptionsKey<O> = StringKey<ArchivedOptions<O>>;

  export type OptionsKey<I, O> = StringKey<Options<I, O>>;

  export interface PrivateOptions {
    timestamps: Timestamp;
  }

  export interface Timestamp {
    createdAt: string;
    updatedAt: string;
  }

  export type ExtensionOptions<ParentInput, ParentOutput, I, O> = Options<
    I,
    O
  > & {
    remove?:
      | StringKey<Merge<ParentInput, ParentOutput>>
      | StringKey<Merge<ParentInput, ParentOutput>>[];
  };
}

type ValidatorResponse<T> =
  | { valid: true; validated: T }
  | { reasons: string[]; valid: false };

type InternalValidatorResponse<T> =
  | { valid: true; validated: T }
  | { otherReasons?: InputPayload; reasons: string[]; valid: false };

type ResponseInput_<K, I, T> =
  | { valid: true; validated?: TypeOf<T> }
  | {
      otherReasons?: { [Key in Exclude<keyof I, K>]: string | string[] };
      reason?: string;
      reasons?: string[];
      valid: false;
    };

type ResponseInput<K, I, T> = boolean | (ResponseInput_<K, I, T> & {});

type Validator<K extends keyof (I & O), I, O> = (
  value: any,
  summary: Summary<I, O> & {}
) => ResponseInput<K, I, (I & O)[K]> | Promise<ResponseInput<K, I, (I & O)[K]>>;

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

type DefinitionRule = (typeof DEFINITION_RULES)[number];

const ALLOWED_ARCHIVED_OPTIONS: Schema.ArchivedOptionsKey<any>[] = [
  "createdAt",
  "onDelete",
  "onSuccess",
];

const ALLOWED_OPTIONS: Schema.OptionsKey<any, any>[] = [
  "errors",
  "onDelete",
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

export {
  ALLOWED_ARCHIVED_OPTIONS,
  ALLOWED_OPTIONS,
  CONSTANT_RULES,
  DEFINITION_RULES,
  VIRTUAL_RULES,
};
