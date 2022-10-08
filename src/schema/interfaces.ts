export type TypeOf<T> = Exclude<T, undefined>;

type Setter<K, T> = (ctx: Readonly<T>) => K extends keyof T ? TypeOf<T[K]> : K;

type Promisable<T> = T | Promise<T>;

type AsyncSetter<K, T> = (
  ctx: Readonly<T>
) => Promisable<K extends keyof T ? TypeOf<T[K]> : K>;

export type StringKey<T> = Extract<keyof T, string>;

export namespace Schema {
  export type PropertyDefinitions<T> = {
    [K in keyof T]?:
      | Constant<K, T>
      | Dependent<K, T>
      | Property<K, T>
      | Readonly<K, T>
      | ReadonlyNoInit<K, T>
      | Required<K, T>
      | RequiredBy<K, T>
      | RequiredReadonly<K, T>
      | RequiredSideEffect<K, T>
      | SideEffect<K, T>;
  };

  export type Definitions<T> = {
    [K in keyof T]?: Listenable<T> & {
      constant?: any;
      default?: any;
      dependent?: boolean;
      readonly?: boolean | "lax";
      required?: boolean | Function;
      requiredError?: string | Function;
      sideEffect?: boolean;
      shouldInit?: boolean;
      validator?: Function;
      value?: any;
    };
  };

  type Listenable<T> = {
    onChange?: LifeCycles.Listener<T> | NonEmptyArray<LifeCycles.Listener<T>>;
    onCreate?: LifeCycles.Listener<T> | NonEmptyArray<LifeCycles.Listener<T>>;
    onDelete?:
      | LifeCycles.VoidListener<T>
      | NonEmptyArray<LifeCycles.VoidListener<T>>;
    onFailure?:
      | LifeCycles.VoidListener<T>
      | NonEmptyArray<LifeCycles.VoidListener<T>>;
    onSuccess?:
      | LifeCycles.VoidListener<T>
      | NonEmptyArray<LifeCycles.VoidListener<T>>;
    onUpdate?: LifeCycles.Listener<T> | NonEmptyArray<LifeCycles.Listener<T>>;
  };

  type Constant<K extends keyof T, T> = {
    constant: true;
    onCreate?: LifeCycles.Listener<T> | NonEmptyArray<LifeCycles.Listener<T>>;
    value: TypeOf<T[K]> | Setter<K, T> | AsyncSetter<K, T>;
  };

  type Dependent<K extends keyof T, T> = Listenable<T> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    dependent: true;
    readonly?: true;
    validator?: Validator<K, T>;
  };

  type Property<K extends keyof T, T> = Listenable<T> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    readonly?: "lax";
    shouldInit?: true;
    validator?: Validator<K, T>;
  };

  type Readonly<K extends keyof T, T> = Listenable<T> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    readonly: "lax";
    validator: Validator<K, T>;
  };

  type ReadonlyNoInit<K extends keyof T, T> = Listenable<T> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    readonly: true;
    shouldInit: false;
    validator?: Validator<K, T>;
  };

  type RequiredReadonly<K extends keyof T, T> = Listenable<T> & {
    readonly: true;
    validator: Validator<K, T>;
  };

  type Required<K extends keyof T, T> = Listenable<T> & {
    required: true;
    validator: Validator<K, T>;
  };

  type RequiredBy<K extends keyof T, T> = Listenable<T> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    required: Setter<boolean, T>;
    requiredError: string | Setter<string, T>;
    readonly?: true;
    validator: Validator<K, T>;
  };

  type SideEffect<K extends keyof T, T> = {
    sideEffect: true;
    onChange: LifeCycles.Listener<T> | NonEmptyArray<LifeCycles.Listener<T>>;
    onFailure?:
      | LifeCycles.VoidListener<T>
      | NonEmptyArray<LifeCycles.VoidListener<T>>;
    shouldInit?: false;
    validator: Validator<K, T>;
  };

  type RequiredSideEffect<K extends keyof T, T> = {
    sideEffect: true;
    onChange: LifeCycles.Listener<T> | NonEmptyArray<LifeCycles.Listener<T>>;
    onFailure?:
      | LifeCycles.VoidListener<T>
      | NonEmptyArray<LifeCycles.VoidListener<T>>;
    required: Setter<boolean, T>;
    requiredError: string | Setter<string, T>;
    validator: Validator<K, T>;
  };

  // options
  export interface CloneOptions<T> {
    reset?: StringKey<T> | StringKey<T>[];
  }

  export interface ExtensionOptions<T> {
    remove?: StringKey<T> | StringKey<T>[];
  }

  export interface Options {
    errors?: "silent" | "throw";
    timestamps?:
      | boolean
      | {
          createdAt?: string;
          updatedAt?: string;
        };
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

  export type Listener<T> = (
    ctx: Readonly<T>
  ) => Partial<T> | Promise<Partial<T>> | void | Promise<void>;

  export type VoidListener<T> = (ctx: Readonly<T>) => void | Promise<void>;
}

export interface ValidatorResponse<T> {
  reasons?: string[];
  valid: boolean;
  validated?: T;
}

export type ResponseInput<T> = {
  reason?: string;
  reasons?: string[];
  valid: boolean;
  validated?: TypeOf<T>;
};

type Validator<K extends keyof T, T> = (
  value: any,
  ctx: Readonly<T>
) => ResponseInput<T[K]> | Promise<ResponseInput<T[K]>>;

export type NonEmptyArray<T> = [T, ...T[]];

export type PropDefinitionRule =
  | "constant"
  | "default"
  | "dependent"
  | "onChange"
  | "onCreate"
  | "onDelete"
  | "onFailure"
  | "onSuccess"
  | "onUpdate"
  | "readonly"
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
