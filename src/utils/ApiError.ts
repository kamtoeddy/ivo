import { ApiErrorProps, ErrorPayload } from "./interfaces";

export class ApiError extends Error {
  _isError = true;
  name = "ApiError";
  payload: ErrorPayload;
  statusCode: number;

  constructor({ message, payload = {}, statusCode = 400 }: ApiErrorProps) {
    super(message);
    this.payload = payload;
    this.statusCode = statusCode;
  }

  add(field: string, value: string | string[]) {
    const toAdd = Array.isArray(value) ? [...value] : [value];

    if (!this.payload[field]) return (this.payload[field] = toAdd);

    this.payload[field] = [...this.payload[field], ...toAdd];
  }

  clear = () => (this.payload = {});

  remove = (field: string) => delete this.payload?.[field];

  setMessage = (message: string) => (this.message = message);
}
