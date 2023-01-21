import { ObjectType } from "../utils/interfaces";

export type TypeOf<T> = Exclude<T, undefined>;

type Setter<K, T> = (
  ctx: Readonly<T>,
  lifeCycle: LifeCycles.LifeCycle
) => K extends keyof T ? TypeOf<T[K]> : K;

type AsyncSetter<K, T> = (
  ctx: Readonly<T>,
  lifeCycle: LifeCycles.LifeCycle
) => Awaited<K extends keyof T ? TypeOf<T[K]> : K>;

export type StringKey<T> = Extract<keyof T, string>;

export namespace Schema {
  export type PropertyDefinitions<I, O = I> = {
    [K in keyof I]?:
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

  export type Definitions<I, O = I> = {
    [K in keyof I]?: Listenable<I, O> & {
      constant?: any;
      default?: any;
      dependent?: boolean;
      dependsOn?: StringKey<I> | NonEmptyArray<StringKey<I>>;
      readonly?: boolean | "lax";
      resolver?: Function;
      required?: boolean | Function;
      requiredError?: string | Function;
      sideEffect?: boolean;
      shouldInit?: false | Setter<boolean, I>;
      validator?: Function;
      value?: any;
    };
  };

  export type DependencyMap<T extends ObjectType> = {
    [K in StringKey<T>]?: StringKey<T>[];
  };

  type Listenable<I, O = I> = {
    onChange?:
      | LifeCycles.ChangeListener<I>
      | NonEmptyArray<LifeCycles.ChangeListener<I>>;
    onCreate?: LifeCycles.Listener<I> | NonEmptyArray<LifeCycles.Listener<I>>;
    onDelete?:
      | LifeCycles.DeleteListener<O>
      | NonEmptyArray<LifeCycles.DeleteListener<O>>;
    onFailure?:
      | LifeCycles.VoidListener<I>
      | NonEmptyArray<LifeCycles.VoidListener<I>>;
    onSuccess?:
      | LifeCycles.SuccessListener<I>
      | NonEmptyArray<LifeCycles.SuccessListener<I>>;
    onUpdate?: LifeCycles.Listener<I> | NonEmptyArray<LifeCycles.Listener<I>>;
  };

  type Constant<K extends keyof T, T, O = T> = {
    constant: true;
    onCreate?: LifeCycles.Listener<T> | NonEmptyArray<LifeCycles.Listener<T>>;
    onDelete?:
      | LifeCycles.DeleteListener<O>
      | NonEmptyArray<LifeCycles.DeleteListener<O>>;
    value: TypeOf<T[K]> | Setter<K, T> | AsyncSetter<K, T>;
  };

  type Dependent<K extends keyof T, T, O = T> = Listenable<T, O> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    dependent: true;
    dependsOn:
      | Exclude<StringKey<T>, K>
      | NonEmptyArray<Exclude<StringKey<T>, K>>;
    readonly?: true;
    resolver: Setter<K, T> | AsyncSetter<K, T>;
  };

  type Property<K extends keyof T, T, O = T> = Listenable<T, O> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    readonly?: "lax";
    shouldInit?: false | Setter<boolean, T>;
    validator?: Validator<K, T>;
  };

  type Readonly<K extends keyof T, T, O = T> = Listenable<T, O> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    readonly: "lax";
    validator: Validator<K, T>;
  };

  type ReadonlyNoInit<K extends keyof T, T, O = T> = Listenable<T, O> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    readonly: true;
    shouldInit: false | Setter<boolean, T>;
    validator?: Validator<K, T>;
  };

  type RequiredReadonly<K extends keyof T, T, O = T> = Listenable<T, O> & {
    readonly: true;
    validator: Validator<K, T>;
  };

  type Required<K extends keyof T, T, O = T> = Listenable<T, O> & {
    required: true;
    validator: Validator<K, T>;
  };

  type RequiredBy<K extends keyof T, T, O = T> = Listenable<T, O> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    required: Setter<boolean, T>;
    requiredError: string | Setter<string, T>;
    readonly?: true;
    validator: Validator<K, T>;
  };

  type SideEffect<K extends keyof T, T> = {
    sideEffect: true;
    onFailure?:
      | LifeCycles.VoidListener<T>
      | NonEmptyArray<LifeCycles.VoidListener<T>>;
    onSuccess?:
      | LifeCycles.SuccessListener<T>
      | NonEmptyArray<LifeCycles.SuccessListener<T>>;
    shouldInit?: false | Setter<boolean, T>;
    validator: Validator<K, T>;
  };

  type RequiredSideEffect<K extends keyof T, T> = SideEffect<K, T> & {
    required: Setter<boolean, T>;
    requiredError: string | Setter<string, T>;
  };

  // options
  export interface CloneOptions<T> {
    reset?: StringKey<T> | StringKey<T>[];
  }

  export type ExtensionOptions<T> = { remove?: T | T[] };

  export interface Options {
    errors?: "silent" | "throw";
    timestamps?: boolean | { createdAt?: string; updatedAt?: string };
  }
}

export namespace LifeCycles {
  export type Rule =
    | "onChange"
    | "onCreate"
    | "onDelete"
    | "onFailure"
    | "onSuccess"
    | "onUpdate";

  export type LifeCycle = "onCreate" | "onUpdate";

  export type Listener<T> = (
    ctx: Readonly<T>
  ) => Partial<T> | Promise<Partial<T>> | void | Promise<void>;

  export type ChangeListener<T> = (
    ctx: Readonly<T>,
    lifeCycle: LifeCycle
  ) => Partial<T> | Promise<Partial<T>> | void | Promise<void>;

  export type DeleteListener<T> = (ctx: Readonly<T>) => void | Promise<void>;

  export type SuccessListener<T> = (
    ctx: Readonly<T>,
    lifeCycle: LifeCycle
  ) => void | Promise<void>;
  export type VoidListener<T> = (ctx: Readonly<T>) => void | Promise<void>;
}

export type ValidatorResponse<T> =
  | { valid: true; validated: T }
  | { reasons: string[]; valid: false };

export type ResponseInput<T> =
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
  | "onChange"
  | "onCreate"
  | "onDelete"
  | "onFailure"
  | "onSuccess"
  | "onUpdate"
  | "readonly"
  | "resolver"
  | "required"
  | "requiredError"
  | "sideEffect"
  | "shouldInit"
  | "validator"
  | "value";

export interface ITimestamp {
  createdAt: string;
  updatedAt: string;
}

export interface Private_ISchemaOptions {
  timestamps: ITimestamp;
}
