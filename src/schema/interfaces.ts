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
  | "onCreate"
  | "onUpdate"
  | "readonly"
  | "required"
  | "sideEffect"
  | "shouldInit"
  | "validator";

export type LifeCycleRule = "onCreate" | "onUpdate";

interface IOptionsTimestamp {
  createdAt?: string;
  updatedAt?: string;
}

export interface ITimestamp {
  createdAt: string;
  updatedAt: string;
}

export interface ISchemaOptions {
  timestamps?: boolean | IOptionsTimestamp;
}

export interface Private_ISchemaOptions {
  timestamps: ITimestamp;
}

export interface IExtensionOptions {
  remove?: string | string[];
}

export interface ICloneOptions {
  reset?: string | string[];
}

export interface IValidateProps {
  prop: string;
  value: any;
}

export type ModelCreateMethod<T extends ILooseObject> = () => Promise<T>;

export type ModelCloneMethod = (
  options?: ICloneOptions
) => Promise<ILooseObject>;

export type ModelValidateMethod = (
  props: IValidateProps
) => Promise<ValidatorResponse>;

export type ModelUpdateMethod = (
  changed: Record<string, any>
) => Promise<ILooseObject>;

// export interface IModel {
//   create<>: ModelCreateMethod<T>;
//   clone: ModelCloneMethod;
//   update: ModelUpdateMethod;
// }
