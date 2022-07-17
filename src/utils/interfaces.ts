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

export interface looseObject {
  [key: string]: any;
}

export interface stringPropTypes {
  enums?: string[];
  match?: RegExp;
  maxLength?: number;
  minLength?: number;
}
