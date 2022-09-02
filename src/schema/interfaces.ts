export type Listener<T> = (ctx: T) => Partial<T> | Promise<Partial<T>>;

export type StringKeys<T> = Extract<keyof T, string>;

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

export type Validator<T> = (
  value: any,
  ctx: T
) => ResponseInput | Promise<ResponseInput>;

export type NonEmptyArray<T> = [T, ...T[]];

export type PropDefinitionRules<T> = {
  [K in keyof T]?: {
    default?: any;
    dependent?: boolean;
    onChange?: Listener<T> | NonEmptyArray<Listener<T>>;
    onCreate?: Listener<T> | NonEmptyArray<Listener<T>>;
    onUpdate?: Listener<T> | NonEmptyArray<Listener<T>>;
    readonly?: boolean | "lax";
    required?: boolean;
    sideEffect?: boolean;
    shouldInit?: boolean;
    validator?: Validator<T>;
  };
};

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

export type LifeCycleRule = "onChange" | "onCreate" | "onUpdate";

interface IOptionsTimestamp {
  createdAt?: string;
  updatedAt?: string;
}

export interface ITimestamp {
  createdAt: string;
  updatedAt: string;
}

export interface SchemaOptions {
  timestamps?: boolean | IOptionsTimestamp;
}

export interface Private_ISchemaOptions {
  timestamps: ITimestamp;
}
export interface SchemaCloneOptions<T> {
  reset?: StringKeys<T> | StringKeys<T>[];
}

export interface SchemaExtensionOptions<T> {
  remove?: StringKeys<T> | StringKeys<T>[];
}
