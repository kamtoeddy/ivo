import { ILooseObject } from "../utils/interfaces";

export type fxLooseObject = (
  ...args: any
) => ILooseObject | Promise<ILooseObject>;

export interface ValidatorResponse {
  reasons?: string[];
  valid: boolean;
  validated?: any;
}

export type Validator = (
  ...args: any
) => ValidatorResponse | Promise<ValidatorResponse>;

export type NonEmptyArray<T> = [T, ...T[]];

export interface PropDefinitionRules {
  [key: string]: {
    default?: any;
    dependent?: boolean;
    onChange?: NonEmptyArray<fxLooseObject>;
    onCreate?: NonEmptyArray<fxLooseObject>;
    onUpdate?: NonEmptyArray<fxLooseObject>;
    readonly?: boolean;
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

export interface SchemaExtensionOptions {
  remove?: string | string[];
}

export interface ICloneOptions {
  reset?: string | string[];
}
