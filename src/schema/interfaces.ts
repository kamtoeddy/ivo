import { ObjectType } from "../utils/interfaces";

export type {
  CombineTypes,
  ITimestamp,
  LifeCycles,
  NonEmptyArray,
  OptionsKey,
  Private_ISchemaOptions,
  PropDefinitionRule,
  RealType,
  ResponseInput,
  Schema,
  StringKey,
  TypeOf,
  Validator,
  ValidatorResponse,
};

type TypeOf<T> = Exclude<T, undefined>;

type CombineTypes<I, O> = RealType<I & O>;

type RealType_<T> = T extends (...args: any) => infer I ? I : T;

type RealType<T> = {
  [K in keyof T]: Exclude<T[K], Function> | RealType_<T[K]>;
} & {};

type ConditionalRequiredSetter<T> = (
  ctx: Readonly<T>,
  lifeCycle: LifeCycles.LifeCycle
) => boolean | [boolean, string];

type AsyncSetter<K extends keyof T, T> = (
  ctx: Readonly<T>,
  lifeCycle: LifeCycles.LifeCycle
) => TypeOf<T[K]> | Promise<TypeOf<T[K]>>;

type BooleanSetter<T> = (ctx: Readonly<T>) => boolean;

type StringKey<T> = Extract<keyof T, string>;

namespace Schema {
  export type PropertyDefinitions<I, O = I, A extends ObjectType = {}> = {
    [K in keyof I]?:
      | Constant<K, I, O>
      | Dependent<K, I, O>
      | Property<K, I, O>
      | Readonly<K, I, O>
      | ReadonlyNoInit<K, I, O>
      | Required<K, I, O>
      | RequiredBy<K, I, O>
      | ReadonlyRequired<K, I, O>
      | RequiredVirtual<K, I, A>
      | Virtual<K, I, A>;
  };

  export type Definitions<I> = {
    [K in keyof I]?: Listenable<I, I> & {
      alias?: string;
      constant?: any;
      default?: any;
      dependent?: boolean;
      dependsOn?: StringKey<I> | StringKey<I>[];
      readonly?: boolean | "lax";
      resolver?: Function;
      required?: boolean | ConditionalRequiredSetter<I>;
      sanitizer?: AsyncSetter<K, I>;
      shouldInit?: false | BooleanSetter<I>;
      shouldUpdate?: false | BooleanSetter<I>;
      validator?: Function;
      value?: any;
      virtual?: boolean;
    };
  };

  export type AliasMap<T extends ObjectType> = Record<string, StringKey<T>>;

  export type DependencyMap<T extends ObjectType> = {
    [K in StringKey<T>]?: StringKey<T>[];
  };

  type Listenable<I, O> = {
    onDelete?: LifeCycles.Listener<O> | NonEmptyArray<LifeCycles.Listener<O>>;
    onFailure?: LifeCycles.Listener<I> | NonEmptyArray<LifeCycles.Listener<I>>;
    onSuccess?:
      | LifeCycles.SuccessListener<I>
      | NonEmptyArray<LifeCycles.SuccessListener<I>>;
  };

  type Constant<K extends keyof T, T, O = T> = {
    constant: true;
    onDelete?: LifeCycles.Listener<O> | NonEmptyArray<LifeCycles.Listener<O>>;
    onSuccess?:
      | LifeCycles.SuccessListener<T>
      | NonEmptyArray<LifeCycles.SuccessListener<T>>;
    value: TypeOf<T[K]> | AsyncSetter<K, T>;
  };

  type Dependent<K extends keyof T, T, O = T> = Listenable<T, O> & {
    default: TypeOf<T[K]> | AsyncSetter<K, T>;
    dependent: true;
    dependsOn: Exclude<StringKey<T>, K> | Exclude<StringKey<T>, K>[];
    readonly?: true;
    resolver: AsyncSetter<K, T>;
  };

  type Property<K extends keyof T, T, O = T> = Listenable<T, O> & {
    default: TypeOf<T[K]> | AsyncSetter<K, T>;
    readonly?: "lax";
    shouldInit?: false | BooleanSetter<T>;
    shouldUpdate?: BooleanSetter<T>;
    validator?: Validator<K, T>;
  };

