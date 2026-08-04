import { sortKeys } from "../utils";
import type { IvoErrorPayload } from "./types";
import type { FieldError } from "./types";

export { ErrorTool };

class ErrorTool<Metadata, PayloadKeys extends string = string> {
  private _payload: IvoErrorPayload<Metadata, PayloadKeys> = {} as never;

  set(field: PayloadKeys, value: FieldError<Metadata>) {
    this._payload[field] = value;
  }

  get fields() {
    return Object.keys(this._payload);
  }

  get hasErrors() {
    return Object.keys(this._payload).length > 0;
  }

  get payload() {
    return sortKeys(this._payload);
  }
}
