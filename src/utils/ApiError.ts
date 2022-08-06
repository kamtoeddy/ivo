import { ApiErrorProps, PayloadType } from "./interfaces";

export class ApiError extends Error {
  name = "ApiError";
  payload: PayloadType;
  statusCode: number;

  constructor({ message, payload = {}, statusCode = 400 }: ApiErrorProps) {
    super(message);
    this.payload = payload;
    this.statusCode = statusCode;
  }

  get isPayloadLoaded() {
    return Object.keys(this.payload).length > 0;
  }

  private _has = (field: string) => this.payload.hasOwnProperty(field);

  add(field: string, value?: string | string[]) {
    if (!value) value = [];

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

  remove = (field: string) => {
    delete this.payload?.[field];
    return this;
  };

  setMessage = (message: string) => {
    this.message = message;
    return this;
  };
}