  type Readonly<K extends keyof T, T, O = T> = Listenable<T, O> & {
    default: TypeOf<T[K]> | AsyncSetter<K, T>;
    readonly: "lax";
    shouldUpdate?: BooleanSetter<T>;
    validator: Validator<K, T>;
  };

  type ReadonlyNoInit<K extends keyof T, T, O = T> = Listenable<T, O> & {
    default: TypeOf<T[K]> | AsyncSetter<K, T>;
    readonly: true;
    shouldInit: false | BooleanSetter<T>;
    shouldUpdate?: BooleanSetter<T>;
    validator?: Validator<K, T>;
  };

  type ReadonlyRequired<K extends keyof T, T, O = T> = Listenable<T, O> & {
    readonly: true;
    validator: Validator<K, T>;
  };

  type Required<K extends keyof T, T, O = T> = Listenable<T, O> & {
    required: true;
    shouldUpdate?: BooleanSetter<T>;
    validator: Validator<K, T>;
  };

  type RequiredBy<K extends keyof T, T, O = T> = Listenable<T, O> & {
    default: TypeOf<T[K]> | AsyncSetter<K, T>;
    required: ConditionalRequiredSetter<T>;
    readonly?: true;
    shouldUpdate?: BooleanSetter<T>;
    validator: Validator<K, T>;
  };

  type Virtual<K extends keyof T, T, A> = {
    alias?: Exclude<StringKey<A>, K> extends undefined
      ? string
      : Exclude<StringKey<A>, K>;
    virtual: true;
    sanitizer?: AsyncSetter<K, T>;
    onFailure?: LifeCycles.Listener<T> | NonEmptyArray<LifeCycles.Listener<T>>;
    onSuccess?:
      | LifeCycles.SuccessListener<T>
      | NonEmptyArray<LifeCycles.SuccessListener<T>>;
    shouldInit?: false | BooleanSetter<T>;
    shouldUpdate?: false | BooleanSetter<T>;
    validator: Validator<K, T>;
  };

  type RequiredVirtual<K extends keyof T, T, A> = Virtual<K, T, A> & {
    required: ConditionalRequiredSetter<T>;
    shouldUpdate?: false | BooleanSetter<T>;
  };

  // options
  export interface CloneOptions<T> {
    reset?: StringKey<T> | StringKey<T>[];
  }

  export interface Options {
    errors?: "silent" | "throw";
    timestamps?:
      | boolean
      | { createdAt?: boolean | string; updatedAt?: boolean | string };
  }

  export type ExtensionOptions<T> = Options & { remove?: T | T[] };
}

namespace LifeCycles {
  export type Rule = "onDelete" | "onFailure" | "onSuccess";

  export type LifeCycle = "creating" | "updating";

  export type Listener<T> = (ctx: Readonly<T>) => void | Promise<void>;

  export type SuccessListener<T> = (
    ctx: Readonly<T>,
    lifeCycle: LifeCycle
  ) => void | Promise<void>;
}

type ValidatorResponse<T> =
  | { valid: true; validated: T }
  | { reasons: string[]; valid: false };

type ResponseInput<T> =
  | { valid: true; validated?: TypeOf<T> }
  | { reason?: string; reasons?: string[]; valid: false };

type Validator<K extends keyof T, T> = (
  value: any,
  ctx: Readonly<T>
) => ResponseInput<T[K]> | Promise<ResponseInput<T[K]>>;

type NonEmptyArray<T> = [T, ...T[]];

interface ITimestamp {
  createdAt: string;
  updatedAt: string;
}

interface Private_ISchemaOptions {
  timestamps: ITimestamp;
}

type OptionsKey = StringKey<Schema.Options>;

const PropDefinitionRules = [
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

type PropDefinitionRule = typeof PropDefinitionRules[number];

const allowedOptions: OptionsKey[] = ["errors", "timestamps"];
const constantRules = ["constant", "onDelete", "onSuccess", "value"];
const virtualRules = [
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

export { allowedOptions, constantRules, PropDefinitionRules, virtualRules };
