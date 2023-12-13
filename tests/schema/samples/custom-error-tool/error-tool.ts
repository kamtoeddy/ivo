import {
  FieldError,
  IErrorTool,
  IValidationError,
  FieldKey,
  VALIDATION_ERRORS
} from '../../../../dist';
import { Ctx_Options } from './types';

type vError<Keys> = {
  field: Keys;
  value: any;
  messages: string[];
  metadata: any;
};

type ErrorData<Keys> = { errors: vError<Keys>[] };

const { VALIDATION_ERROR } = VALIDATION_ERRORS;

export class VError<Keys> implements IErrorTool<ErrorData<Keys>> {
  private _errors = new Map<FieldKey, vError<Keys>>();

  constructor(public message, private ctxOptions: Ctx_Options) {}

  private getVMessage() {
    const { lang } = this.ctxOptions;

    const message =
      this.message == VALIDATION_ERROR ? 'validation' : 'no-update';

    return `${message}${lang ? ' -' + lang : ''}`;
  }

  get data(): IValidationError<ErrorData<Keys>> {
    return { message: this.message, errors: Array.from(this._errors.values()) };
  }

  get error() {
    return new ErrorSummary(
      `VError - ${this.getVMessage()} 😢`,
      this.data.errors
    );
  }

  get fields() {
    return Array.from(this._errors.keys()) as string[];
  }

  get isLoaded() {
    return this._errors.size > 0;
  }

  add(field: FieldKey, { metadata, reasons }: FieldError, value): this {
    let err = this._errors.get(field);

    if (err) err.messages = [...err.messages, ...reasons];
    else err = { field, value, messages: reasons } as vError<Keys>;

    if (metadata) err.metadata = metadata;

    this._errors.set(field, err);

    return this;
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
