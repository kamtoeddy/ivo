export interface ApiErrorProps {
  message: string;
  payload?: PayloadType;
  statusCode?: number;
}

export interface IArrayOptions<T> {
  empty?: boolean;
  filter?: (data: T) => boolean;
  modifier?: (data: T) => any;
  sorted?: boolean;
  sorter?: (a: T, b: T) => number;
  sortOrder?: number;
  unique?: boolean;
  uniqueKey?: string;
}

export type PayloadType = Record<number | string, string[]>;

export type ObjectType = Record<number | string, any>;

export interface IStringOptions {
  enums?: string[];
  maxLength?: number;
  minLength?: number;
  regExp?: RegExp;
}
