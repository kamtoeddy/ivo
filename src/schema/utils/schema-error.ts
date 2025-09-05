import { type FieldKey, toArray } from '../../utils';
import { SCHEMA_ERRORS } from './types';

export { SchemaErrorTool, SchemaError };

class SchemaError extends Error {
  constructor(public payload: ErrorPayload) {
    super(SCHEMA_ERRORS.INVALID_SCHEMA);
  }
}

class SchemaErrorTool {
  private _payload: ErrorPayload = {};

  get isPayloadLoaded() {
    return Object.keys(this._payload).length > 0;
  }

  add(field: FieldKey, value?: string | string[]) {
    value = toArray(value ?? []);

    if (field in this._payload) {
      const currentValues = this._payload[field];

      value.forEach((v) => {
        if (!currentValues.includes(v)) currentValues.push(v);
      });

      this._payload[field] = currentValues;
    } else this._payload[field] = value;

    return this;
  }

  throw() {
    console.error('\nSchema errors:');

    for (const [prop, messages] of Object.entries(this._payload)) {
      if (messages.length === 1) {
        console.error(`  [${prop}]: ${messages[0]}`);
        continue;
      }

      console.error(`  [${prop}]:`);
      messages.map((m, i) => console.error(`    ${i + 1}) ${m}`));
    }

    throw new SchemaError(this._payload);
  }
}

type ErrorPayload = Record<FieldKey, string[]>;
