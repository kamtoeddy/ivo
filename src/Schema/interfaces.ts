import { ILooseObject } from "../utils/interfaces";

export type fxLooseObject = (
  ...args: any
) => ILooseObject | Promise<ILooseObject>;

export interface IValidateResponse {
  reasons?: string[];
  valid: boolean;
  validated?: any;
}

export type PropValidatorFunc = (
  ...args: any
) => IValidateResponse | Promise<IValidateResponse>;

export interface PropDefinitionRules {
  [key: string]: {
    default?: any;
    dependent?: boolean;
    onCreate?: fxLooseObject[];
    onUpdate?: fxLooseObject[];
    readonly?: boolean;
    required?: boolean;
    sideEffect?: boolean;
    shouldInit?: boolean;
    validator?: PropValidatorFunc;
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
) => Promise<IValidateResponse>;

export type ModelUpdateMethod = (
  changed: Record<string, any>
) => Promise<ILooseObject>;

// export interface IModel {
//   create<>: ModelCreateMethod<T>;
//   clone: ModelCloneMethod;
//   update: ModelUpdateMethod;
// }
