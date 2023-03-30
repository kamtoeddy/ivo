import { Merge, RealType } from "./merge-types";

export type {
  CombinedType,
  ContextType,
  DefinitionRule,
  ISchema,
  ITimestamp,
  NonEmptyArray,
  OperationSummary,
  OptionsKey,
  Private_ISchemaOptions,
  RealType,
  ResponseInput,
  StringKey,
  TypeOf,
  Validator,
  ValidatorResponse,
};

type OperationSummary<ValueType, ContextType> = ISchema.OperationSummary<
  ValueType,
  ContextType
>;

type TypeOf<T> = Exclude<T, undefined>;

type CombinedType<I, O> = RealType<Merge<I, O>>;

type ContextType<I, O> = Readonly<Merge<I, O>>;

type AsyncSetter<K extends keyof I, I, O> = (
  context: ContextType<I, O>,
  operation: ISchema.OperationName
) => TypeOf<I[K]> | Promise<TypeOf<I[K]>>;

type BooleanSetter<I, O> = (context: ContextType<I, O>) => boolean;

type ConditionalRequiredSetter<I, O> = (
  context: ContextType<I, O>,
  operation: ISchema.OperationName
) => boolean | [boolean, string];

type StringKey<T> = Extract<keyof T, string>;

namespace ISchema {
  export type LifeCycles = "onDelete" | "onFailure" | "onSuccess";

  export type OperationName = "creation" | "update";

  export type Listener<T> = (context: Readonly<T>) => void | Promise<void>;

  export type SuccessListener<OutputType, ContextType> = (
    summary: OperationSummary<OutputType, ContextType>
  ) => void | Promise<void>;

  export type OperationSummary<ValueType, ContextType> =
    | {
        context: ContextType;
        operation: "creation";
        previousValue: undefined;
        value: ValueType;
      }
    | {
        context: ContextType;
        operation: "update";
        previousValue: ValueType;
        value: ValueType;
      };

  export type Definitions<I, O = I, A = {}> = {
    [K in keyof I]?: Property<K, I, O, A>;
  };

  export type Definitions_<I, O> = {
    [K in keyof I]?: Listenable<K, I, O> & {
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

  type Property<K extends keyof I, I, O, A> =
    | Constant<K, I, O>
    | Dependent<K, I, O>
    | LaxProperty<K, I, O>
    | Readonly_<K, I, O>
    | ReadonlyNoInit<K, I, O>
    | Required<K, I, O>
    | RequiredBy<K, I, O>
    | ReadonlyRequired<K, I, O>
    | Virtual<K, I, O, A>;

  type Listenable<K extends keyof I, I, O> = {
    onDelete?:
      | ISchema.Listener<ContextType<I, O>>
      | NonEmptyArray<ISchema.Listener<ContextType<I, O>>>;
    onFailure?:
      | ISchema.Listener<ContextType<I, O>>
      | NonEmptyArray<ISchema.Listener<ContextType<I, O>>>;
    onSuccess?:
      | ISchema.SuccessListener<I[K], ContextType<I, O>>
      | NonEmptyArray<ISchema.SuccessListener<I[K], ContextType<I, O>>>;
  };

  type Constant<K extends keyof I, I, O = I> = {
    constant: true;
    onDelete?:
      | ISchema.Listener<ContextType<I, O>>
      | NonEmptyArray<ISchema.Listener<ContextType<I, O>>>;
    onSuccess?:
      | ISchema.SuccessListener<O, ContextType<I, O>>
      | NonEmptyArray<ISchema.SuccessListener<O, ContextType<I, O>>>;
    value: TypeOf<I[K]> | AsyncSetter<K, I, O>;
  };

  type Dependent<K extends keyof I, I, O = I> = Listenable<K, I, O> & {
    default: TypeOf<I[K]> | AsyncSetter<K, I, O>;
    dependent: true;
    dependsOn: Exclude<StringKey<I>, K> | Exclude<StringKey<I>, K>[];
    readonly?: true;
    resolver: AsyncSetter<K, I, O>;
  };

  type LaxProperty<K extends keyof I, I, O = I> = Listenable<K, I, O> & {
    default: TypeOf<I[K]> | AsyncSetter<K, I, O>;
    readonly?: "lax";
    shouldInit?: false | BooleanSetter<I, O>;
    shouldUpdate?: BooleanSetter<I, O>;
    validator?: Validator<K, I, O>;
  };

  type Readonly_<K extends keyof I, I, O = I> = Listenable<K, I, O> & {
    default: TypeOf<I[K]> | AsyncSetter<K, I, O>;
    readonly: "lax";
    shouldUpdate?: BooleanSetter<I, O>;
    validator: Validator<K, I, O>;
  };

  type ReadonlyNoInit<K extends keyof I, I, O = I> = Listenable<K, I, O> & {
    default: TypeOf<I[K]> | AsyncSetter<K, I, O>;
    readonly: true;
    shouldInit: false | BooleanSetter<I, O>;
    shouldUpdate?: BooleanSetter<I, O>;
    validator?: Validator<K, I, O>;
  };

  type ReadonlyRequired<K extends keyof I, I, O = I> = Listenable<K, I, O> & {
    readonly: true;
    validator: Validator<K, I, O>;
  };

  type Required<K extends keyof I, I, O = I> = Listenable<K, I, O> & {
    required: true;
    shouldUpdate?: BooleanSetter<I, O>;
    validator: Validator<K, I, O>;
  };

  type RequiredBy<K extends keyof I, I, O = I> = Listenable<K, I, O> & {
    default: TypeOf<I[K]> | AsyncSetter<K, I, O>;
    required: ConditionalRequiredSetter<I, O>;
    readonly?: true;
    shouldUpdate?: BooleanSetter<I, O>;
    validator: Validator<K, I, O>;
  };

  type Virtual<K extends keyof I, I, O, A> = {
    alias?: Exclude<StringKey<A>, K> extends undefined
      ? string
      : Exclude<StringKey<A>, K>;
    required?: ConditionalRequiredSetter<I, O>;
    virtual: true;
    sanitizer?: AsyncSetter<K, I, O>;
    onFailure?:
      | ISchema.Listener<ContextType<I, O>>
      | NonEmptyArray<ISchema.Listener<ContextType<I, O>>>;
    onSuccess?:
      | ISchema.SuccessListener<O, ContextType<I, O>>
      | NonEmptyArray<ISchema.SuccessListener<O, ContextType<I, O>>>;
    shouldInit?: false | BooleanSetter<I, O>;
    shouldUpdate?: false | BooleanSetter<I, O>;
    validator: Validator<K, I, O>;
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

type ValidatorResponse<T> =
  | { valid: true; validated: T }
  | { reasons: string[]; valid: false };

type ResponseInput<T> =
  | { valid: true; validated?: TypeOf<T> }
  | { reason?: string; reasons?: string[]; valid: false };

type Validator<K extends keyof I, I, O> = (
  value: any,
  context: ContextType<I, O>
) => ResponseInput<I[K]> | Promise<ResponseInput<I[K]>>;

type NonEmptyArray<T> = [T, ...T[]];

interface ITimestamp {
  createdAt: string;
  updatedAt: string;
}

interface Private_ISchemaOptions {
  timestamps: ITimestamp;
}

type OptionsKey = StringKey<ISchema.Options>;

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

const ALLOWED_OPTIONS: OptionsKey[] = ["errors", "timestamps"];
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
