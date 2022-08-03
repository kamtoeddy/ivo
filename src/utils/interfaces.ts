export interface ApiErrorProps {
  message: string;
  payload?: ErrorPayload;
  statusCode?: number;
}

export interface ErrorPayload {
  [key: string]: string[];
}

export interface datePropTypes {
  max?: number;
  min?: number;
  enums?: Date[];
}

export type ILooseObject = Record<string, any>;

export interface IStringOptions {
  enums?: string[];
  maxLength?: number;
  minLength?: number;
  regExp?: RegExp;
}
