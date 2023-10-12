import { isPropertyOf, FieldKey, toArray } from '../../utils';
import { SCHEMA_ERRORS } from './types';

export { SchemaErrorTool, SchemaError };

class SchemaError extends Error {
  constructor(public payload: ErrorPayload) {
    super(SCHEMA_ERRORS.INVALID_SCHEMA);
  }
}

class SchemaErrorTool {
  private _payload: ErrorPayload = {};

  constructor() {}

  get isPayloadLoaded() {
    return Object.keys(this._payload).length > 0;
  }

  private _has = (field: FieldKey) => isPropertyOf(field, this._payload);

  add(field: FieldKey, value?: string | string[]) {
    value = toArray(value ?? []);

    if (this._has(field)) {
      const currentValues = this._payload[field];

      value.forEach((v) => {
        if (!currentValues.includes(v)) currentValues.push(v);
      });

      this._payload[field] = currentValues;
    } else this._payload[field] = value;

    return this;
  }

  throw() {
    throw new SchemaError(this._payload);
  }
}

type ErrorPayload = Record<FieldKey, string[]>;
