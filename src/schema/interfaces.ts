import { ObjectType } from "../utils/interfaces";

export type Listener = (
  ctx: any
) => Partial<ObjectType> | Promise<Partial<ObjectType>>;

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

export type Validator = (
  ...args: any
) => ResponseInput | Promise<ResponseInput>;

export type NonEmptyArray<T> = [T, ...T[]];

export interface PropDefinitionRules {
  [key: string]: {
    default?: any;
    dependent?: boolean;
    onChange?: Listener | NonEmptyArray<Listener>;
    onCreate?: Listener | NonEmptyArray<Listener>;
    onUpdate?: Listener | NonEmptyArray<Listener>;
    readonly?: boolean | "lax";
    required?: boolean;
    sideEffect?: boolean;
    shouldInit?: boolean;
    validator?: Validator;
  };
}

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
export interface SchemaCloneOptions {
  reset?: string | string[];
}

export interface SchemaExtensionOptions {
  remove?: string | string[];
}
