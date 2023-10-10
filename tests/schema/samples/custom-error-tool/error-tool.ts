import {
  FieldError,
  IErrorTool,
  IValidationError,
  FieldKey
} from '../../../../dist';

type vError<Keys> = { field: Keys; messages: string[] };

type ErrorData<Keys> = { errors: vError<Keys>[] };

export class VError<Keys> implements IErrorTool<ErrorData<Keys>> {
  private _errors = new Map<FieldKey, vError<Keys>>();

  constructor(public message) {}

  get data(): IValidationError<ErrorData<Keys>> {
    return { message: this.message, errors: Array.from(this._errors.values()) };
  }

  get error() {
    return new ErrorSummary('VError 😢', this.data.errors);
  }

  add(field: FieldKey, { reasons }: FieldError): this {
    const err = this._errors.get(field);

    if (err) {
      err.messages = [...err.messages, ...reasons];

      this._errors.set(field, err);

      return this;
    }

    this._errors.set(field, { field, messages: reasons } as any);

    return this;
  }

  get isLoaded() {
    return this._errors.size > 0;
  }

  get fields() {
    return Array.from(this._errors.keys()) as string[];
  }

  setMessage(
    message: 'INVALID_DATA' | 'NOTHING_TO_UPDATE' | 'VALIDATION_ERROR'
  ) {
    this.message = message;

    return this;
  }
}

class ErrorSummary extends Error {
  constructor(public message: string, public errors: any) {
    super(message);
  }
}
