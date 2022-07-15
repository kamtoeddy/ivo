import { ApiErrorProps, ErrorPayload } from "./interfaces";

export class ApiError extends Error {
  name = "ApiError";
  payload: ErrorPayload;
  statusCode: number;

  constructor({ message, payload = {}, statusCode = 400 }: ApiErrorProps) {
    super(message);
    this.payload = payload;
    this.statusCode = statusCode;
  }

  private _has = (field: string) => this.payload.hasOwnProperty(field);

  add(field: string, value: string | string[]) {
    value = Array.isArray(value) ? [...value] : [value];

    this.payload[field] = this._has(field)
      ? [...this.payload[field], ...value]
      : value;

    return this;
  }

  clear = () => {
    this.payload = {};
    return this;
  };

  getInfo = () => {
    return {
      _isError: true,
      message: this.message,
      payload: this.payload,
      statusCode: this.statusCode,
    };
  };

  isPayloadLoaded = () => Object.keys(this.payload).length > 0;

  remove = (field: string) => {
    delete this.payload?.[field];
    return this;
  };

  setMessage = (message: string) => {
    this.message = message;
    return this;
  };
}
