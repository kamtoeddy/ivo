import { ObjectType } from "../utils/interfaces";

export type {
  CombineTypes,
  SpreadType,
  TypeOf,
  ResponseInput,
  Validator,
  ValidatorResponse,
};

type TypeOf<T> = Exclude<T, undefined>;

type GetCommonProps<I, O> = {
  [K in keyof (I | O)]: I[K] extends never
    ? O[K]
    : O[K] extends never
    ? I[K]
    : O[K];
};

type OmitNever<T> = { [K in keyof T as T[K] extends never ? never : K]: T[K] };

type Combined<I, O> = OmitNever<I & O> extends never
  ? O
  : OmitNever<I & O> & GetCommonProps<I, O>;

type SpreadType<T> = { [K in keyof T]: T[K] };

type CombineTypes<I, O> = SpreadType<Combined<I, O>>;

type Setter<K, T> = (
  ctx: Readonly<T>,
  lifeCycle: LifeCycles.LifeCycle
) => K extends keyof T ? TypeOf<T[K]> : K;

type ConditionalRequiredSetter<T> = (
  ctx: Readonly<T>,
  lifeCycle: LifeCycles.LifeCycle
) => boolean | [boolean, string];

type AsyncSetter<K, T> = (
  ctx: Readonly<T>,
  lifeCycle: LifeCycles.LifeCycle
) => Awaited<K extends keyof T ? TypeOf<T[K]> : K>;

export type StringKey<T> = Extract<keyof T, string>;

export namespace Schema {
  export type PropertyDefinitions<I, O = I> = {
    [K in keyof I]:
      | Constant<K, I, O>
      | Dependent<K, I, O>
      | Property<K, I, O>
      | Readonly<K, I, O>
      | ReadonlyNoInit<K, I, O>
      | Required<K, I, O>
      | RequiredBy<K, I, O>
      | RequiredReadonly<K, I, O>
      | RequiredSideEffect<K, I>
      | SideEffect<K, I>;
  };

  export type Definitions<I> = {
    [K in keyof I]?: Listenable<I> & {
      constant?: any;
      default?: any;
      dependent?: boolean;
      dependsOn?: StringKey<I> | StringKey<I>[];
      readonly?: boolean | "lax";
      resolver?: Function;
      required?: boolean | ConditionalRequiredSetter<I>;
      sanitizer?: Setter<I[StringKey<I>], I> | AsyncSetter<I[StringKey<I>], I>;
      sideEffect?: boolean;
      shouldInit?: false | Setter<boolean, I>;
      shouldUpdate?: false | Setter<boolean, I>;
      validator?: Function;
      value?: any;
    };
  };

  export type DependencyMap<T extends ObjectType> = {
    [K in StringKey<T>]?: StringKey<T>[];
  };

  type Listenable<I, O = I> = {
    onDelete?:
      | LifeCycles.DeleteListener<O>
      | NonEmptyArray<LifeCycles.DeleteListener<O>>;
    onFailure?:
      | LifeCycles.VoidListener<I>
      | NonEmptyArray<LifeCycles.VoidListener<I>>;
    onSuccess?:
      | LifeCycles.SuccessListener<I>
      | NonEmptyArray<LifeCycles.SuccessListener<I>>;
  };

  type Constant<K extends keyof T, T, O = T> = {
    constant: true;
    onDelete?:
      | LifeCycles.DeleteListener<O>
      | NonEmptyArray<LifeCycles.DeleteListener<O>>;
    onSuccess?:
      | LifeCycles.SuccessListener<T>
      | NonEmptyArray<LifeCycles.SuccessListener<T>>;
    value: TypeOf<T[K]> | Setter<K, T> | AsyncSetter<K, T>;
  };

  type Dependent<K extends keyof T, T, O = T> = Listenable<T, O> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    dependent: true;
    dependsOn: Exclude<keyof T, K> | Exclude<keyof T, K>[];
    readonly?: true;
    resolver: Setter<K, T> | AsyncSetter<K, T>;
  };

  type Property<K extends keyof T, T, O = T> = Listenable<T, O> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    readonly?: "lax";
    shouldInit?: false | Setter<boolean, T>;
    shouldUpdate?: false | Setter<boolean, T>;
    validator?: Validator<K, T>;
  };

  type Readonly<K extends keyof T, T, O = T> = Listenable<T, O> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    readonly: "lax";
    shouldUpdate?: false | Setter<boolean, T>;
    validator: Validator<K, T>;
  };

  type ReadonlyNoInit<K extends keyof T, T, O = T> = Listenable<T, O> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    readonly: true;
    shouldInit: false | Setter<boolean, T>;
    shouldUpdate?: false | Setter<boolean, T>;
    validator?: Validator<K, T>;
  };

  type RequiredReadonly<K extends keyof T, T, O = T> = Listenable<T, O> & {
    readonly: true;
    shouldUpdate?: false | Setter<boolean, T>;
    validator: Validator<K, T>;
  };

  type Required<K extends keyof T, T, O = T> = Listenable<T, O> & {
    required: true;
    shouldUpdate?: false | Setter<boolean, T>;
    validator: Validator<K, T>;
  };

  type RequiredBy<K extends keyof T, T, O = T> = Listenable<T, O> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    required: ConditionalRequiredSetter<T>;
    readonly?: true;
    shouldUpdate?: false | Setter<boolean, T>;
    validator: Validator<K, T>;
  };

  type SideEffect<K extends keyof T, T> = {
    sideEffect: true;
    sanitizer?: Setter<T[StringKey<T>], T> | AsyncSetter<T[StringKey<T>], T>;
    onFailure?:
      | LifeCycles.VoidListener<T>
      | NonEmptyArray<LifeCycles.VoidListener<T>>;
    onSuccess?:
      | LifeCycles.SuccessListener<T>
      | NonEmptyArray<LifeCycles.SuccessListener<T>>;
    shouldInit?: false | Setter<boolean, T>;
    shouldUpdate?: false | Setter<boolean, T>;
    validator: Validator<K, T>;
  };

  type RequiredSideEffect<K extends keyof T, T> = SideEffect<K, T> & {
    required: Setter<boolean, T>;
    shouldUpdate?: false | Setter<boolean, T>;
    requiredError: string | Setter<string, T>;
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

  export type ExtensionOptions<T> = Options & {
    remove?: T | T[];
  };
}

export namespace LifeCycles {
  export type Rule = "onDelete" | "onFailure" | "onSuccess";

  export type LifeCycle = "creating" | "updating";

  export type Listener<T> = (
    ctx: Readonly<T>
  ) => Partial<T> | Promise<Partial<T>> | void | Promise<void>;

  export type DeleteListener<T> = (ctx: Readonly<T>) => void | Promise<void>;

  export type SuccessListener<T> = (
    ctx: Readonly<T>,
    lifeCycle: LifeCycle
  ) => void | Promise<void>;
  export type VoidListener<T> = (ctx: Readonly<T>) => void | Promise<void>;
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

export type NonEmptyArray<T> = [T, ...T[]];

export type PropDefinitionRule =
  | "constant"
  | "default"
  | "dependent"
  | "dependsOn"
  | "onDelete"
  | "onFailure"
  | "onSuccess"
  | "readonly"
  | "resolver"
  | "required"
  | "sideEffect"
  | "sanitizer"
  | "shouldInit"
  | "shouldUpdate"
  | "validator"
  | "value";

export interface ITimestamp {
  createdAt: string;
  updatedAt: string;
}

export interface Private_ISchemaOptions {
  timestamps: ITimestamp;
}
