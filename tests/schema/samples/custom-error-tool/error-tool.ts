import {
  FieldError,
  IErrorTool,
  IValidationError,
  FieldKey,
} from "../../../../dist";

type vError<Keys> = {
  field: Keys;
  value: unknown;
  message: string;
  metadata: any;
};

type ErrorData<Keys> = { errors: vError<Keys>[] };

export class VError<Keys> implements IErrorTool<ErrorData<Keys>> {
  private _errors = new Map<FieldKey, vError<Keys>>();

  constructor(public message) {}

  get data(): IValidationError<ErrorData<Keys>> {
    return { message: this.message, errors: Array.from(this._errors.values()) };
  }

  get fields() {
    return Array.from(this._errors.keys()) as string[];
  }

  get isLoaded() {
    return this._errors.size > 0;
  }

  set(field: FieldKey, { reason, metadata }: FieldError, value): this {
    let err = this._errors.get(field);

    if (err) err.message = reason;
    else err = { field, value, message: reason } as vError<Keys>;

    if (metadata) err.metadata = metadata;

    this._errors.set(field, err);

    return this;
  }

  setMessage(
    message: "INVALID_DATA" | "NOTHING_TO_UPDATE" | "VALIDATION_ERROR",
  ) {
    this.message = message;

    return this;
  }
}
