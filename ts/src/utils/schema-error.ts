import { toArray } from "../utils";

export { SchemaError, SchemaErrorTool };

class SchemaError extends Error {
  constructor(public payload: ErrorPayload) {
    super("INVALID_SCHEMA");
  }
}

class SchemaErrorTool {
  private _payload: ErrorPayload = {};

  get isPayloadLoaded() {
    return Object.keys(this._payload).length > 0;
  }

  add(fieldName: string, value?: string | string[]) {
    value = toArray(value ?? []);

    if (fieldName in this._payload) {
      const currentValues = this._payload[fieldName];

      value.forEach((v) => {
        if (!currentValues.includes(v)) currentValues.push(v);
      });

      this._payload[fieldName] = currentValues;
    } else this._payload[fieldName] = value;

    return this;
  }

  throw() {
    console.error("\nSchema errors:");

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

type ErrorPayload = Record<string, string[]>;
