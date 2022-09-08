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
      | SideEffect<T, K>;
  };

  export type Definitions<T> = {
    [K in keyof T]?: Listenable<T> & {
      constant?: any;
      default?: any;
      dependent?: boolean;
      readonly?: boolean | "lax";
      required?: boolean;
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
    constant: boolean;
    default?: undefined;
    onCreate?: LifeCycles.Listener<T> | NonEmptyArray<LifeCycles.Listener<T>>;
    value: T[K] | Setter<T, K>;
  };

  type Dependent<T, K extends keyof T> = Listenable<T> & {
    default: T[K] | Setter<T, K>;
    dependent: true;
    readonly?: boolean;
    validator?: Validator<T, K>;
  };

  type Property<T, K extends keyof T> = Listenable<T> & {
    default: T[K] | Setter<T, K>;
    readonly?: "lax";
    shouldInit?: boolean;
    validator?: Validator<T, K>;
  };

  type Readonly<T, K extends keyof T> = Listenable<T> & {
    default: T[K] | Setter<T, K>;
    readonly: "lax";
    validator: Validator<T, K>;
  };

  type ReadonlyNoInit<T, K extends keyof T> = Listenable<T> & {
    default: T[K] | Setter<T, K>;
    readonly: true;
    shouldInit: false;
    validator?: Validator<T, K>;
  };

  type RequiredReadonly<T, K extends keyof T> = Listenable<T> & {
    readonly: true;
    validator: Validator<T, K>;
  };

  type Required<T, K extends keyof T> = Listenable<T> & {
    required: true;
    validator: Validator<T, K>;
  };

  type SideEffect<T, K extends keyof T> = {
    sideEffect: true;
    onChange: LifeCycles.Listener<T> | NonEmptyArray<LifeCycles.Listener<T>>;
    shouldInit?: boolean;
    validator: Validator<T, K>;
  };

  // options
  export interface CloneOptions<T> {
    reset?: StringKey<T> | StringKey<T>[];
  }

  export interface ExtensionOptions<T> {
    remove?: StringKey<T> | StringKey<T>[];
  }

  export interface Options {
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

export interface ValidatorResponse<T = any> {
  reasons?: string[];
  valid: boolean;
  validated?: T;
}

export type ResponseInput<T> = {
  reason?: string;
  reasons?: string[];
  valid: boolean;
  validated?: T;
};

export type Setter<T, K> = (
  ctx: Readonly<T>
) => K extends keyof T ? Exclude<T[K], undefined> : K;

export type Validator<T, K extends keyof T> = (
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
