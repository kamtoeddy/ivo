export type TypeOf<T> = Exclude<T, undefined>;

type Setter<K, T> = (ctx: Readonly<T>) => K extends keyof T ? TypeOf<T[K]> : K;

export type StringKey<T> = Extract<keyof T, string>;

export namespace Schema {
  export type PropertyDefinitions<T> = {
    [K in keyof T]?:
      | Constant<T, K>
      | Property<T, K>
      | Dependent<T, K>
      | Readonly<T, K>
      | ReadonlyNoInit<T, K>
      | RequiredReadonly<T, K>
      | Required<T, K>
      | RequiredBy<T, K>
      | SideEffect<T, K>;
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
    onUpdate?: LifeCycles.Listener<T> | NonEmptyArray<LifeCycles.Listener<T>>;
  };

  type Constant<T, K extends keyof T> = {
    constant: true;
    onCreate?: LifeCycles.Listener<T> | NonEmptyArray<LifeCycles.Listener<T>>;
    value: TypeOf<T[K]> | Setter<K, T>;
  };

  type Dependent<T, K extends keyof T> = Listenable<T> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    dependent: true;
    readonly?: true;
    validator?: Validator<K, T>;
  };

  type Property<T, K extends keyof T> = Listenable<T> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    readonly?: "lax";
    shouldInit?: true;
    validator?: Validator<K, T>;
  };

  type Readonly<T, K extends keyof T> = Listenable<T> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    readonly: "lax";
    validator: Validator<K, T>;
  };

  type ReadonlyNoInit<T, K extends keyof T> = Listenable<T> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    readonly: true;
    shouldInit: false;
    validator?: Validator<K, T>;
  };

  type RequiredReadonly<T, K extends keyof T> = Listenable<T> & {
    readonly: true;
    validator: Validator<K, T>;
  };

  type Required<T, K extends keyof T> = Listenable<T> & {
    required: true;
    validator: Validator<K, T>;
  };

  type RequiredBy<T, K extends keyof T> = Listenable<T> & {
    default: TypeOf<T[K]> | Setter<K, T>;
    required: Setter<T, boolean>;
    requiredError: string | Setter<T, string>;
    readonly?: true;
    validator: Validator<K, T>;
  };

  type SideEffect<T, K extends keyof T> = {
    sideEffect: true;
    onChange: LifeCycles.Listener<T> | NonEmptyArray<LifeCycles.Listener<T>>;
    shouldInit?: false;
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
  export type Rule = "onChange" | "onCreate" | "onUpdate";

  export type Listener<T> = (
    ctx: Readonly<T>
  ) => Partial<T> | Promise<Partial<T>> | void | Promise<void>;
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
