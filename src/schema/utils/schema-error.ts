import { isPropertyOn, sortKeys, toArray } from "../../utils/functions";
import {
  ErrorPayload,
  ErrorToolProps,
  InputPayload,
  PayloadKey,
  SchemaErrorProps,
} from "../../utils/interfaces";

export class SchemaError extends Error {
  payload: ErrorPayload = {};
  statusCode: number;

  constructor({ message, payload = {}, statusCode = 400 }: SchemaErrorProps) {
    super(message);
    this.payload = payload;
    this.statusCode = statusCode;
  }
}

export class ErrorTool extends Error {
  payload: ErrorPayload = {};
  statusCode: number;
  private _initMessage: string;
  private _initStatusCode: number;

  constructor({ message, payload = {}, statusCode = 400 }: ErrorToolProps) {
    super(message);
    this._initMessage = message;
    this._initStatusCode = this.statusCode = statusCode;
    this._setPayload(payload);
  }

  get isPayloadLoaded() {
    return Object.keys(this.payload).length > 0;
  }

  get summary() {
    return {
      message: this.message,
      payload: sortKeys(this.payload),
      statusCode: this.statusCode,
    };
  }

  private _has = (field: PayloadKey) => isPropertyOn(field, this.payload);

  private _setPayload = (payload: InputPayload) => {
    Object.entries(payload).forEach(([key, value]) => {
      this.add(key, value);
    });
  };

  add(field: PayloadKey, value?: string | string[]) {
    if (!value) value = [];
    else value = toArray(value);

    if (this._has(field)) {
      const currentValues = this.payload[field];

      value.forEach((v) => {
        if (!currentValues.includes(v)) currentValues.push(v);
      });

      this.payload[field] = currentValues;
    } else this.payload[field] = value;

    return this;
  }

  remove = (field: PayloadKey) => {
    delete this.payload?.[field];
    return this;
  };

  reset = () => {
    this.message = this._initMessage;
    this.payload = {};
    this.statusCode = this._initStatusCode;

    return this;
  };

  setMessage = (message: string) => {
    this.message = message;
    return this;
  };

  throw = () => {
    const summary = this.summary;
    this.reset();

    throw new SchemaError(summary);
  };
}
