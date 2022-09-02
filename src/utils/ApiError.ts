import { sortKeys, toArray } from "./functions";
import {
  ApiErrorProps,
  ErrorPayload,
  InputPayload,
  PayloadKey,
} from "./interfaces";

export class ApiError extends Error {
  name = "ApiError";
  payload: ErrorPayload = {};
  statusCode: number;

  constructor({ message, payload = {}, statusCode = 400 }: ApiErrorProps) {
    super(message);
    this._setPayload(payload);
    this.statusCode = statusCode;
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

  private _has = (field: PayloadKey) => this.payload.hasOwnProperty(field);

  private _setPayload = (payload: InputPayload) => {
    Object.entries(payload).forEach(([key, value]) => {
      this.add(key, value);
    });

    this.payload = sortKeys(this.payload);
  };

  add(field: PayloadKey, value?: string | string[]) {
    if (value) {
      value = toArray(value);

      this.payload[field] = this._has(field)
        ? [...this.payload[field], ...value]
        : value;
    }

    return this;
  }

  clear = () => {
    this.payload = {};
    return this;
  };

  remove = (field: PayloadKey) => {
    delete this.payload?.[field];
    return this;
  };

  setMessage = (message: string) => {
    this.message = message;
    return this;
  };
}
