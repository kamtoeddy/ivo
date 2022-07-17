import { looseObject } from "../utils/interfaces";

export type fxLooseObject = (...args: any) => Promise<looseObject>;

export interface IValidateResponse {
  reasons?: string[];
  valid: boolean;
  validated?: any;
}

export type PropValidatorFunc = (...args: any) => IValidateResponse;

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

interface IOptionsTimestamp {
  createdAt?: string;
  updatedAt?: string;
}

export interface ITimestamp {
  createdAt: string;
  updatedAt: string;
}

export interface ISchemaOptions {
  timestamp?: boolean | IOptionsTimestamp;
}

export interface Private_ISchemaOptions {
  timestamp: ITimestamp;
}

export interface IExtensionOptions {
  remove?: string | string[];
}

export interface ICloneOptions {
  toReset?: string[];
}

export interface IValidateProps {
  prop: string;
  value: any;
}

export type ModelCreateMethod = () => Promise<looseObject>;

export type ModelCloneMethod = (
  options?: ICloneOptions
) => Promise<looseObject>;

export type ModelValidateMethod = (
  props: IValidateProps
) => Promise<IValidateResponse>;

export type ModelUpdateMethod = (
  changed: Record<string, any>
) => Promise<looseObject>;

export interface IModel {
  create: ModelCreateMethod;
  clone: ModelCloneMethod;
  update: ModelUpdateMethod;
}
