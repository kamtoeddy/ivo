export interface looseObject {
  [key: string]: any;
}

export interface datePropTypes {
  max?: number;
  min?: number;
  enums?: Date[];
}

export interface stringPropTypes {
  maxLength?: number;
  minLength?: number;
  enums?: string[];
}
