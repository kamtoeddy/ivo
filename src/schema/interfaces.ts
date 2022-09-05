export type StringKeys<T> = Extract<keyof T, string>;

export namespace Schema {
  export type PropertyDefinitions<T> = {
    [K in keyof T]?: DefinitionRule<T> extends { default?: infer R }
      ? R extends Function
        ? DefinitionRule<T, DefaultSetter<T>>
        : DefinitionRule<T, R>
      : DefinitionRule<T>;
  };

  type DefinitionRule<T, K = any> = {
    default?: K;
    dependent?: boolean;
    onChange?: LifeCycle.Listener<T> | NonEmptyArray<LifeCycle.Listener<T>>;
    onCreate?: LifeCycle.Listener<T> | NonEmptyArray<LifeCycle.Listener<T>>;
    onUpdate?: LifeCycle.Listener<T> | NonEmptyArray<LifeCycle.Listener<T>>;
    readonly?: boolean | "lax";
    required?: boolean;
    sideEffect?: boolean;
    shouldInit?: boolean;
    validator?: Validator<T>;
  };

  // options
  export interface CloneOptions<T> {
    reset?: StringKeys<T> | StringKeys<T>[];
  }

  export interface ExtensionOptions<T> {
    remove?: StringKeys<T> | StringKeys<T>[];
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

export namespace LifeCycle {
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

export type ResponseInput = {
  reason?: string;
  reasons?: string[];
  valid: boolean;
  validated?: any;
};

export type DefaultSetter<T> = (ctx: Readonly<T>) => any;

export type Validator<T> = (
  value: any,
  ctx: Readonly<T>
) => ResponseInput | Promise<ResponseInput>;

export type NonEmptyArray<T> = [T, ...T[]];

export type PropDefinitionRule =
  | "default"
  | "dependent"
  | "onChange"
  | "onCreate"
  | "onUpdate"
  | "readonly"
  | "required"
  | "sideEffect"
  | "shouldInit"
  | "validator";

export interface ITimestamp {
  createdAt: string;
  updatedAt: string;
}

export interface Private_ISchemaOptions {
  timestamps: ITimestamp;
}
